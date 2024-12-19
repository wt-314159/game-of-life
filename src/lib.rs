mod utils;

extern crate js_sys;
extern crate web_sys;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;
use std:: {
    cmp:: { max, min },
    fmt
};

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

// Pattern struct to hold various patterns we might want
// to add to our universe. Use type aliasing as Patterns
// need essentially the exact same fields as Universe, 
// but we don't want constructor methods for Patterns to
// be accessible under Universe type
type Pattern = Universe;

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

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells.toggle(idx);
    }

    pub fn insert_pattern(&mut self, pattern: &Pattern, row: u32, column: u32) {
        let max_row = min(row + pattern.height, self.height) - row;
        let max_col = min(column + pattern.width, self.width) - column;

        for r in 0..max_row {
            let u_row = r + row;
            for c in 0..max_col {
                let u_col = c + column;
                let u_idx = self.get_index(u_row, u_col);
                let p_idx = pattern.get_index(r, c);
                self.cells.set(u_idx, pattern.cells[p_idx]);
            } 
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

// Patterns to create
#[wasm_bindgen]
impl Pattern {
    fn new_plain(width: u32, height: u32) -> Pattern {
        let size = (width * height) as usize;
        let cells = FixedBitSet::with_capacity(size);
        Pattern { width, height, cells }
    }

    // Constructor methods for simple oscillators
    // ------------------------------------------
    pub fn blinker() -> Pattern {
        let mut pattern = Pattern::new_plain(5, 5);

        for i in 1..=3 {
            pattern.toggle_cell(2, i);
        }
        pattern
    }

    pub fn toad() -> Pattern {
        let mut pattern = Pattern::new_plain(6, 6);

        let mut offset = 0;
        for i in 2..=3 {
            for j in 2..=4 {
                pattern.toggle_cell(i, j - offset);
            }
            offset = 1;
        }
        pattern
    }

    pub fn beacon() -> Pattern {
        let mut pattern = Pattern::new_plain(6, 6);

        for r in 0..2 {
            let offset = r * 2;
            for i in 1..=2 {
                for j in 1..=2 {
                    pattern.toggle_cell(i + offset, j + offset);
                }
            }
        }
        pattern
    }

    pub fn pulsar() -> Pattern {
        let mut pattern = Pattern::new_plain(17, 17);

        pattern.set_cells(&[
            (2,4), (2,5), (2,6), (2,10), (2,11), (2,12), 
            (4,2), (4,7), (4,9),(4,14),
            (5,2), (5,7), (5,9),(5,14),
            (6,2), (6,7), (6,9),(6,14),
            (7,4), (7,5), (7,6), (7,10), (7,11), (7,12),
            (9,4), (9,5), (9,6), (9,10), (9,11), (9,12),
            (10,2), (10,7), (10,9),(10,14),
            (11,2), (11,7), (11,9),(11,14),
            (12,2), (12,7), (12,9),(12,14),
            (14,4), (14,5), (14,6), (14,10), (14,11), (14,12),]);
        pattern
    }
    // ------------------------------------------

    // Constructor methods for simple spaceships 
    // -----------------------------------------
     pub fn glider() -> Pattern {
        let mut pattern = Pattern::new_plain(5, 5);

        pattern.set_cells(&[(1,2), (2,3), (3,1), (3,2), (3,3)]);
        pattern
     }

     pub fn lightweight_spaceship() -> Pattern {
        let mut pattern = Pattern::new_plain(7, 6);

        pattern.set_cells(
            &[(1,1), (1,4), (2,5), (3,1), (3,5), (4,2), (4,3), (4,4), (4,5)]);
        pattern
     }

     pub fn midweight_spaceship() -> Pattern {
        let mut pattern = Pattern::new_plain(8, 7);
        pattern.set_cells(
            &[(1,3), (2,1), (2,5), (3,6), (4,1), (4,6), (5,2), (5,3), (5,4), (5,5), (5,6)]);
        pattern
     }

     pub fn heavyweight_spaceship() -> Pattern {
        let mut pattern = Pattern::new_plain(9, 7);
        pattern.set_cells(
            &[(1,3), (1,4), (2,1), (2,6), (3,7), (4,1), (4,7), (5,2), (5,3), (5,4), (5,5), (5,6), (5,7)]);
        pattern
     }
    // -----------------------------------------
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