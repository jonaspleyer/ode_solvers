// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![feature(test)]

extern crate test;

use ode_integrate::concepts::errors::CalcError;


pub fn rhs_vec(y: &Vec<f64>, dy: &mut Vec<f64>, _t: &f64, p: &f64) -> Result<(), CalcError> {
    for (yi, dyi) in y.iter().zip(dy) {
        *dyi = - p * *yi;
    }
    Ok(())
}


#[cfg(test)]
mod bench_euler {
    use super::*;
    use test::Bencher;
    use ode_integrate::solvers::fixed_step::{Euler};
    use ode_integrate::concepts::steppers::Stepper;

    fn bench_vec(size: u32, iter: u32) {
        let mut y:  Vec<f64> = (1..size).map(f64::from).collect();
    
        let p = 2.0;
    
        let dt = 0.1;
        let mut t = 0.0;
    
        let mut eu = Euler::from((&y, &t, &dt, &p));
    
        for _ in 1..iter {
            eu.do_step_iter(&rhs_vec, &mut y, &t, &dt, &p).unwrap();
            t += dt;
        }
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size______1_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(1,100));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size_____10_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(10,100));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size____100_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(100,100));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size___1000_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(1000,100));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size__10000_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(10000,100));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size_100000_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(100000,100));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size_____10_iter_____10(b: &mut Bencher) {
        b.iter(|| bench_vec(10,10));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size_____10_iter___1000(b: &mut Bencher) {
        b.iter(|| bench_vec(10,1000));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size_____10_iter__10000(b: &mut Bencher) {
        b.iter(|| bench_vec(10,10000));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size_____10_iter_100000(b: &mut Bencher) {
        b.iter(|| bench_vec(10,100000));
    }
}



#[cfg(test)]
mod bench_rk4 {
    use super::*;
    use test::Bencher;
    use ode_integrate::solvers::fixed_step::{RK4};
    use ode_integrate::concepts::steppers::Stepper;

    fn bench_vec(size: u32, iter: u32) {
        let mut y:  Vec<f64> = (1..size).map(f64::from).collect();
    
        let p = 2.0;
    
        let dt = 0.1;
        let mut t = 0.0;
    
        let mut rk4 = RK4::from((&y, &t, &dt, &p));
    
        for _ in 1..iter {
            rk4.do_step_iter(&rhs_vec, &mut y, &t, &dt, &p).unwrap();
            t += dt;
        }
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size______1_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(1,100));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size_____10_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(10,100));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size____100_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(100,100));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size___1000_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(1000,100));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size__10000_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(10000,100));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size_100000_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(100000,100));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size_____10_iter_____10(b: &mut Bencher) {
        b.iter(|| bench_vec(10,10));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size_____10_iter___1000(b: &mut Bencher) {
        b.iter(|| bench_vec(10,1000));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size_____10_iter__10000(b: &mut Bencher) {
        b.iter(|| bench_vec(10,10000));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn vec_size_____10_iter_100000(b: &mut Bencher) {
        b.iter(|| bench_vec(10,100000));
    }
}