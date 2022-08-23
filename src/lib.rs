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

pub mod concepts;
pub mod solvers;