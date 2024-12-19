//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::assert_eq;

use wasm_bindgen_test::*;

extern crate game_of_life;
use game_of_life::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[cfg(test)]
pub fn input_spaceship() -> Universe {
    let mut universe = Universe::new(6, 6);
    // Manually set the cells to create a spaceship
    universe.set_cells(&[(1,2), (2,3), (3,1), (3,2), (3,3)]);
    universe
}

#[cfg(test)]
pub fn expected_spaceship() -> Universe {
    let mut universe = Universe::new(6,6);
    // Manually set the cells to the expected spaceship after 1 tick
    universe.set_cells(&[(2,1), (2,3), (3,2), (3,3), (4,2)]);
    universe
}

#[wasm_bindgen_test]
pub fn test_tick() {
    // Create a spaceship to test
    let mut input_universe = input_spaceship();
    // Create the expected outcome after 1 tick
    let expected_universe = expected_spaceship();

    // tick the input universe once and check the cells match the expected universe
    input_universe.tick();
    assert_eq!(input_universe.get_cells(), expected_universe.get_cells());
}