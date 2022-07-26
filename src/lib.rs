// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use pyo3::prelude::*;


/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn ode_solvers(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(solve_exponential_decay, m)?)?;
    m.add_function(wrap_pyfunction!(test, m)?)?;
    // m.add_function(wrap_pyfunction!(solve_any_function, m)?)?;
    Ok(())
}


/* pub fn solve_any_function(
    pyfunc: &PyAny,
    y0: f64,
    t0: f64,
    dt: f64,
    steps: usize,
    p: &Vec<f64>
) -> PyResult<Vec<Vec<f64>>> {
    let mut y = y0;
    let mut res = Vec::new();
    let rk4 = RK4{};
    let func: Py<PyAny> = pyfunc.into();

    for n in 0..steps {
        let t = n as f64 * dt;

        Python::with_gil(|py| {
            let args = (y, t, p);
            // let r = func.call1(py, args)?;
            // println!("{}", r);
        })
    }
    // for n in 0..steps {
    //     rk4.do_step(&pyfunc, &mut y, &(n as f64 * dt), &dt, &p);
    //     res.push(y);
    // }
    Ok(res)
}*/


#[pyfunction]
fn test(pyfunc: &PyAny, n0: f64, steps: usize, dt: f64, params: Vec<f64>) -> PyResult<()> {
    // println!("{}\n{}\n{}\n{:?}", n0, steps, dt, params);
    let func: Py<PyAny> = pyfunc.into();
    let rk4 = RK4{};
    let mut y = n0;
    let mut t: f64 = 0.0;

    let wrapper = |y: f64, t: f64, p: Vec<f64>| -> f64 {
        return n0;
    };

    Python::with_gil(|py| {
        
        // let kwargs = [(key1, val1)].into_py_dict(py);
        for n in 0..steps {
            t = n as f64 * dt;
            let args = (y, t, params.clone());
            let dy: f64 = func.call1(py, args)?.extract(py)?;

            // rk4.do_step(&wrapper, &mut y, &t, &dt, &params);

            y += dt * dy;
            println!("{}, {}", y, dy);
        }
        Ok(())
    })
}




/*
#[pyfunction]
pub fn solve_exponential_decay(n0: f64, lambda: f64, steps: usize, dt: f64) -> PyResult<Vec<f64>> {
    let mut y = n0;
    let mut res = Vec::new();
    let rk4 = RK4{};
    for n in 0..steps {
        rk4.do_step(&exponential_decay, &mut y, &(n as f64 * dt), &dt, &lambda);
        res.push(y);
    }
    Ok(res)
}


fn exponential_decay(v: &f64, _t: &f64, lambda: &f64) -> f64 {
    return 200. - lambda * v;
}
*/

struct RK4 {
    // y1 = y0 + (⅙) (k1 + 2k2 + 2k3 + k4)
    // k1 = hf(x0, y0)
    // k2 = hf[x0 + (½)h, y0 + (½)k1]
    // k3 = hf[x0 + (½)h, y0 + (½)k2]
    // k4 = hf(x0 + h, y0 + k3)
}


impl Stepper<f64, f64, Vec<f64>> for RK4 {
    fn do_step(&self, func: &dyn Fn(&f64, &f64, &Vec<f64>) -> f64, inout: &mut f64, t: &f64, dt: &f64, p: &Vec<f64>) {
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