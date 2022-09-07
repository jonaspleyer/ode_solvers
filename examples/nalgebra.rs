// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use nalgebra::{Vector3,Rotation3};

use ode_integrate::prelude::*;


fn rhs(y: &Vector3<f64>, dy: &mut Vector3<f64>, _t: &f64, p: &Rotation3<f64>) -> Result<(), CalcError> {
    *dy = p * y;
    Ok(())
}


fn main() {
    let y0 = Vector3::from([1.0 ,2.0, 3.0]);

    let axis = Vector3::x_axis();
    let angle = 0.1;
    let p = Rotation3::from_axis_angle(&axis, angle);

    let dt = 0.1;
    let steps: usize = 10;
    let t0 = 0.0;
    let mut t_series = Vec::with_capacity(steps);

    for n in 0..steps {
        t_series.push(t0 + n as f64 * dt);
    }

    let res = solve_ode_time_series_single_step_iter(&y0, &t_series, &rhs, &p, Rk4);

    match res {
        Ok(y_res) => {
            for yi in y_res {
                println!("{}", yi);
            }
        },
        Err(e) => println!("{e}"),
    }
}
