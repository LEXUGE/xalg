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
    crate::traits::Required,
    rand::{thread_rng, Rng},
};

#[derive(Debug)]
pub struct Monomial<T: Required> {
    exponents: Vec<T>,
    coefficient: T,
}

impl<T: Required> Monomial<T> {
    pub fn generate(mut exponent_limit: T, coefficient_limit: T) -> Self {
        let mut rng = thread_rng();
        let coefficient = rng.gen_range(T::one(), coefficient_limit);
        let mut exponents: Vec<T> = Vec::new();
        while exponent_limit > T::one() {
            let exp = rng.gen_range(T::zero(), exponent_limit);
            exponents.push(exp);
            exponent_limit -= exp;
        }
        Self {
            exponents,
            coefficient,
        }
    }
    pub fn export(&self) -> String {
        let mut flag = false;
        let mut origin = if self.coefficient == T::zero() {
            return self.coefficient.to_string();
        } else if self.coefficient == T::one() {
            flag = true;
            "".to_string()
        } else {
            self.coefficient.to_string()
        };
        for i in 0..self.exponents.len() {
            if self.exponents[i] == T::zero() {
            } else if self.exponents[i] == T::one() {
                flag = false;
                origin.push_str(&format!("x_{{{}}}", (i + 1)))
            } else {
                flag = false;
                origin.push_str(&format!("x_{{{}}}^{{{}}}", (i + 1), self.exponents[i]))
            };
        }
        if flag {
            origin.push('1');
        }
        origin
    }
}
