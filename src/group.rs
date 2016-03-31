use std::collections::hash_set::{self, HashSet};
use std::collections::hash_map::HashMap;
use std::iter::FromIterator;
use std::fmt::{self, Display};

use location::Location;
use stone::Stone;
use accum::Accum;

#[derive(Debug, Clone)]
pub struct Group {
    colour: Stone,
    group: HashSet<Location>,
}

impl Group {
    pub fn new(stone: Stone, loc: Location) -> Group {
        let mut g = Group {
            colour: stone,
            group: HashSet::new()
        };
        g.group.insert(loc);

        g
    }

    /// Return set of locations adjacent to group stones, including internal
    pub fn neighbours(&self) -> HashSet<Location> {
        self.group.iter()
            .flat_map(|l| l.neighbours())
            .collect::<HashSet<Location>>()
            .difference(&self.group)
            .cloned()
            .collect()
    }

    pub fn groups<SI, GO>(s: SI) -> GO
        where SI: IntoIterator<Item=(Location, Stone)>,
              GO: FromIterator<Group>
    {
        GroupIterator::new(s).collect()
    }

    pub fn has<L>(&self, stone: Stone, loc: L) -> bool
        where L: AsRef<Location>
    {
        let loc = loc.as_ref();
        self.colour == stone && self.group.contains(loc)
    }

    pub fn colour(&self) -> Stone { self.colour }

    pub fn locations(&self) -> hash_set::Iter<Location> {
        self.group.iter()
    }

    pub fn adjacent<L>(&self, stone: Stone, loc: L) -> bool
        where L: AsRef<Location>
    {
        let loc = loc.as_ref();
        self.colour == stone && !self.has(stone, loc) &&
        loc.neighbours().any(|l| self.has(stone, &l))
    }

    pub fn contains<L>(&self, loc: L) -> bool
        where L: AsRef<Location>
    {
        let loc = loc.as_ref();

        self.group.contains(loc)
    }

    pub fn groupadjacent<G>(&self, other: G) -> bool
        where G: AsRef<Group>
    {
        let other = other.as_ref();
        assert!(self.group.is_disjoint(&other.group));

        self.colour == other.colour &&
        other.group.iter().any(|l| self.adjacent(other.colour, l))
    }

    pub fn merge<G>(&self, other: G) -> Option<Group>
        where G: AsRef<Group>
    {
        let other = other.as_ref();

        if self.colour == other.colour {
            Some(Group {
                colour: self.colour,
                group: self.group.union(&other.group).cloned().collect()
            })
        } else {
            None
        }
    }
}

impl Display for Group {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "{} [ ", self.colour));
        for l in &self.group {
            try!(write!(fmt, "{} ", l))
        }
        write!(fmt, "]")
    }
}

impl AsRef<Group> for Group {
    fn as_ref(&self) -> &Self { self }
}

impl AsRef<HashSet<Location>> for Group {
    fn as_ref(&self) -> &HashSet<Location> { &self.group }
}

pub struct GroupIterator {
    stones: HashMap<Location, Stone>,
}

impl GroupIterator {
    pub fn new<SI>(s: SI) -> GroupIterator
        where SI: IntoIterator<Item=(Location, Stone)>
    {
        GroupIterator {
            stones: s.accum(HashMap::new(), |mut map, (l, stone)| {
                let _ = map.insert(l, stone);
            }),
        }
    }
}

impl Iterator for GroupIterator {
    type Item = Group;

    fn next(&mut self) -> Option<Group> {
        if self.stones.is_empty() { return None }
        // init group with first stone
        let mut colour = None;
        let mut g: HashSet<Location> =
            self.stones.iter()
                .take(1)
                .inspect(|&(_, s)| colour = Some(*s))
                .map(|(l, _)| l)
                .cloned().collect();

        let colour = colour.expect("colourless stone?");

        loop {
            // remove group from candidates
            for s in &g {
                let _ = self.stones.remove(s);
            }

            // n is fringe of new neighbour locations
            let n: HashSet<Location> =
                g.iter()
                    .flat_map(|l| l.neighbours().filter(|l| self.stones.get(l) == Some(&colour)))
                    .collect();

            if n.is_empty() { break }

            // add fringe
            g = &g | &n;
        }

        Some(Group { colour: colour, group: g })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::Group;
    use location::Location;
    use stone::Stone::*;

    #[test] fn simple() {
        let stones = [((1, 1), Black)];
        let locs = stones.iter()
                        .map(|&(loc, col)| (Location::from(loc), col));
        let gs: Vec<_> = Group::groups(locs);

        for g in gs {
            assert_eq!(g.colour(), Black);
            assert_eq!(g.locations().collect::<Vec<_>>(), vec![&Location::from((1,1))]);
        }
    }

    #[test] fn single() {
        let stones = [((1, 1), Black), ((1, 2), Black), ((1, 3), White),
                      ((2, 1), White), ((2, 2), Black), ((2, 3), Black),];
        let locs = stones.into_iter()
                        .filter(|&&(_, c)| c == Black)
                        .map(|&(loc, col)| (Location::from(loc), col));
        let gs: Vec<_> = Group::groups(locs);

        assert_eq!(gs.len(), 1);

        let expect = [ (1,1), (1,2), (2,2), (2,3) ]
            .iter().map(Location::from).collect();

        assert_eq!(gs[0].colour(), Black);
        assert_eq!(gs[0].locations().cloned().collect::<HashSet<_>>(), expect);
    }

    #[test] fn neighbours() {
        let stones = [((1, 1), Black), ((1, 2), Black), ((1, 3), White),
                      ((2, 1), White), ((2, 2), Black), ((2, 3), Black),];
        let locs = stones.iter()
                        .map(|&(loc, col)| (Location::from(loc), col));
        let gs: Vec<_> = Group::groups::<_, Vec<_>>(locs).into_iter()
            .filter(|g| g.colour() == Black)
            .collect();

        assert_eq!(gs.len(), 1);

        let neighbours = gs[0].neighbours();

        assert!(neighbours.is_disjoint(&gs[0].locations().cloned().collect()));

        let expect = [
            (0,1), (0,2), (1,0), (1,3), (2,1), (2,4), (3,2), (3,3)
        ].iter().map(Location::from).collect();

        assert_eq!(neighbours, expect);
    }
}
