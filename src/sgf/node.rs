use std::collections::hash_map::{self, HashMap};
use std::ops::{Index, Range, RangeFrom, RangeTo, RangeFull};

use sgf::property::{self, Property};

#[derive(Debug, Clone)]
pub struct Node {
    props: HashMap<String, Property>,
    children: Vec<Node>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            props: HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn addprop(&mut self, p: Property) {
        let _ = self.props.insert(p.id().to_string(), p);
    }

    pub fn addchild(&mut self, node: Node) {
        self.children.push(node)
    }

    pub fn rootnode(&self) -> bool {
        self.props.values().any(|p| p.ptype() == Some(property::Type::Root))
    }

    pub fn setupnode(&self) -> bool {
        self.props.values().any(|p| p.ptype() == Some(property::Type::Setup))
    }

    pub fn movenode(&self) -> bool {
        self.props.values().any(|p| p.ptype() == Some(property::Type::Move))
    }

    pub fn properties<'a>(&'a self) -> hash_map::Values<'a, String, Property> {
        self.props.values()
    }

    pub fn prop(&self, id: &str) -> Option<&Property> {
        self.props.get(id)
    }

    pub fn len(&self) -> usize { self.children.len() }
}

impl<'a> Index<&'a str> for Node {
    type Output = Property;
    fn index(&self, idx: &'a str) -> &Property {
        self.prop(idx).unwrap()
    }
}

impl Index<usize> for Node {
    type Output = Self;
    fn index(&self, idx: usize) -> &Node {
        &self.children[idx]
    }
}

impl Index<Range<usize>> for Node {
    type Output = [Node];
    fn index(&self, idx: Range<usize>) -> &[Node] {
        &self.children[idx.start..idx.end]
    }
}

impl Index<RangeFrom<usize>> for Node {
    type Output = [Node];
    fn index(&self, idx: RangeFrom<usize>) -> &[Node] {
        &self.children[idx.start..]
    }
}

impl Index<RangeTo<usize>> for Node {
    type Output = [Node];
    fn index(&self, idx: RangeTo<usize>) -> &[Node] {
        &self.children[..idx.end]
    }
}

impl Index<RangeFull> for Node {
    type Output = [Node];
    fn index(&self, _idx: RangeFull) -> &[Node] {
        &self.children[..]
    }
}
