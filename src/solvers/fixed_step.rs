// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::concepts::steppers::*;

use std::ops::{Add,Mul,AddAssign};

pub struct Euler {}

impl Stepper for Euler {
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
        J: Iterator<Item=&'b mut F>
    {
        func(y, dy, t, p)?;
        for (yi, dyi) in y.into_iter().zip(dy.into_iter()) {
            *yi += *dt * *dyi;
        }
        Ok(())
    }

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
        F: Copy + Add<Output=F> + Add<F,Output=F> + AddAssign + Mul<F,Output=F> + Mul<I,Output=I> + From<f32>
    {
        func(y, dy, t, p)?;
        *y += *dt * *dy;
        Ok(())
    }
}

/// Runge-Kutta 4th order stepper
/// The Runge-Kutta 4th order solving scheme works with the following equations
/// y1 = y0 + (⅙) (k1 + 2k2 + 2k3 + k4)
/// k1 = hf(x0, y0)
/// k2 = hf[x0 + (½)h, y0 + (½)k1]
/// k3 = hf[x0 + (½)h, y0 + (½)k2]
/// k4 = hf(x0 + h, y0 + k3)
pub struct RK4 {}

// Implement the pytonic version of the RK4 stepper
impl Stepper for RK4 {
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
        J: Iterator<Item=&'b mut F>
    {
        func(y, dy, t, p)?;
        for (yi, dyi) in y.into_iter().zip(dy.into_iter()) {
            // TODO
            // This is not a Runge-Kutta solver yet!
            *yi += *dt * *dyi;
        }
        Ok(())
    }

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
        F: Copy + Add<Output=F> + Add<F,Output=F> + AddAssign + Mul<F,Output=F> + Mul<I,Output=I> + From<f32>
    {
        func(y, dy, t, p)?;
        // TODO
        // This is not a Runge-kutta solver yet!
        *y += *dt * *dy;
        Ok(())
    }
}