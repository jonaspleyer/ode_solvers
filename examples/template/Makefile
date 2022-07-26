# Copyright: Jonas Pleyer
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

libname=ode_solvers

all: run_python_script

compile_release:
	cargo build --release

copy_binary: compile_release
	cp -f ../../target/release/lib$(libname).so ./$(libname).so

run_python_script: copy_binary
	python3 benchmark.py
