#[allow(unused_imports)]
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

extern crate game_of_life;

#[allow(dead_code)]
fn tick_benchmark(c: &mut Criterion) {
    let (width, height) = (800, 800);
    let mut universe = game_of_life::Universe::new_sparse(width, height, 4);

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
            BenchmarkId::new("separate get neighbours and count neighbours", index), 
            index, 
            |b, &index| b.iter(
                || {
                    // use 'max' to iterate over iterator
                    let _max = game_of_life::Universe::get_neighbours(black_box(index), width, height).max();
                    universe.index_neighbour_count(black_box(index));
                }
            ));

        group.bench_with_input(
            BenchmarkId::new("combined get and count neighbours", index), 
            index, 
            |b, &index| b.iter(
                || {
                    let mut _count = 0;
                    let indices = game_of_life::Universe::get_neighbour_array(index, width, height);
                    unsafe {
                        // counting live neighbours from vec
                        for i in indices.iter() {
                            _count += cells.contains_unchecked(black_box(*i)) as u8;
                        }
                        // simulating iterating through vec again to add to active cells
                        let _max = indices.iter().max();
                    }
                }
            ));

        group.bench_with_input(
            BenchmarkId::new("counting with filer", index), 
            index,
            |b, &index| b.iter(
                || {
                    let indices = game_of_life::Universe::get_neighbour_array(index, width, height);
                    unsafe {
                        let _count = indices.iter()
                            .filter(|&&i| cells.contains_unchecked(black_box(i)))
                            .count();
                        let _max = indices.iter().max();
                    }
                }
            ));
    }
    group.finish();
}

#[allow(dead_code)]
fn bench_get_neighbours(c: &mut Criterion) {
    let (width, height) = (200, 200);

    for index in [0, 1, width + 1].iter() {

        c.bench_with_input(
            BenchmarkId::new("Get Neighbours", index), 
            index, 
            |b, &index| b.iter(
                || game_of_life::Universe::get_neighbours(black_box(index), width, height).max()
            ));
    }
}

criterion_group!(benches, tick_benchmark);
criterion_main!(benches);