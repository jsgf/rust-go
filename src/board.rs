use std::collections::{HashSet};
use std::collections::hash_map::{HashMap};
use std::iter::{self, FromIterator};

use stone::Stone;
use group::{Group, GroupIterator};
use location::{Location, AllLocations};

pub type PointSet = HashSet<Location>;

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

    // return locations of colour `s` who are part of groups whose last liberty is `loc`
    fn killed<Out>(&self, s: Stone, loc: &Location) -> Out
        where Out: FromIterator<Location>
    {
        let libset: HashSet<Location> = iter::once(loc).map(|l| *l).collect();

        GroupIterator::new(self.points.iter().map(|(l, s)| (*l, *s)), s)
                .filter(|g| self.liberties::<HashSet<_>>(g) == libset)
                .flat_map(|g| g.locations().map(|l| *l).collect::<Vec<_>>())
                .collect()
    }

    pub fn play(&mut self, loc: &Location, s: Stone) -> bool {
        // valid play is:
        // 1. location is in bounds
        // 2. location is Empty
        // 3. if stone removes last liberty of opposite coloured groups, they are removed
        // 4. if stone's group has no liberties after removing dead groups, it is removed (suicide)

        if !self.validloc(loc) { return false }
        if self.get(loc).is_some() { return false }

        // find opposite coloured stones killed and remove them
        let dead = self.killed::<Vec<_>>(!s, loc);
        for d in dead {
            let ds = self.points.remove(&d);
            assert_eq!(ds, Some(!s));
        }

        // see if this is a suicide move
        let dead = self.killed::<Vec<_>>(s, loc);
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
            .map(|l| *l)
            .collect()
    }
}
