use crate::concepts::steppers::*;

use std::ops::{Add,Mul};

pub struct Euler {}

impl Stepper for Euler {
    fn do_step<V, T, P, Err>(
        &self,
        func: &dyn Fn(&V, &T, &P) -> Result<V, Err>,
        input: &V,
        t: &T,
        dt: &T,
        p: &P
    ) -> Result<V, Err>
    where
        V: Add<Output=V> + Copy + Mul<f64, Output=V>,
        T: Add<Output=T> + Copy + Mul<f64, Output=T> + Mul<V, Output=V>,
        f64: Mul<V, Output=V> + Mul<T, Output=T>,
    {
        let r = func(input, t, p)?;
        return Ok(*input + *dt * r)
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
    fn do_step<V, T, P, Err>(
        &self,
        func: &dyn Fn(&V, &T, &P) -> Result<V, Err>,
        input: &V,
        t: &T,
        dt: &T,
        p: &P
    ) -> Result<V, Err>
    where
        V: Add<Output=V> + Copy + Mul<f64, Output=V>,
        T: Add<Output=T> + Copy + Mul<f64, Output=T> + Mul<V, Output=V>,
        f64: Mul<V, Output=V> + Mul<T, Output=T>,
    {
        let k1 = *dt * func(input, t, p)?;
        let k2 = *dt * func(&(*input + 0.5 * k1), &(*t + 0.5 * *dt), p)?;
        let k3 = *dt * func(&(*input + 0.5 * k2), &(*t + 0.5 * *dt), p)?;
        let k4 = *dt * func(&(*input + k3), &(*t + *dt), p)?;
        return Ok(*input + (1.0/6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4));
    }
}