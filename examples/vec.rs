// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use ode_integrate::concepts::errors::CalcError;
use ode_integrate::solvers::fixed_step::{RK4,Euler};
use ode_integrate::concepts::steppers::Stepper;


fn rhs_vec(y: &Vec<f64>, dy: &mut Vec<f64>, _t: &f64, p: &f64) -> Result<(), CalcError> {
    for (yi, dyi) in y.iter().zip(dy) {
        *dyi = - p * *yi;
    }
    Ok(())
}


fn rhs_arr(y: &[f64; 3], dy: &mut [f64; 3], _t: &f64, p: &f64) -> Result<(), CalcError> {
    dy[0] = -p * y[0];
    dy[1] = -p * y[1];
    dy[2] = -p * y[2];
    Ok(())
}


fn rhs_scalar(y: &f64, dy: &mut f64, _t: &f64, p: &f64) -> Result<(), CalcError> {
    *dy = - p * *y;
    Ok(())
}



fn main() {
    let mut x = 2.0;

    let mut x2 = 2.0;

    let mut y = vec![1.0 ,2.0, 3.0];

    let mut z = [1.0, 2.0, 3.0];

    let p = 2.0;

    let dt = 0.05;
    let mut t = 0.0;
    let tmax = 2.0;

    let mut rk4_x = RK4::from((&x, &t, &dt, &p));
    let mut rk4_y = RK4::from((&y, &t, &dt, &p));
    let mut rk4_z = RK4::from((&z, &t, &dt, &p));

    let mut eu = Euler::from((&x2, &t, &dt, &p));

    while t<tmax {
        println!("t={:6.4} x={:6.4} x2={:6.4} y=[{:6.4} {:6.4} {:6.4}] z=[{:6.4} {:6.4} {:6.4}]", t, x, x2, y[0], y[1], y[2], z[0], z[1], z[2]);
        rk4_x.do_step_add(&rhs_scalar, &mut x, &t, &dt, &p).unwrap();
        eu.do_step_add(&rhs_scalar, &mut x2, &t, &dt, &p).unwrap();
        rk4_y.do_step_iter(&rhs_vec, &mut y, &t, &dt, &p).unwrap();
        rk4_z.do_step_iter(&rhs_arr, &mut z, &t, &dt, &p).unwrap();
        t += dt;
    }
    println!("t={:6.4} x={:6.4} x2={:6.4} y=[{:6.4} {:6.4} {:6.4}] z=[{:6.4} {:6.4} {:6.4}]", t, x, x2, y[0], y[1], y[2], z[0], z[1], z[2]);
}
