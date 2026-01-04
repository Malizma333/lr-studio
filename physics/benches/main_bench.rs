use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

// TODO benchmarks
// Free falling long length
// Heavy red multiline
// Heavy multirider
// Large single skeleton
// Long line

fn bench_engine_simulation(c: &mut Criterion) {
    for case in LINE_CASES {
        let lines = get_lines(case.step_flags);
        let mut group = c.benchmark_group(format!("grid/add_line/{}", case.name));
        bench_add_lines(&mut group, &lines);
        group.finish();
    }
}

criterion_group!(benches, bench_engine_simulation);
criterion_main!(benches);
