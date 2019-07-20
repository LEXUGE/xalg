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

// Documentation
//! A library for generating random formulas.
//! # Getting Started
//! ```
//! generate::<i32>(5,3,3).unwrap().export();
//! ```

#![deny(missing_docs)]

// mod(s)
mod monomial;
/// Module of formula AST.
pub mod formula;
mod traits;

// use(s)
use crate::{formula::Formula, traits::Required};

/// The enum for denoting error(s).
#[derive(Debug)]
pub enum ErrorKind {
    /// At least one of the arguments can't meet criterion.
    WrongNumber,
}

/// Generate the formula AST.
///
/// - `depth`: the depth of the AST of the formula. (depth >= 0)
/// - `exponent_limit`: the max of the exponent of the monomials. Generates in [0..max). (exponent_limit >= 1)
/// - `coefficient_limit`: the max of the coefficient of the monomials. Generates in [0..max). (coefficient_limit >= 2)
pub fn generate<T: Required>(
    depth: T,
    exponent_limit: T,
    coefficient_limit: T,
) -> Result<Formula<T>, ErrorKind> {
    if (depth < T::zero())
        || (exponent_limit < T::one())
        || (coefficient_limit < (T::one() + T::one()))
    {
        Err(ErrorKind::WrongNumber)
    } else {
        Ok(Formula::generate(
            depth,
            exponent_limit,
            coefficient_limit,
        ))
    }
}
