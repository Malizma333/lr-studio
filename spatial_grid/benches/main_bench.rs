use std::hint::black_box;

use criterion::{
    BenchmarkGroup, BenchmarkId, Criterion, criterion_group, criterion_main, measurement::WallTime,
};
use geometry::Line;
use spatial_grid::{Grid, GridVersion};
use vector2d::Vector2Df;

#[inline]
fn perform_test(mut group: BenchmarkGroup<'_, WallTime>, lines: Vec<Line>) {
    let mut index = 0;

    for version in [GridVersion::V6_0, GridVersion::V6_1, GridVersion::V6_2] {
        group.bench_function(BenchmarkId::from_parameter(version.to_string()), |b| {
            b.iter(|| {
                index += 1;
                let mut grid = Grid::new(version);
                grid.add_line(black_box(&lines[index % lines.len()]));
                black_box(grid)
            });
        });
    }

    group.finish();
}

#[inline]
fn get_lines(step_x1: bool, step_y1: bool, step_x2: bool, step_y2: bool) -> Vec<Line> {
    let range = (-140..140).step_by(7);
    let mut lines = Vec::new();
    for x1 in range.clone() {
        for y1 in range.clone() {
            for x2 in range.clone() {
                for y2 in range.clone() {
                    let x1 = if step_x1 { f64::from(x1) } else { 0.0 };
                    let y1 = if step_y1 { f64::from(y1) } else { 0.0 };
                    let x2 = if step_x2 { f64::from(x2) } else { 0.0 };
                    let y2 = if step_y2 { f64::from(y2) } else { 0.0 };
                    lines.push(Line::new(Vector2Df::new(x1, y1), Vector2Df::new(x2, y2)));
                }
            }
        }
    }
    lines
}

fn add_line_at_origin(c: &mut Criterion) {
    let group = c.benchmark_group("grid_add_line_at_origin");
    let lines = get_lines(false, false, true, true);
    perform_test(group, lines);
}

fn add_line_at_origin_flipped(c: &mut Criterion) {
    let group = c.benchmark_group("grid_add_line_at_origin_flipped");
    let lines = get_lines(true, true, false, false);
    perform_test(group, lines);
}

fn add_line_horizontal(c: &mut Criterion) {
    let group = c.benchmark_group("grid_add_line_horizontal");
    let lines = get_lines(true, false, true, false);
    perform_test(group, lines);
}

fn add_line_vertical(c: &mut Criterion) {
    let group = c.benchmark_group("grid_add_line_vertical");
    let lines = get_lines(false, true, false, true);
    perform_test(group, lines);
}

fn add_line_everywhere(c: &mut Criterion) {
    let group = c.benchmark_group("grid_add_line_everywhere");
    let lines = get_lines(true, true, true, true);
    perform_test(group, lines);
}

criterion_group!(
    benches,
    add_line_at_origin,
    add_line_at_origin_flipped,
    add_line_horizontal,
    add_line_vertical,
    add_line_everywhere
);
criterion_main!(benches);
