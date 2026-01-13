#![allow(clippy::expect_used)]
#![allow(clippy::unwrap_used)]
use criterion::{
    BenchmarkGroup, BenchmarkId, Criterion, criterion_group, criterion_main, measurement::WallTime,
};
use lr_format_core::Track;
use lr_physics_engine::{
    PhysicsEngine,
    entity_registry::{EntityTemplateBuilder, RemountVersion},
    line_registry::PhysicsLineBuilder,
};
use lr_physics_grid::GridVersion;
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
    engine: &mut PhysicsEngine,
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

// Repeated from tests, no good way to share functions between tests and benchmarks (at the moment)
fn from_track(track: &Track, lra: bool) -> PhysicsEngine {
    let grid_version = match track.grid_version() {
        lr_format_core::GridVersion::V6_0 => GridVersion::V6_0,
        lr_format_core::GridVersion::V6_1 => GridVersion::V6_1,
        lr_format_core::GridVersion::V6_2 => GridVersion::V6_2,
    };

    let mut engine = PhysicsEngine::new(grid_version);

    for line in track.standard_lines() {
        let physics_line = PhysicsLineBuilder::new(line.endpoints())
            .flipped(line.flipped())
            .left_extension(line.left_extension())
            .right_extension(line.right_extension())
            .height(line.height())
            .acceleration_multiplier(line.multiplier())
            .build();
        engine.add_line(physics_line);
    }

    let template_none_id =
        engine.register_entity_template(EntityTemplateBuilder::default_rider(RemountVersion::None));
    let template_comv1_id = engine
        .register_entity_template(EntityTemplateBuilder::default_rider(RemountVersion::ComV1));
    let template_comv2_id = engine
        .register_entity_template(EntityTemplateBuilder::default_rider(RemountVersion::ComV2));
    let template_lra_id =
        engine.register_entity_template(EntityTemplateBuilder::default_rider(RemountVersion::LRA));

    for rider in track.riders() {
        let template_id = if lra {
            template_lra_id
        } else {
            match rider.remount_version() {
                lr_format_core::RemountVersion::None => template_none_id,
                lr_format_core::RemountVersion::ComV1 => template_comv1_id,
                lr_format_core::RemountVersion::ComV2 => template_comv2_id,
                lr_format_core::RemountVersion::LRA => template_lra_id,
            }
        };

        let entity_id = engine
            .add_entity(template_id)
            .expect("Template id should be valid");

        if let Some(offset) = rider.start_offset() {
            engine
                .set_entity_initial_offset(entity_id, offset)
                .expect("Entity id should be valid");
        }

        if let Some(velocity) = rider.start_velocity() {
            engine
                .set_entity_initial_velocity(entity_id, velocity)
                .expect("Entity id should be valid");
        }
    }

    engine
}

fn bench_engine_simulation(c: &mut Criterion) {
    for benchmark in BENCHMARKS {
        let file_name = format!(
            "../fixtures/lr_physics_engine/benchmarks/{}.track.json",
            benchmark.file
        );
        let file = fs::read(file_name).expect("Failed to read JSON file");
        let track = lr_format_json::read(&file).expect("Failed to parse track file");
        let mut engine = from_track(&track, false);
        let mut group = c.benchmark_group(benchmark.file.to_string());
        bench_view_frame(&mut group, &mut engine, benchmark.target_frame);
        group.finish();
    }
}

criterion_group!(benches, bench_engine_simulation);
criterion_main!(benches);
