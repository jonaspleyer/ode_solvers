// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::concepts::ode_def::*;

/// # Steppers
/// This trait allows increasing the current value of an ODE to the next time step via differnt methods.
/// If the inspected object is iterable, we can update the contents by iterating over individual elements.
/// This algorithm intrinsically assumes that by continuous iteration of the type I, order of variables remains unchanged.
/// While the update step is done this way, the function \\(f(y, t, p)\\) can still be specified arbitrarily.
///
/// The second method is for an additive type I. Here, we do not iterate over individual elements but assume that the type can be easily
// TODO is this copy or clone?
/// copied/cloned and thus the ODE integrated this way.
// TODO consider using slices instead of iterators https://users.rust-lang.org/t/solved-function-taking-slice-of-objects-as-well-as-slice-of-references-to-objects/13553/2
pub trait Stepper<I, F, P, Err> {
    fn do_step_iter(&mut self, y: &mut I, t: &F, dt: &F, p: &P) -> Result<(), Err>
    where
        for<'m> &'m mut I: IntoIterator<Item = &'m mut F>,
        for<'m> &'m I: IntoIterator<Item = &'m F>,
        F: FloatLikeType;

    fn do_step_add(&mut self, y: &mut I, t: &F, dt: &F, p: &P) -> Result<(), Err>
    where
        I: MathVecLikeType<F>,
        F: FloatLikeType + Mul<I, Output = I>;
}

pub trait AdaptiveStepper<I, F, P, Err> {
    fn do_step_iter(&mut self, y: &mut I, t: &F, dt: &F, p: &P) -> Result<Option<F>, Err>
    where
        for<'m> &'m mut I: IntoIterator<Item = &'m mut F>,
        for<'m> &'m I: IntoIterator<Item = &'m F>,
        F: FloatLikeType;

    fn do_step_add(&mut self, y: &mut I, t: &F, dt: &F, p: &P) -> Result<Option<F>, Err>
    where
        I: MathVecLikeType<F>,
        F: FloatLikeType + Mul<I, Output = I>;
}
