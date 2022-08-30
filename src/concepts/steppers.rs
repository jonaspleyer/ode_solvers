// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ops::{Add,Sub,Mul,AddAssign,SubAssign,Div,Neg};

// Hopefully we can in the future use trait aliases: https://github.com/rust-lang/rust/issues/41517
/*
pub trait Field =
    // Mathematical operations
    Add<Self,Output=Self> +
    Sub<Self,Output=Self> +
    Mul<Self,Output=Self> +
    Div<Self,Output=Self> +
    AddAssign +
    SubAssign +
    Neg +
    // Other operations necessary
    Copy +
    Sized
*/


pub trait Stepper {
    fn do_step_iter<'a, 'b, I, F: 'a, P, Err>
    (
        &self,
        func: &dyn Fn(&I, &mut I, &F, &P) -> Result<(), Err>,
        y:  &'a mut I,
        dy: &'a mut I,
        t:  &'b F,
        dt: &'b F,
        p:  &P
    ) -> Result<(), Err>
    where
        for<'m>&'m mut I: IntoIterator<Item=&'m mut F>,
        for<'m>&'m I: IntoIterator<Item=&'m F>,
        F: Add<F,Output=F> + Sub<F,Output=F> + Mul<F,Output=F> + Div<F,Output=F> + AddAssign + SubAssign + Neg<Output=F> + Copy + From<i8>,
        P: std::panic::RefUnwindSafe;
    
    
    fn do_step_add<'a, 'b, I, F: 'b, P, Err>
    (
        &self,
        func: &dyn Fn(&I, &mut I, &F, &P) -> Result<(), Err>,
        y:  &'a mut I,
        dy: &'a mut I,
        t:  &'b F,
        dt: &'b F,
        p:  &P
    ) -> Result<(), Err>
    where
        I: AddAssign + Clone + Mul<F,Output=I> + Mul<F,Output=I>,
        F: Add<F,Output=F> + Sub<F,Output=F> + Mul<F,Output=F> + Div<F,Output=F> + AddAssign + SubAssign + Neg<Output=F> + Copy + From<i8> + Mul<I,Output=I>,
        P: std::panic::RefUnwindSafe;
}