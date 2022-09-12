// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![feature(test)]

extern crate test;

use ode_integrate::concepts::errors::CalcError;
use ode_integrate::concepts::ode_def::*;
use ode_integrate::solvers::fixed_step::*;
use ode_integrate::methods::helper_functions::*;


pub fn rhs_vec(y: &Vec<f64>, dy: &mut Vec<f64>, _t: &f64, p: &f64) -> Result<(), CalcError> {
    for (yi, dyi) in y.iter().zip(dy) {
        *dyi = - p * *yi;
    }
    Ok(())
}


fn bench_vec(size: u32, iter: u32, solver: FixedStepSolvers) {
    let mut y:  Vec<f64> = (1..size).map(f64::from).collect();

    let p = 2.0;

    let dt = 0.1;
    let mut t = 0.0;

    let ode_def = OdeDefinition { y0: y.clone(), t0: t.clone(), func: &rhs_vec };

    let mut s = get_fixed_step_stepper(solver, ode_def);

    for _ in 1..iter {
        s.do_step_iter(&mut y, &t, &dt, &p).unwrap();
        t += dt;
    }
}


#[cfg(test)]
mod bench_euler {
    use super::*;
    use test::Bencher;

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size______1_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(1,100, FixedStepSolvers::Euler));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size_____10_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(10,100, FixedStepSolvers::Euler));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size____100_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(100,100, FixedStepSolvers::Euler));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size___1000_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(1000,100, FixedStepSolvers::Euler));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size__10000_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(10000,100, FixedStepSolvers::Euler));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size_100000_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(100000,100, FixedStepSolvers::Euler));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size_____10_iter_____10(b: &mut Bencher) {
        b.iter(|| bench_vec(10,10, FixedStepSolvers::Euler));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size_____10_iter___1000(b: &mut Bencher) {
        b.iter(|| bench_vec(10,1000, FixedStepSolvers::Euler));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size_____10_iter__10000(b: &mut Bencher) {
        b.iter(|| bench_vec(10,10000, FixedStepSolvers::Euler));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size_____10_iter_100000(b: &mut Bencher) {
        b.iter(|| bench_vec(10,100000, FixedStepSolvers::Euler));
    }
}


#[cfg(test)]
#[allow(non_snake_case)]
mod bench___rk4 {
    use super::*;
    use test::Bencher;

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size______1_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(1,100, FixedStepSolvers::Rk4));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size_____10_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(10,100, FixedStepSolvers::Rk4));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size____100_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(100,100, FixedStepSolvers::Rk4));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size___1000_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(1000,100, FixedStepSolvers::Rk4));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size__10000_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(10000,100, FixedStepSolvers::Rk4));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size_100000_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(100000,100, FixedStepSolvers::Rk4));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size_____10_iter_____10(b: &mut Bencher) {
        b.iter(|| bench_vec(10,10, FixedStepSolvers::Rk4));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size_____10_iter___1000(b: &mut Bencher) {
        b.iter(|| bench_vec(10,1000, FixedStepSolvers::Rk4));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size_____10_iter__10000(b: &mut Bencher) {
        b.iter(|| bench_vec(10,10000, FixedStepSolvers::Rk4));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size_____10_iter_100000(b: &mut Bencher) {
        b.iter(|| bench_vec(10,100000, FixedStepSolvers::Rk4));
    }
}
