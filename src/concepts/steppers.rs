// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ops::{Add,Sub,Mul,AddAssign,SubAssign,Div,Neg};

/// This type allows one to arbitrary floating point types and even in theory exact decimal fractions to numerically integrate the ODE
/// Since some algorithms require the use of constants, we need to be able to map at least from natural numbers to our FloatLikeType
/// The type i8 was chosen since implementations for f64 and f32 were already present.
pub trait FloatLikeType:
    Add<Self,Output=Self> +
    Sub<Self,Output=Self> +
    Mul<Self,Output=Self> +
    Div<Self,Output=Self> +
    AddAssign +
    SubAssign +
    Neg<Output=Self> +
    Copy +
    From<i8>
{}

impl<T> FloatLikeType for T
where
    T: Add<Self,Output=Self> + Sub<Self,Output=Self> + Mul<Self,Output=Self> + Div<Self,Output=Self> + AddAssign + SubAssign + Neg<Output=Self> + Copy + From<i8>
{}


/// This type is ment to represent a mathematical type similar to a fixed-size vector in a vector space
/// For a definition look at eg. <https://lyryx.com/first-course-linear-algebra/>
pub trait MathVecLikeType<F>:
    Add<Output=Self> +
    AddAssign +
    Clone +
    Mul<F,Output=Self>
{}

impl<T, F> MathVecLikeType<F> for T
where
    T: Add<Output=Self> + AddAssign + Clone + Mul<F,Output=Self>,
    F: FloatLikeType
{}

// Hopefully we can in the future use trait aliases: https://github.com/rust-lang/rust/issues/41517


pub trait Stepper<I, F, P, Err> {
    fn do_step_iter
    (
        &mut self,
        func: &dyn Fn(&I, &mut I, &F, &P) -> Result<(), Err>,
        y:  &mut I,
        t:  &F,
        dt: &F,
        p:  &P
    ) -> Result<(), Err>
    where
        for<'m>&'m mut I: IntoIterator<Item=&'m mut F>,
        for<'m>&'m I: IntoIterator<Item=&'m F>,
        F: FloatLikeType;
    
    
    fn do_step_add
    (
        &mut self,
        func: &dyn Fn(&I, &mut I, &F, &P) -> Result<(), Err>,
        y:  &mut I,
        t:  &F,
        dt: &F,
        p:  &P
    ) -> Result<(), Err>
    where
        I: MathVecLikeType<F>,
        F: FloatLikeType + Mul<I,Output=I>;
}