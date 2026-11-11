use super::*;

#[test]
fn test_add_1() {
    let mut set = IntervalSetBuf::new();
    set.add_range(1, 2);
    assert_eq!(set.as_slice(), &[Interval { a: 1, b: 2 }]);
    set.add_range(2, 3);
    assert_eq!(set.as_slice(), &[Interval { a: 1, b: 3 }]);
    set.add_range(1, 5);
    assert_eq!(set.as_slice(), &[Interval { a: 1, b: 5 }]);
}

#[test]
fn test_add_2() {
    let mut set = IntervalSetBuf::new();
    set.add_range(1, 3);
    set.add_range(5, 6);
    assert_eq!(
        set.as_slice(),
        &[Interval { a: 1, b: 3 }, Interval { a: 5, b: 6 }]
    );
    set.add_range(3, 4);
    assert_eq!(set.as_slice(), &[Interval { a: 1, b: 6 }]);
}

#[test]
fn test_remove() {
    let mut set = IntervalSetBuf::new();
    set.add_range(1, 5);
    set.remove_one(3);
    assert_eq!(
        set.as_slice(),
        &[Interval { a: 1, b: 2 }, Interval { a: 4, b: 5 }]
    );
}

#[test]
fn test_substract() {
    let mut set1 = IntervalSetBuf::new();
    set1.add_range(1, 2);
    set1.add_range(4, 5);
    let mut set2 = IntervalSetBuf::new();
    set2.add_range(2, 4);
    set1.substract(&set2);
    assert_eq!(
        set1.as_slice(),
        &[Interval { a: 1, b: 1 }, Interval { a: 5, b: 5 }]
    );
}

#[test]
fn test_add_merges_many_overlapped() {
    let mut set = IntervalSetBuf::new();
    set.add_range(1, 2);
    set.add_range(4, 5);
    set.add_range(7, 8);
    set.add_range(10, 11);
    set.add_range(2, 10);
    assert_eq!(set.as_slice(), &[Interval { a: 1, b: 11 }]);
}

#[test]
fn test_add_matches_reference_set() {
    let mut seed: u64 = 0x2545_f491_4f6c_dd1d;
    let mut next = move |bound: i32| {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed % bound as u64) as i32
    };
    for _ in 0..500 {
        let mut set = IntervalSetBuf::new();
        let mut reference = std::collections::BTreeSet::new();
        for _ in 0..next(12) {
            let a = next(60);
            let width = if next(4) == 0 { 40 } else { 4 };
            let b = a + next(width);
            set.add_range(a, b);
            reference.extend(a..=b);
        }
        for w in set.as_slice().windows(2) {
            assert!(w[0].b + 1 < w[1].a, "not normalized: {:?}", set.as_slice());
        }
        for item in -1..70 {
            assert_eq!(set.contains(item), reference.contains(&item), "item {item}");
        }
    }
}

#[test]
fn test_contains_after_multi_merge() {
    let mut set = IntervalSetBuf::new();
    set.add_range(11, 20);
    set.add_range(30, 30);
    set.add_range(62, 62);
    set.add_range(80, 80);
    set.add_range(15, 63);
    assert_eq!(
        set.as_slice(),
        &[Interval { a: 11, b: 63 }, Interval { a: 80, b: 80 }]
    );
    assert!(set.contains(63));
}
