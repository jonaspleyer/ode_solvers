// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::concepts::steppers::*;
use crate::concepts::ode_def::*;

use std::ops::{Mul};

/// # Euler stepper
/// This stepper is meant as an example and is one that should generally not be used.
/// Solving of the ODE is done via
/// \begin{equation}
///     y_1 = y_0 + dt f(y, t, p)
/// \end{equation}
pub struct Euler<'a, I, F, P, Err> {
    ode_def: OdeDefinition<'a, I, F, P, Err>,
    dy: I,
}

/// Create an Euler stepper from a OdeDefinition
impl<'a, I, F, P, Err> From<OdeDefinition<'a, I, F, P, Err>> for Euler<'a, I, F, P, Err>
where
    I: Clone,
    F: Copy,
    P: Clone,
{
    fn from(input: OdeDefinition<'a, I, F, P, Err>) -> Euler<'a, I, F, P, Err> {
        let dy = input.y0.clone();
        Euler {
            ode_def: input,
            dy: dy,
        }
    }
}

impl<'a, I, F, P, Err> Stepper<I, F, P, Err> for Euler<'a, I, F, P, Err> {
    fn do_step_iter
    (
        &mut self,
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
        (self.ode_def.func)(y, &mut self.dy, t, p)?;
        for (yi, dyi) in y.into_iter().zip(self.dy.into_iter()) {
            *yi += *dt * *dyi;
        }
        Ok(())
    }

    fn do_step_add
    (
        &mut self,
        y:  &mut I,
        t:  &F,
        dt: &F,
        p:  &P
    ) -> Result<(), Err>
    where
        I: MathVecLikeType<F>,
        F: FloatLikeType + Mul<I,Output=I>,
    {
        (self.ode_def.func)(y, &mut self.dy, t, p)?;
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
pub struct RK4<'a, I, F, P, Err> {
    ode_def: OdeDefinition<'a, I, F, P, Err>,
    // Helper variables
    k1: I,
    k2: I,
    k3: I,
    k4: I,
    dy: I,
    ym: I,
}

/// Create a RK4 stepper from a 
impl<'a, I, F, P, Err> From<OdeDefinition<'a, I, F, P, Err>> for RK4<'a, I, F, P, Err>
where
    I: Clone,
    F: Copy,
    P: Clone,
{
    fn from(input: OdeDefinition<'a, I, F, P, Err>) -> RK4<'a, I, F, P, Err> {
        let dy = input.y0.clone();
        RK4 {
            ode_def: input,
            k1: dy.clone(),
            k2: dy.clone(),
            k3: dy.clone(),
            k4: dy.clone(),
            dy: dy.clone(),
            ym: dy.clone(),
        }
    }
}

// Implement the RK4 stepper
impl<'a, I, F, P, Err> Stepper<I, F, P, Err> for RK4<'a, I, F, P, Err> {
    fn do_step_iter
    (
        &mut self,
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
        (self.ode_def.func)(y, &mut self.dy, t, p)?;
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

        (self.ode_def.func)(y, &mut self.dy, t, p)?;
        // TODO
        // Find more optimal version of this code
        self.k1 = *dt * self.dy.clone();
        self.ym = y.clone() + half * self.k1.clone();
        (self.ode_def.func)(&self.ym, &mut self.dy, &(*t + half * *dt), p)?;
        self.k2 = *dt * self.dy.clone();
        self.ym = y.clone() + half * self.k2.clone();
        (self.ode_def.func)(&self.ym, &mut self.dy, &(*t + half * *dt), p)?;
        self.k3 = *dt * self.dy.clone();
        self.ym = y.clone() + self.k3.clone();
        (self.ode_def.func)(&self.ym, &mut self.dy, &(*t + *dt), p)?;
        self.k4 = *dt * self.dy.clone();
        *y += half / F::from(3) * (self.k1.clone() + F::from(2) * self.k2.clone() + F::from(2) * self.k3.clone() + self.k4.clone());
        Ok(())
    }
}
