mod utils;

extern crate js_sys;
extern crate web_sys;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;
use std::fmt;

// A macro to provide console logging syntax
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t)* ).into());
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbour_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0 , 1].iter().cloned() {
            for delta_col in [self.width -1, 0, 1].iter().cloned() {
                // Skip the cell we're getting the neighbour count for
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

// Public methods, exposed to JavaScript via bindgen
#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        // Enable logging for panics
        utils::set_panic_hook();
        let size = (width * height) as usize;
        let cells = FixedBitSet::with_capacity(size);
        Universe { width, height, cells }
    }

    pub fn new_pattern(width: u32, height: u32) -> Universe {
        // Enable logging for panics
        utils::set_panic_hook();
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, i % 2 == 0 || i % 7 == 0);
        }
        
        Universe {
            width,
            height,
            cells
        }
    }

    pub fn new_rand(width: u32, height: u32) -> Universe {
        // Enable logging for panics
        utils::set_panic_hook();
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size{
            let state = js_sys::Math::random() < 0.5;
            cells.set(i, state);
        }

        Universe {
            width,
            height,
            cells
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = FixedBitSet::with_capacity((width * self.height) as usize);
    } 

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = FixedBitSet::with_capacity((self.width * height) as usize);
    }

    pub fn cells(&self) -> *const usize {
        self.cells.as_slice().as_ptr()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width{
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_count(row, col);
                
                next.set(idx, match(cell, live_neighbours) {
                    // Live cells with less than 2 neighbours die, underpopulation
                    (true, x) if x < 2 => false,
                    // Live cells with more than 3 neighbours die, overpopulation
                    (true, x) if x > 3 => false,
                    // Dead cells with 3 neighbours become alive, reproduction
                    (false, 3) => true,
                    // All other cells remain in same state
                    (other, _) => other
                });
            }
        }

        self.cells = next;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

// Public methods not exposed to JavaScript
impl Universe {
    // Get all the cells in the universe
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    // Set cells to be alive by passing row and col
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let symbol = if self.cells[idx] { '◼' } else { '◻' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}