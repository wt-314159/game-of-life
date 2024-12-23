mod utils;

extern crate js_sys;
extern crate web_sys;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;
use web_sys::console;
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

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
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

        let north = if row == 0 {
            self.height - 1
        } else {
            row - 1
        };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let south = if row == self.height - 1 {
            0
        } else {
            row + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;
        
        count
    }

    fn angle_width(&self, angle: u32) -> u32 {
        match angle {
            90 | 270 => self.height,
            _ => self.width
        }
    }

    fn angle_height(&self, angle: u32) -> u32 {
        match angle {
            90 | 270 => self.width,
            _ => self.height
        }
    }

    fn get_angle_index(&self, row: u32, col: u32, angle: u32) -> usize {
        match angle {
            90 => ((self.height - col - 1) * self.width + row) as usize, 
            180 => self.cells.len() - (row * self.width + col + 1) as usize,
            270 => (col * self.width + (self.width - row - 1)) as usize,
            _ => (row * self.width + col) as usize
        }
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
        // time how long the tick method takes
        let _timer = Timer::new("Universe::tick");
        let mut next = { 
            let _timer = Timer::new("allocate next cells");
            self.cells.clone()
        };

        {
            let _timer = Timer::new("new generation");
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
        }
        {
            let _timer = Timer::new("free old cells");
            self.cells = next;
        }
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells.toggle(idx);
    }

    pub fn insert_pattern(&mut self, pattern: &Pattern, row: u32, column: u32, angle: u32) {
        let max_row = min(row + pattern.angle_height(angle), self.height) - row;
        let max_col = min(column + pattern.angle_width(angle), self.width) - column;

        for r in 0..max_row {
            let u_row = r + row;
            for c in 0..max_col {
                let u_col = c + column;
                let u_idx = self.get_index(u_row, u_col);
                let p_idx = pattern.get_angle_index(r, c, angle);
                self.cells.set(u_idx, pattern.cells[p_idx]);
            } 
        }
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

    pub fn pentadecathlon() -> Pattern {
        let mut pattern = Pattern::new_plain(11, 18);
        pattern.set_cells(
            &[(4,5), (5,5), (6,4), (6,6), (7,5), (8,5), (9,5), (10,5), (11,4), (11,6), (12,5), (13,5)]);
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

    // Constructor methods for methuselah patterns
    // -------------------------------------------
    pub fn r_pentomino() -> Pattern {
        let mut pattern = Pattern::new_plain(5, 5);
        pattern.set_cells(&[(1,2), (1,3), (2,1), (2,2), (3,2)]);
        pattern
    }

    pub fn diehard() -> Pattern {
        let mut pattern = Pattern::new_plain(10, 5);
        pattern.set_cells(&[(1,7), (2,1), (2,2), (3,2), (3,6), (3,7), (3,8)]);
        pattern
    }
    // -------------------------------------------

    // Constructor methods for glider gun patterns
    // -------------------------------------------
    pub fn gosper_glider_gun() -> Pattern {
        let mut pattern = Pattern::new_plain(38, 11);
        pattern.set_cells(&[
            (1,25), (2,23), (2,25), (3,13), (3,14), (3,21), (3,22), (3,35), (3,36),
            (4,12), (4,16), (4,21), (4,22), (4,35), (4,36), (5,1), (5,2), (5,11),
            (5,17), (5,21), (5,22), (6,1), (6,2), (6,11), (6,15), (6,17), (6,18),
            (6,23), (6,25), (7,11), (7,17), (7,25), (8,12), (8,16), (9,13), (9,14)
        ]);
        pattern
    }
    // -------------------------------------------

    // Constructor methods for block laying engines
    // --------------------------------------------
    pub fn minimal_block_engine() -> Pattern {
        let mut pattern = Pattern:: new_plain(10, 8);
        pattern.set_cells(
            &[(1,7), (2,5), (2,7), (2,8), (3,5), (3,7), (4,5), (5,3), (6,1), (6,3)]);
        pattern
    }

    pub fn small_block_engine() -> Pattern {
        let mut pattern = Pattern::new_plain(7,7);
        pattern.set_cells(&[
            (1,1), (1,2), (1,3), (1,5), (2,1), (3,4), 
            (3,5), (4,2), (4,3), (4,5), (5,1), (5,3), (5,5)
        ]);
        pattern
    }

    pub fn linear_engine() -> Pattern {
        let mut pattern = Pattern::new_plain(41, 3);
        pattern.set_cells(&[
            (1,1), (1,2), (1,3), (1,4), (1,5), (1,6), (1,7), (1,8),
            (1,10), (1,11), (1,12), (1,13), (1,14), (1,18), (1,19),
            (1,20), (1,27), (1,28), (1,29), (1,30), (1,31), (1,32),
            (1,33), (1,35), (1,36), (1,37), (1,38), (1,39)
        ]);
        pattern
    }
    // --------------------------------------------

    // Constructor methods for creating eater patterns
    // -----------------------------------------------
    pub fn eater_one() -> Pattern {
        let mut pattern = Pattern::new_plain(6, 6);
        pattern.set_cells(&[(1,1), (1,2), (2,1), (2,3), (3,3), (4,3), (4,4)]);
        pattern
    }
    // -----------------------------------------------
}


//                      Testing
// ======================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_angle_index_90() {
        let universe = Universe::new(5,3);

        let start_index = universe.get_angle_index(0, 0, 90);
        assert_eq!(start_index, 10);

        let row_0_col_1 = universe.get_angle_index(0, 1, 90);
        assert_eq!(row_0_col_1, 5);

        let row_0_col_2 = universe.get_angle_index(0, 2, 90);
        assert_eq!(row_0_col_2, 0);

        let row_1_col_1 = universe.get_angle_index(1, 1, 90);
        assert_eq!(row_1_col_1, 6);

        let last = universe.get_angle_index(4, 2, 90);
        assert_eq!(last, 4);

        let row_4_col_0 = universe.get_angle_index(4, 0, 90);
        assert_eq!(row_4_col_0, 14);
    }

    #[test]
    fn test_get_angle_index_180() {
        let universe = Universe::new(5,3);

        let start_index = universe.get_angle_index(0, 0, 180);
        assert_eq!(start_index, 14);

        let row_2_col_3 = universe.get_angle_index(2, 3, 180);
        assert_eq!(row_2_col_3, 1);

        let row_1_col_3 = universe.get_angle_index(1, 3, 180);
        assert_eq!(row_1_col_3, 6);
    }

    #[test]
    fn test_get_angle_index_270() {
        let universe = Universe::new(5, 3);

        let start_index = universe.get_angle_index(0, 0, 270);
        assert_eq!(start_index, 4);

        let row_0_col_1 = universe.get_angle_index(0, 1, 270);
        assert_eq!(row_0_col_1, 9);

        let row_1_col_2 = universe.get_angle_index(1, 2, 270);
        assert_eq!(row_1_col_2, 13);

        let row_2_col_1 = universe.get_angle_index(2, 1, 270);
        assert_eq!(row_2_col_1, 7);
    }
}