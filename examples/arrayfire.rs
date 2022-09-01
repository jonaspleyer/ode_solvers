// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use ode_integrate::concepts::errors::CalcError;
use ode_integrate::solvers::fixed_step::{Euler};
use ode_integrate::concepts::steppers::Stepper;

use arrayfire::{Array, Dim4, print};


fn rhs_arrayfire(y: &Array<f64>, dy: &mut Array<f64>, _t: &f64, p: &(f64, Array<f64>)) -> Result<(), CalcError> {
    *dy = p.1.clone() - p.0 * y.clone();
    Ok(())
}



fn main() {
    const SIZE1: usize = 3;
    const SIZE2: usize = 3;
    const SIZE3: usize = 2;
    const SIZE4: usize = 2;
    const SIZE: usize = SIZE1 * SIZE2 * SIZE3 * SIZE4;
    
    let mut values: [f64; SIZE] = [0.0; SIZE];
    let target: [f64; SIZE] = [100.0; SIZE];
    for i in 1..SIZE {
        values[i] = i as f64;
    }
    let mut y = Array::new(&values, Dim4::new(&[SIZE1 as u64, SIZE2 as u64, SIZE3 as u64, SIZE4 as u64]));

    let p1 = Array::new(&target, Dim4::new(&[SIZE1 as u64, SIZE2 as u64, SIZE3 as u64, SIZE4 as u64]));
    let p = (1.4893, p1);

    let dt = 0.1;
    let mut t = 0.0;
    let tmax = 500.0;

    let mut eu = Euler::from((&y, &t, &dt, &p));

    print(&y);
    while t<tmax {
        
        eu.do_step_add(&rhs_arrayfire, &mut y, &t, &dt, &p).unwrap();
        t += dt;
    }
    print(&y);
}
