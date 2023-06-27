#![no_std]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
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

/// # Abstract Concepts
/// Contains Traits and objects common to every ODE problem.
pub mod concepts;

/// # Solving Algorithms
/// Contains generic implementations of different solving algorithms.
/// When supplied with concrete types, these can solve ODEs.
pub mod solvers;

/// # Quality of life Re-Exports
/// Collection of functions and objects which should suffice to solve any ODE.
pub mod prelude;

/// # User Solving Functions
/// These functions are meant to be called directly and many of them are reexported in the prelude file.
pub mod methods;
