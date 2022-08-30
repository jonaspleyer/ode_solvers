// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![feature(test)]

extern crate test;

use ode_integrate::concepts::errors::CalcError;


#[macro_export]
macro_rules! bench_array {
    ($n: expr, $it: expr) => {
        type A = [f64; $n];

        pub fn rhs_arr(y: &A, dy: &mut A, _t: &f64, p: &f64) -> Result<(), CalcError> {
            for (yi, dyi) in y.iter().zip(dy) {
                *dyi = - p * *yi;
            }
            Ok(())
        } 

        {
            let mut y   = [0.0f64; $n];
            let mut dy  = [0.0f64; $n];
            for i in (1..$n) {
                y[i] = i as f64;
            }

            let p               = 2.0;
            
            let dt              = 0.1;
            let mut t           = 0.0;
            let eu              = Euler {};

            for _ in 1..$it {
                eu.do_step_iter(&rhs_arr, &mut y, &mut dy, &t, &dt, &p).unwrap();
                t += dt;
            }
        }
    };
}


#[cfg(test)]
mod bench_euler {
    use super::*;
    use test::Bencher;
    use ode_integrate::solvers::fixed_step::{Euler};
    use ode_integrate::concepts::steppers::Stepper;

    #[bench]
    #[allow(non_snake_case)]
    fn array_size______1_iter____100(b: &mut Bencher) {
        b.iter(|| {bench_array!(1,100); });
    }

    #[bench]
    #[allow(non_snake_case)]
    fn array_size_____10_iter____100(b: &mut Bencher) {
        b.iter(|| {bench_array!(10,100); });
    }

    #[bench]
    #[allow(non_snake_case)]
    fn array_size____100_iter____100(b: &mut Bencher) {
        b.iter(|| {bench_array!(100,100); });
    }

    #[bench]
    #[allow(non_snake_case)]
    fn array_size___1000_iter____100(b: &mut Bencher) {
        b.iter(|| {bench_array!(1000,100); });
    }

    #[bench]
    #[allow(non_snake_case)]
    fn array_size__10000_iter____100(b: &mut Bencher) {
        b.iter(|| {bench_array!(10000,100); });
    }

    #[bench]
    #[allow(non_snake_case)]
    fn array_size_100000_iter____100(b: &mut Bencher) {
        b.iter(|| {bench_array!(100000,100); });
    }

    #[bench]
    #[allow(non_snake_case)]
    fn array_size____100_iter_____10(b: &mut Bencher) {
        b.iter(|| {bench_array!(100,10); });
    }

    #[bench]
    #[allow(non_snake_case)]
    fn array_size____100_iter___1000(b: &mut Bencher) {
        b.iter(|| {bench_array!(100,1000); });
    }

    #[bench]
    #[allow(non_snake_case)]
    fn array_size____100_iter__10000(b: &mut Bencher) {
        b.iter(|| {bench_array!(100,10000); });
    }

    #[bench]
    #[allow(non_snake_case)]
    fn array_size____100_iter_100000(b: &mut Bencher) {
        b.iter(|| {bench_array!(100,100000); });
    }
}