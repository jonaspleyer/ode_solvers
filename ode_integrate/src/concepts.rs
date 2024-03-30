pub use core::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};
use core::fmt;

use alloc::string::String;

/// # Error while calculating RHS of ODE
/// When the evaluation of the RHS of the ODE
/// \begin{equation}
///     f(y, t, p) = \dots
/// \end{equation}
/// encounters an error, this one should be used.
#[derive(Debug, Clone)]
pub struct CalcError(String);

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Calculation Error occured due to: {}", self.0)
    }
}

impl From<String> for CalcError {
    fn from(string: String) -> Self {
        CalcError(string)
    }
}

impl From<&String> for CalcError {
    fn from(string: &String) -> Self {
        CalcError(string.clone())
    }
}

impl From<&str> for CalcError {
    fn from(string: &str) -> Self {
        CalcError(alloc::format!("{}", string))
    }
}

/// # Error during solving process
/// When the solving process, which depends on the solver used does not produce
/// a result, this error should be used.
#[derive(Debug, Clone)]
pub struct SolvingError(String);

impl fmt::Display for SolvingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Solving Error occurred due to: {}", self.0)
    }
}

impl From<CalcError> for SolvingError {
    fn from(error: CalcError) -> Self {
        SolvingError(error.0)
    }
}

impl From<String> for SolvingError {
    fn from(string: String) -> Self {
        SolvingError(string)
    }
}

impl From<&String> for SolvingError {
    fn from(string: &String) -> Self {
        SolvingError(string.clone())
    }
}

impl From<&str> for SolvingError {
    fn from(string: &str) -> Self {
        SolvingError(alloc::format!("{}", string))
    }
}

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
