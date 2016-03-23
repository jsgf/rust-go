use std::collections::{HashSet};
use std::collections::hash_map::{HashMap};
use std::iter::{self, FromIterator};
use std::cmp::max;
use std::str::FromStr;
use std::fmt::{self, Display};

use stone::Stone;
use group::{Group, GroupIterator};
use location::{Location, AllLocations};

pub type PointSet = HashSet<Location>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point(Location, Option<Stone>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Board {
    size: usize,
    points: HashMap<Location, Stone>,
}

impl Board {
    pub fn new() -> Board { Board::new_with_size(19) }
    pub fn new_with_size(size: usize) -> Board {
        Board {
            size: size,
            points: HashMap::new(),
        }
    }

    pub fn size(&self) -> usize { self.size }

    pub fn validloc<L>(&self, loc: L) -> bool
        where L: AsRef<Location>
    {
        let loc = loc.as_ref();
        loc.row() < self.size && loc.col() < self.size
    }

    pub fn get<L>(&self, loc: L) -> Option<Stone>
        where L: AsRef<Location>
    {
        self.points.get(loc.as_ref()).map(|s| *s)
    }

    pub fn add<L, S>(&mut self, loc: L, s: S) -> Option<Stone>
        where L: AsRef<Location>, S: AsRef<Stone>
    {
        let loc = loc.as_ref();
        let s = s.as_ref();
        assert!(self.validloc(loc));
        self.points.insert(*loc, *s)
    }

    // return locations of colour `s` who are part of groups whose last liberty is `loc`
    fn killed<L, S, Out>(&self, s: S, loc: L) -> Out
        where L: AsRef<Location>, S: AsRef<Stone>, Out: FromIterator<Location>
    {
        let loc = loc.as_ref();
        let s = *s.as_ref();

        let libset: HashSet<Location> = iter::once(loc).cloned().collect();

        GroupIterator::new(self.points.iter().map(|(l, s)| (*l, *s)), s)
                .filter(|g| self.liberties::<HashSet<_>>(g) == libset)
                .flat_map(|g| g.locations().cloned().collect::<Vec<_>>())
                .collect()
    }

    pub fn play<L, S>(&mut self, loc: L, s: S) -> bool
        where L: AsRef<Location>, S: AsRef<Stone>
    {
        let loc = loc.as_ref();
        let s = *s.as_ref();

        // valid play is:
        // 1. location is in bounds
        // 2. location is Empty
        // 3. if stone removes last liberty of opposite coloured groups, they are removed
        // 4. if stone's group has no liberties after removing dead groups, it is removed (suicide)

        if !self.validloc(loc) { return false }
        if self.get(loc).is_some() { return false }

        // find opposite coloured stones killed and remove them
        let dead: Vec<_> = self.killed(!s, loc);
        for d in dead {
            let ds = self.points.remove(&d);
            assert_eq!(ds, Some(!s));
        }

        // see if this is a suicide move
        let dead: Vec<_> = self.killed(s, loc);
        if dead.is_empty() {
            let ps = self.add(loc, s);
            assert_eq!(ps, None);
        } else {
            for d in dead {
                let ds = self.points.remove(&d);
                assert_eq!(ds, Some(s));
            }
        }
        true
    }

    pub fn remove(&mut self, loc: &Location) -> Option<Stone> {
        self.points.remove(loc)
    }

    pub fn locations(&self) -> AllLocations {
        AllLocations::new(self.size)
    }

    pub fn point(&self, loc: &Location) -> Point {
        Point(*loc, self.get(loc))
    }

    pub fn groups<GO>(&self, colour: Stone) -> GO
        where GO: FromIterator<Group>
    {
        Group::groups(self.points.iter().map(|(l, s)| (*l, *s)), colour)
    }

    pub fn liberties<Out>(&self, group: &Group) -> Out
        where Out: FromIterator<Location>
    {
        group.neighbours().iter()
            .filter(|l| self.get(l).is_none())
            .cloned()
            .collect()
    }
}

impl FromStr for Board {
    type Err = ();
    fn from_str(s: &str) -> Result<Board, ()> {
        // Given a single string containing one row per line:
        //    . . . # . O
        //    . . . # # O
        // generate a Board containing that position.
        //
        // The board is always upper-left. The dimensions are max(width,height)
        // of the text rows.
        //
        // In each row, spaces are ignored, '.' is a blank space, # is black,
        // O is white.

        let layout: Vec<Vec<Option<Stone>>> =
            s.lines()
                .map(|l| l.chars()
                            .filter_map(|c|
                                match c {
                                    '.'         => Some(None),
                                    '#' | 'X'   => Some(Some(Stone::Black)),
                                    'O' | 'o'   => Some(Some(Stone::White)),
                                    _ => None,
                                })
                            .collect())
                .collect();

        let w = layout.iter().map(|r| r.len()).max().unwrap_or(0);
        let h = layout.len();
        let sz = max(w, h);

        let mut board = Board::new_with_size(sz);

        for (rnum, row) in layout.into_iter().enumerate() {
            for (cnum, stone) in row.into_iter().enumerate() {
                let loc = Location::from((cnum, sz - rnum));

                if let Some(s) = stone {
                    let _ = board.add(loc, s);
                }
            }
        }

        Ok(board)
    }
}

impl Display for Board {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let sz = self.size();
        for row in 0..sz {
            for col in 0..sz {
                let loc = Location::new(col, sz-row-1);
                let c =
                    match self.get(loc) {
                        None =>                 '.',
                        Some(Stone::Black) =>   '#',
                        Some(Stone::White) =>   'O',
                    };
                try!(write!(fmt, "{} ", c));
            }
            try!(write!(fmt, "\n"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Board;
    use location::Location;
    use stone::Stone::{Black, White};

    #[test] fn fromstr() {
        let b = Board::from_str("
        . . . # O O
        . . . # # O
        . . . . O #
        . . . . . . .
        ").expect("failed");
        println!("Board:\n{}", b);
        let bstr = format!("{}", b);
        assert_eq!(bstr, "\
. . . # O O . \n\
. . . # # O . \n\
. . . . O # . \n\
. . . . . . . \n\
. . . . . . . \n\
. . . . . . . \n\
. . . . . . . \n\
");
    }

    #[test] fn play() {
        let mut b = Board::new_with_size(5);
        assert!(b.play(Location::new(0,0), Black));

        let bstr = format!("{}", b);
        assert_eq!(bstr, "\
. . . . . \n\
. . . . . \n\
. . . . . \n\
. . . . . \n\
# . . . . \n\
");

        assert!(!b.play(Location::new(0,0), Black));
        assert!(!b.play(Location::new(0,0), White));

        assert!(b.play(Location::new(1,0), White));
        assert!(b.play(Location::new(1,2), Black));
        assert!(b.play(Location::new(0,1), White)); // capture

        assert!(b.play(Location::new(0,2), Black));
        assert!(b.play(Location::new(1,1), White));
        assert!(b.play(Location::new(2,0), Black));
        assert!(b.play(Location::new(3,3), White));
        assert!(b.play(Location::new(2,1), Black));
        assert!(b.play(Location::new(3,2), White));
        assert!(b.play(Location::new(0,0), Black)); // capture


        println!("Board:\n{}", b);

        let bstr = format!("{}", b);
        assert_eq!(bstr, "\
. . . . . \n\
. . . O . \n\
# # . O . \n\
. . # . . \n\
# . # . . \n\
");

    }
}
