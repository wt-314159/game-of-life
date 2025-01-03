use std::f32::consts::PI;
#[allow(unused_imports)]
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

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
    #[allow(unused_variables)]
    let (width, height) = (width as usize, height as usize);

    let mut group = c.benchmark_group("Live Neighbours");

    for index in [0, 1, width].iter() {
        group.bench_with_input(
            BenchmarkId::new("old", index),
            index,
            |b, &index| b.iter(
                || universe.index_neighbour_count(black_box(index))
            )
        );

        group.bench_with_input(
            BenchmarkId::new("new", index), 
            index, 
            |b, &index| b.iter(
                || universe.alt_live_neighbour_count(black_box(index))
            )
        );

        group.bench_with_input(
            BenchmarkId::new("new alt", index), 
            index, 
            |b, &index| b.iter(
                || universe.new_alt_live_neighbour_count(black_box(index))
            )
        );
    }
    group.finish();
}

#[allow(dead_code)]
fn bench_get_neighbours(c: &mut Criterion) {
    let (width, height) = (200, 200);

    let mut group = c.benchmark_group("Get Neighbours");

    for index in [0, 1, width + 1].iter() {

        group.bench_with_input(
            BenchmarkId::new("old", index), 
            index, 
            |b, &index| b.iter(
                || game_of_life::Universe::get_neighbours(black_box(index), width, height)
            ));
        
        group.bench_with_input(
            BenchmarkId::new("new", index), 
            index, 
        |b, &index| b.iter(
            || game_of_life::Universe::alt_get_neighbours(black_box(index), width, height)
        ));
    }
    group.finish();
}

criterion_group!(benches, live_neighbours_benchmark);
criterion_main!(benches);