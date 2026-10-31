//! Merge tests ported from `TestGraphNodes` of the Java ANTLR tool test suite.
//!
//! The expected DOT graphs are the Java ones, unchanged. A return state
//! number `n` is the ATN state with number `n`.
#![allow(non_snake_case)]

use std::collections::{HashMap, VecDeque};
use std::pin::Pin;

use super::*;
use crate::atn_state::{ATNState, BaseATNState, BasicState, ATNSTATE_BASIC};
use crate::parser_atn_simulator::MergeCache;

fn root_is_wildcard() -> bool {
    true
}

fn full_ctx() -> bool {
    false
}

struct Graph {
    states: Vec<Pin<Box<ATNState>>>,
    scratch: bumpalo::Bump,
}

impl Graph {
    fn new() -> Self {
        let states = (0..=10)
            .map(|number| {
                Box::pin(BasicState::create(BaseATNState::new(
                    number,
                    0,
                    ATNSTATE_BASIC,
                )))
            })
            .collect();
        Self {
            states,
            scratch: bumpalo::Bump::new(),
        }
    }

    fn empty(&self) -> PredictionContextRef<'_> {
        PredictionContextRef::new_empty()
    }

    fn singleton<'a>(
        &'a self,
        parent: PredictionContextRef<'a>,
        payload: i32,
    ) -> PredictionContextRef<'a> {
        let return_state = ATNStateRef::from(&self.states[payload as usize]);
        PredictionContextRef::from(
            &*self
                .scratch
                .alloc(PredictionContext::new_singleton(Some(parent), return_state)),
        )
    }

    fn array<'a>(&'a self, nodes: &[PredictionContextRef<'a>]) -> PredictionContextRef<'a> {
        let parents = self
            .scratch
            .alloc_slice_fill_iter(nodes.iter().map(|node| node.get_parent(0)));
        let return_states = self
            .scratch
            .alloc_slice_fill_iter(nodes.iter().map(|node| node.get_return_state(0)));
        PredictionContextRef::from(
            &*self
                .scratch
                .alloc(PredictionContext::new_array(parents, return_states)),
        )
    }

    fn merge<'a>(
        &'a self,
        a: PredictionContextRef<'a>,
        b: PredictionContextRef<'a>,
        root_is_wildcard: bool,
    ) -> PredictionContextRef<'a> {
        let mut cache = MergeCache::new(&self.scratch);
        PredictionContext::merge(a, b, root_is_wildcard, &mut cache)
    }

    fn a(&self) -> PredictionContextRef<'_> {
        self.singleton(self.empty(), 1)
    }

    fn b(&self) -> PredictionContextRef<'_> {
        self.singleton(self.empty(), 2)
    }

    fn c(&self) -> PredictionContextRef<'_> {
        self.singleton(self.empty(), 3)
    }

    fn d(&self) -> PredictionContextRef<'_> {
        self.singleton(self.empty(), 4)
    }

    fn u(&self) -> PredictionContextRef<'_> {
        self.singleton(self.empty(), 6)
    }

    fn v(&self) -> PredictionContextRef<'_> {
        self.singleton(self.empty(), 7)
    }

    fn w(&self) -> PredictionContextRef<'_> {
        self.singleton(self.empty(), 8)
    }

    fn x(&self) -> PredictionContextRef<'_> {
        self.singleton(self.empty(), 9)
    }

    fn y(&self) -> PredictionContextRef<'_> {
        self.singleton(self.empty(), 10)
    }
}

fn to_dot_string(context: PredictionContextRef<'_>, root_is_wildcard: bool) -> String {
    fn identity(context: PredictionContextRef<'_>) -> *const () {
        context.as_ref() as *const PredictionContext<'_> as *const ()
    }

    let mut nodes = String::new();
    let mut edges = String::new();
    let mut context_ids = HashMap::new();
    let mut work_list = VecDeque::new();
    context_ids.insert(identity(context), 0);
    work_list.push_back(context);
    while let Some(current) = work_list.pop_front() {
        let current_id = context_ids[&identity(current)];
        nodes.push_str(&format!("  s{current_id}["));
        if current.length() > 1 {
            nodes.push_str("shape=record, ");
        }
        nodes.push_str("label=\"");
        let root = if root_is_wildcard { '*' } else { '$' };
        if current.is_empty() {
            nodes.push(root);
        } else if current.length() > 1 {
            for i in 0..current.length() {
                if i > 0 {
                    nodes.push('|');
                }
                nodes.push_str(&format!("<p{i}>"));
                if current.get_return_state(i) == ATNStateRef::empty_return() {
                    nodes.push(root);
                }
            }
        } else {
            nodes.push_str(&current_id.to_string());
        }
        nodes.push_str("\"];\n");

        if current.is_empty() {
            continue;
        }
        for i in 0..current.length() {
            if current.get_return_state(i) == ATNStateRef::empty_return() {
                continue;
            }
            let parent = current.get_parent(i).expect("a return state has a parent");
            let next_id = context_ids.len();
            let parent_id = *context_ids.entry(identity(parent)).or_insert_with(|| {
                work_list.push_front(parent);
                next_id
            });
            edges.push_str(&format!("  s{current_id}"));
            if current.length() > 1 {
                edges.push_str(&format!(":p{i}"));
            }
            edges.push_str(&format!(
                "->s{parent_id}[label=\"{}\"];\n",
                current.get_return_state(i).get_state_number()
            ));
        }
    }
    format!("digraph G {{\nrankdir=LR;\n{nodes}{edges}}}\n")
}

#[test]
fn test_e_e() {
    let g = Graph::new();
    let r = g.merge(g.empty(), g.empty(), root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[label=\"*\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_e_e_fullctx() {
    let g = Graph::new();
    let r = g.merge(g.empty(), g.empty(), full_ctx());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[label=\"$\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, full_ctx()));
}

#[test]
fn test_x_e() {
    let g = Graph::new();
    let r = g.merge(g.x(), g.empty(), root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[label=\"*\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_x_e_fullctx() {
    let g = Graph::new();
    let r = g.merge(g.x(), g.empty(), full_ctx());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[shape=record, label=\"<p0>|<p1>$\"];\n",
        "  s1[label=\"$\"];\n",
        "  s0:p0->s1[label=\"9\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, full_ctx()));
}

#[test]
fn test_e_x() {
    let g = Graph::new();
    let r = g.merge(g.empty(), g.x(), root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[label=\"*\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_e_x_fullctx() {
    let g = Graph::new();
    let r = g.merge(g.empty(), g.x(), full_ctx());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[shape=record, label=\"<p0>|<p1>$\"];\n",
        "  s1[label=\"$\"];\n",
        "  s0:p0->s1[label=\"9\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, full_ctx()));
}

#[test]
fn test_a_a() {
    let g = Graph::new();
    let r = g.merge(g.a(), g.a(), root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[label=\"0\"];\n",
        "  s1[label=\"*\"];\n",
        "  s0->s1[label=\"1\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_ae_ax() {
    let g = Graph::new();
    let a1 = g.a();
    let x = g.x();
    let a2 = g.singleton(x, 1);
    let r = g.merge(a1, a2, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[label=\"0\"];\n",
        "  s1[label=\"*\"];\n",
        "  s0->s1[label=\"1\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_ae_ax_fullctx() {
    let g = Graph::new();
    let a1 = g.a();
    let x = g.x();
    let a2 = g.singleton(x, 1);
    let r = g.merge(a1, a2, full_ctx());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[label=\"0\"];\n",
        "  s1[shape=record, label=\"<p0>|<p1>$\"];\n",
        "  s2[label=\"$\"];\n",
        "  s0->s1[label=\"1\"];\n",
        "  s1:p0->s2[label=\"9\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, full_ctx()));
}

#[test]
fn test_axe_ae() {
    let g = Graph::new();
    let x = g.x();
    let a1 = g.singleton(x, 1);
    let a2 = g.a();
    let r = g.merge(a1, a2, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[label=\"0\"];\n",
        "  s1[label=\"*\"];\n",
        "  s0->s1[label=\"1\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_aae_ae_e_fullCtx() {
    let g = Graph::new();
    let empty = g.empty();
    let child1 = g.singleton(empty, 8);
    let right = g.merge(empty, child1, false);
    let left = g.singleton(right, 8);
    let merged = g.merge(left, right, false);
    let actual = to_dot_string(merged, false);
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[shape=record, label=\"<p0>|<p1>$\"];\n",
        "  s1[shape=record, label=\"<p0>|<p1>$\"];\n",
        "  s2[label=\"$\"];\n",
        "  s0:p0->s1[label=\"8\"];\n",
        "  s1:p0->s2[label=\"8\"];\n",
        "}\n",
    );
    assert_eq!(expecting, actual);
}

#[test]
fn test_axe_ae_fullctx() {
    let g = Graph::new();
    let x = g.x();
    let a1 = g.singleton(x, 1);
    let a2 = g.a();
    let r = g.merge(a1, a2, full_ctx());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[label=\"0\"];\n",
        "  s1[shape=record, label=\"<p0>|<p1>$\"];\n",
        "  s2[label=\"$\"];\n",
        "  s0->s1[label=\"1\"];\n",
        "  s1:p0->s2[label=\"9\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, full_ctx()));
}

#[test]
fn test_a_b() {
    let g = Graph::new();
    let r = g.merge(g.a(), g.b(), root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[shape=record, label=\"<p0>|<p1>\"];\n",
        "  s1[label=\"*\"];\n",
        "  s0:p0->s1[label=\"1\"];\n",
        "  s0:p1->s1[label=\"2\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_ax_ax_same() {
    let g = Graph::new();
    let x = g.x();
    let a1 = g.singleton(x, 1);
    let a2 = g.singleton(x, 1);
    let r = g.merge(a1, a2, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[label=\"0\"];\n",
        "  s1[label=\"1\"];\n",
        "  s2[label=\"*\"];\n",
        "  s0->s1[label=\"1\"];\n",
        "  s1->s2[label=\"9\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_ax_ax() {
    let g = Graph::new();
    let x1 = g.x();
    let x2 = g.x();
    let a1 = g.singleton(x1, 1);
    let a2 = g.singleton(x2, 1);
    let r = g.merge(a1, a2, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[label=\"0\"];\n",
        "  s1[label=\"1\"];\n",
        "  s2[label=\"*\"];\n",
        "  s0->s1[label=\"1\"];\n",
        "  s1->s2[label=\"9\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_abx_abx() {
    let g = Graph::new();
    let x1 = g.x();
    let x2 = g.x();
    let b1 = g.singleton(x1, 2);
    let b2 = g.singleton(x2, 2);
    let a1 = g.singleton(b1, 1);
    let a2 = g.singleton(b2, 1);
    let r = g.merge(a1, a2, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[label=\"0\"];\n",
        "  s1[label=\"1\"];\n",
        "  s2[label=\"2\"];\n",
        "  s3[label=\"*\"];\n",
        "  s0->s1[label=\"1\"];\n",
        "  s1->s2[label=\"2\"];\n",
        "  s2->s3[label=\"9\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_abx_acx() {
    let g = Graph::new();
    let x1 = g.x();
    let x2 = g.x();
    let b = g.singleton(x1, 2);
    let c = g.singleton(x2, 3);
    let a1 = g.singleton(b, 1);
    let a2 = g.singleton(c, 1);
    let r = g.merge(a1, a2, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[label=\"0\"];\n",
        "  s1[shape=record, label=\"<p0>|<p1>\"];\n",
        "  s2[label=\"2\"];\n",
        "  s3[label=\"*\"];\n",
        "  s0->s1[label=\"1\"];\n",
        "  s1:p0->s2[label=\"2\"];\n",
        "  s1:p1->s2[label=\"3\"];\n",
        "  s2->s3[label=\"9\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_ax_bx_same() {
    let g = Graph::new();
    let x = g.x();
    let a = g.singleton(x, 1);
    let b = g.singleton(x, 2);
    let r = g.merge(a, b, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[shape=record, label=\"<p0>|<p1>\"];\n",
        "  s1[label=\"1\"];\n",
        "  s2[label=\"*\"];\n",
        "  s0:p0->s1[label=\"1\"];\n",
        "  s0:p1->s1[label=\"2\"];\n",
        "  s1->s2[label=\"9\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_ax_bx() {
    let g = Graph::new();
    let x1 = g.x();
    let x2 = g.x();
    let a = g.singleton(x1, 1);
    let b = g.singleton(x2, 2);
    let r = g.merge(a, b, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[shape=record, label=\"<p0>|<p1>\"];\n",
        "  s1[label=\"1\"];\n",
        "  s2[label=\"*\"];\n",
        "  s0:p0->s1[label=\"1\"];\n",
        "  s0:p1->s1[label=\"2\"];\n",
        "  s1->s2[label=\"9\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_ax_by() {
    let g = Graph::new();
    let a = g.singleton(g.x(), 1);
    let b = g.singleton(g.y(), 2);
    let r = g.merge(a, b, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[shape=record, label=\"<p0>|<p1>\"];\n",
        "  s2[label=\"2\"];\n",
        "  s3[label=\"*\"];\n",
        "  s1[label=\"1\"];\n",
        "  s0:p0->s1[label=\"1\"];\n",
        "  s0:p1->s2[label=\"2\"];\n",
        "  s2->s3[label=\"10\"];\n",
        "  s1->s3[label=\"9\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_ae_bx() {
    let g = Graph::new();
    let x2 = g.x();
    let a = g.a();
    let b = g.singleton(x2, 2);
    let r = g.merge(a, b, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[shape=record, label=\"<p0>|<p1>\"];\n",
        "  s2[label=\"2\"];\n",
        "  s1[label=\"*\"];\n",
        "  s0:p0->s1[label=\"1\"];\n",
        "  s0:p1->s2[label=\"2\"];\n",
        "  s2->s1[label=\"9\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_ae_bx_fullctx() {
    let g = Graph::new();
    let x2 = g.x();
    let a = g.a();
    let b = g.singleton(x2, 2);
    let r = g.merge(a, b, full_ctx());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[shape=record, label=\"<p0>|<p1>\"];\n",
        "  s2[label=\"2\"];\n",
        "  s1[label=\"$\"];\n",
        "  s0:p0->s1[label=\"1\"];\n",
        "  s0:p1->s2[label=\"2\"];\n",
        "  s2->s1[label=\"9\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, full_ctx()));
}

#[test]
#[ignore = "Known inefficiency but deferring resolving the issue for now"]
fn test_aex_bfx() {
    let g = Graph::new();
    let x1 = g.x();
    let x2 = g.x();
    let e = g.singleton(x1, 5);
    let f = g.singleton(x2, 6);
    let a = g.singleton(e, 1);
    let b = g.singleton(f, 2);
    let r = g.merge(a, b, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[shape=record, label=\"<p0>|<p1>\"];\n",
        "  s2[label=\"2\"];\n",
        "  s3[label=\"3\"];\n",
        "  s4[label=\"*\"];\n",
        "  s1[label=\"1\"];\n",
        "  s0:p0->s1[label=\"1\"];\n",
        "  s0:p1->s2[label=\"2\"];\n",
        "  s2->s3[label=\"6\"];\n",
        "  s3->s4[label=\"9\"];\n",
        "  s1->s3[label=\"5\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_Ae_Ae_fullctx() {
    let g = Graph::new();
    let A1 = g.array(&[g.empty()]);
    let A2 = g.array(&[g.empty()]);
    let r = g.merge(A1, A2, full_ctx());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[label=\"$\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, full_ctx()));
}

#[test]
fn test_Aab_Ac() {
    let g = Graph::new();
    let a = g.a();
    let b = g.b();
    let c = g.c();
    let A1 = g.array(&[a, b]);
    let A2 = g.array(&[c]);
    let r = g.merge(A1, A2, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[shape=record, label=\"<p0>|<p1>|<p2>\"];\n",
        "  s1[label=\"*\"];\n",
        "  s0:p0->s1[label=\"1\"];\n",
        "  s0:p1->s1[label=\"2\"];\n",
        "  s0:p2->s1[label=\"3\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_Aa_Aa() {
    let g = Graph::new();
    let a1 = g.a();
    let a2 = g.a();
    let A1 = g.array(&[a1]);
    let A2 = g.array(&[a2]);
    let r = g.merge(A1, A2, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[label=\"0\"];\n",
        "  s1[label=\"*\"];\n",
        "  s0->s1[label=\"1\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_Aa_Abc() {
    let g = Graph::new();
    let a = g.a();
    let b = g.b();
    let c = g.c();
    let A1 = g.array(&[a]);
    let A2 = g.array(&[b, c]);
    let r = g.merge(A1, A2, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[shape=record, label=\"<p0>|<p1>|<p2>\"];\n",
        "  s1[label=\"*\"];\n",
        "  s0:p0->s1[label=\"1\"];\n",
        "  s0:p1->s1[label=\"2\"];\n",
        "  s0:p2->s1[label=\"3\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_Aac_Ab() {
    let g = Graph::new();
    let a = g.a();
    let b = g.b();
    let c = g.c();
    let A1 = g.array(&[a, c]);
    let A2 = g.array(&[b]);
    let r = g.merge(A1, A2, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[shape=record, label=\"<p0>|<p1>|<p2>\"];\n",
        "  s1[label=\"*\"];\n",
        "  s0:p0->s1[label=\"1\"];\n",
        "  s0:p1->s1[label=\"2\"];\n",
        "  s0:p2->s1[label=\"3\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_Aab_Aa() {
    let g = Graph::new();
    let A1 = g.array(&[g.a(), g.b()]);
    let A2 = g.array(&[g.a()]);
    let r = g.merge(A1, A2, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[shape=record, label=\"<p0>|<p1>\"];\n",
        "  s1[label=\"*\"];\n",
        "  s0:p0->s1[label=\"1\"];\n",
        "  s0:p1->s1[label=\"2\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_Aab_Ab() {
    let g = Graph::new();
    let A1 = g.array(&[g.a(), g.b()]);
    let A2 = g.array(&[g.b()]);
    let r = g.merge(A1, A2, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[shape=record, label=\"<p0>|<p1>\"];\n",
        "  s1[label=\"*\"];\n",
        "  s0:p0->s1[label=\"1\"];\n",
        "  s0:p1->s1[label=\"2\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_Aax_Aby() {
    let g = Graph::new();
    let a = g.singleton(g.x(), 1);
    let b = g.singleton(g.y(), 2);
    let A1 = g.array(&[a]);
    let A2 = g.array(&[b]);
    let r = g.merge(A1, A2, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[shape=record, label=\"<p0>|<p1>\"];\n",
        "  s2[label=\"2\"];\n",
        "  s3[label=\"*\"];\n",
        "  s1[label=\"1\"];\n",
        "  s0:p0->s1[label=\"1\"];\n",
        "  s0:p1->s2[label=\"2\"];\n",
        "  s2->s3[label=\"10\"];\n",
        "  s1->s3[label=\"9\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_Aax_Aay() {
    let g = Graph::new();
    let a1 = g.singleton(g.x(), 1);
    let a2 = g.singleton(g.y(), 1);
    let A1 = g.array(&[a1]);
    let A2 = g.array(&[a2]);
    let r = g.merge(A1, A2, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[label=\"0\"];\n",
        "  s1[shape=record, label=\"<p0>|<p1>\"];\n",
        "  s2[label=\"*\"];\n",
        "  s0->s1[label=\"1\"];\n",
        "  s1:p0->s2[label=\"9\"];\n",
        "  s1:p1->s2[label=\"10\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_Aaxc_Aayd() {
    let g = Graph::new();
    let a1 = g.singleton(g.x(), 1);
    let a2 = g.singleton(g.y(), 1);
    let A1 = g.array(&[a1, g.c()]);
    let A2 = g.array(&[a2, g.d()]);
    let r = g.merge(A1, A2, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[shape=record, label=\"<p0>|<p1>|<p2>\"];\n",
        "  s2[label=\"*\"];\n",
        "  s1[shape=record, label=\"<p0>|<p1>\"];\n",
        "  s0:p0->s1[label=\"1\"];\n",
        "  s0:p1->s2[label=\"3\"];\n",
        "  s0:p2->s2[label=\"4\"];\n",
        "  s1:p0->s2[label=\"9\"];\n",
        "  s1:p1->s2[label=\"10\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_Aaubv_Acwdx() {
    let g = Graph::new();
    let a = g.singleton(g.u(), 1);
    let b = g.singleton(g.v(), 2);
    let c = g.singleton(g.w(), 3);
    let d = g.singleton(g.x(), 4);
    let A1 = g.array(&[a, b]);
    let A2 = g.array(&[c, d]);
    let r = g.merge(A1, A2, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[shape=record, label=\"<p0>|<p1>|<p2>|<p3>\"];\n",
        "  s4[label=\"4\"];\n",
        "  s5[label=\"*\"];\n",
        "  s3[label=\"3\"];\n",
        "  s2[label=\"2\"];\n",
        "  s1[label=\"1\"];\n",
        "  s0:p0->s1[label=\"1\"];\n",
        "  s0:p1->s2[label=\"2\"];\n",
        "  s0:p2->s3[label=\"3\"];\n",
        "  s0:p3->s4[label=\"4\"];\n",
        "  s4->s5[label=\"9\"];\n",
        "  s3->s5[label=\"8\"];\n",
        "  s2->s5[label=\"7\"];\n",
        "  s1->s5[label=\"6\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_Aaubv_Abvdx() {
    let g = Graph::new();
    let a = g.singleton(g.u(), 1);
    let b1 = g.singleton(g.v(), 2);
    let b2 = g.singleton(g.v(), 2);
    let d = g.singleton(g.x(), 4);
    let A1 = g.array(&[a, b1]);
    let A2 = g.array(&[b2, d]);
    let r = g.merge(A1, A2, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[shape=record, label=\"<p0>|<p1>|<p2>\"];\n",
        "  s3[label=\"3\"];\n",
        "  s4[label=\"*\"];\n",
        "  s2[label=\"2\"];\n",
        "  s1[label=\"1\"];\n",
        "  s0:p0->s1[label=\"1\"];\n",
        "  s0:p1->s2[label=\"2\"];\n",
        "  s0:p2->s3[label=\"4\"];\n",
        "  s3->s4[label=\"9\"];\n",
        "  s2->s4[label=\"7\"];\n",
        "  s1->s4[label=\"6\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_Aaubv_Abwdx() {
    let g = Graph::new();
    let a = g.singleton(g.u(), 1);
    let b1 = g.singleton(g.v(), 2);
    let b2 = g.singleton(g.w(), 2);
    let d = g.singleton(g.x(), 4);
    let A1 = g.array(&[a, b1]);
    let A2 = g.array(&[b2, d]);
    let r = g.merge(A1, A2, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[shape=record, label=\"<p0>|<p1>|<p2>\"];\n",
        "  s3[label=\"3\"];\n",
        "  s4[label=\"*\"];\n",
        "  s2[shape=record, label=\"<p0>|<p1>\"];\n",
        "  s1[label=\"1\"];\n",
        "  s0:p0->s1[label=\"1\"];\n",
        "  s0:p1->s2[label=\"2\"];\n",
        "  s0:p2->s3[label=\"4\"];\n",
        "  s3->s4[label=\"9\"];\n",
        "  s2:p0->s4[label=\"7\"];\n",
        "  s2:p1->s4[label=\"8\"];\n",
        "  s1->s4[label=\"6\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_Aaubv_Abvdu() {
    let g = Graph::new();
    let a = g.singleton(g.u(), 1);
    let b1 = g.singleton(g.v(), 2);
    let b2 = g.singleton(g.v(), 2);
    let d = g.singleton(g.u(), 4);
    let A1 = g.array(&[a, b1]);
    let A2 = g.array(&[b2, d]);
    let r = g.merge(A1, A2, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[shape=record, label=\"<p0>|<p1>|<p2>\"];\n",
        "  s2[label=\"2\"];\n",
        "  s3[label=\"*\"];\n",
        "  s1[label=\"1\"];\n",
        "  s0:p0->s1[label=\"1\"];\n",
        "  s0:p1->s2[label=\"2\"];\n",
        "  s0:p2->s1[label=\"4\"];\n",
        "  s2->s3[label=\"7\"];\n",
        "  s1->s3[label=\"6\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

#[test]
fn test_Aaubu_Acudu() {
    let g = Graph::new();
    let a = g.singleton(g.u(), 1);
    let b = g.singleton(g.u(), 2);
    let c = g.singleton(g.u(), 3);
    let d = g.singleton(g.u(), 4);
    let A1 = g.array(&[a, b]);
    let A2 = g.array(&[c, d]);
    let r = g.merge(A1, A2, root_is_wildcard());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[shape=record, label=\"<p0>|<p1>|<p2>|<p3>\"];\n",
        "  s1[label=\"1\"];\n",
        "  s2[label=\"*\"];\n",
        "  s0:p0->s1[label=\"1\"];\n",
        "  s0:p1->s1[label=\"2\"];\n",
        "  s0:p2->s1[label=\"3\"];\n",
        "  s0:p3->s1[label=\"4\"];\n",
        "  s1->s2[label=\"6\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, root_is_wildcard()));
}

// Cases beyond the Java suite: full-context merges of several empty paths.
// The expected graphs come from the Java runtime.

const X_Y_EMPTY: &str = concat!(
    "digraph G {\n",
    "rankdir=LR;\n",
    "  s0[shape=record, label=\"<p0>|<p1>|<p2>$\"];\n",
    "  s1[label=\"$\"];\n",
    "  s0:p0->s1[label=\"9\"];\n",
    "  s0:p1->s1[label=\"10\"];\n",
    "}\n",
);

#[test]
fn test_xe_ye_fullctx() {
    let g = Graph::new();
    let xe = g.merge(g.x(), g.empty(), full_ctx());
    let ye = g.merge(g.y(), g.empty(), full_ctx());
    let r = g.merge(xe, ye, full_ctx());
    assert_eq!(X_Y_EMPTY, to_dot_string(r, full_ctx()));
    assert!(!r.is_empty());
    assert!(r.has_empty_path());
    assert_eq!(r.get_parent(r.length() - 1), None);
}

#[test]
fn test_ye_xe_fullctx() {
    let g = Graph::new();
    let ye = g.merge(g.y(), g.empty(), full_ctx());
    let xe = g.merge(g.x(), g.empty(), full_ctx());
    let r = g.merge(ye, xe, full_ctx());
    assert_eq!(X_Y_EMPTY, to_dot_string(r, full_ctx()));
}

#[test]
fn test_e_xeye_fullctx() {
    let g = Graph::new();
    let xe = g.merge(g.x(), g.empty(), full_ctx());
    let ye = g.merge(g.y(), g.empty(), full_ctx());
    let xeye = g.merge(xe, ye, full_ctx());
    let r = g.merge(g.empty(), xeye, full_ctx());
    assert_eq!(X_Y_EMPTY, to_dot_string(r, full_ctx()));
}

#[test]
fn test_xeye_e_fullctx() {
    let g = Graph::new();
    let xe = g.merge(g.x(), g.empty(), full_ctx());
    let ye = g.merge(g.y(), g.empty(), full_ctx());
    let xeye = g.merge(xe, ye, full_ctx());
    let r = g.merge(xeye, g.empty(), full_ctx());
    assert_eq!(X_Y_EMPTY, to_dot_string(r, full_ctx()));
}

#[test]
fn test_xeye_xe_fullctx() {
    let g = Graph::new();
    let xe = g.merge(g.x(), g.empty(), full_ctx());
    let ye = g.merge(g.y(), g.empty(), full_ctx());
    let xeye = g.merge(xe, ye, full_ctx());
    let r = g.merge(xeye, g.merge(g.x(), g.empty(), full_ctx()), full_ctx());
    assert_eq!(X_Y_EMPTY, to_dot_string(r, full_ctx()));
}

#[test]
fn test_axe_aye_fullctx() {
    let g = Graph::new();
    let axe = g.singleton(g.merge(g.x(), g.empty(), full_ctx()), 1);
    let aye = g.singleton(g.merge(g.y(), g.empty(), full_ctx()), 1);
    let r = g.merge(axe, aye, full_ctx());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[label=\"0\"];\n",
        "  s1[shape=record, label=\"<p0>|<p1>|<p2>$\"];\n",
        "  s2[label=\"$\"];\n",
        "  s0->s1[label=\"1\"];\n",
        "  s1:p0->s2[label=\"9\"];\n",
        "  s1:p1->s2[label=\"10\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, full_ctx()));
}

#[test]
fn test_axe_ae_nested_fullctx() {
    let g = Graph::new();
    let axe = g.singleton(g.merge(g.x(), g.empty(), full_ctx()), 1);
    let r = g.merge(axe, g.a(), full_ctx());
    let expecting = concat!(
        "digraph G {\n",
        "rankdir=LR;\n",
        "  s0[label=\"0\"];\n",
        "  s1[shape=record, label=\"<p0>|<p1>$\"];\n",
        "  s2[label=\"$\"];\n",
        "  s0->s1[label=\"1\"];\n",
        "  s1:p0->s2[label=\"9\"];\n",
        "}\n",
    );
    assert_eq!(expecting, to_dot_string(r, full_ctx()));
}
