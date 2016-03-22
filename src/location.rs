#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Location { row: usize, col: usize }

impl Location {
    pub fn new(col: usize, row: usize) -> Location {
        Location { row: row, col: col }
    }
}

impl AsRef<Location> for Location {
    fn as_ref(&self) -> &Self { self }
}

impl Location {
    #[inline] pub fn row(&self) -> usize { self.row }
    #[inline] pub fn col(&self) -> usize { self.col }

    pub fn neighbours(&self) -> Neighbours {
        Neighbours::new(self)
    }
}

impl From<(usize, usize)> for Location {
    fn from((c, r): (usize, usize)) -> Self {
        Location::new(c, r)
    }
}

impl<'a> From<&'a (usize, usize)> for Location {
    fn from(&(c, r): &'a (usize, usize)) -> Self {
        Location::new(c, r)
    }
}

pub struct AllLocations {
    size: usize,
    r: usize,
    c: usize,
}

impl AllLocations {
    pub fn new(size: usize) -> Self {
        AllLocations {
            size: size, r: 0, c: 0,
        }
    }
}

impl Iterator for AllLocations {
    type Item = Location;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = Location::new(self.c, self.r);

        self.r += 1;

        if self.r >= self.size {
            self.r = 0;
            self.c += 1;
        }

        if ret.col >= self.size {
            None
        } else {
            Some(ret)
        }
    }
}

pub struct Neighbours {
    loc: Location,
    n: usize,
}

impl Neighbours {
    pub fn new(loc: &Location) -> Self {
        Neighbours { loc: *loc, n: 0 }
    }
}

impl Iterator for Neighbours {
    type Item = Location;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let n = self.n;
            self.n += 1;

            match n {
                0 => if self.loc.col > 0 { return Some(Location::new(self.loc.col - 1, self.loc.row)) },
                1 => return Some(Location::new(self.loc.col + 1, self.loc.row)),
                2 => if self.loc.row > 0 { return Some(Location::new(self.loc.col, self.loc.row - 1)) },
                3 => return Some(Location::new(self.loc.col, self.loc.row + 1)),

                _ => return None,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Location, AllLocations};

    #[test] fn allloc() {
        assert_eq!(AllLocations::new(3).collect::<Vec<Location>>(),
                    vec![
                        Location::new(0, 0), Location::new(0, 1), Location::new(0, 2),
                        Location::new(1, 0), Location::new(1, 1), Location::new(1, 2),
                        Location::new(2, 0), Location::new(2, 1), Location::new(2, 2),
                    ]);
    }

    #[test] fn neighbours() {
        assert_eq!(Location::new(0,0).neighbours().collect::<Vec<_>>(),
                    vec![Location::new(1,0), Location::new(0,1)]);

        assert_eq!(Location::new(1,1).neighbours().collect::<Vec<_>>(),
                    vec![Location::new(0,1), Location::new(2,1),
                         Location::new(1,0), Location::new(1,2)]);
    }
}
