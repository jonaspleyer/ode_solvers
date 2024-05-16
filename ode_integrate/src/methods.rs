use core::fmt::Display;
use core::ops::Mul;

use crate::concepts::*;
use crate::solvers::{Euler, FixedStepSolvers, Rk4};

use alloc::boxed::Box;
use alloc::{vec, vec::Vec};

/// # Solve ODE for specified time points and single steps in between
/// Solves a ODE supplied via initial parameters and RHS function
/// for the given time points. It uses single steps in between meaning for time points
/// \\(t_0,\dots,t_n\\),
/// the corresponding time intervals will be \\(\textrm{d}t_i = t_{i+1} - t_i\\).
/// This means, the solving routine will do exactly \\(n\\) steps to obtain the results.

/// ## Example
/// First we define the RHS of the ODE \\(f(y, t, p) = \dots\\).
/// Then specify initial values \\(y_0\\), parameters \\(p\\), and time points \\(t_i\\).
/// Afterwards integrate the ODE and display results.
/// ```
/// use ode_integrate::*;
///
/// fn rhs_arr(y: &[f64; 3], dy: &mut [f64; 3], _t: &f64, p: &f64) -> Result<(), CalcError> {
///     dy[0] = -p * y[0];
///     dy[1] = -p * y[1];
///     dy[2] = -p * y[2];
///     Ok(())
/// }
///
/// // Define initial values and parameters for the ODE
/// let y0 = [1.0 ,2.0, 3.0];
/// let p = 2.0;
///
/// // Define the time series on which to solve the ODE
/// let mut t_series = Vec::<f64>::new();
/// for n in 0..50 {
///     t_series.push(n as f64 * 0.01);
/// }
///
/// // Actually numerically integrate the ODE
/// let res = solve_ode_time_series_single_step_iter(&y0, &t_series, &rhs_arr, &p,
/// FixedStepSolvers::Rk4);
///
/// // Check if solving was successfull and print if so
/// match res {
///     Ok(y_res) => {
///         for (ti, yi) in t_series.iter().zip(y_res.iter()) {
///             println!("t={:6.4}, y=[{:6.4} {:6.4} {:6.4}]", ti, yi[0], yi[1], yi[2]);
///         }
///     }
///     Err(error) => {
///         println!("An error occurred: {error}");
///     }
/// }
/// ```
// TODO find way to specify the solver. This should be a common interface.
// TODO add function for additive object
pub fn solve_ode_time_series_single_step_iter<'a, I, F, P, E, V>(
    y0: &I,
    t_series: &V,
    rhs: RHS<'a, I, F, P, E>,
    p: &P,
    solver_type: FixedStepSolvers,
) -> Result<Vec<I>, SolvingError>
where
    for<'m> &'m mut I: IntoIterator<Item = &'m mut F>,
    for<'m> &'m I: IntoIterator<Item = &'m F>,
    I: Clone,
    F: FloatLikeType,
    P: Clone,
    E: Display + Clone,
    for<'m> &'m V: IntoIterator<Item = &'m F>,
{
    let t_i = t_series.into_iter().next();
    let t0 = match t_i {
        Some(t) => t,
        None => return Err(SolvingError::from("Did not supply enough time steps.")),
    };
    let ode_def = OdeDefinition {
        y0: y0.clone(),
        t0: *t0,
        func: rhs,
    };

    let mut stepper = get_fixed_step_stepper(solver_type, ode_def);
    let mut y = y0.clone();

    // TODO In the future use the method: with_capacity(t_series.len())
    // This is currently not possible since len() is a function inherent to std::Vec and not any trait.
    let mut y_res = vec![y0.clone()];

    let mut dt: F;
    let mut t_further = t_series.into_iter();
    t_further.next();
    for (t_i, t_j) in t_series.into_iter().zip(t_further) {
        dt = *t_j - *t_i;
        if dt < F::from(0) {
            return Err(SolvingError::from("Time steps need to be increasing"));
        }
        match stepper.do_step_iter(&mut y, t_i, &dt, p) {
            Ok(()) => (),
            Err(error) => return Err(SolvingError::from(alloc::format!("{error}"))),
        }
        y_res.push(y.clone());
    }
    Ok(y_res)
}

pub fn solve_ode_time_series_single_step_add<'a, I, F, P, E, V>(
    y0: &I,
    t_series: &V,
    rhs: RHS<'a, I, F, P, E>,
    p: &P,
    solver_type: FixedStepSolvers,
) -> Result<Vec<I>, SolvingError>
where
    I: MathVecLikeType<F>,
    F: FloatLikeType + Mul<I, Output = I>,
    P: Clone,
    E: Display + Clone,
    for<'m> &'m V: IntoIterator<Item = &'m F>,
{
    let t_i = t_series.into_iter().next();
    let t0 = match t_i {
        Some(t) => t,
        None => return Err(SolvingError::from("Did not supply enough time steps.")),
    };
    let ode_def = OdeDefinition {
        y0: y0.clone(),
        t0: *t0,
        func: rhs,
    };

    let mut stepper = get_fixed_step_stepper(solver_type, ode_def);
    let mut y = y0.clone();

    // TODO In the future use the method: with_capacity(t_series.len())
    // This is currently not possible since len() is a function inherent to std::Vec and not any trait.
    let mut y_res = vec![y0.clone()];

    let mut dt: F;
    let mut t_further = t_series.into_iter();
    t_further.next();
    for (t_i, t_j) in t_series.into_iter().zip(t_further) {
        dt = *t_j - *t_i;
        if dt < F::from(0) {
            return Err(SolvingError::from("Time steps need to be increasing"));
        }
        match stepper.do_step_add(&mut y, t_i, &dt, p) {
            Ok(()) => (),
            Err(error) => return Err(SolvingError::from(alloc::format!("{error}"))),
        }
        y_res.push(y.clone());
    }
    Ok(y_res)
}

/// ## Example
/// First we define the RHS of the ODE \\(f(y, t, p) = \dots\\).
/// Then specify initial values \\(y_0\\), parameters \\(p\\), and time points \\(t_i\\).
/// Afterwards integrate the ODE and display results.
/// ```
/// use ode_integrate::*;
///
/// fn rhs_arr(y: &[f64; 3], dy: &mut [f64; 3], _t: &f64, p: &f64) -> Result<(), CalcError> {
///     dy[0] = -p * y[0];
///     dy[1] = -p * y[1];
///     dy[2] = -p * y[2];
///     Ok(())
/// }
///
/// // Define initial values and parameters for the ODE
/// let y0 = [1.0 ,2.0, 3.0];
/// let p = 2.0;
///
/// // Define the time series on which to solve the ODE
/// let mut t_series = Vec::<f64>::new();
/// for n in 0..50 {
///     t_series.push(n as f64 * 0.01);
/// }
///
/// // Actually numerically integrate the ODE
/// let res = solve_ode_time_series_minimal_step_iter(&y0, &t_series, &rhs_arr, &p,
/// FixedStepSolvers::Rk4, &0.004);
///
/// // Check if solving was successfull and print if so
/// match res {
///     Ok(y_res) => {
///         for (ti, yi) in t_series.iter().zip(y_res.iter()) {
///             println!("t={:6.4}, y=[{:6.4} {:6.4} {:6.4}]", ti, yi[0], yi[1], yi[2]);
///         }
///     }
///     Err(error) => {
///         println!("An error occurred: {error}");
///     }
/// }
/// ```
pub fn solve_ode_time_series_minimal_step_iter<'a, I, F, P, E, V>(
    y0: &I,
    t_series: &V,
    rhs: RHS<'a, I, F, P, E>,
    p: &P,
    solver_type: FixedStepSolvers,
    dt: &F,
) -> Result<Vec<I>, SolvingError>
where
    for<'m> &'m mut I: IntoIterator<Item = &'m mut F>,
    for<'m> &'m I: IntoIterator<Item = &'m F>,
    I: Clone,
    F: FloatLikeType + core::fmt::Debug,
    P: Clone,
    E: Display + Clone,
    for<'m> &'m V: IntoIterator<Item = &'m F>,
{
    let t_initial = t_series.into_iter().next();
    let t0 = match t_initial {
        Some(t) => t,
        None => return Err(SolvingError::from("Did not supply enough time steps.")),
    };
    let ode_def = OdeDefinition {
        y0: y0.clone(),
        t0: *t0,
        func: rhs,
    };

    let mut stepper = get_fixed_step_stepper(solver_type, ode_def);
    let mut y = y0.clone();

    // TODO In the future use the method: with_capacity(t_series.len())
    // This is currently not possible since len() is a function inherent to std::Vec and not any trait.
    let mut y_res = vec![y0.clone()];

    let mut dtau: F;
    let mut t_further = t_series.into_iter();
    t_further.next();
    for (t_i, t_j) in t_series.into_iter().zip(t_further) {
        let mut t = *t_i;
        while t < *t_j {
            if *dt > *t_j - t {
                dtau = *t_j - t;
            } else {
                dtau = *dt;
            }
            if dtau < F::from(0) {
                return Err(SolvingError::from("Time steps need to be increasing"));
            }
            // Do step and save
            match stepper.do_step_iter(&mut y, &t, &dt, p) {
                Ok(()) => (),
                Err(error) => return Err(SolvingError::from(alloc::format!("{error}"))),
            }
            t += dtau;
        }
        y_res.push(y.clone());
    }
    Ok(y_res)
}

pub fn solve_ode_time_series_minimal_step_add<'a, I, F, P, E, V>(
    y0: &I,
    t_series: &V,
    rhs: RHS<'a, I, F, P, E>,
    p: &P,
    solver_type: FixedStepSolvers,
    dt: &F,
) -> Result<Vec<I>, SolvingError>
where
    I: MathVecLikeType<F>,
    F: FloatLikeType + Mul<I, Output = I>,
    P: Clone,
    E: Display + Clone,
    for<'m> &'m V: IntoIterator<Item = &'m F>,
{
    let t_initial = t_series.into_iter().next();
    let t0 = match t_initial {
        Some(t) => t,
        None => return Err(SolvingError::from("Did not supply enough time steps.")),
    };
    let ode_def = OdeDefinition {
        y0: y0.clone(),
        t0: *t0,
        func: rhs,
    };

    let mut stepper = get_fixed_step_stepper(solver_type, ode_def);
    let mut y = y0.clone();

    // TODO In the future use the method: with_capacity(t_series.len())
    // This is currently not possible since len() is a function inherent to std::Vec and not any trait.
    let mut y_res = vec![y0.clone()];

    let mut dtau: F;
    let mut t_further = t_series.into_iter();
    t_further.next();
    for (t_i, t_j) in t_series.into_iter().zip(t_further) {
        let mut t = *t_i;
        while t < *t_j {
            if *dt > *t_j - t {
                dtau = *t_j - t;
            } else {
                dtau = *dt;
            }
            if dtau < F::from(0) {
                return Err(SolvingError::from("Time steps need to be increasing"));
            }
            // Do step and save
            match stepper.do_step_add(&mut y, &t, &dt, p) {
                Ok(()) => (),
                Err(error) => return Err(SolvingError::from(alloc::format!("{error}"))),
            }
            t += dtau;
        }
        y_res.push(y.clone());
    }
    Ok(y_res)
}

/// # Initializes fixed size stepper from argument
/// Helper function to obtain a Stepper Trait Object from the enum of steppers
pub fn get_fixed_step_stepper<'a, I, F, P, E>(
    solver_type: FixedStepSolvers,
    ode_def: OdeDefinition<'a, I, F, P, E>,
) -> Box<dyn Stepper<I, F, P, E> + 'a>
where
    I: Clone,
    F: FloatLikeType,
    P: Clone,
    E: Clone,
{
    match solver_type {
        FixedStepSolvers::Euler => Box::new(Euler::from(ode_def)) as Box<dyn Stepper<I, F, P, E>>,
        FixedStepSolvers::Rk4 => Box::new(Rk4::from(ode_def)) as Box<dyn Stepper<I, F, P, E>>,
    }
}
