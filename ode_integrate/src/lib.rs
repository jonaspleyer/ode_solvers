#![no_std]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![crate_name = "ode_integrate"]
//! The crate ode_integrate is a collection of different solving techniques for
//! general Ordinary Differential Equations (ODEs).
//!
//! The solvers are written with generics such that they can be applied
//! in many scenarios. Python bindings are provided with ode_solvers_python_bindings.
//!
//! ODEs are accepted in the following form:
//! *f(y, dy, t, p)*

extern crate alloc;

mod concepts;
mod methods;
mod solvers;

pub use concepts::*;
pub use methods::*;
pub use solvers::*;
