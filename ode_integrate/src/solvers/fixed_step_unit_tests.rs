// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// use crate::concepts::steppers::*;
use crate::concepts::errors::CalcError;
use crate::concepts::ode_def::*;
use crate::methods::helper_functions::*;
use crate::solvers::fixed_step::*;

use alloc::vec::Vec;

enum Operations {
    Add,
    Iter,
}

enum Ethos {
    Good,
    Bad,
}

#[macro_export]
macro_rules! do_step {
    ($f: ty, $s: expr, $conf: expr) => {
        type F = $f;

        fn rhs_iter_good(x: &Vec<F>, dx: &mut Vec<F>, t: &F, p: &F) -> Result<(), CalcError> {
            for (xi, dxi) in x.into_iter().zip(dx.into_iter()) {
                *dxi = -*p * *xi * *t;
            }
            Ok(())
        }

        fn rhs_iter_bad(x: &Vec<F>, dx: &mut Vec<F>, t: &F, p: &F) -> Result<(), CalcError> {
            for (xi, dxi) in x.into_iter().zip(dx.into_iter()) {
                *dxi = -*p * *xi * *t;
            }
            panic!("Test panic inserted here");
        }

        fn rhs_add_good(x: &F, dx: &mut F, t: &F, p: &F) -> Result<(), CalcError> {
            *dx = -*p * *x * *t;
            Ok(())
        }

        fn rhs_add_bad(x: &F, dx: &mut F, t: &F, p: &F) -> Result<(), CalcError> {
            *dx = -*p * *x * *t;
            panic!("Test panic inserted here");
        }

        const VEC_MIN: u8 = 0;
        const VEC_MAX: u8 = 20;

        let dt = F::from(1u8) / F::from(10u8);
        let mut t = F::from(2u8);
        let p = F::from(4u8);

        match $conf {
            (Operations::Iter, Ethos::Good) => {
                let mut x0: Vec<F> = (VEC_MIN as u8..VEC_MAX as u8).map(F::from).collect();
                let ode_def = OdeDefinition {
                    y0: x0.clone(),
                    t0: t,
                    func: &rhs_iter_good,
                };
                let mut s = get_fixed_step_stepper($s, ode_def);
                for _ in 0..100 {
                    s.do_step_iter(&mut x0, &t, &dt, &p).unwrap();
                    t += dt;
                }
            }
            (Operations::Iter, Ethos::Bad) => {
                let mut x0: Vec<F> = (VEC_MIN as u8..VEC_MAX as u8).map(F::from).collect();
                let ode_def = OdeDefinition {
                    y0: x0.clone(),
                    t0: t,
                    func: &rhs_iter_bad,
                };
                let mut s = get_fixed_step_stepper($s, ode_def);
                for _ in 0..100 {
                    s.do_step_iter(&mut x0, &t, &dt, &p).unwrap();
                    t += dt;
                }
            }
            (Operations::Add, Ethos::Good) => {
                let mut x0: F = F::from(10u8);
                let ode_def = OdeDefinition {
                    y0: x0.clone(),
                    t0: t,
                    func: &rhs_add_good,
                };
                let mut s = get_fixed_step_stepper($s, ode_def);
                for _ in 0..100 {
                    s.do_step_add(&mut x0, &t, &dt, &p).unwrap();
                    t += dt;
                }
            }
            (Operations::Add, Ethos::Bad) => {
                let mut x0: F = F::from(10u8);
                let ode_def = OdeDefinition {
                    y0: x0.clone(),
                    t0: t,
                    func: &rhs_add_bad,
                };
                let mut s = get_fixed_step_stepper($s, ode_def);
                for _ in 0..100 {
                    s.do_step_add(&mut x0, &t, &dt, &p).unwrap();
                    t += dt;
                }
            }
        }
    };
}

// TODO can we somehow automate this mess? We only want to iterate over all combinations.
mod euler {
    use super::*;
    use f128::f128;
    use half::f16;

    #[test]
    fn add_good_f128() {
        do_step!(
            f128,
            FixedStepSolvers::Euler,
            (Operations::Add, Ethos::Good)
        );
    }

    #[test]
    fn iter_good_f128() {
        do_step!(
            f128,
            FixedStepSolvers::Euler,
            (Operations::Iter, Ethos::Good)
        );
    }

    #[test]
    #[should_panic]
    fn add_bad_f128() {
        do_step!(f128, FixedStepSolvers::Euler, (Operations::Add, Ethos::Bad));
    }

    #[test]
    #[should_panic]
    fn iter_bad_f128() {
        do_step!(
            f128,
            FixedStepSolvers::Euler,
            (Operations::Iter, Ethos::Bad)
        );
    }

    #[test]
    fn add_good_f64() {
        do_step!(f64, FixedStepSolvers::Euler, (Operations::Add, Ethos::Good));
    }

    #[test]
    fn iter_good_f64() {
        do_step!(
            f64,
            FixedStepSolvers::Euler,
            (Operations::Iter, Ethos::Good)
        );
    }

    #[test]
    #[should_panic]
    fn add_bad_f64() {
        do_step!(f64, FixedStepSolvers::Euler, (Operations::Add, Ethos::Bad));
    }

    #[test]
    #[should_panic]
    fn iter_bad_f64() {
        do_step!(f64, FixedStepSolvers::Euler, (Operations::Iter, Ethos::Bad));
    }

    #[test]
    fn add_good_f32() {
        do_step!(f32, FixedStepSolvers::Euler, (Operations::Add, Ethos::Good));
    }

    #[test]
    fn iter_good_f32() {
        do_step!(
            f32,
            FixedStepSolvers::Euler,
            (Operations::Iter, Ethos::Good)
        );
    }

    #[test]
    #[should_panic]
    fn add_bad_f32() {
        do_step!(f32, FixedStepSolvers::Euler, (Operations::Add, Ethos::Bad));
    }

    #[test]
    #[should_panic]
    fn iter_bad_f32() {
        do_step!(f32, FixedStepSolvers::Euler, (Operations::Iter, Ethos::Bad));
    }

    #[test]
    fn add_good_f16() {
        do_step!(f16, FixedStepSolvers::Euler, (Operations::Add, Ethos::Good));
    }

    #[test]
    fn iter_good_f16() {
        do_step!(
            f16,
            FixedStepSolvers::Euler,
            (Operations::Iter, Ethos::Good)
        );
    }

    #[test]
    #[should_panic]
    fn add_bad_f16() {
        do_step!(f16, FixedStepSolvers::Euler, (Operations::Add, Ethos::Bad));
    }

    #[test]
    #[should_panic]
    fn iter_bad_f16() {
        do_step!(f16, FixedStepSolvers::Euler, (Operations::Iter, Ethos::Bad));
    }
}

mod rk4 {
    use super::*;
    use f128::f128;
    use half::f16;

    #[test]
    fn add_good_f128() {
        do_step!(f128, FixedStepSolvers::Rk4, (Operations::Add, Ethos::Good));
    }

    #[test]
    fn iter_good_f128() {
        do_step!(f128, FixedStepSolvers::Rk4, (Operations::Iter, Ethos::Good));
    }

    #[test]
    #[should_panic]
    fn add_bad_f128() {
        do_step!(f128, FixedStepSolvers::Rk4, (Operations::Add, Ethos::Bad));
    }

    #[test]
    #[should_panic]
    fn iter_bad_f128() {
        do_step!(f128, FixedStepSolvers::Rk4, (Operations::Iter, Ethos::Bad));
    }

    #[test]
    fn add_good_f64() {
        do_step!(f64, FixedStepSolvers::Rk4, (Operations::Add, Ethos::Good));
    }

    #[test]
    fn iter_good_f64() {
        do_step!(f64, FixedStepSolvers::Rk4, (Operations::Iter, Ethos::Good));
    }

    #[test]
    #[should_panic]
    fn add_bad_f64() {
        do_step!(f64, FixedStepSolvers::Rk4, (Operations::Add, Ethos::Bad));
    }

    #[test]
    #[should_panic]
    fn iter_bad_f64() {
        do_step!(f64, FixedStepSolvers::Rk4, (Operations::Iter, Ethos::Bad));
    }

    #[test]
    fn add_good_f32() {
        do_step!(f32, FixedStepSolvers::Rk4, (Operations::Add, Ethos::Good));
    }

    #[test]
    fn iter_good_f32() {
        do_step!(f32, FixedStepSolvers::Rk4, (Operations::Iter, Ethos::Good));
    }

    #[test]
    #[should_panic]
    fn add_bad_f32() {
        do_step!(f32, FixedStepSolvers::Rk4, (Operations::Add, Ethos::Bad));
    }

    #[test]
    #[should_panic]
    fn iter_bad_f32() {
        do_step!(f32, FixedStepSolvers::Rk4, (Operations::Iter, Ethos::Bad));
    }

    #[test]
    fn add_good_f16() {
        do_step!(f16, FixedStepSolvers::Rk4, (Operations::Add, Ethos::Good));
    }

    #[test]
    fn iter_good_f16() {
        do_step!(f16, FixedStepSolvers::Rk4, (Operations::Iter, Ethos::Good));
    }

    #[test]
    #[should_panic]
    fn add_bad_f16() {
        do_step!(f16, FixedStepSolvers::Rk4, (Operations::Add, Ethos::Bad));
    }

    #[test]
    #[should_panic]
    fn iter_bad_f16() {
        do_step!(f16, FixedStepSolvers::Rk4, (Operations::Iter, Ethos::Bad));
    }
}
