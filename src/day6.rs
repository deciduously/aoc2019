use super::get_puzzle_string;
use std::{io, str::FromStr};

#[derive(Debug, Clone)]
struct OrbitObject {
    idx: usize,
    name: String,
    orbits: Option<usize>,
    parents: Vec<usize>,
}

impl OrbitObject {
    fn new(idx: usize, name: &str) -> Self {
        Self {
            idx,
            name: name.into(),
            orbits: None,
            parents: vec![],
        }
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
        println!("{:?}", ret);
        Ok(ret)
    }
}

impl OrbitSystem {
    fn new_node(&mut self, name: &str) -> usize {
        let idx = self.object_arena.len();
        self.object_arena.push(OrbitObject::new(idx, name));
        idx
    }
    fn insert(&mut self, orbit: &str) {
        // Init nodes
        let split = orbit.split(')').collect::<Vec<&str>>();
        let outer = split[1];
        let inner = split[0];

        if self.object_arena.is_empty() {
            // Root node
            self.new_node(inner);
        }
        // always create a new node for the outer
        let outer_idx = self.new_node(outer);

        // add link
        for node in self.object_arena.iter_mut() {
            if node.name == inner {
                // Found match, create links
                node.parents.push(outer_idx);
                self.object_arena[outer_idx].orbits = Some(node.idx);
                break;
            }
        }
    }
    fn direct_orbits(&self) -> usize {
        // count successful traversals
        let mut ret = 0;
        for obj in &self.object_arena {
            ret += obj.parents.len();
        }
        ret
    }
    fn indirect_orbits(&self) -> usize {
        // Sum all hops, subtract one for the direct hop if any found
        self.object_arena.iter().fold(0, |acc, o| {
            let result = acc + self.hops_to_root(o.idx);
            if result > 0 {
                result - 1
            } else {
                result
            }
        })
    }
    fn num_orbits(&self) -> usize {
        self.direct_orbits() + self.indirect_orbits()
    }
    fn hops_to_root(&self, idx: usize) -> usize {
        match self.object_arena[idx].orbits {
            Some(id) => 1 + self.hops_to_root(id),
            None => 0,
        }
    }
}

pub fn run() {
    println!(
        "{}",
        OrbitSystem::from_str(&get_puzzle_string(6).unwrap())
            .unwrap()
            .num_orbits()
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_sample() {
        assert_eq!(
            OrbitSystem::from_str("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L")
                .unwrap()
                .num_orbits(),
            42
        );
    }
    #[test]
    fn test_direct() {
        assert_eq!(
            OrbitSystem::from_str("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L")
                .unwrap()
                .direct_orbits(),
            11
        );
    }
    #[test]
    fn test_indirect() {
        assert_eq!(
            OrbitSystem::from_str("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L")
                .unwrap()
                .indirect_orbits(),
            31
        );
    }
}
