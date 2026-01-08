use criterion::{
    BenchmarkGroup, BenchmarkId, Criterion, criterion_group, criterion_main, measurement::WallTime,
};
use geometry::{Line, Point};
use lr_physics_grid::{Grid, GridVersion};
use std::hint::black_box;

struct GridBenchmark {
    name: &'static str,
    line_flags: u8,
}

const BENCHMARKS: &[GridBenchmark] = &[
    GridBenchmark {
        name: "first_point_at_origin",
        line_flags: 0b0011,
    },
    GridBenchmark {
        name: "second_point_at_origin",
        line_flags: 0b1100,
    },
    GridBenchmark {
        name: "both_points_at_origin",
        line_flags: 0b0000,
    },
    GridBenchmark {
        name: "horizontal_with_origin",
        line_flags: 0b1010,
    },
    GridBenchmark {
        name: "vertical_with_origin",
        line_flags: 0b0101,
    },
    GridBenchmark {
        name: "everywhere",
        line_flags: 0b1111,
    },
];

fn get_lines(flags: u8) -> Vec<Line> {
    let range = (-140..140).step_by(7);
    let mut lines = Vec::new();
    for x1 in range.clone() {
        for y1 in range.clone() {
            for x2 in range.clone() {
                for y2 in range.clone() {
                    let x1 = if flags & 0b1000 != 0 { x1 } else { 0 };
                    let y1 = if flags & 0b0100 != 0 { y1 } else { 0 };
                    let x2 = if flags & 0b0010 != 0 { x2 } else { 0 };
                    let y2 = if flags & 0b0001 != 0 { y2 } else { 0 };
                    lines.push(Line::new(
                        Point::new(f64::from(x1), f64::from(y1)),
                        Point::new(f64::from(x2), f64::from(y2)),
                    ));
                }
            }
        }
    }
    lines
}

fn bench_add_lines(group: &mut BenchmarkGroup<'_, WallTime>, lines: &[Line]) {
    for version in [GridVersion::V6_0, GridVersion::V6_1, GridVersion::V6_2] {
        let id = BenchmarkId::from_parameter(version.to_string());

        group.bench_function(id, |b| {
            let mut index = 0;
            b.iter(|| {
                let mut grid = Grid::new(version);
                grid.add_line(black_box(lines[index % lines.len()]));
                index += 1;
                black_box(grid)
            })
        });
    }
}

fn bench_grid_add_line(c: &mut Criterion) {
    for benchmark in BENCHMARKS {
        let lines = get_lines(benchmark.line_flags);
        let mut group = c.benchmark_group(format!("grid/add_line/{}", benchmark.name));
        bench_add_lines(&mut group, &lines);
        group.finish();
    }
}

criterion_group!(benches, bench_grid_add_line);
criterion_main!(benches);
