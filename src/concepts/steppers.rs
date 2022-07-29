// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

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