#![feature(test)]

extern crate test;
extern crate game_of_life;

#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut universe = game_of_life::Universe::new(128, 128);

    b.iter(|| {
        universe.tick();
    })
}