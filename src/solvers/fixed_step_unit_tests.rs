// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::concepts::steppers::*;
use crate::solvers::fixed_step::*;
use std::ops::{Add,Sub,Mul,AddAssign,SubAssign,Div,Neg};


mod tests_euler {
    use super::*;
    use crate::concepts::errors::CalcError;

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
        I: Add<Output=I> + AddAssign + Clone + Mul<F,Output=I> + Mul<F,Output=I> + std::ops::Neg<Output=I>,
        F: Add<F,Output=F> + Sub<F,Output=F> + Mul<F,Output=F> + Div<F,Output=F> + AddAssign + SubAssign + Neg<Output=F> + Copy + From<i8> + Mul<I,Output=I>,
    {
        *dx = - *p * x.clone() * *t;
        Ok(())
    }


    #[test]
    fn do_step_iter_f64() {
        let mut x = vec!(2.0, 3.0, 4.0, 5.0);
        let dt = 0.1;
        let t = 2.0;
        let p = 4.0;
        let mut eu = Euler::from((&x, &t, &dt, &p));
        eu.do_step_iter(&rhs_vec, &mut x, &t, &dt, &p).unwrap();
    }

    #[test]
    fn do_step_iter_f32() {
        let mut x = vec!(2.0f32, 3.0f32, 4.0f32, 5.0f32);
        let dt = 0.1f32;
        let t = 2.0f32;
        let p = 4.0f32;
        let mut eu = Euler::from((&x, &t, &dt, &p));
        eu.do_step_iter(&rhs_vec, &mut x, &t, &dt, &p).unwrap();
    }

    #[test]
    fn do_step_add_f64() {
        let mut x = 2.0;
        let dt = 0.1;
        let t = 2.0;
        let p = 4.0;
        let mut eu = Euler::from((&x, &t, &dt, &p));
        eu.do_step_add(&rhs_add, &mut x, &t, &dt, &p).unwrap();
    }

    #[test]
    fn do_step_add_f32() {
        let mut x = 2.0f32;
        let dt = 0.1f32;
        let t = 2.0f32;
        let p = 4.0f32;
        let mut eu = Euler::from((&x, &t, &dt, &p));
        eu.do_step_add(&rhs_add, &mut x, &t, &dt, &p).unwrap();
    }

    fn rhs_bad_add<I, F>(_x: &I, _dx: &mut I, _t: &F, _p: &F) -> Result<(), CalcError>
    where
        I: Add<Output=I> + AddAssign + Clone + Mul<F,Output=I> + Mul<F,Output=I> + std::ops::Neg<Output=I>,
        F: Add<F,Output=F> + Sub<F,Output=F> + Mul<F,Output=F> + Div<F,Output=F> + AddAssign + SubAssign + Neg + Copy + From<i8> + Mul<I,Output=I>,
    {
        panic!("Purposefully panic to test Solver!");
    }

    #[test]
    #[should_panic]
    fn no_catch_calc_panic_add_f64() {
        let mut x = 1.2943859;
        let dt = 0.323987;
        let t = 5.23423987;
        let p = 345.394857;
        let mut eu = Euler::from((&x, &t, &dt, &p));
        match eu.do_step_add(&rhs_bad_add, &mut x, &t, &dt, &p) {
            Ok(()) => panic!("We did not catch the error"),
            Err(CalcError) => (),
        };
    }

    #[test]
    #[should_panic]
    fn no_catch_calc_panic_add_f32() {
        let mut x = 1.2943859f32;
        let dt = 0.323987f32;
        let t = 5.23423987f32;
        let p = 345.394857f32;
        let mut eu = Euler::from((&x, &t, &dt, &p));
        match eu.do_step_add(&rhs_bad_add, &mut x, &t, &dt, &p) {
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
        let mut x = vec!(1.2943859f32, 239489.2394879f32, 11.987908234f32);
        let dt = 0.323987f32;
        let t = 5.23423987f32;
        let p = 345.394857f32;
        let mut eu = Euler::from((&x, &t, &dt, &p));
        match eu.do_step_iter(&rhs_bad_iter, &mut x, &t, &dt, &p) {
            Ok(()) => panic!("We did not catch the error"),
            Err(CalcError) => (),
        };
    }

    #[test]
    #[should_panic]
    fn no_catch_calc_panic_iter_f64() {
        let mut x = vec!(1.2943859, 239489.2394879, 11.987908234);
        let dt = 0.323987;
        let t = 5.23423987;
        let p = 345.394857;
        let mut eu = Euler::from((&x, &t, &dt, &p));
        match eu.do_step_iter(&rhs_bad_iter, &mut x, &t, &dt, &p) {
            Ok(()) => panic!("We did not catch the error"),
            Err(CalcError) => (),
        };
    }
}


mod tests_rk4 {
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
        I: Add<Output=I> + AddAssign + Clone + Mul<F,Output=I> + Mul<F,Output=I> + std::ops::Neg<Output=I>,
        F: Add<F,Output=F> + Sub<F,Output=F> + Mul<F,Output=F> + Div<F,Output=F> + AddAssign + SubAssign + Neg<Output=F> + Copy + From<i8> + Mul<I,Output=I>,
    {
        *dx = - *p * x.clone() * *t;
        Ok(())
    }


    #[test]
    fn do_step_iter_f64() {
        let mut x = vec!(2.0, 3.0, 4.0, 5.0);
        let dt = 0.1;
        let t = 2.0;
        let p = 4.0;
        let mut rk4 = RK4::from((&x, &t, &dt, &p));
        rk4.do_step_iter(&rhs_vec, &mut x, &t, &dt, &p).unwrap();
    }

    #[test]
    fn do_step_iter_f32() {
        let mut x = vec!(2.0f32, 3.0f32, 4.0f32, 5.0f32);
        let dt = 0.1f32;
        let t = 2.0f32;
        let p = 4.0f32;
        let mut rk4 = RK4::from((&x, &t, &dt, &p));
        rk4.do_step_iter(&rhs_vec, &mut x, &t, &dt, &p).unwrap();
    }

    #[test]
    fn do_step_add_f64() {
        let mut x = 2.0;
        let dt = 0.1;
        let t = 2.0;
        let p = 4.0;
        let mut rk4 = RK4::from((&x, &t, &dt, &p));
        rk4.do_step_add(&rhs_add, &mut x, &t, &dt, &p).unwrap();
    }

    #[test]
    fn do_step_add_f32() {
        let mut x = 2.0f32;
        let dt = 0.1f32;
        let t = 2.0f32;
        let p = 4.0f32;
        let mut rk4 = RK4::from((&x, &t, &dt, &p));
        rk4.do_step_add(&rhs_add, &mut x, &t, &dt, &p).unwrap();
    }

    fn rhs_bad_add<I, F>(_x: &I, _dx: &mut I, _t: &F, _p: &F) -> Result<(), CalcError>
    where
        I: Add<Output=I> + AddAssign + Clone + Mul<F,Output=I> + Mul<F,Output=I> + std::ops::Neg<Output=I>,
        F: Add<F,Output=F> + Sub<F,Output=F> + Mul<F,Output=F> + Div<F,Output=F> + AddAssign + SubAssign + Neg + Copy + From<i8> + Mul<I,Output=I>,
    {
        panic!("Purposefully panic to test Solver!");
    }

    #[test]
    #[should_panic]
    fn no_catch_calc_panic_add_f64() {
        let mut x = 1.2943859;
        let dt = 0.323987;
        let t = 5.23423987;
        let p = 345.394857;
        let mut rk4 = RK4::from((&x, &t, &dt, &p));
        match rk4.do_step_add(&rhs_bad_add, &mut x, &t, &dt, &p) {
            Ok(()) => panic!("We did not catch the error"),
            Err(CalcError) => (),
        };
    }

    #[test]
    #[should_panic]
    fn no_catch_calc_panic_add_f32() {
        let mut x = 1.2943859f32;
        let dt = 0.323987f32;
        let t = 5.23423987f32;
        let p = 345.394857f32;
        let mut rk4 = RK4::from((&x, &t, &dt, &p));
        match rk4.do_step_add(&rhs_bad_add, &mut x, &t, &dt, &p) {
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
        let mut x = vec!(1.2943859f32, 239489.2394879f32, 11.987908234f32);
        let dt = 0.323987f32;
        let t = 5.23423987f32;
        let p = 345.394857f32;
        let mut rk4 = RK4::from((&x, &t, &dt, &p));
        match rk4.do_step_iter(&rhs_bad_iter, &mut x, &t, &dt, &p) {
            Ok(()) => panic!("We did not catch the error"),
            Err(CalcError) => (),
        };
    }

    #[test]
    #[should_panic]
    fn no_catch_calc_panic_iter_f64() {
        let mut x = vec!(1.2943859, 239489.2394879, 11.987908234);
        let dt = 0.323987;
        let t = 5.23423987;
        let p = 345.394857;
        let mut rk4 = RK4::from((&x, &t, &dt, &p));
        match rk4.do_step_iter(&rhs_bad_iter, &mut x, &t, &dt, &p) {
            Ok(()) => panic!("We did not catch the error"),
            Err(CalcError) => (),
        };
    }
}