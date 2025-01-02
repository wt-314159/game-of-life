#[allow(unused_imports)]
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};

extern crate game_of_life;

fn benchmark(c: &mut Criterion) {
    let mut universe = game_of_life::Universe::new(800, 800);

    c.bench_function(
        "tick", 
        |b| b.iter(|| {
            universe.tick();
    }));
}

// fn universe_live_neighbours(b: &mut test::Bencher) {
//     let (width, height) = (500, 500);
//     let universe = game_of_life::Universe::new_rand(width, height);
//     let cells = universe.get_cells();
    
//     let last_row = height - 1;
//     let last_col = width - 1;
//     let mid_row = height / 2;
//     let mid_col = width / 2;

//     let mut count = 0;
//     b.iter(|| {
//         count += game_of_life::Universe::live_neighbour_count(width, height, cells, 0, 0);
//         count += game_of_life::Universe::live_neighbour_count(width, height, cells, 1, 1);
//         count += game_of_life::Universe::live_neighbour_count(width, height, cells, 2, 2);
//         count += game_of_life::Universe::live_neighbour_count(width, height, cells, mid_row, mid_col);
//         count += game_of_life::Universe::live_neighbour_count(width, height, cells, last_row, last_col);
//         count += game_of_life::Universe::live_neighbour_count(width, height, cells, last_row - 1, last_col - 1);
//         count += game_of_life::Universe::live_neighbour_count(width, height, cells, last_row - 2, last_col - 2);
//     })
// }

criterion_group!(benches, benchmark);
criterion_main!(benches);