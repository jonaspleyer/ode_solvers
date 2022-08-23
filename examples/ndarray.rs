// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use ode_integrate::concepts::errors::CalcError;
use ode_integrate::solvers::fixed_step::{Euler};
use ode_integrate::concepts::steppers::Stepper;

use ndarray::{array,Array1};


fn rhs(y: &Array1<f64>, dy: &mut Array1<f64>, _t: &f64, p: &f64) -> Result<(), CalcError> {
    *dy = - p * y;
    Ok(())
}


fn main() {
    let mut y = array![1.0 ,2.0, 3.0];
    let mut dy = array![0.0, 0.0, 0.0];

    let p = 2.0;

    let dt = 0.1;
    let mut t = 0.0;
    let tmax = 2.0;

    let eu = Euler{};

    while t<tmax {
        // do_step(&rhs, &mut y, &mut dy, &t, &dt, &p);
        eu.do_step_iter(&rhs, &mut y, &mut dy, &t, &dt, &p).unwrap();
        println!("{:6.4} {:6.4}", t, y);
        t += dt;
    }
}
