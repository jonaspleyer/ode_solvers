// Copyright: Jonas Pleyer
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::error::Error;
use std::fmt;

/// # Error while calculating RHS of ODE
/// When the evaluation of the RHS of the ODE
/// \begin{equation}
///     f(y, t, p) = \dots
/// \end{equation}
/// encounters an error, this one should be used.
#[derive(Debug,Clone)]
pub struct CalcError {
    message: String,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Calculation Error occured due to: {}", self.message)
    }
}

impl Error for CalcError {}

impl<'a> From<&'a str> for CalcError {
    fn from(string: &'a str) -> Self {
        CalcError {
            message: string.to_owned(),
        }
    }
}

impl From<String> for CalcError {
    fn from(string: String) -> Self {
        CalcError {
            message: string,
        }
    }
}


/// # Error during solving process
/// When the solving process, which depends on the solver used does not produce
/// a result, this error should be used.
#[derive(Debug,Clone)]
pub struct SolvingError {
    message: String,
}

impl fmt::Display for SolvingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Solving Error occurred due to: {}", self.message)
    }
}

impl Error for SolvingError {}

impl From<CalcError> for SolvingError {
    fn from(error: CalcError) -> Self {
        SolvingError {
            message: error.message,
        }
    }
}

impl<'a> From<&'a str> for SolvingError {
    fn from(string: &'a str) -> Self {
        SolvingError {
            message: string.to_owned(),
        }
    }
}

impl From<String> for SolvingError {
    fn from(string: String) -> Self {
        SolvingError {
            message: string,
        }
    }
}