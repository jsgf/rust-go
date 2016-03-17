use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Location { row: u32, col: u32 }

pub type LocationSet = HashSet<Location>;

impl Location {
    pub fn new(col: u32, row: u32) -> Location {
        Location { row: row, col: col }
    }
}

impl Location {
    #[inline]
    pub fn row(&self) -> u32 { self.row }
    #[inline]
    pub fn col(&self) -> u32 { self.col }

    pub fn neighbours(&self, bound: u32) -> Neighbours {
        Neighbours::new(*self, bound)
    }
}

pub struct AllLocations {
    size: u32,
    r: u32,
    c: u32,
}

impl AllLocations {
    pub fn new(size: u32) -> Self {
        AllLocations {
            size: size, r: 0, c: 0,
        }
    }
}

impl Iterator for AllLocations {
    type Item = Location;
    fn next(&mut self) -> Option<Self::Item> {
        if self.r >= self.size {
            self.r = 0;
            self.c += 1;
        }
        if self.c >= self.size {
            None
        } else {
            Some(Location::new(self.c, self.r))
        }
    }
}

pub struct Neighbours {
    bound: u32,
    loc: Location,
    n: u32,
}

impl Neighbours {
    pub fn new(loc: Location, bound: u32) -> Self {
        Neighbours { bound: bound, loc: loc, n: 0 }
    }
}

impl Iterator for Neighbours {
    type Item = Location;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.n {
                0 => if self.loc.col > 0 {
                    return Some(Location::new(self.loc.col - 1, self.loc.row))
                },
                1 => if self.loc.col < self.bound-1 {
                    return Some(Location::new(self.loc.col + 1, self.loc.row))
                },
                2 => if self.loc.row > 0 {
                    return Some(Location::new(self.loc.col, self.loc.row - 1))
                },
                3 => if self.loc.row < self.bound-1 {
                    return Some(Location::new(self.loc.col, self.loc.row + 1))
                },
                _ => return None,
            };
            self.n += 1;
        }
    }
}
