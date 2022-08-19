// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// ode_solvers is a collection of different solving techniques for
/// general ODEs. The solvers are written with generics such that they can be applied
/// in most scenarios. Python bindings are provided with [ode_solvers_python_bindings].

pub mod concepts;
pub mod solvers;