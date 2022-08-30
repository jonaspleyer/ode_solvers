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
    let values: [f64; 12] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0];
    let mut y = Array::new(&values, Dim4::new(&[3, 2, 2, 1]));
    let mut dy = y.clone();

    let p = (2.0, y.clone());

    let dt = 0.1;
    let mut t = 0.0;
    let tmax = 500.0;

    let eu = Euler {};

    print(&y);
    while t<tmax {
        
        eu.do_step_add(&rhs_arrayfire, &mut y, &mut dy, &t, &dt, &p).unwrap();
        t += dt;
    }
    print(&y);
}
