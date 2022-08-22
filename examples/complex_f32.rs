// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use ode_solvers::concepts::errors::CalcError;
use ode_solvers::solvers::fixed_step::{Euler};
use ode_solvers::concepts::steppers::Stepper;

type C1 = num::complex::Complex<f32>;


fn rhs_complex_f32(x: &C1, dx: &mut C1, _t: &f32, p: &f32) -> Result<(), CalcError>
{
    *dx = - p * *x;
    Ok(())
}


fn solve_complex_ode_f32() {
    let mut x       = num::complex::Complex::new(10.0f32, 20.0f32);
    let mut dx      = num::complex::Complex::new(0.0f32, 0.0f32);

    let p: f32      = 2.0;

    let dt: f32     = 0.1;
    let mut t: f32  = 0.0;
    let tmax: f32   = 2.0;

    let eu          = Euler {};

    while t<tmax {
        println!("t={:6.4} x={:6.4}", t, x);
        eu.do_step_add(&rhs_complex_f32, &mut x, &mut dx, &t, &dt, &p).unwrap();
        t += dt;
    }
    println!("t={:6.4} x={:6.4}", t, x);
}


fn main() {
    solve_complex_ode_f32();
}
