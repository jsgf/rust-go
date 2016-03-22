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

        let libset: HashSet<Location> = iter::once(loc).map(|l| *l).collect();

        GroupIterator::new(self.points.iter().map(|(l, s)| (*l, *s)), s)
                .filter(|g| self.liberties::<HashSet<_>>(g) == libset)
                .flat_map(|g| g.locations().map(|l| *l).collect::<Vec<_>>())
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
            .map(|l| *l)
            .collect()
    }
}
