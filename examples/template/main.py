#!/usr/bin/env python3

# Copyright: Jonas Pleyer
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

# IMPORTANT NOTICE!
# THIS SCRIPT ASSUMES THAT THE TARGET BINARY HAS BEEN 
# COMPILED AND IS PRESENT IN THE CURRENT DIRECTORY!
# IF THIS IS NOT THE CASE REFER TO THE README.md FILE
# AFTERWARDS MOVE THE BINARY mylib.so INTO THIS DIRECTORY
# THIS PYTHON FILE CAN NOT BE BLINDLY COPIED YET
# 
# USE THE ATTACHED MAKEFILE TO GENERATE THE NECESSARY BINARY

import ode_solvers


def rhs(y, t, p):
    return p[0] - p[1]* y


if __name__ == "__main__":
    n0 = 1e+1
    p = (1.0, 2.0)
    steps = 3
    dt = 0.01

    ode_solvers.test(rhs, n0, steps, dt, params=p)