use std::collections::hash_set::{self, HashSet};
use std::iter::FromIterator;

use location::Location;
use stone::Stone;

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
            .map(|l| *l)
            .collect()
    }

    pub fn groups<SI, GO>(s: SI, colour: Stone) -> GO
        where SI: IntoIterator<Item=(Location, Stone)>,
              GO: FromIterator<Group>
    {
        GroupIterator::new(s, colour).collect()
    }

    pub fn has(&self, stone: Stone, loc: &Location) -> bool {
        self.colour == stone && self.group.contains(loc)
    }

    pub fn colour(&self) -> Stone { self.colour }

    pub fn locations(&self) -> hash_set::Iter<Location> {
        self.group.iter()
    }

    pub fn adjacent(&self, stone: Stone, loc: &Location) -> bool {
        self.colour == stone && !self.has(stone, loc) &&
        loc.neighbours().any(|l| self.has(stone, &l))
    }

    pub fn groupadjacent(&self, other: &Group) -> bool {
        assert!(self.group.is_disjoint(&other.group));

        self.colour == other.colour &&
        other.group.iter().any(|l| self.adjacent(other.colour, l))
    }

    pub fn merge(&self, other: &Group) -> Option<Group> {
        if self.colour == other.colour {
            Some(Group {
                colour: self.colour,
                group: self.group.union(&other.group).map(|l| *l).collect()
            })
        } else {
            None
        }
    }
}

pub struct GroupIterator {
    stones: HashSet<Location>,
    colour: Stone,
}

impl GroupIterator {
    pub fn new<SI>(s: SI, colour: Stone) -> GroupIterator
        where SI: IntoIterator<Item=(Location, Stone)>
    {
        GroupIterator {
            stones: s.into_iter()
                        .filter(|&(_, c)| c == colour)
                        .map(|(l, _)| l)
                        .collect(),
            colour: colour,
        }
    }
}

impl Iterator for GroupIterator {
    type Item = Group;

    fn next(&mut self) -> Option<Group> {
        if self.stones.is_empty() { return None }
        // init group with first stone
        let mut g: HashSet<Location> =
            self.stones.iter().take(1).map(|l| *l).collect();

        loop {
            // remove group from candidates
            self.stones = &self.stones - &g;

            // n is fringe of new neighbour locations
            let n: HashSet<Location> =
                g.iter()
                    .flat_map(|l| l.neighbours())
                    .collect::<HashSet<Location>>().intersection(&self.stones).map(|l| *l)
                    .collect();

            if n.is_empty() { break }

            // add fringe
            g = &g | &n;
        }

        Some(Group { colour: self.colour, group: g })
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
        let gs: Vec<_> = Group::groups(locs, Black);

        assert_eq!(gs.iter().map(|g| g.locations().collect::<Vec<_>>()).collect::<Vec<_>>(),
                    vec![vec![&Location::from((1,1))]])
    }

    #[test] fn single() {
        let stones = [((1, 1), Black), ((1, 2), Black), ((1, 3), White),
                      ((2, 1), White), ((2, 2), Black), ((2, 3), Black),];
        let locs = stones.iter()
                        .map(|&(loc, col)| (Location::from(loc), col));
        let gs: Vec<_> = Group::groups(locs, Black);

        assert_eq!(gs.len(), 1);

        let expect = [ (1,1), (1,2), (2,2), (2,3) ]
            .iter().map(Location::from).collect();

        assert_eq!(gs[0].colour(), Black);
        assert_eq!(gs[0].locations().map(|l| *l).collect::<HashSet<_>>(), expect);
    }

    #[test] fn neighbours() {
        let stones = [((1, 1), Black), ((1, 2), Black), ((1, 3), White),
                      ((2, 1), White), ((2, 2), Black), ((2, 3), Black),];
        let locs = stones.iter()
                        .map(|&(loc, col)| (Location::from(loc), col));
        let gs: Vec<_> = Group::groups(locs, Black);

        assert_eq!(gs.len(), 1);

        let neighbours = gs[0].neighbours();

        assert!(neighbours.is_disjoint(&gs[0].locations().map(|l| *l).collect()));

        let expect = [
            (0,1), (0,2), (1,0), (1,3), (2,1), (2,4), (3,2), (3,3)
        ].iter().map(Location::from).collect();

        assert_eq!(neighbours, expect);
    }
}
