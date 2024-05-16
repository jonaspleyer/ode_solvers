use nalgebra::{Rotation3, Vector3};

use ode_integrate::*;

// Defines the RHS of the ODE
fn rhs(
    y: &Vector3<f64>,
    dy: &mut Vector3<f64>,
    _t: &f64,
    p: &Rotation3<f64>,
) -> Result<(), CalcError> {
    *dy = p * y;
    Ok(())
}

fn main() {
    // Define initial values of ODE
    let y0 = Vector3::from([1.0, 2.0, 3.0]);

    // Define parameters for the ODE
    let axis = Vector3::x_axis();
    let angle = 0.1;
    let p = Rotation3::from_axis_angle(&axis, angle);

    // Create a vector with times for which to get output
    let dt = 0.1;
    let steps: usize = 10;
    let t0 = 0.0;
    let mut t_series = Vec::with_capacity(steps);

    for n in 0..steps {
        t_series.push(t0 + n as f64 * dt);
    }

    // Solve the ODE for the times defined
    let res =
        solve_ode_time_series_single_step_iter(&y0, &t_series, &rhs, &p, FixedStepSolvers::Rk4);

    // Print output
    match res {
        Ok(y_res) => {
            for yi in y_res {
                println!("{}", yi);
            }
        }
        Err(e) => println!("{e}"),
    }
}
