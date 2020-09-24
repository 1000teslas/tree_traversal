#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::collections::VecDeque;

pub fn stack_to_queue(s: &[u8]) -> Vec<u8> {
    bf_traverse(parse_stack(s))
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Expr {
    Num(u8),
    Op(u8, Box<Expr>, Box<Expr>),
}

pub fn debug() {
    let ss = b"xyPzwIM";
    dbg!(parse_stack(ss));
}

fn parse_stack(s: &[u8]) -> Box<Expr> {
    let mut stack = vec![];
    for &c in s {
        if c.is_ascii_lowercase() {
            stack.push(Box::new(Expr::Num(c)));
        } else if c.is_ascii_uppercase() {
            let e1 = stack.pop().unwrap();
            let e2 = stack.pop().unwrap();
            stack.push(Box::new(Expr::Op(c, e2, e1)));
        }
    }
    stack.pop().unwrap()
}

fn bf_traverse(e: Box<Expr>) -> Vec<u8> {
    let mut ser = vec![];
    let mut queue = VecDeque::new();
    queue.push_back(e);
    while let Some(e) = queue.pop_front() {
        match *e {
            Expr::Num(c) => {
                ser.push(c);
            }
            Expr::Op(c, e1, e2) => {
                ser.push(c);
                queue.push_back(e1);
                queue.push_back(e2);
            }
        }
    }
    ser.reverse();
    ser
}

#[cfg(test)]
mod test {
    use super::stack_to_queue;
    use crate::{bf_traverse, parse_stack, Expr};
    use quickcheck::{Arbitrary, Gen};
    use rand::{distributions::Uniform, prelude::*};
    use std::collections::VecDeque;

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

    fn traverse(e: Box<Expr>) -> Vec<u8> {
        let mut ser = vec![];
        let mut stack = vec![e];
        while let Some(e) = stack.pop() {
            match *e {
                Expr::Num(c) => {
                    ser.push(c);
                }
                Expr::Op(c, e1, e2) => {
                    ser.push(c);
                    stack.push(e1);
                    stack.push(e2);
                }
            }
        }
        ser.reverse();
        ser
    }

    fn parse_queue(s: &[u8]) -> Box<Expr> {
        let mut queue = VecDeque::new();
        for &c in s {
            if c.is_ascii_lowercase() {
                queue.push_back(Box::new(Expr::Num(c)));
            } else if c.is_ascii_uppercase() {
                let e1 = queue.pop_front().unwrap();
                let e2 = queue.pop_front().unwrap();
                queue.push_back(Box::new(Expr::Op(c, e2, e1)));
            }
        }
        queue.pop_front().unwrap()
    }

    #[test]
    fn test_1() {
        let ss = b"xyPzwIM";
        let sq = b"wzyxIPM";
        assert_eq!(stack_to_queue(ss), sq);
    }

    #[test]
    fn test_2() {
        let ss = b"xyPzwIM";
        let sq = b"wzyxIPM";
        assert_eq!(parse_stack(ss), parse_queue(sq));
    }

    #[test]
    fn test_3() {
        let ss = b"abcABdefgCDEF";
        let sq = b"gfCecbDdAaEBF";
        assert_eq!(stack_to_queue(ss), sq);
    }

    #[test]
    fn test_4() {
        let ss = b"abcABdefgCDEF";
        let sq = b"gfCecbDdAaEBF";
        assert_eq!(parse_stack(ss), parse_queue(sq));
    }

    #[quickcheck]
    fn test_traverse(e: Box<Expr>) -> bool {
        parse_stack(&traverse(e.clone())) == e
    }

    #[quickcheck]
    fn test_bf_traverse(e: Box<Expr>) -> bool {
        parse_queue(&bf_traverse(e.clone())) == e
    }

    #[quickcheck]
    fn test_stack_to_queue(e: Box<Expr>) -> bool {
        let ss = traverse(e.clone());
        let sq = bf_traverse(e);
        stack_to_queue(&ss) == sq
    }
}
