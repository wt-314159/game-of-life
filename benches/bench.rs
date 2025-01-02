#![feature(test)]

extern crate test;
extern crate game_of_life;

#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut universe = game_of_life::Universe::new(800, 800);

    b.iter(|| {
        universe.tick();
    })
}

#[bench]
fn universe_live_neighbours(b: &mut test::Bencher) {
    
}