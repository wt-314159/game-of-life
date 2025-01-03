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

// pub fn alt_live_neighbour_count(&self, index: usize) -> usize {
//     Self::get_neighbours(index, self.width, self.height)
//         .filter(|&neighbour| self.buffers[self.curr_index][neighbour])
//         .count()
// }

// pub fn live_neighbour_count(width: usize, height: usize, cells: &FixedBitSet, row: usize, column: usize) -> u8 {

#[allow(dead_code)]
fn live_neighbours_benchmark(c: &mut Criterion) {
    let (width, height) = (200, 200);
    let universe = game_of_life::Universe::new(width, height);
    let (width, height) = (width as usize, height as usize);
    let cells = universe.get_cells();

    let mut group = c.benchmark_group("Live Neighbours");

    for (index, row, col) in [(0, 0, 0), (1, 0, 1), (width + 1, 1, 1)].iter() {
        group.bench_with_input(
            BenchmarkId::new("old", index), 
            &(row, col),
            |b, (&row, &col)| b.iter(
                || game_of_life::Universe::live_neighbour_count(width, height, cells, row, col)));
        
        group.bench_with_input(
            BenchmarkId::new("new", index),
            index,
            |b, &index| b.iter(
                || universe.alt_live_neighbour_count(index)
            )
        );
    }
    group.finish();
}

criterion_group!(benches, live_neighbours_benchmark);
criterion_main!(benches);