// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::concepts::steppers::*;

use std::ops::{Mul};

/// # Euler stepper
/// This stepper is meant as an example and is one that should generally not be used.
/// Solving of the ODE is done via
/// \begin{equation}
///     y_1 = y_0 + dt f(y, t, p)
/// \end{equation}
pub struct Euler<I> {
    dy: I,
}

impl<I, F, P> From<(&I, &F, &F, &P)> for Euler<I>
where
    I: Clone,
    F: Copy,
    P: Clone,
{
    fn from(input: (&I, &F, &F, &P)) -> Euler<I> {
        Euler {
            dy: input.0.clone(),
        }
    }
}

impl<I, F, P, Err> Stepper<I, F, P, Err> for Euler<I> {
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
        F: FloatLikeType,
    {
        func(y, &mut self.dy, t, p)?;
        for (yi, dyi) in y.into_iter().zip(self.dy.into_iter()) {
            *yi += *dt * *dyi;
        }
        Ok(())
    }

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
        F: FloatLikeType + Mul<I,Output=I>,
    {
        func(y, &mut self.dy, t, p)?;
        *y += *dt * self.dy.clone();
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
/* pub struct RK4<I, F, P> {
    y: I,
    t: F,
    dt: F,
    k1: I,
    k2: I,
    k3: I,
    k4: I,
    p: P,
}*/

pub struct RK4<I> {
    // Helper variables
    k1: I,
    k2: I,
    k3: I,
    k4: I,
    dy: I,
    ym: I,
}

impl<I, F, P> From<(&I, &F, &F, &P)> for RK4<I>
where
    I: Clone,
    F: Copy,
    P: Clone,
{
    fn from(input: (&I, &F, &F, &P)) -> RK4<I> {
        RK4 {
            k1: input.0.clone(),
            k2: input.0.clone(),
            k3: input.0.clone(),
            k4: input.0.clone(),
            dy: input.0.clone(),
            ym: input.0.clone(),
        }
    }
}

// Implement the RK4 stepper
impl<I, F, P, Err> Stepper<I, F, P, Err> for RK4<I> {
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
        F: FloatLikeType,
    {
        func(y, &mut self.dy, t, p)?;
        for (yi, dyi) in y.into_iter().zip(self.dy.into_iter()) {
            // TODO
            // This is not a Runge-Kutta solver yet!
            *yi += *dt * *dyi;
        }
        Ok(())
    }

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
        F: FloatLikeType + Mul<I,Output=I>,
    {
        let half = F::from(1)/F::from(2);

        func(y, &mut self.dy, t, p)?;
        // TODO
        // Find more optimal version of this code
        self.k1 = *dt * self.dy.clone();
        self.ym = y.clone() + half * self.k1.clone();
        func(&self.ym, &mut self.dy, &(*t + half * *dt), p)?;
        self.k2 = *dt * self.dy.clone();
        self.ym = y.clone() + half * self.k2.clone();
        func(&self.ym, &mut self.dy, &(*t + half * *dt), p)?;
        self.k3 = *dt * self.dy.clone();
        self.ym = y.clone() + self.k3.clone();
        func(&self.ym, &mut self.dy, &(*t + *dt), p)?;
        self.k4 = *dt * self.dy.clone();
        *y += half / F::from(3) * (self.k1.clone() + F::from(2) * self.k2.clone() + F::from(2) * self.k3.clone() + self.k4.clone());
        Ok(())
    }
}

