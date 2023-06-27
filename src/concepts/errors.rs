// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

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
