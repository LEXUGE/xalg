// Copyright 2019 LEXUGE
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// use(s)
use {
    self::OperatorFlag::*,
    crate::{formula::NeedBrackets::*, monomial::Monomial, traits::Required},
    rand::{seq::IteratorRandom, thread_rng, Rng},
    std::fmt,
};

#[derive(PartialEq, Copy, Clone)]
// The enum which denotes whether an operator need brackets or not.
enum NeedBrackets {
    // The kid(s) of it need brackets.
    True,
    // The kid(s) of it do(es)n't need brackets.
    False,
    // Never need brackets for both its self and its kid(s).
    Never,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
/// The enum which denotes different operators.
pub enum OperatorFlag {
    /// feature flag of addition.
    Add,
    /// feature flag of subtraction.
    Sub,
    /// feature flag of multiplication.
    Mul,
    /// feature flag of division.
    Div,
    /// feature flag of power.
    Pow,
}

enum Operators<T: Required> {
    Add(Box<Formula<T>>, Box<Formula<T>>),
    Sub(Box<Formula<T>>, Box<Formula<T>>),
    Mul(Box<Formula<T>>, Box<Formula<T>>),
    Div(Box<Formula<T>>, Box<Formula<T>>),
    Pow(Box<Formula<T>>, T),
    Lit(Monomial<T>),
}

#[derive(Copy, Clone)]
pub(crate) struct Info<'a, T: Required> {
    depth: T,
    exponent_limit: T,
    coefficient_limit: T,
    operator_flags: &'a [OperatorFlag],
}

impl<'a, T: Required> Info<'a, T> {
    pub(crate) fn new(
        depth: T,
        exponent_limit: T,
        coefficient_limit: T,
        operator_flags: &'a [OperatorFlag],
    ) -> Self {
        Self {
            depth,
            exponent_limit,
            coefficient_limit,
            operator_flags,
        }
    }
}

/// The formula AST.
pub struct Formula<T: Required> {
    operator: Operators<T>,
}

impl<T: Required> Formula<T> {
    #[cfg(test)]
    // No need to use `pub` attr due to the fact that module `tests` is the descendant of `self`.
    fn new(operator: Operators<T>) -> Self {
        Self { operator }
    }
    pub(crate) fn generate(mut info: Info<T>) -> Self {
        let mut rng = thread_rng();
        if info.depth == T::zero() {
            Self {
                operator: Operators::Lit(Monomial::generate(
                    rng.gen_range(T::zero(), info.exponent_limit),
                    info.coefficient_limit,
                )),
            }
        } else {
            info.depth -= T::one();
            match info.operator_flags.iter().choose(&mut rng).unwrap() {
                Add => Self {
                    operator: Operators::Add(
                        Box::new(Formula::generate(info)),
                        Box::new(Formula::generate(info)),
                    ),
                },
                Sub => Self {
                    operator: Operators::Sub(
                        Box::new(Formula::generate(info)),
                        Box::new(Formula::generate(info)),
                    ),
                },
                Mul => Self {
                    operator: Operators::Mul(
                        Box::new(Formula::generate(info)),
                        Box::new(Formula::generate(info)),
                    ),
                },
                Div => Self {
                    operator: Operators::Div(
                        Box::new(Formula::generate(info)),
                        Box::new(Formula::generate(info)),
                    ),
                },
                Pow => Self {
                    operator: Operators::Pow(
                        Box::new(Formula::generate(info)),
                        if T::one() + T::one() >= info.exponent_limit {
                            T::one() + T::one()
                        } else {
                            rng.gen_range(T::one() + T::one(), info.exponent_limit)
                        },
                    ),
                },
            }
        }
    }
    fn need_brackets(&self) -> NeedBrackets {
        use self::Operators::*;
        match self.operator {
            Add(_, _) | Sub(_, _) => False,
            Mul(_, _) => True,
            Lit(_) | Div(_, _) | Pow(_, _) => Never,
        }
    }
}

impl<T: Required> fmt::Display for Formula<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // A wrapper which is used for wrapping a layer of brackets around `Formula`.
        struct Wrapper<'a, T: Required>(&'a Formula<T>, bool);
        impl<'a, T: Required> fmt::Display for Wrapper<'a, T> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if self.1 {
                    write!(f, "({})", self.0)
                } else {
                    write!(f, "{}", self.0)
                }
            }
        }
        // Determines whether it should be wrapped or not.
        fn wrap<'a, T: Required>(
            formula: &'a Formula<T>,
            parent_need_brackets: NeedBrackets,
        ) -> Wrapper<'a, T> {
            if (parent_need_brackets == True) && (formula.need_brackets() == False) {
                Wrapper(formula, true)
            } else {
                Wrapper(formula, false)
            }
        }
        use self::Operators::*;
        let need_brackets = self.need_brackets();
        match &self.operator {
            Lit(v) => write!(f, "{}", v),
            Pow(a, p) => write!(f, "({})^{{{}}}", wrap(a, need_brackets), p),
            Mul(l, r) => write!(
                f,
                "{}\\times{{}}{}",
                wrap(l, need_brackets),
                wrap(r, need_brackets)
            ),
            Div(l, r) => write!(
                f,
                "\\frac{{{}}}{{{}}}",
                wrap(l, need_brackets),
                wrap(r, need_brackets)
            ),
            Add(l, r) => write!(f, "{}+{}", wrap(l, need_brackets), wrap(r, need_brackets)),
            Sub(l, r) => write!(f, "{}-{}", wrap(l, need_brackets), wrap(r, need_brackets)),
        }
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use {
        super::{Formula, Operators},
        crate::monomial::Monomial,
    };

    #[test]
    fn export_test() {
        assert_eq!(
            Formula::new(Operators::Add(
                Box::new(Formula::new(Operators::Lit(Monomial::new(
                    vec![1, 0, 2],
                    10
                )))),
                Box::new(Formula::new(Operators::Sub(
                    Box::new(Formula::new(Operators::Lit(Monomial::new(vec![], 1)))),
                    Box::new(Formula::new(Operators::Lit(Monomial::new(vec![0], 13)))),
                ))),
            ))
            .to_string(),
            r"10x_{1}x_{3}^{2}+1-13"
        );
        assert_eq!(
            Formula::new(Operators::Mul(
                Box::new(Formula::new(Operators::Lit(Monomial::new(
                    vec![1, 0, 2],
                    10
                )))),
                Box::new(Formula::new(Operators::Sub(
                    Box::new(Formula::new(Operators::Lit(Monomial::new(vec![], 1)))),
                    Box::new(Formula::new(Operators::Lit(Monomial::new(vec![0], 13)))),
                ))),
            ))
            .to_string(),
            r"10x_{1}x_{3}^{2}\times{}(1-13)"
        )
    }
}
