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
        I: AddAssign + Clone + Mul<F,Output=I> + Mul<F,Output=I>,
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
        I: AddAssign + Clone + Mul<F,Output=I> + Mul<F,Output=I>,
        F: Add<F,Output=F> + Sub<F,Output=F> + Mul<F,Output=F> + Div<F,Output=F> + AddAssign + SubAssign + Neg<Output=F> + Copy + From<i8> + Mul<I,Output=I>,
    {
        func(y, dy, t, p)?;
        // TODO
        // This is not a Runge-kutta solver yet!
        *y += *dt * dy.clone();
        Ok(())
    }
}


#[cfg(test)]
mod tests_euler {
    use super::*;
    use super::super::super::concepts::errors::CalcError;

    fn rhs_vec<F>(x: &Vec<F>, dx: &mut Vec<F>, t: &F, p: &F) -> Result<(), CalcError>
    where
        F: Copy + Add<Output=F> + Add<F,Output=F> + AddAssign + Mul<F,Output=F> + Div<F,Output=F> + Neg<Output=F> + From<i8>,
    {
        for (xi, dxi) in x.into_iter().zip(dx.into_iter()) {
            *dxi = - *p * *xi * *t;
        }
        Ok(())
    }


    fn rhs_add<I, F>(x: &I, dx: &mut I, t: &F, p: &F) -> Result<(), CalcError>
    where
        I: AddAssign + Clone + Mul<F,Output=I> + Mul<F,Output=I> + std::ops::Neg<Output=I>,
        F: Add<F,Output=F> + Sub<F,Output=F> + Mul<F,Output=F> + Div<F,Output=F> + AddAssign + SubAssign + Neg<Output=F> + Copy + From<i8> + Mul<I,Output=I>,
    {
        *dx = - *p * x.clone() * *t;
        Ok(())
    }


    #[test]
    fn do_step_iter_f64() {
        let eu = Euler {};
        let mut x = vec!(2.0, 3.0, 4.0, 5.0);
        let mut dx = vec!(0.0, 0.0, 0.0, 0.0);
        let dt = 0.1;
        let t = 2.0;
        let p = 4.0;
        eu.do_step_iter(&rhs_vec, &mut x, &mut dx, &t, &dt, &p).unwrap();
    }

    #[test]
    fn do_step_iter_f32() {
        let eu = Euler {};
        let mut x = vec!(2.0f32, 3.0f32, 4.0f32, 5.0f32);
        let mut dx = vec!(0.0f32, 0.0f32, 0.0f32, 0.0f32);
        let dt = 0.1f32;
        let t = 2.0f32;
        let p = 4.0f32;
        eu.do_step_iter(&rhs_vec, &mut x, &mut dx, &t, &dt, &p).unwrap();
    }

    #[test]
    fn do_step_add_f64() {
        let eu = Euler {};
        let mut x = 2.0;
        let mut dx = 0.0;
        let dt = 0.1;
        let t = 2.0;
        let p = 4.0;
        eu.do_step_add(&rhs_add, &mut x, &mut dx, &t, &dt, &p).unwrap();
    }

    #[test]
    fn do_step_add_f32() {
        let eu = Euler {};
        let mut x = 2.0f32;
        let mut dx = 0.0f32;
        let dt = 0.1f32;
        let t = 2.0f32;
        let p = 4.0f32;
        eu.do_step_add(&rhs_add, &mut x, &mut dx, &t, &dt, &p).unwrap();
    }

    fn rhs_bad_add<I, F>(_x: &I, _dx: &mut I, _t: &F, _p: &F) -> Result<(), CalcError>
    where
        I: AddAssign + Clone + Mul<F,Output=I> + Mul<F,Output=I> + std::ops::Neg<Output=I>,
        F: Add<F,Output=F> + Sub<F,Output=F> + Mul<F,Output=F> + Div<F,Output=F> + AddAssign + SubAssign + Neg + Copy + From<i8> + Mul<I,Output=I>,
    {
        panic!("Purposefully panic to test Solver!");
    }

    #[test]
    #[should_panic]
    fn no_catch_calc_panic_add_f64() {
        let eu = Euler {};
        let mut x = 1.2943859;
        let mut dx = 0.0;
        let dt = 0.323987;
        let t = 5.23423987;
        let p = 345.394857;
        match eu.do_step_add(&rhs_bad_add, &mut x, &mut dx, &t, &dt, &p) {
            Ok(()) => panic!("We did not catch the error"),
            Err(CalcError) => (),
        };
    }

    #[test]
    #[should_panic]
    fn no_catch_calc_panic_add_f32() {
        let eu = Euler {};
        let mut x = 1.2943859f32;
        let mut dx = 0.0f32;
        let dt = 0.323987f32;
        let t = 5.23423987f32;
        let p = 345.394857f32;
        match eu.do_step_add(&rhs_bad_add, &mut x, &mut dx, &t, &dt, &p) {
            Ok(()) => panic!("We did not catch the error"),
            Err(CalcError) => (),
        };
    }

    fn rhs_bad_iter<F>(x: &Vec<F>, dx: &mut Vec<F>, _t: &F, p: &F) -> Result<(), CalcError>
    where
        F: Copy + Add<Output=F> + Add<F,Output=F> + AddAssign + Mul<F,Output=F> + std::ops::Neg<Output=F> + Div<F,Output=F> + From<i8>,
    {   
        let l  =  x.len();
        let dl = dx.len();
        dx[dl+1] = - *p * x[l+1];
        Ok(())
    }

    #[test]
    #[should_panic]
    fn no_catch_calc_panic_iter_f32() {
        let eu = Euler {};
        let mut x = vec!(1.2943859f32, 239489.2394879f32, 11.987908234f32);
        let mut dx = vec!(0.0f32, 0.0f32, 0.0f32);
        let dt = 0.323987f32;
        let t = 5.23423987f32;
        let p = 345.394857f32;
        match eu.do_step_iter(&rhs_bad_iter, &mut x, &mut dx, &t, &dt, &p) {
            Ok(()) => panic!("We did not catch the error"),
            Err(CalcError) => (),
        };
    }

    #[test]
    #[should_panic]
    fn no_catch_calc_panic_iter_f64() {
        let eu = Euler {};
        let mut x = vec!(1.2943859, 239489.2394879, 11.987908234);
        let mut dx = vec!(0.0, 0.0, 0.0);
        let dt = 0.323987;
        let t = 5.23423987;
        let p = 345.394857;
        match eu.do_step_iter(&rhs_bad_iter, &mut x, &mut dx, &t, &dt, &p) {
            Ok(()) => panic!("We did not catch the error"),
            Err(CalcError) => (),
        };
    }
}