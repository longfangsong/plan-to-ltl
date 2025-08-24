use std::{fmt::Display, ops::BitAnd, ptr::write};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord, Default)]
pub enum LTL {
    Top,
    #[default]
    Bottom,
    Atom(u32),
    Not(Box<LTL>),
    Globally(Box<LTL>),
    Eventually(Box<LTL>),
    Next(Box<LTL>),
    And(Vec<LTL>),
    Or(Vec<LTL>),
}

impl LTL {
    pub fn and(self, other: LTL) -> LTL {
        match (self, other) {
            (LTL::Bottom, _) | (_, LTL::Bottom) => LTL::Bottom,
            (LTL::Top, other) | (other, LTL::Top) => other,
            (LTL::And(lhs), LTL::And(rhs)) => {
                LTL::And(lhs.into_iter().chain(rhs.into_iter()).collect())
            }
            (LTL::And(lhs), rhs) => LTL::And(lhs.into_iter().chain(std::iter::once(rhs)).collect()),
            (lhs, LTL::And(rhs)) => LTL::And(std::iter::once(lhs).chain(rhs.into_iter()).collect()),
            (lhs, rhs) => LTL::And(vec![lhs, rhs]),
        }
    }

    pub fn not(self) -> LTL {
        LTL::Not(Box::new(self))
    }

    pub fn or(self, other: LTL) -> LTL {
        match (self, other) {
            (LTL::Or(lhs), LTL::Or(rhs)) => {
                LTL::Or(lhs.into_iter().chain(rhs.into_iter()).collect())
            }
            (LTL::Or(lhs), rhs) => LTL::Or(lhs.into_iter().chain(std::iter::once(rhs)).collect()),
            (lhs, LTL::Or(rhs)) => LTL::Or(std::iter::once(lhs).chain(rhs.into_iter()).collect()),
            (lhs, rhs) => LTL::Or(vec![lhs, rhs]),
        }
    }

    pub fn implies(self, other: LTL) -> LTL {
        self.not().or(other)
    }

    pub fn next(self) -> LTL {
        LTL::Next(Box::new(self))
    }

    pub fn eventually(self) -> LTL {
        LTL::Eventually(Box::new(self))
    }
}

impl Display for LTL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LTL::Top => write!(f, "⊤"),
            LTL::Bottom => write!(f, "⊥"),
            LTL::Atom(id) => write!(f, "o{}", id),
            LTL::Not(ltl) => write!(f, "!{}", ltl),
            LTL::Globally(ltl) => write!(f, "G{}", ltl),
            LTL::Eventually(ltl) => write!(f, "F{}", ltl),
            LTL::Next(ltl) => write!(f, "X{}", ltl),
            LTL::And(ltls) => {
                if ltls.len() == 0 {
                    write!(f, "?")
                } else {
                    write!(f, "({}", ltls[0])?;
                    for ltl in &ltls[1..] {
                        write!(f, " & {}", ltl)?;
                    }
                    write!(f, ")")
                }
            }
            LTL::Or(ltls) if ltls.len() == 2 && matches!(&ltls[0], LTL::Not(inner)) => {
                let LTL::Not(inner) = &ltls[0] else {
                    unreachable!()
                };
                write!(f, "({}", inner)?;
                write!(f, " -> {}", ltls[1])?;
                write!(f, ")")
            }
            LTL::Or(ltls) => {
                write!(f, "({}", ltls[0])?;
                for ltl in &ltls[1..] {
                    write!(f, " | {}", ltl)?;
                }
                write!(f, ")")
            }
        }
    }
}

impl BitAnd for LTL {
    type Output = LTL;

    fn bitand(self, rhs: LTL) -> Self::Output {
        self.and(rhs)
    }
}
