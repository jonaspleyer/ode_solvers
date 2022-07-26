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
import matplotlib.pyplot as plt
from scipy.integrate import solve_ivp
import numpy as np
import time

def rhs(y, t, p):
    return p[0] * (p[1] - y)

def rhs2(t, y, p):
    return p[0] * (p[1] - y)


def benchmark_methods(n0, p, steps, dt, verbose=False):
    # Custom solution using rust
    start_time = time.time()
    t_rk4, y_rk4 = ode_solvers.solve_scalar_ode_rk4(rhs, n0, steps, dt, params=p)
    time1 = time.time() - start_time
    if verbose:
        print("Custom Solving RK4:       {:06.4e}".format(time1))
    
    start_time = time.time()
    t_eul, y_eul = ode_solvers.solve_scalar_ode_euler(rhs, n0, steps, dt, params=p)
    time2 = time.time() - start_time
    if verbose:
        print("Custom Solving Euler:     {:06.4e}".format(time2))
    
    # Odeints wrapper using Fotran LSODA
    start_time = time.time()
    t_scipy = np.arange(0, steps*dt, dt)
    time3_5 = time.time() - start_time
    y_scipy = solve_ivp(rhs2, (t_scipy[0], t_scipy[-1]), (n0,), t_eval=t_scipy, args=(p,), method="RK45").y
    time3 = time.time() - start_time
    if verbose:
        print("Custom Solving solve_ivp: {:06.4e} and {:06.4e} for np.arange".format(time3, time3_5))
    return time1, time2, time3, time3_5


def plot_results(t_rk4, y_rk4, t_eul, y_eul, t_scipy, y_scipy):
    plt.plot(t_rk4, y_rk4, label="Runge-Kutta 4th")
    plt.plot(t_eul, y_eul, label="Euler-Method")
    plt.plot(t_scipy, y_scipy.T, label="scipy.integrate.solve_ivp")
    plt.legend()
    plt.show()


if __name__ == "__main__":
    n0 = 1e+1
    p = (2e-2, n0/2.0)
    steps = 3*10**3
    dt = 0.1

    steps_many = 2**np.arange(0, 10)
    times = np.array([benchmark_methods(n0, (2e-4 / n, n0/2.0), steps * n, dt) for n in steps_many])
    plt.plot(steps_many, times[:,0:3], label=["Rust Euler", "Rust RK4", "scipy"])
    plt.xscale("log")
    plt.yscale("log")
    plt.legend()
    plt.show()