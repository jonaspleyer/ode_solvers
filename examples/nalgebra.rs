// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use nalgebra::{Vector3,Rotation3};

use ode_solvers::concepts::errors::CalcError;
use ode_solvers::solvers::fixed_step::{Euler};
use ode_solvers::concepts::steppers::Stepper;


fn rhs(y: &Vector3<f64>, dy: &mut Vector3<f64>, _t: &f64, p: &Rotation3<f64>) -> Result<(), CalcError> {
    *dy = p * y;
    Ok(())
}


fn main() {
    let mut y  = Vector3::from([1.0 ,2.0, 3.0]);
    let mut dy = Vector3::from([0.0, 0.0, 0.0]);

    let axis   = Vector3::x_axis();
    let angle  = 0.1;
    let p      = Rotation3::from_axis_angle(&axis, angle);

    let dt     = 0.1;
    let mut t  = 0.0;
    let tmax   = 2.0;

    let eu     = Euler{};

    while t<tmax {
        // do_step(&rhs, &mut y, &mut dy, &t, &dt, &p);
        eu.do_step_iter(&rhs, &mut y, &mut dy, &t, &dt, &p).unwrap();
        println!("{:6.4} y=[{:6.4} {:6.4} {:6.4}]", t, y[0], y[1], y[2]);
        t += dt;
    }
}
