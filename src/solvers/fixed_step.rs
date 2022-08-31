// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::concepts::steppers::*;

use std::ops::{Add,Sub,Mul,AddAssign,SubAssign,Div,Neg};

pub struct Euler {}

impl Stepper for Euler {
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
        I: Add<Output=I> + AddAssign + Clone + Mul<F,Output=I> + Mul<F,Output=I>,
        F: Add<F,Output=F> + Sub<F,Output=F> + Mul<F,Output=F> + Div<F,Output=F> + AddAssign + SubAssign + Neg + Copy + From<i8> + Mul<I,Output=I>,
    {
        func(y, dy, t, p)?;
        *y += *dt * dy.clone();
        Ok(())
    }
}

/// # Runge-Kutta 4th order stepper
/// The Runge-Kutta 4th order solving scheme works with the following equations
/// First we compute the assisting variables
/// \begin{equation}
///     \begin{alignedat}{7}
///         k_1 &= &&hf(t_0 &,& y_0&&)\\\\
///         k_2 &= &&hf(t_0 + \tfrac{1}{2} &h ,& y_0 + \tfrac{1}{2} &k_1&)\\\\
///         k_3 &= &&hf(t_0 + \tfrac{1}{2} &h ,& y_0 + \tfrac{1}{2} &k_2&)\\\\
///         k_4 &= &&hf(t_0 + &h,& y_0 + &k_3&)
///     \end{alignedat}
/// \end{equation}
/// and finally combine them with
/// \begin{equation}
///     y_1 = y_0 + \tfrac{1}{6} (k_1 + 2 k_2 + 2 k_3 + k_4).
/// \end{equation}
pub struct RK4 {}

// Implement the RK4 stepper
impl Stepper for RK4 {
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
        I: Add<Output=I> + AddAssign + Clone + Mul<F,Output=I> + Mul<F,Output=I>,
        F: Add<F,Output=F> + Sub<F,Output=F> + Mul<F,Output=F> + Div<F,Output=F> + AddAssign + SubAssign + Neg<Output=F> + Copy + From<i8> + Mul<I,Output=I>,
    {
        let mut ym: I;
        let k1: I;
        let k2: I;
        let k3: I;
        let k4: I;
        let half = F::from(1)/F::from(2);

        func(y, dy, t, p)?;
        // TODO
        // Find more optimal version of this code
        k1 = *dt * dy.clone();
        ym = y.clone() + half * k1.clone();
        func(&ym, dy, &(*t + half * *dt), p)?;
        k2 = *dt * dy.clone();
        ym = y.clone() + half * k2.clone();
        func(&ym, dy, &(*t + half * *dt), p)?;
        k3 = *dt * dy.clone();
        ym = y.clone() + k3.clone();
        func(&ym, dy, &(*t + *dt), p)?;
        k4 = *dt * dy.clone();
        *y += half / F::from(3) * (k1 + F::from(2) * k2 + F::from(2) * k3 + k4);
        Ok(())
    }
}

