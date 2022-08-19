// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ops::{Add,Mul,AddAssign};

pub trait Stepper {
    fn do_step_iter<'a, 'b, I, J, F: 'b, P, Err>
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
        &'a mut I: IntoIterator<Item=&'b mut F, IntoIter=J>,
        F: Copy + Add<Output=F> + Add<F,Output=F> + AddAssign + Mul<F,Output=F> + From<f32>,
        J: Iterator<Item=&'b mut F>;
    
    
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
        I: AddAssign + Copy + Mul<F,Output=I> + Mul<f64,Output=I>,
        F: Copy + Add<Output=F> + Add<F,Output=F> + AddAssign + Mul<F,Output=F> + Mul<I,Output=I> + From<f32>;
}