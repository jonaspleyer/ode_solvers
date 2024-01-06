use crate::concepts::ode_def::*;
use crate::concepts::steppers::*;
use crate::solvers::fixed_step::*;

use alloc::boxed::Box;

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
