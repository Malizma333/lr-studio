use criterion::{
    BenchmarkGroup, BenchmarkId, Criterion, criterion_group, criterion_main, measurement::WallTime,
};
use physics::Engine;
use std::{fs, hint::black_box};

struct EngineBenchmark {
    file: &'static str,
    target_frame: u32,
}

const BENCHMARKS: &[EngineBenchmark] = &[
    EngineBenchmark {
        file: "free_fall",
        target_frame: 2400,
    },
    EngineBenchmark {
        file: "heavy_red_multiline",
        target_frame: 5,
    },
    EngineBenchmark {
        file: "free_fall_multirider",
        target_frame: 3,
    },
    EngineBenchmark {
        file: "multirider_with_track",
        target_frame: 100,
    },
];

fn bench_view_frame(
    group: &mut BenchmarkGroup<'_, WallTime>,
    engine: &mut Engine,
    target_frame: u32,
) {
    let id = BenchmarkId::from_parameter("view_frame");
    group.bench_function(id, |b| {
        b.iter(|| {
            black_box(engine.view_frame(black_box(target_frame)));
            engine.clear_cache();
        });
    });
}

fn bench_engine_simulation(c: &mut Criterion) {
    for benchmark in BENCHMARKS {
        let file_name = format!(
            "../fixtures/physics/benchmarks/{}.track.json",
            benchmark.file
        );
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let track = format_json::read(&file).expect("Failed to parse track file");
        let mut engine = Engine::from_track(track, false);
        let mut group = c.benchmark_group(format!("physics/simulate/{}", benchmark.file));
        bench_view_frame(&mut group, &mut engine, benchmark.target_frame);
        group.finish();
    }
}

criterion_group!(benches, bench_engine_simulation);
criterion_main!(benches);
