// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use ode_integrate::prelude::*;

type C1 = num::complex::Complex<f32>;


// Defines the RHS of the ODE
fn rhs_complex_f32(x: &C1, dx: &mut C1, _t: &f32, p: &f32) -> Result<(), CalcError>
{
    *dx = - p * *x;
    Ok(())
}


fn main() {
    // Define initial values of ODE
    let x0 = C1::new(10.0, 20.0);
    let p: f32 = 2.0;

    // Define solving times and minilal time-step
    let dt: f32 = 0.1;
    let t_series = [0.0, 0.01, 0.02, 0.03, 0.1, 0.3, 2.0, 2.1, 2.2, 4.0, 4.1, 4.2];

    // Solve equation and print if successful
    match solve_ode_time_series_minimal_step_add(&x0, &t_series, &rhs_complex_f32, &p, Rk4, &dt) {
        Ok(y_res) => {
            for (ti, yi) in t_series.iter().zip(y_res.iter()) {
                println!("t={:6.4} y={:6.4}", ti, yi);
            }
        }
        Err(error) => {
            println!("An error occurred: {error}");
        }
    }
}
