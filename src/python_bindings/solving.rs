use crate::concepts::steppers::*;
use crate::solvers::fixed_step::*;

use pyo3::prelude::*;

/// Copied from the pyo3 documentation
/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
pub fn ode_solvers(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve_scalar_ode_rk4, m)?)?;
    m.add_function(wrap_pyfunction!(solve_scalar_ode_euler, m)?)?;
    Ok(())
}


#[pyfunction]
pub fn solve_scalar_ode_euler(
    pyfunc: &PyAny,
    n0: f64,
    steps: usize,
    dt: f64,
    params: Vec<f64>
) -> PyResult<(Vec<f64>, Vec<f64>)>
{
    let euler = Euler{};
    solve_scalar_ode(pyfunc, n0, steps, dt, params, euler)
}


#[pyfunction]
pub fn solve_scalar_ode_rk4(
    pyfunc: &PyAny,
    n0: f64,
    steps: usize,
    dt: f64,
    params: Vec<f64>
) -> PyResult<(Vec<f64>, Vec<f64>)>
{
    let rk4 = RK4{};
    solve_scalar_ode(pyfunc, n0, steps, dt, params, rk4)
}


// TODO this needs optimizations!
pub fn solve_scalar_ode<St: Stepper>(
    pyfunc: &PyAny,
    n0: f64,
    steps: usize,
    dt: f64,
    params: Vec<f64>,
    stepper: St
) -> PyResult<(Vec<f64>, Vec<f64>)>
{
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