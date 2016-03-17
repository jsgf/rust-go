use std::collections::hash_map::{self, HashMap};

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
}
