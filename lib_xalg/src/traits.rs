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
    num_traits::identities::{One, Zero},
    rand::distributions::uniform::SampleUniform,
    std::{fmt::Display, marker::Sized, ops::*},
};

// marco
macro_rules! do_impl {
    ($t:ty) => {
        impl Required for $t {}
    };
}

pub trait Required:
    PartialEq
    + PartialOrd
    + One
    + Zero
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Display
    + SampleUniform
    + Clone
    + Copy
where
    Self: Sized,
{
    //omitted
}

do_impl!(u8);
do_impl!(u16);
do_impl!(u32);
do_impl!(u64);
do_impl!(u128);
do_impl!(usize);

do_impl!(i8);
do_impl!(i16);
do_impl!(i32);
do_impl!(i64);
do_impl!(i128);
do_impl!(isize);
