// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use ode_integrate::prelude::*;

use arrayfire::{print, Array, Dim4};

fn rhs_arrayfire(
    y: &Array<f64>,
    dy: &mut Array<f64>,
    _t: &f64,
    p: &(f64, Array<f64>),
) -> Result<(), CalcError> {
    *dy = p.1.clone() - p.0 * y.clone();
    Ok(())
}

fn main() {
    // Define sizes for 4-dimensional array object on GPU
    const SIZE1: usize = 4;
    const SIZE2: usize = 10;
    const SIZE3: usize = 2;
    const SIZE4: usize = 2;
    // Total number of entries in the object
    const SIZE: usize = SIZE1 * SIZE2 * SIZE3 * SIZE4;

    // We want to cast values from a constantly sized array
    let mut values: [f64; SIZE] = [0.0; SIZE];
    let target: [f64; SIZE] = [100.0; SIZE];
    for i in 1..SIZE {
        values[i] = i as f64;
    }

    // Create Array on GPU with previously defined initial values
    let y0 = Array::new(
        &values,
        Dim4::new(&[SIZE1 as u64, SIZE2 as u64, SIZE3 as u64, SIZE4 as u64]),
    );

    // Create another object on the GPU as a parameter for the ODE
    let p1 = Array::new(
        &target,
        Dim4::new(&[SIZE1 as u64, SIZE2 as u64, SIZE3 as u64, SIZE4 as u64]),
    );
    let p = (1.4893, p1);

    // Define solving times and steps for which to print out
    let mut t_series = Vec::<f64>::new();
    let solving_steps = 501;
    let dt = 0.001;
    let print_steps = 125;

    // Fill the vector with time values for which to solve
    for n in 0..solving_steps {
        t_series.push(n as f64 * dt);
    }

    // Actually numerically integrate the ODE
    let res = solve_ode_time_series_single_step_add(&y0, &t_series, &rhs_arrayfire, &p, Rk4);

    // Check if solving was successfull and print if so
    match res {
        Ok(y_res) => {
            for (ti, yi) in t_series.iter().zip(y_res.iter()).step_by(print_steps) {
                println!("t={:6.4}", ti);
                print(yi);
            }
        }
        Err(error) => {
            println!("An error occurred: {error}");
        }
    }
}
