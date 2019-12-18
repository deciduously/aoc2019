use std::{io, str::FromStr};
use super::get_puzzle_string;

/*
pub struct Node<T> {
    parent: Option<NodeId>,
    previous_sibling: Option<NodeId>,
    next_sibling: Option<NodeId>,
    first_child: Option<NodeId>,
    last_child: Option<NodeId>,

    /// The actual data which will be stored within the tree
    pub data: T,
}

pub struct NodeId {
    index: usize,
}
*/

#[derive(Debug, Clone)]
struct OrbitObject {
    idx: usize,
    name: String,
    parent: Option<usize>,
    child: Option<usize>,
}

impl OrbitObject {
    fn new(idx: usize, name: &str) -> Self {
        Self { idx, name: name.into(), parent: None, child: None }
    }
}

impl PartialEq for OrbitObject {
    fn eq(&self, rhs: &Self) -> bool {
        self.name == rhs.name
    }
}

#[derive(Debug, Default)]
struct OrbitSystem {
    object_arena: Vec<OrbitObject>,
}

impl FromStr for OrbitSystem {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret = Self::default();
        let _ = s.split('\n').for_each(|o| ret.insert(o));
        Ok(ret)
    }
}

impl OrbitSystem {
    fn new_node(&mut self, name: &str) -> usize {
        let idx = self.object_arena.len();
        self.object_arena.push(OrbitObject::new(idx, name));
        idx
    }
    fn set_parent(&mut self, idx: usize, parent: usize) {
        self.object_arena[idx].parent = Some(parent);
        // TODO also update child
    }
    fn insert_child(&mut self, idx: usize) {
        let child = self.object_arena[idx].name.clone();
        for node in self.object_arena.iter_mut() {
            if node.name == child {
                node.parent = Some(idx);
            }
        }
    }
    fn insert(&mut self, orbit: &str) {
        // Init nodes
        let split = orbit.split(')').collect::<Vec<&str>>();
        let parent = self.new_node(split[1]);
        let child = self.new_node(split[0]);

       // set child's parent
       self.set_parent(child, parent);

       // attempt to set child's ancestor
       self.insert_child(child);
    }
    fn num_orbits(&self) -> usize {
        self.object_arena.len()
    }
}

pub fn run() {
    println!("{}", OrbitSystem::from_str(&get_puzzle_string(6).unwrap()).unwrap().num_orbits());
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_sample() {
        assert_eq!(OrbitSystem::from_str("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L").unwrap().num_orbits(), 42);
    }
}