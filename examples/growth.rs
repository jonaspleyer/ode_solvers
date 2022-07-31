// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use ode_solvers::solvers::fixed_step::*;
use ode_solvers::concepts::steppers::Stepper;

use pyo3::prelude::*;


fn decay_rhs(y: &f64, _t: &f64, p: &[f64; 2]) -> Result<f64,PyErr> {
    return Ok(p[0] * (p[1] - y));
}


fn solve_ode(y0: &f64, t_start: &f64, t_end: &f64, dt: &f64, p: &[f64; 2]) -> (Vec<f64>, Vec<f64>) {
    let mut t = *t_start;

    let mut y = *y0;
    let mut y_all = vec![*y0];
    let mut t_all = vec![*t_start];

    let rk4 = RK4{};
    while t<*t_end {
        y = rk4.do_step(&decay_rhs, &y, &t, &dt, &p).unwrap();
        y_all.push(y);
        t_all.push(t);
        t += dt;
    }
    (y_all, t_all)
}


fn main() {
    // Define start, end and time step
    let t_start = 0.0;
    let t_end   = 30000.0;
    let dt_all  = [0.1];// [0.5, 0.1, 0.05, 0.01, 0.005, 0.001, 0.0005, 0.0001];//, 0.00005, 0.00001];

    // Define parameters and initial values for ode
    let y0      = 10.0;
    let p       = [0.02, y0 / 2.0];

    // Actually solve the ode
    for dt in dt_all {
        // TODO we want to time this command here!
        let (y, t) = solve_ode(&y0, &t_start, &t_end, &dt, &p);
        println!("{:10} {:10}", y.len(), t.len());
    }
}