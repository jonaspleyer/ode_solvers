// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub use core::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

/// # Floating point type
/// This type allows one to arbitrary floating point types and even in theory exact decimal fractions to numerically integrate the ODE.
/// Since some algorithms require the use of constants, we need to be able to map at least from natural numbers to our FloatLikeType
/// The type i8 was chosen since implementations for f64 and f32 were already present.
pub trait FloatLikeType:
    Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + AddAssign
    + SubAssign
    + Neg<Output = Self>
    + core::cmp::PartialOrd<Self>
    + Copy
    + From<i8>
{
}

impl<T> FloatLikeType for T where
    T: Add<Self, Output = Self>
        + Sub<Self, Output = Self>
        + Mul<Self, Output = Self>
        + Div<Self, Output = Self>
        + AddAssign
        + SubAssign
        + Neg<Output = Self>
        + core::cmp::PartialOrd<Self>
        + Copy
        + From<i8>
{
}

/// # Abstract mathematical additive (vector-like) object
/// This type is ment to represent a mathematical type similar to a fixed-size vector in a vector space \\(\vec{v}\in\mathbb{R}^n\\)
/// For a definition look at eg. <https://lyryx.com/first-course-linear-algebra/>
// Hopefully we can in the future use trait aliases: https://github.com/rust-lang/rust/issues/41517
pub trait MathVecLikeType<F>:
    Add<Output = Self> + AddAssign + Clone + Mul<F, Output = Self>
{
}

impl<T, F> MathVecLikeType<F> for T
where
    T: Add<Output = Self> + AddAssign + Clone + Mul<F, Output = Self>,
    F: FloatLikeType,
{
}

/// # RHS of ODE
/// We define how the Right hand Side of an ODE looks like.
pub type RHS<'a, I, F, P, Err> = &'a dyn Fn(&I, &mut I, &F, &P) -> Result<(), Err>;

/// # ODE Definition
/// A Ordinary Differential Equation (ODE) is defined by
/// \begin{align}
///     \frac{dy}{dt} &= f(y, t, p)\\\\
///     y(t_0) &= y_0
/// \end{align}
/// meaning by the right-hand side of the first equation and initial values
/// ```
/// use ode_integrate::concepts::errors::CalcError;
/// use ode_integrate::concepts::ode_def::OdeDefinition;
///
/// fn rhs(y: &Vec<f64>, dy: &mut Vec<f64>, t: &f64, p: &[f64; 2]) -> Result<(), CalcError> {
///     for (yi, dyi) in y.iter().zip(dy.iter_mut()) {
///         *dyi = p[0] - p[1] * (-t).exp() * yi;
///     }
///     Ok(())
/// }
///
///
/// let y0 = vec![1.0, 3.3, 84.4];
/// let t0 = 2.0;
/// let ode_def = OdeDefinition { y0, t0, func: &rhs };
/// ```
#[derive(Clone)]
pub struct OdeDefinition<'a, I, F, P, Err> {
    pub y0: I,
    pub t0: F,
    pub func: RHS<'a, I, F, P, Err>,
}
