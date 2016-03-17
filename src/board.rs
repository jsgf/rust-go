use std::collections::{HashSet, HashMap};

use stone::Stone;
use location::{Location, AllLocations};

pub type PointSet = HashSet<Location>;
pub type Group = HashMap<Location, Stone>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point(Location, Option<Stone>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Board {
    size: u32,
    points: HashMap<Location, Stone>,
}

impl Board {
    pub fn new() -> Board { Board::new_with_size(19) }
    pub fn new_with_size(size: u32) -> Board {
        Board {
            size: size,
            points: HashMap::new(),
        }
    }

    pub fn size(&self) -> u32 { self.size }
    pub fn validloc(&self, loc: &Location) -> bool {
        loc.row() < self.size && loc.col() < self.size
    }

    pub fn get(&self, loc: &Location) -> Option<Stone> {
        self.points.get(loc).map(|s| *s)
    }

    pub fn add(&mut self, loc: &Location, s: Stone) -> Option<Stone> {
        assert!(self.validloc(loc));
        self.points.insert(*loc, s)
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
}
