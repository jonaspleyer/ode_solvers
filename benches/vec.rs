// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![feature(test)]

extern crate test;

use ode_solvers::concepts::errors::CalcError;


pub fn rhs_vec(y: &Vec<f64>, dy: &mut Vec<f64>, _t: &f64, p: &f64) -> Result<(), CalcError> {
    for (yi, dyi) in y.iter().zip(dy) {
        *dyi = - p * *yi;
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use ode_solvers::solvers::fixed_step::{Euler};
    use ode_solvers::concepts::steppers::Stepper;

    fn bench_vec_size(n: u16) {
        let mut y:  Vec<f64> = (1..n).map(f64::from).collect();
        let mut dy: Vec<f64> = (1..n).map(f64::from).collect();
    
        let p = 2.0;
    
        let dt = 0.1;
        let mut t = 0.0;
    
        let eu = Euler {};
    
        for _ in 1..100 {
            eu.do_step_iter(&rhs_vec, &mut y, &mut dy, &t, &dt, &p).unwrap();
            t += dt;
        }
    }

    #[bench]
    fn vec_size_1(b: &mut Bencher) {
        b.iter(|| bench_vec_size(1));
    }

    #[bench]
    fn vec_size_10(b: &mut Bencher) {
        b.iter(|| bench_vec_size(10));
    }

    #[bench]
    fn vec_size_100(b: &mut Bencher) {
        b.iter(|| bench_vec_size(100));
    }

    #[bench]
    fn vec_size_1000(b: &mut Bencher) {
        b.iter(|| bench_vec_size(1000));
    }

    fn bench_vec_iter(n: u16) {
        let mut y:  Vec<f64> = (1..10).map(f64::from).collect();
        let mut dy: Vec<f64> = (1..10).map(f64::from).collect();
    
        let p = 2.0;
    
        let dt = 0.1;
        let mut t = 0.0;
    
        let eu = Euler {};
    
        for _ in 1..n {
            eu.do_step_iter(&rhs_vec, &mut y, &mut dy, &t, &dt, &p).unwrap();
            t += dt;
        }
    }

    #[bench]
    fn vec_iter_10(b: &mut Bencher) {
        b.iter(|| bench_vec_iter(10));
    }

    #[bench]
    fn vec_iter_100(b: &mut Bencher) {
        b.iter(|| bench_vec_iter(100));
    }

    #[bench]
    fn vec_iter_1000(b: &mut Bencher) {
        b.iter(|| bench_vec_iter(1000));
    }

    #[bench]
    fn vec_iter_10000(b: &mut Bencher) {
        b.iter(|| bench_vec_iter(10000));
    }
}