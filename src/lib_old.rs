// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
extern crate cpython;

use cpython::{PyResult, Python, py_module_initializer, py_fn};


py_module_initializer!(ode_solvers, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "get_result", py_fn!(py, get_result(val: &str)))?;
    m.add(py, "calculate_pi", py_fn!(py, calculate_pi(n: u64)))?;
    m.add(py, "solve_exponential_decay", py_fn!(py, solve_exponential_decay(n0: f64, lambda: f64, steps: usize, dt: f64)))?;
    m.add(py, "solve_any_function", py_fn!(py, solve_any_function(pyfunc: &&dyn Fn(&f64, &f64, &f64) -> f64, y0: f64, t0: f64, dt: f64, steps: usize, p: f64)))?;
    Ok(())
});


/// Simple function to see if interaction with Rust is working correctly
pub fn get_result(_py: Python, val: &str) -> PyResult<String> {
    Ok("Rust says: ".to_owned() + val)
}


/// First calculation test. Calculate pi via a simple sum.
pub fn calculate_pi(_py: Python, n: u64) -> PyResult<f64> {
    let mut res = 0_f64;
    for i in 0..n+1 {
        res += nth_pi_sum(i);
    }
    res *= 12_f64.powf(0.5);
    return Ok(res);
}


fn nth_pi_sum(n: u64) -> f64 {
    let m = n as f64;
    (-1_f64/3_f64).powf(m)/(2_f64*m+1_f64)
}


pub fn solve_any_function(
    _py: Python,
    pyfunc: &&dyn Fn(&f64, &f64, &f64) -> f64,
    y0: f64,
    t0: f64,
    dt: f64,
    steps: usize,
    p: f64
) -> PyResult<std::vec::Vec<f64>> {
    let mut y = y0;
    let mut res = std::vec::Vec::new();
    let rk4 = RK4{};
    for n in 0..steps {
        rk4.do_step(&pyfunc, &mut y, &(n as f64 * dt), &dt, &p);
        res.push(y);
    }
    Ok(res)
}


pub fn solve_exponential_decay(_py: Python, n0: f64, lambda: f64, steps: usize, dt: f64) -> PyResult<std::vec::Vec<f64>> {
    let mut y = n0;
    let mut res = std::vec::Vec::new();
    let rk4 = RK4{};
    for n in 0..steps {
        rk4.do_step(&exponential_decay, &mut y, &(n as f64 * dt), &dt, &lambda);
        res.push(y);
    }
    Ok(res)
}


fn exponential_decay(v: &f64, _t: &f64, lambda: &f64) -> f64 {
    return - lambda * v;
}


struct RK4 {
    // y1 = y0 + (⅙) (k1 + 2k2 + 2k3 + k4)
    // k1 = hf(x0, y0)
    // k2 = hf[x0 + (½)h, y0 + (½)k1]
    // k3 = hf[x0 + (½)h, y0 + (½)k2]
    // k4 = hf(x0 + h, y0 + k3)
}


impl Stepper<f64, f64, f64> for RK4 {
    fn do_step(&self, func: &dyn Fn(&f64, &f64, &f64) -> f64, inout: &mut f64, t: &f64, dt: &f64, p: &f64) {
        let k1 = dt * func(inout, t, p);
        let k2 = dt * func(&(*inout + 0.5 * k1), &(t + 0.5 * dt), p);
        let k3 = dt * func(&(*inout + 0.5 * k2), &(t + 0.5 * dt), p);
        let k4 = dt * func(&(*inout + k3), &(t + dt), p);
        *inout += 1.0/6.0 * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
    }
}


trait Stepper<V, T, P> {
    fn do_step(&self, func: &dyn Fn(&V, &T, &P) -> V, inout: &mut V, t: &T, dt: &T, p: &P);
    // fn do_step(func: &F, in: &V, t: &T, dt: &T) -> V;
}