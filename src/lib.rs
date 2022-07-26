// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use pyo3::prelude::*;
use std::ops::Add;


/// Copied from the pyo3 documentation
/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn ode_solvers(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve_scalar_ode_rk4, m)?)?;
    m.add_function(wrap_pyfunction!(solve_scalar_ode_euler, m)?)?;
    Ok(())
}


#[pyfunction]
fn solve_scalar_ode_rk4(pyfunc: &PyAny, n0: f64, steps: usize, dt: f64, params: Vec<f64>) -> PyResult<(Vec<f64>, Vec<f64>)> {
    let rk4 = RK4{};
    solve_scalar_ode(pyfunc, n0, steps, dt, params, rk4)
}


#[pyfunction]
fn solve_scalar_ode_euler(pyfunc: &PyAny, n0: f64, steps: usize, dt: f64, params: Vec<f64>) -> PyResult<(Vec<f64>, Vec<f64>)> {
    let euler = Euler{};
    solve_scalar_ode(pyfunc, n0, steps, dt, params, euler)
}



// TODO this needs optimizations to increase speed
fn solve_scalar_ode<St: Stepper<f64, f64, Vec<f64>, PyErr>>(pyfunc: &PyAny, n0: f64, steps: usize, dt: f64, params: Vec<f64>, stepper: St) -> PyResult<(Vec<f64>, Vec<f64>)> {
    // println!("{}\n{}\n{}\n{:?}", n0, steps, dt, params);
    let func: Py<PyAny> = pyfunc.into();
    let mut y = n0;
    let mut t: f64 = 0.0;
    let mut y_res = vec![y];
    let mut t_res = vec![0.0];

    Python::with_gil(|py| {
        let wrapper = |y: &f64, t: &f64, p: &Vec<f64>| -> Result<f64, PyErr> {
            func.call1(py, (y.clone(), t.clone(), p.clone()))?.extract(py)
        };
        
        // let kwargs = [(key1, val1)].into_py_dict(py);
        for n in 1..steps+1 {
            t = n as f64 * dt;

            y = stepper.do_step(&wrapper, &mut y, &t, &dt, &params)?;
            y_res.push(y);
            t_res.push(t);
        }
        Ok((t_res, y_res))
    })
}


/// Runge-Kutta 4th order stepper
/// The Runge-Kutta 4th order solving scheme works with the following equations
/// y1 = y0 + (⅙) (k1 + 2k2 + 2k3 + k4)
/// k1 = hf(x0, y0)
/// k2 = hf[x0 + (½)h, y0 + (½)k1]
/// k3 = hf[x0 + (½)h, y0 + (½)k2]
/// k4 = hf(x0 + h, y0 + k3)
struct RK4 {}


// Implement the pytonic version of the RK4 stepper
impl Stepper<f64, f64, Vec<f64>, PyErr> for RK4 {
    fn do_step(&self, func: &dyn Fn(&f64, &f64, &Vec<f64>) -> Result<f64, PyErr>, input: &mut f64, t: &f64, dt: &f64, p: &Vec<f64>) -> Result<f64, PyErr> {
        let k1 = dt * func(input, t, p)?;
        let k2 = dt * func(&(*input + 0.5 * k1), &(t + 0.5 * dt), p)?;
        let k3 = dt * func(&(*input + 0.5 * k2), &(t + 0.5 * dt), p)?;
        let k4 = dt * func(&(*input + k3), &(t + dt), p)?;
        return Ok(*input + 1.0/6.0 * (k1 + 2.0 * k2 + 2.0 * k3 + k4));
    }
}


/// Euler Stepper
/// This is more an educational stepper than one that should be generally used to solve equations.
struct Euler {}

// Implement the pytonic version of the RK4 stepper
impl Stepper<f64, f64, Vec<f64>, PyErr> for Euler {
    fn do_step(&self, func: &dyn Fn(&f64, &f64, &Vec<f64>) -> Result<f64, PyErr>, input: &mut f64, t: &f64, dt: &f64, p: &Vec<f64>) -> Result<f64, PyErr> {
        let r = func(input, t, p)?;
        return Ok(*input + dt * r);
    }
}


trait Stepper<V: Add, T: Add, P, Err> {
    fn do_step(&self, func: &dyn Fn(&V, &T, &P) -> Result<V, Err>, input: &mut V, t: &T, dt: &T, p: &P) -> Result<V, Err>;
    // fn do_step(func: &F, in: &V, t: &T, dt: &T) -> V;
}