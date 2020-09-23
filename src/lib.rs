#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use quickcheck::{Arbitrary, Gen};
use rand::{distributions::Uniform, prelude::*};
use std::collections::VecDeque;

pub fn stack_to_queue(s: &[u8]) -> Vec<u8> {
    bf_traverse(parse_stack(s))
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Expr {
    Num(u8),
    Op(u8, Box<Expr>, Box<Expr>),
}
impl Arbitrary for Expr {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        if g.gen() {
            Expr::Num(Uniform::from(b'a'..=b'z').sample(g))
        } else {
            Expr::Op(
                Uniform::from(b'A'..=b'Z').sample(g),
                Box::new(Expr::arbitrary(g)),
                Box::new(Expr::arbitrary(g)),
            )
        }
    }
}

fn parse_stack(s: &[u8]) -> Box<Expr> {
    let mut v = vec![];
    for &c in s {
        match c {
            b'a'..=b'z' => {
                v.push(Box::new(Expr::Num(c)));
            }
            b'A'..=b'Z' => {
                let e1 = v.pop().unwrap();
                let e2 = v.pop().unwrap();
                v.push(Box::new(Expr::Op(c, e1, e2)));
            }
            _ => panic!(),
        }
    }
    v.pop().unwrap()
}

fn parse_queue(s: &[u8]) -> Box<Expr> {
    let mut v = VecDeque::new();
    for &c in s {
        match c {
            b'a'..=b'z' => {
                v.push_back(Box::new(Expr::Num(c)));
            }
            b'A'..=b'Z' => {
                let e1 = v.pop_front().unwrap();
                let e2 = v.pop_front().unwrap();
                v.push_back(Box::new(Expr::Op(c, e1, e2)));
            }
            _ => panic!(),
        }
    }
    v.pop_front().unwrap()
}

fn bf_traverse(e: Box<Expr>) -> Vec<u8> {
    let mut s = vec![];
    let mut v = VecDeque::new();
    v.push_back(e);
    while let Some(e) = v.pop_front() {
        match *e {
            Expr::Num(c) => {
                s.push(c);
            }
            Expr::Op(c, e1, e2) => {
                s.push(c);
                v.push_back(e2);
                v.push_back(e1);
            }
        }
    }
    s.reverse();
    s
}

fn post_traverse(e: Box<Expr>) -> Vec<u8> {
    match *e {
        Expr::Num(c) => vec![c],
        Expr::Op(c, e1, e2) => {
            let mut v = post_traverse(e2);
            v.append(&mut post_traverse(e1));
            v.push(c);
            v
        }
    }
}

#[cfg(test)]
mod test {
    use super::stack_to_queue;
    use crate::{bf_traverse, parse_queue, post_traverse, Expr};

    #[test]
    fn test_1() {
        let ss = b"xyPzwIM";
        let sq = b"wzyxIPM";
        assert_eq!(stack_to_queue(ss), sq);
    }

    #[test]
    fn test_2() {
        let ss = b"abcABdefgCDEF";
        let sq = b"gfCecbDdAaEBF";
        assert_eq!(stack_to_queue(ss), sq);
    }

    #[test]
    fn test_3() {
        let sq = b"gfCecbDdAaEBF";
        let ss = b"abcABdefgCDEF";
        let e = parse_queue(sq);
        assert_eq!(post_traverse(e), ss);
    }

    #[quickcheck]
    fn test_stack_to_queue(e: Box<Expr>) -> bool {
        let ss = post_traverse(e.clone());
        let sq = bf_traverse(e);
        stack_to_queue(&ss) == sq
    }
}
