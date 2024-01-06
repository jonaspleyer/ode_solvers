use ode_integrate::concepts::errors::CalcError;
use ode_integrate::concepts::ode_def::OdeDefinition;
use ode_integrate::concepts::steppers::Stepper;
use ode_integrate::solvers::fixed_step::Rk4;

use approx::ulps_eq;

/// # Scalar exponential decay
/// Exact solution of the exponential decay differential equation depending on the initial value $y_0$
/// \begin{equation}
///     y(t)  = y_0 \exp\left(-p t\right)
/// \end{equation}
fn exponential_decay_f64_exact(y0: &f64, t: &f64, p: &f64) -> f64 {
    y0 * (-p * t).exp()
}

/// ODE Right hand side of the exponential decay given by
/// \begin{equation}
///     f(y, t, p) = -p \times y
/// \end{equation}
fn rhs_exp_decay(y: &f64, dy: &mut f64, _t: &f64, p: &f64) -> Result<(), CalcError> {
    *dy = -*p * *y;
    Ok(())
}

#[test]
fn exponential_decay_f64() {
    // Define initial parameters for exponential decay
    let x0 = 93.3477397479;
    let mut x = x0.clone();

    // Dummy variables to store previous results and variables for error estimate
    let mut x_prev = 0.0;
    let mut x_pprev;
    let mut x_double_step;

    // Decay parameter
    let p = 1.7946939332;

    // Time definitions
    let mut t = 0.0;
    let dt = 0.005;
    let iter = 100;

    // Solver type
    let ode_def = OdeDefinition {
        y0: x0,
        t0: t,
        func: &rhs_exp_decay,
    };
    let mut rk4 = Rk4::from(ode_def);

    // Solve the ode numerically and test at each step difference to exact result
    for n in 1..iter {
        // Do the numerical integration
        rk4.do_step_add(&mut x, &t, &dt, &p).unwrap();

        // Error-Estimate:
        // Additionally integrate from two steps previously in a larger step 2*dt and compare results
        // See: https://math.stackexchange.com/questions/2176843/how-to-obtain-error-bound-on-runge-kutta-4th-order-method
        x_pprev = x_prev;
        x_prev = x;
        if n > 2 {
            // Integrate the variable x_double_step with double the stepsize 2.0*dt
            // from the result of the second last calculation
            // such that the new value is at the same time point as the current one.
            x_double_step = x_pprev;
            rk4.do_step_add(&mut x_double_step, &t, &(2.0 * dt), &p)
                .unwrap();

            // Actually calculate the error estimate
            // TODO get a good source for this error
            let e = (x_pprev - x_double_step) / 30.0;

            // Calculate the exact result
            let y = exponential_decay_f64_exact(&x0, &(t + dt), &p);

            // Check if the difference is smaller than the error
            assert!(ulps_eq!(&y, &x, epsilon = e));
        }
        t += dt;
    }
}
