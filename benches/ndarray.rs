// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![feature(test)]

extern crate test;

use ode_solvers::concepts::errors::CalcError;

use ndarray::{Array1};


pub fn rhs_ndarray(y: &Array1<f64>, dy: &mut Array1<f64>, _t: &f64, p: &f64) -> Result<(), CalcError> {
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

    fn bench_vec(size: u32, iter: u32) {
        let mut y: Array1<f64>  = (1..size).map(f64::from).collect();
        let mut dy: Array1<f64> = (1..size).map(f64::from).collect();
    
        let p       = 2.0;
    
        let dt      = 0.1;
        let mut t   = 0.0;
    
        let eu      = Euler {};
    
        for _ in 1..iter {
            eu.do_step_iter(&rhs_ndarray, &mut y, &mut dy, &t, &dt, &p).unwrap();
            t += dt;
        }
    }

    #[bench]
    #[allow(non_snake_case)]
    fn ndarray_size______1_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(1,100));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn ndarray_size_____10_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(10,100));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn ndarray_size____100_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(100,100));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn ndarray_size___1000_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(1000,100));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn ndarray_size__10000_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(10000,100));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn ndarray_size_100000_iter____100(b: &mut Bencher) {
        b.iter(|| bench_vec(100000,100));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn ndarray_size_____10_iter_____10(b: &mut Bencher) {
        b.iter(|| bench_vec(10,10));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn ndarray_size_____10_iter___1000(b: &mut Bencher) {
        b.iter(|| bench_vec(10,1000));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn ndarray_size_____10_iter__10000(b: &mut Bencher) {
        b.iter(|| bench_vec(10,10000));
    }

    #[bench]
    #[allow(non_snake_case)]
    fn ndarray_size_____10_iter_100000(b: &mut Bencher) {
        b.iter(|| bench_vec(10,100000));
    }
}