use criterion::BenchmarkId;
use criterion::{criterion_group, criterion_main, Criterion};

use ode_integrate::concepts::errors::CalcError;
use ode_integrate::concepts::ode_def::OdeDefinition;
use ode_integrate::methods::helper_functions::*;
use ode_integrate::solvers::fixed_step::*;

#[macro_export]
macro_rules! bench_array {
    ($n: expr, $it: expr, $s: expr) => {
        type A = [f64; $n];

        pub fn rhs_arr(y: &A, dy: &mut A, _t: &f64, p: &f64) -> Result<(), CalcError> {
            for (yi, dyi) in y.iter().zip(dy) {
                *dyi = -p * *yi;
            }
            Ok(())
        }

        let mut y = [0.0f64; $n];
        for i in (1..$n) {
            y[i] = i as f64;
        }

        let p = 2.0;

        let dt = 0.1;
        let mut t = 0.0;
        let ode_def = OdeDefinition {
            y0: y,
            t0: t,
            func: &rhs_arr,
        };

        let mut s = get_fixed_step_stepper($s, ode_def);

        for _ in 1..$it {
            s.do_step_iter(&mut y, &t, &dt, &p).unwrap();
            t += dt;
        }
    };
}

#[macro_export]
macro_rules! bench_many_arrays {
    ($group: expr, $iter_range: expr, $solver: expr, $size:expr) => {
        for (index, iter) in $iter_range.enumerate() {
            let id = BenchmarkId::new(format!("size_{:06.0}_iter_{:06.0}", $size, iter), index);
            $group.bench_with_input(id, &($size, iter),
                |b, _| {
                    b.iter(|| {
                        bench_array!($size, iter, $solver);
                    })
                });
        }
    };

    ($group: expr, $iter_range: expr, $solver: expr, $size:expr, $($sizes:expr),+) => {{
        bench_many_arrays!($group, $iter_range.clone(), $solver, $size);
        bench_many_arrays!($group, $iter_range, $solver, $($sizes),+);
    }};
}

fn bench_euler(c: &mut Criterion) {
    let mut group = c.benchmark_group("array_bench_euler");
    let comb_iter = (0..5).map(|i| 10_i32.pow(i));

    bench_many_arrays!(
        group,
        comb_iter,
        FixedStepSolvers::Euler,
        10,
        100,
        1000,
        10000
    );
    group.finish();
}

fn bench_rk4(c: &mut Criterion) {
    let mut group = c.benchmark_group("array_bench_rk4");
    let comb_iter = (1..6).map(|i| 10_i32.pow(i));

    bench_many_arrays!(
        group,
        comb_iter,
        FixedStepSolvers::Rk4,
        10,
        100,
        1000,
        10000
    );
    group.finish();
}

criterion_group!(benches, bench_euler, bench_rk4);
criterion_main!(benches);
