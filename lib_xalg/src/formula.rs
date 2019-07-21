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
};

#[derive(PartialEq, Copy, Clone)]
/// The enum which denotes whether an operator need brackets or not.
pub enum NeedBrackets {
    /// The kid(s) of it need brackets.
    True,
    /// The kid(s) of it do(es)n't need brackets.
    False,
    /// Never need brackets for both its self and its kid(s).
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

#[derive(Debug)]
enum Operators<T: Required> {
    Add(Box<Formula<T>>, Box<Formula<T>>),
    Sub(Box<Formula<T>>, Box<Formula<T>>),
    Mul(Box<Formula<T>>, Box<Formula<T>>),
    Div(Box<Formula<T>>, Box<Formula<T>>),
    Pow(Box<Formula<T>>, T),
    Wrap(Monomial<T>),
}

#[derive(Debug)]
/// The formula AST.
pub struct Formula<T: Required> {
    operator: Operators<T>,
}

impl<T: Required> Formula<T> {
    pub(crate) fn generate(
        depth: T,
        exponent_limit: T,
        coefficient_limit: T,
        operator_flags: &[OperatorFlag],
    ) -> Self {
        let mut rng = thread_rng();
        let operator = if depth == T::zero() {
            Operators::Wrap(Monomial::generate(
                rng.gen_range(T::zero(), exponent_limit),
                coefficient_limit,
            ))
        } else {
            let depth = depth - T::one();
            match operator_flags.iter().choose(&mut rng).unwrap() {
                Add => Operators::Add(
                    Box::new(Formula::generate(
                        depth,
                        exponent_limit,
                        coefficient_limit,
                        operator_flags,
                    )),
                    Box::new(Formula::generate(
                        depth,
                        exponent_limit,
                        coefficient_limit,
                        operator_flags,
                    )),
                ),
                Sub => Operators::Sub(
                    Box::new(Formula::generate(
                        depth,
                        exponent_limit,
                        coefficient_limit,
                        operator_flags,
                    )),
                    Box::new(Formula::generate(
                        depth,
                        exponent_limit,
                        coefficient_limit,
                        operator_flags,
                    )),
                ),
                Mul => Operators::Mul(
                    Box::new(Formula::generate(
                        depth,
                        exponent_limit,
                        coefficient_limit,
                        operator_flags,
                    )),
                    Box::new(Formula::generate(
                        depth,
                        exponent_limit,
                        coefficient_limit,
                        operator_flags,
                    )),
                ),
                Div => Operators::Div(
                    Box::new(Formula::generate(
                        depth,
                        exponent_limit,
                        coefficient_limit,
                        operator_flags,
                    )),
                    Box::new(Formula::generate(
                        depth,
                        exponent_limit,
                        coefficient_limit,
                        operator_flags,
                    )),
                ),
                Pow => Operators::Pow(
                    Box::new(Formula::generate(
                        depth,
                        exponent_limit,
                        coefficient_limit,
                        operator_flags,
                    )),
                    if T::one() + T::one() >= exponent_limit {
                        T::one() + T::one()
                    } else {
                        rng.gen_range(T::one() + T::one(), exponent_limit)
                    },
                ),
            }
        };
        Self { operator }
    }
    /// export the AST to LaTeX code. Sets the argument to `Needbrackets::False` to use it.
    pub fn export(&self, parent_need_brackets: NeedBrackets) -> String {
        let self_need_brackets = self.need_brackets();
        let origin = match &self.operator {
            Operators::Add(a, b) => format!(
                "{}+{}",
                a.export(self_need_brackets),
                b.export(self_need_brackets)
            ),
            Operators::Sub(a, b) => format!(
                "{}-{}",
                a.export(self_need_brackets),
                b.export(self_need_brackets)
            ),
            Operators::Mul(a, b) => format!(
                "{}\\times{{}}{}",
                a.export(self_need_brackets),
                b.export(self_need_brackets)
            ),
            Operators::Div(a, b) => format!(
                "\\frac{{{}}}{{{}}}",
                a.export(self_need_brackets),
                b.export(self_need_brackets)
            ),
            Operators::Pow(a, p) => format!("({})^{{{}}}", a.export(self_need_brackets), p),
            Operators::Wrap(a) => a.export(),
        };
        if (parent_need_brackets == True) && (self_need_brackets == False) {
            format!("({})", origin)
        } else {
            origin
        }
    }
    fn need_brackets(&self) -> NeedBrackets {
        match &self.operator {
            Operators::Add(_, _) | Operators::Sub(_, _) => False,
            Operators::Mul(_, _) => True,
            Operators::Wrap(_) | Operators::Div(_, _) | Operators::Pow(_, _) => Never,
        }
    }
}
