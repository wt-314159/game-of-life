use super::Universe;

pub struct NeighbourIter {
    pub width: usize,
    pub row: usize,
    pub col: usize,
    pub north: usize,
    pub south: usize,
    pub east: usize,
    pub west: usize,
    pub state: u8
}

impl Iterator for NeighbourIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.state {
            0 => Some(Universe::get_index(self.width, self.north, self.west)),
            1 => Some(Universe::get_index(self.width, self.north, self.col)),
            2 => Some(Universe::get_index(self.width, self.north, self.east)),
            3 => Some(Universe::get_index(self.width, self.row, self.west)),
            4 => Some(Universe::get_index(self.width, self.row, self.east)),
            5 => Some(Universe::get_index(self.width, self.south, self.west)),
            6 => Some(Universe::get_index(self.width, self.south, self.col)),
            7 => Some(Universe::get_index(self.width, self.south, self.east)),
            _ => None,
        };
        self.state += 1;
        result
    }
}