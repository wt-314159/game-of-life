mod utils;
mod timer;
extern crate js_sys;
extern crate web_sys;

extern crate fixedbitset;
use fixedbitset::FixedBitSet;

use rustc_hash::FxHashSet;
#[allow(unused_imports)]
use timer::Timer;
#[allow(unused_imports)]
use web_sys::console;
#[allow(unused_imports)]
use wasm_bindgen::prelude::*;
use std:: {
    cmp::min,
    collections::HashSet,
    fmt,
};

// A macro to provide console logging syntax
#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t)* ).into());
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    buffers: [FixedBitSet; 2],
    active_cell_buffers: [FixedBitSet; 2],
    curr_index: usize
}

// Pattern struct to hold various patterns we might want
// to add to our universe. Use type aliasing as Patterns
// need essentially the exact same fields as Universe, 
// but we don't want constructor methods for Patterns to
// be accessible under Universe type
type Pattern = Universe;

impl Universe {
    #[inline(always)]
    fn get_index(width: usize, row: usize, column: usize) -> usize {
        row * width + column
    }

    fn angle_width(&self, angle: u32) -> usize {
        match angle {
            90 | 270 => self.height,
            _ => self.width
        }
    }

    fn angle_height(&self, angle: u32) -> usize {
        match angle {
            90 | 270 => self.width,
            _ => self.height
        }
    }

    fn get_angle_index(&self, row: usize, col: usize, angle: u32) -> usize {
        match angle {
            90 => ((self.height - col - 1) * self.width + row) as usize, 
            180 => self.buffers[0].len() - (row * self.width + col + 1) as usize,
            270 => (col * self.width + (self.width - row - 1)) as usize,
            _ => (row * self.width + col) as usize
        }
    }

    fn insert_neighbours(active_cells: &mut FixedBitSet, index: usize, width: usize, height: usize) {
        unsafe {
            for i in Self::get_neighbours(index, width, height) {
                active_cells.insert_unchecked(i);
            }
        }
    }
}

// Public methods, exposed to JavaScript via bindgen
#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        // Enable logging for panics
        utils::set_panic_hook();
        let (width, height) = (width as usize, height as usize);
        let size = width * height;
        let current = FixedBitSet::with_capacity(size);
        let next = FixedBitSet::with_capacity(size); 
        let curr_active = FixedBitSet::with_capacity(size);
        let next_active = FixedBitSet::with_capacity(size);

        Universe { width, height, buffers: [current, next], active_cell_buffers: [curr_active, next_active], curr_index: 0 }
    }

    pub fn new_rand(width: u32, height: u32) -> Universe {
        // Enable logging for panics
        utils::set_panic_hook();
        let (width, height) = (width as usize, height as usize);
        let size = width * height;
        let mut current = FixedBitSet::with_capacity(size);
        let next = FixedBitSet::with_capacity(size);
        let mut curr_active = FixedBitSet::with_capacity(size);
        let next_active = FixedBitSet::with_capacity(size);

        for i in 0..size{
            let state = js_sys::Math::random() < 0.25;
            current.set(i, state);
            curr_active.insert(i);
            Self::insert_neighbours(&mut curr_active, i, width, height);
        }
        
        Universe { width, height, buffers: [current, next], active_cell_buffers: [curr_active, next_active], curr_index: 0 }
    }

    

    pub fn new_sparse(width: u32, height: u32, scarcity: u32) -> Universe {
        // Enable logging for panics
        utils::set_panic_hook();
        let (width, height, scarcity) = (width as usize, height as usize, scarcity as usize);
        let size = width * height;
        let mut current = FixedBitSet::with_capacity(size);
        let next = FixedBitSet::with_capacity(size);
        let mut curr_active = FixedBitSet::with_capacity(size);
        let next_active = FixedBitSet::with_capacity(size);

        for i in 0..size {
            let state = i % scarcity == 0;
            current.set(i, state);
            curr_active.insert(i);
            Self::insert_neighbours(&mut curr_active, i, width, height);
        }

        Universe { width, height, buffers: [current, next], active_cell_buffers: [curr_active, next_active], curr_index: 0 }
    }

    pub fn new_oscillators(width: u32, height: u32, spacing: u32) -> Universe {
        // Enable logging for panics
        utils::set_panic_hook();
        let (w, h) = (width as usize, height as usize);
        let size = w * h;
        let current = FixedBitSet::with_capacity(size);
        let next = FixedBitSet::with_capacity(size);
        let curr_active = FixedBitSet::with_capacity(size);
        let next_active = FixedBitSet::with_capacity(size);
        
        let mut universe = Universe 
        { width: w, height: h, buffers: [current, next], active_cell_buffers: [curr_active, next_active], curr_index: 0 };

        let pattern = Pattern::blinker();

        if width > spacing && height > spacing {
            let spacing = spacing as usize;

            for row in (0..height).step_by(spacing) {
                for column in (0..width).step_by(spacing) {
                    universe.insert_pattern(&pattern, row, column, 0);
                }
            }
        }

        universe
    }

    pub fn width(&self) -> u32 {
        self.width as u32
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width as usize;
        let size = self.width * self.height;
        self.buffers[0] = FixedBitSet::with_capacity(size);
        self.buffers[1] = FixedBitSet::with_capacity(size);
    } 

    pub fn height(&self) -> u32 {
        self.height as u32
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height as usize;
        let size = self.width * self.height;
        self.buffers[0] = FixedBitSet::with_capacity(size);
        self.buffers[1] = FixedBitSet::with_capacity(size);
    }

    pub fn cells(&self) -> *const usize {
        self.buffers[self.curr_index].as_slice().as_ptr()
    }

    pub fn tick(&mut self) {
        let next_index = 1 - self.curr_index;
        let (width, height) = (self.width, self.height);
        unsafe {
            let current = self.buffers.as_mut_ptr().add(self.curr_index) as *mut FixedBitSet;
            let next = self.buffers.as_mut_ptr().add(next_index) as *mut FixedBitSet;
            let curr_active = self.active_cell_buffers.as_mut_ptr().add(self.curr_index) as *mut FixedBitSet;
            let next_active = self.active_cell_buffers.as_mut_ptr().add(next_index) as *mut FixedBitSet;
            let len = (*current).len();

            for idx in 0..len {
                let active = (*curr_active).contains_unchecked(idx);
                if !active { continue; }

                let cell = (*current).contains_unchecked(idx);
                let neighbours = Self::get_neighbour_array(idx, width, height);

                let mut live_neighbours = 0;
                for &n in neighbours.iter() {
                    live_neighbours += (*current).contains_unchecked(n) as u8;
                }

                let (live, changed) = match(cell, live_neighbours) {
                    (true, x) if x < 2 => (false, true),
                    (true, x) if x > 3 => (false, true),
                    (false, 3) => (true, true),
                    (other, _) => (other, false)
                };
                (*next).set_unchecked(idx, live);

                if changed {
                    (*next_active).insert(idx);
                    for n in neighbours {
                        (*next_active).insert(n);
                    }
                }
            }
        }
        self.curr_index = next_index;
    }

    pub fn toggle_cell_not_active(&mut self, row: u32, column: u32) {
        let idx = Self::get_index(self.width, row as usize, column as usize);
        self.buffers[self.curr_index].toggle(idx);
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = Self::get_index(self.width, row as usize, column as usize);
        self.buffers[self.curr_index].toggle(idx);
        self.active_cell_buffers[self.curr_index].insert(idx);
        Self::insert_neighbours(&mut self.active_cell_buffers[self.curr_index], idx, self.width, self.height);
    }

    pub fn insert_pattern(&mut self, pattern: &Pattern, row: u32, column: u32, angle: u32) {
        let (row, column) = (row as usize, column as usize);
        let max_row = min(row + pattern.angle_height(angle), self.height) - row;
        let max_col = min(column + pattern.angle_width(angle), self.width) - column;

        for r in 0..max_row {
            let u_row = r + row;
            for c in 0..max_col {
                let u_col = c + column;
                let u_idx = Self::get_index(self.width, u_row, u_col);
                let p_idx = pattern.get_angle_index(r, c, angle);

                self.buffers[self.curr_index].set(u_idx, pattern.buffers[pattern.curr_index][p_idx]);
                self.active_cell_buffers[self.curr_index].insert(u_idx);
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
        &self.buffers[self.curr_index]
    }

    // Set cells to be alive by passing row and col
    pub fn set_cells(&mut self, cells: &[(usize, usize)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = Self::get_index(self.width, row, col);
            self.buffers[self.curr_index].set(idx, true);
        }
    }

    pub fn get_neighbour_array(index: usize, width: usize, height: usize) -> [usize; 8] {
        let row = index / width;
        let col = index % width;

        let north = if row == 0 { height - 1 } else { row - 1 };
        let west = if col == 0 { width - 1 } else { col - 1 };
        let east = if col == width - 1 { 0 } else { col + 1 };
        let south = if row == height - 1 { 0 } else { row + 1 };

        let north_row_idx = width * north;
        let row_idx = width * row;
        let south_row_idx = width * south;

        let indices = [
            north_row_idx + west,
            north_row_idx + col,
            north_row_idx + east,
            row_idx + west,
            row_idx + east,
            south_row_idx + west,
            south_row_idx + col, 
            south_row_idx + east
        ];
        indices
    }

    pub fn get_neighbours(index: usize, width: usize, height: usize) -> impl Iterator<Item = usize> {
        let row = index / width;
        let col = index % width;

        let north = if row == 0 { height - 1 } else { row - 1 };
        let west = if col == 0 { width - 1 } else { col - 1 };
        let east = if col == width - 1 { 0 } else { col + 1 };
        let south = if row == height - 1 { 0 } else { row + 1 };

        let north_row_idx = width * north;
        let row_idx = width * row;
        let south_row_idx = width * south;

        let indices = [
            north_row_idx + west,
            north_row_idx + col,
            north_row_idx + east,
            row_idx + west,
            row_idx + east,
            south_row_idx + west,
            south_row_idx + col, 
            south_row_idx + east
        ];
        IntoIterator::into_iter(indices)
    }

    pub fn index_neighbour_count(&self, index: usize) -> u8 {
        let (width, height) = (self.width, self.height);
        let cells = &self.buffers[self.curr_index];
        let row = index / width;
        let col = index % width;

        let mut count = 0;

        let north = if row == 0 { height - 1 } else { row - 1 };
        let west = if col == 0 { width - 1 } else { col - 1 };
        let east = if col == width - 1 { 0 } else { col + 1 };
        let south = if row == height - 1 { 0 } else { row + 1 };

        let north_row_idx = north * width;
        let row_idx = row * width;
        let south_row_idx = south * width;

        unsafe {
            count += cells.contains_unchecked(north_row_idx + west) as u8;
            count += cells.contains_unchecked(north_row_idx + col) as u8;
            count += cells.contains_unchecked(north_row_idx + east) as u8;
            count += cells.contains_unchecked(row_idx + west) as u8;
            count += cells.contains_unchecked(row_idx + east) as u8;
            count += cells.contains_unchecked(south_row_idx + west) as u8;
            count += cells.contains_unchecked(south_row_idx + col) as u8;
            count += cells.contains_unchecked(south_row_idx + east) as u8;
        }
        count
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = Self::get_index(self.width, row, col);
                let symbol = if self.buffers[self.curr_index][idx] { '◼' } else { '◻' };
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
    fn new_plain(width: usize, height: usize) -> Pattern {
        let size = (width * height) as usize;
        let current = FixedBitSet::with_capacity(size);
        let next = FixedBitSet::with_capacity(0);
        let c = FixedBitSet::with_capacity(0);
        let n = FixedBitSet::with_capacity(0);
        
        Pattern { width, height, buffers: [current, next], active_cell_buffers: [c, n], curr_index: 0 }
    }

    // Constructor methods for simple oscillators
    // ------------------------------------------
    pub fn blinker() -> Pattern {
        let mut pattern = Pattern::new_plain(5, 5);

        for i in 1..=3 {
            pattern.toggle_cell_not_active(2, i);
        }
        pattern
    }

    pub fn toad() -> Pattern {
        let mut pattern = Pattern::new_plain(6, 6);

        let mut offset = 0;
        for i in 2..=3 {
            for j in 2..=4 {
                pattern.toggle_cell_not_active(i, j - offset);
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
                    pattern.toggle_cell_not_active(i + offset, j + offset);
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

    #[test]
    fn test_get_neighbour() {
        let indices: Vec<usize> = Universe::get_neighbours(0, 4, 3).collect();
        
        let expected_indices = [11, 8, 9, 3, 1, 7, 4, 5];

        assert_eq!(indices.len(), expected_indices.len());
        for i in 0..expected_indices.len() {
            assert_eq!(expected_indices[i], indices[i]);
        }
    }

    #[test]
    fn test_get_neighbour_array() {
        let indices = Universe::get_neighbour_array(0, 4, 3);

        let expected_indices = [11, 8, 9, 3, 1, 7, 4, 5];

        assert_eq!(expected_indices, indices);
    }

    #[test]
    fn test_count_neighbours() {
        let (width, height) = (20, 30);
        let universe = Universe::new_sparse(width, height, 3);
        let (width, height) = (width as usize, height as usize);

        let expected_count = universe.index_neighbour_count(0);
        
        let mut count = 0;
        let cells = universe.get_cells();
        for i in Universe::get_neighbour_array(0, width, height) {
            count += cells.contains(i) as u8;
        }

        assert_eq!(expected_count, count);
    }
}