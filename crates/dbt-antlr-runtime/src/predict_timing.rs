//! Optional timing of ALL(*) prediction: how much of a parse is spent deciding
//! which alternative to take, and how much of that is stage-2 full-LL.
//!
//! Off unless the `perf-counters` feature is on, and off then too unless
//! `ANTLR_PREDICT_CLOCK` asks for a clock. With the feature off this module
//! compiles to nothing: [`Span::start`] returns a zero-sized value, charging it
//! is an empty function, and the accumulators on the simulator manager are
//! `()`.
//!
//! # Choosing a clock
//!
//! `ANTLR_PREDICT_CLOCK` takes:
//!
//! | value | clock | cost per read | measures |
//! |-------|-------|---------------|----------|
//! | unset, `off` | none | 0 | nothing; prediction timing is not collected |
//! | `tsc` | `rdtsc` | ~7ns | wall cycles |
//! | `cpu` | `clock_gettime(CLOCK_THREAD_CPUTIME_ID)` | ~101ns | this thread's CPU time |
//!
//! Neither clock settles a question on its own, which is why both are offered.
//! `cpu` measures the quantity a harness reports and can attribute across
//! threads, but at ~101ns a read it perturbs what it measures: 17 million calls
//! is 3.4 seconds of clock. `tsc` is 14x cheaper but counts wall cycles, so it
//! also counts preemption and says nothing useful under contention. Run both:
//! if they agree, neither objection stands. Measured on one corpus they agreed
//! to within 1.5 percentage points on prediction's share of a parse.
//!
//! Whichever clock is chosen, the reads are inside the interval being measured,
//! so a caller reporting these numbers has to correct for them --
//! [`empty_delta_ns`] and [`probe_pair_ns`] are the two constants that takes,
//! and they are not the same number. See their docs.
//!
//! `tsc` is x86-64 only. Elsewhere it falls back to `cpu`.

#[cfg(feature = "perf-counters")]
mod enabled {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::LazyLock;

    #[repr(C)]
    struct Timespec {
        sec: i64,
        nsec: i64,
    }

    extern "C" {
        fn clock_gettime(id: i32, ts: *mut Timespec) -> i32;
    }

    // Linux/BSD values; the two clocks this module can use.
    const CLOCK_MONOTONIC: i32 = 1;
    const CLOCK_THREAD_CPUTIME_ID: i32 = 3;

    fn clock_ns(id: i32) -> u64 {
        let mut ts = Timespec { sec: 0, nsec: 0 };
        // SAFETY: writes a `timespec` through the pointer and reads nothing
        // else; both clock ids are constants every supported target has.
        unsafe { clock_gettime(id, &mut ts) };
        ts.sec as u64 * 1_000_000_000 + ts.nsec as u64
    }

    #[cfg(target_arch = "x86_64")]
    #[inline(always)]
    fn tsc() -> u64 {
        // SAFETY: `rdtsc` is unprivileged and has no side effects.
        unsafe { core::arch::x86_64::_rdtsc() }
    }

    #[cfg(not(target_arch = "x86_64"))]
    #[inline(always)]
    fn tsc() -> u64 {
        clock_ns(CLOCK_THREAD_CPUTIME_ID)
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum Clock {
        /// Collect nothing. The reads are skipped, not merely discarded.
        Off,
        Tsc,
        Cpu,
    }

    /// Read once, on first use, because it decides what happens on a path taken
    /// tens of millions of times and must not be re-read there.
    static CLOCK: LazyLock<Clock> = LazyLock::new(|| {
        let requested = std::env::var("ANTLR_PREDICT_CLOCK").unwrap_or_default();
        match requested.as_str() {
            // `tsc` on a target without one is a request for the cheapest
            // available clock, not an error; `cpu` is what is left.
            "tsc" if cfg!(target_arch = "x86_64") => Clock::Tsc,
            "tsc" | "cpu" => Clock::Cpu,
            _ => Clock::Off,
        }
    });

    /// TSC ticks per second, against `CLOCK_MONOTONIC` over 50ms.
    ///
    /// Only meaningful with an invariant TSC, which every x86-64 part this is
    /// likely to run on has. It is measured rather than read from the CPU
    /// because the nominal frequency is not the tick rate on all parts.
    static TSC_HZ: LazyLock<f64> = LazyLock::new(|| {
        let (t0, c0) = (clock_ns(CLOCK_MONOTONIC), tsc());
        std::thread::sleep(std::time::Duration::from_millis(50));
        let (t1, c1) = (clock_ns(CLOCK_MONOTONIC), tsc());
        (c1 - c0) as f64 * 1e9 / (t1 - t0) as f64
    });

    /// Where a timed region's total accumulates: one of these per manager per
    /// measured region, shared by every simulator the manager hands out.
    pub type Accumulator = AtomicU64;

    /// A borrowed [`Accumulator`], as the simulator holds it.
    pub type Sink = &'static AtomicU64;

    pub fn new_accumulator() -> &'static Accumulator {
        Box::leak(Box::new(AtomicU64::new(0)))
    }

    /// Borrow a manager's accumulator for the simulators it hands out.
    #[inline(always)]
    pub fn sink(accumulator: &'static Accumulator) -> Sink {
        accumulator
    }

    /// A measurement in progress: the clock reading a region started at.
    #[derive(Debug)]
    pub struct Span(u64);

    impl Span {
        #[inline(always)]
        pub fn start() -> Self {
            Span(now())
        }

        /// Add what has elapsed to `sink`.
        ///
        /// `wrapping_sub` because the TSC is 64-bit and monotone but nothing
        /// here is worth a panic if a clock ever misbehaves.
        #[inline(always)]
        pub fn charge(self, sink: Sink) {
            // Returning early rather than adding zero: with the feature compiled
            // in but no clock asked for, this leaves the hot path without even
            // the atomic.
            if *CLOCK == Clock::Off {
                return;
            }
            sink.fetch_add(now().wrapping_sub(self.0), Ordering::Relaxed);
        }
    }

    #[inline(always)]
    fn now() -> u64 {
        match *CLOCK {
            Clock::Off => 0,
            Clock::Tsc => tsc(),
            Clock::Cpu => clock_ns(CLOCK_THREAD_CPUTIME_ID),
        }
    }

    pub fn read_ns(sink: &Accumulator) -> u64 {
        to_ns(sink.load(Ordering::Relaxed))
    }

    /// A count of the selected clock's units as nanoseconds.
    ///
    /// Applied to a total rather than to each reading: the conversion is linear,
    /// so converting once at the end costs nothing and rounds once.
    pub fn to_ns(units: u64) -> u64 {
        match *CLOCK {
            Clock::Off => 0,
            Clock::Tsc => (units as f64 * 1e9 / *TSC_HZ) as u64,
            Clock::Cpu => units,
        }
    }

    pub fn clock() -> Clock {
        *CLOCK
    }

    pub fn clock_name() -> &'static str {
        match *CLOCK {
            Clock::Off => "off",
            Clock::Tsc => "tsc",
            Clock::Cpu => "cpu",
        }
    }

    /// Nanoseconds an *empty* timed region reports: `Span::start()` immediately
    /// charged, with nothing in between.
    ///
    /// This is what to subtract, per timed region, from the totals this module
    /// produces -- the region's own two reads are inside it. It is not the same
    /// as [`probe_pair_ns`], and using one where the other belongs is the
    /// easiest way to get these numbers wrong.
    pub fn empty_delta_ns() -> f64 {
        if *CLOCK == Clock::Off {
            return 0.0;
        }
        const N: u64 = 200_000;
        let sink = new_accumulator();
        for _ in 0..N {
            // Charging a leaked accumulator would be the same measurement; this
            // borrows the local one for exactly as long as the loop.
            let start = now();
            sink.fetch_add(now().wrapping_sub(start), Ordering::Relaxed);
        }
        to_ns(sink.load(Ordering::Relaxed)) as f64 / N as f64
    }

    /// Nanoseconds a pair of reads *costs*, wall clock.
    ///
    /// This is what to subtract from a total measured by some *outer* clock that
    /// contains the timed regions -- a parse time, say, which paid for every
    /// read inside it. Larger than [`empty_delta_ns`], because a read's cost
    /// lands partly outside the interval it delimits.
    pub fn probe_pair_ns() -> f64 {
        if *CLOCK == Clock::Off {
            return 0.0;
        }
        const N: u64 = 200_000;
        let _ = now(); // fault in the lazies before timing anything
        let start = clock_ns(CLOCK_MONOTONIC);
        let mut sink = 0u64;
        for _ in 0..N {
            let a = now();
            let b = now();
            sink = sink.wrapping_add(b.wrapping_sub(a));
        }
        let elapsed = clock_ns(CLOCK_MONOTONIC) - start;
        std::hint::black_box(sink);
        elapsed as f64 / N as f64
    }
}

#[cfg(not(feature = "perf-counters"))]
mod enabled {
    //! The same surface with nothing behind it. Every function is
    //! `#[inline(always)]` and empty, and [`Accumulator`] is zero-sized, so a
    //! build without the feature has no clock reads, no atomics and no bytes on
    //! the simulator manager.

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum Clock {
        Off,
    }

    /// Named zero-sized types rather than `()`: passing a unit value to a
    /// function is a clippy lint, and the call sites should not have to know
    /// which build they are in.
    #[derive(Clone, Copy, Debug)]
    pub struct Accumulator;

    #[derive(Clone, Copy, Debug)]
    pub struct Sink;

    #[inline(always)]
    pub fn new_accumulator() -> &'static Accumulator {
        static ACCUMULATOR: Accumulator = Accumulator;
        &ACCUMULATOR
    }

    #[inline(always)]
    pub fn sink(_accumulator: &'static Accumulator) -> Sink {
        Sink
    }

    #[derive(Debug)]
    pub struct Span;

    impl Span {
        #[inline(always)]
        pub fn start() -> Self {
            Span
        }

        #[inline(always)]
        pub fn charge(self, _sink: Sink) {}
    }

    #[inline(always)]
    pub fn read_ns(_sink: &Accumulator) -> u64 {
        0
    }

    #[inline(always)]
    pub fn to_ns(_units: u64) -> u64 {
        0
    }

    #[inline(always)]
    pub fn clock() -> Clock {
        Clock::Off
    }

    #[inline(always)]
    pub fn clock_name() -> &'static str {
        "off"
    }

    #[inline(always)]
    pub fn empty_delta_ns() -> f64 {
        0.0
    }

    #[inline(always)]
    pub fn probe_pair_ns() -> f64 {
        0.0
    }
}

pub use enabled::*;

/// Whether prediction timing is being collected: the feature is compiled in and
/// a clock was asked for.
///
/// A `const false` in a build without the feature, so a caller's reporting
/// branch folds away with it.
#[inline(always)]
pub fn collecting() -> bool {
    cfg!(feature = "perf-counters") && !matches!(clock(), Clock::Off)
}
