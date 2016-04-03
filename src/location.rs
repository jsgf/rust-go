use std::str::FromStr;
use std::fmt::{self, Display};

use bit_set::bitidx::BitIdx;

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

// Convert into bitset
impl Into<BitIdx> for Location {
    fn into(self) -> BitIdx { BitIdx(self.col * 100 + self.row) }
}

impl<'a> Into<BitIdx> for &'a Location {
    fn into(self) -> BitIdx { BitIdx(self.col * 100 + self.row) }
}

// From bitset
impl From<BitIdx> for Location {
    fn from(BitIdx(bit): BitIdx) -> Location {
        Location {
            col: bit / 100,
            row: bit % 100,
        }
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

impl Display for Location {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let col = match self.col {
            c@0...7 => (c as u8 + 'A' as u8) as char,
            c@8...24 => (c as u8 - 8 + 'J' as u8) as char,
            _ => '#',
        };
        write!(fmt, "{}{}", col, self.row + 1)
    }
}

impl FromStr for Location {
    type Err = &'static str;

    // parse locations in the form "B5", "A17"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 && s.len() != 3 { return Err("bad len") }

        let mut si = s.chars();

        // Letters don't use 'i/I' as a letter, presumably to avoid confusion with '1'
        let col = match si.next() {
            Some(c@'A'...'H') | Some(c@'a'...'h') =>
                c.to_lowercase().next().unwrap() as usize - 'a' as usize,
            Some(c@'J'...'T') | Some(c@'j'...'t') =>
                c.to_lowercase().next().unwrap() as usize - 'j' as usize + 8,
            _ => return Err("bad col"),
        };

        let mut row = match si.next() {
            Some(c@'1'...'9') => c as usize - '0' as usize,
            _ => return Err("bad row 1"),
        };
        row = match si.next() {
            None => row,
            Some(c@'1'...'9') => (row * 10) + (c as usize - '0' as usize),
            _ => return Err("bad row 2"),
        };

        Ok(Location::new(col, row - 1))
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
    use std::str::FromStr;
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

    #[test] fn parseloc() {
        assert_eq!(FromStr::from_str("a1"), Ok(Location::new(0, 0)));
        assert_eq!(FromStr::from_str("h1"), Ok(Location::new(7, 0)));
        assert_eq!(FromStr::from_str("j1"), Ok(Location::new(8, 0)));
        assert_eq!(FromStr::from_str("t1"), Ok(Location::new(18, 0)));
        assert_eq!(FromStr::from_str("a19"), Ok(Location::new(0, 18)));
        assert_eq!(FromStr::from_str("t19"), Ok(Location::new(18, 18)));

        assert_eq!(FromStr::from_str("A1"), Ok(Location::new(0, 0)));
        assert_eq!(FromStr::from_str("H1"), Ok(Location::new(7, 0)));
        assert_eq!(FromStr::from_str("J1"), Ok(Location::new(8, 0)));
        assert_eq!(FromStr::from_str("T1"), Ok(Location::new(18, 0)));
        assert_eq!(FromStr::from_str("A19"), Ok(Location::new(0, 18)));
        assert_eq!(FromStr::from_str("T19"), Ok(Location::new(18, 18)));
    }
}
