use std::ops::{Add,Mul};

pub trait Stepper {
    fn do_step<V, T, P, Err>(
        &self,
        func: &dyn Fn(&V, &T, &P) -> Result<V, Err>,
        input: &V,
        t: &T,
        dt: &T,
        p: &P
    ) -> Result<V, Err>
    where
        V: Add<Output=V> + Copy + Mul<f64, Output=V>,
        T: Add<Output=T> + Copy + Mul<f64, Output=T> + Mul<V, Output=V>,
        f64: Mul<V, Output=V> + Mul<T, Output=T>,;
}