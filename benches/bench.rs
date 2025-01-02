#[allow(unused_imports)]
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};

extern crate game_of_life;

#[allow(dead_code)]
fn tick_benchmark(c: &mut Criterion) {
    let (width, height) = (800, 800);
    let mut universe = game_of_life::Universe::new(width, height);

    c.bench_function(
        "tick", 
        |b| b.iter(|| {
            universe.tick();
    }));
}

#[allow(dead_code)]
fn live_neighbours_benchmark(c: &mut Criterion) {
    let (width, height) = (200, 200);
    let universe = game_of_life::Universe::new(width, height);
    let cells = universe.get_cells();

    c.bench_function(
        "live_neighbours",
        |b| b.iter(|| {
            game_of_life::Universe::live_neighbour_count(width, height, cells, black_box(0), black_box(0));
            game_of_life::Universe::live_neighbour_count(width, height, cells, black_box(0), black_box(2));
            game_of_life::Universe::live_neighbour_count(width, height, cells, black_box(2), black_box(0));
            game_of_life::Universe::live_neighbour_count(width, height, cells, black_box(height - 1), black_box(width - 1));
        })
    );
}

criterion_group!(benches, live_neighbours_benchmark);
criterion_main!(benches);