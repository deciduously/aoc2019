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
        s.split('\n').for_each(|o| ret.insert(o.trim()));
        println!("{:?}", ret);
        Ok(ret)
    }
}

impl OrbitSystem {
    fn node(&mut self, name: &str) -> usize {
        //first see if it exists
        for node in &self.object_arena {
            if node.name == name {
                return node.idx;
            }
        }
        // Otherwise, add new node
        let idx = self.object_arena.len();
        self.object_arena.push(OrbitObject::new(idx, name));
        idx
    }
    fn insert(&mut self, orbit: &str) {
        // Init nodes
        let split = orbit.split(')').collect::<Vec<&str>>();
        // first get node idx
        let inner = self.node(split[0]);
        let outer = self.node(split[1]);

        // set orbit
        match self.object_arena[outer].orbits {
            Some(_) => panic!("Attempt to overwrite existing orbit"),
            None => self.object_arena[outer].orbits = Some(inner),
        }

        // set parents
        self.object_arena[inner].parents.push(outer);
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
            let result = self.hops_to_root(o.idx);
            if result > 0 {
                acc + result - 1
            } else {
                acc + result
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
    fn hops_to_target_parent(&self, idx: usize, target: &str) -> Option<usize> {
        // are we here?  If so, Some(0)
        // If not,try all parents.
        // If it cant be found, return None
        if target == self.object_arena[idx].name {
            return Some(0);
        }
        for p in &self.object_arena[idx].parents {
            if let Some(x) = self.hops_to_target_parent(*p, target) {
                return Some(1 + x);
            }
        }
        None
    }
    fn minimal_orbit_distance(&mut self, from: &str, target: &str) -> Option<usize> {
        // If it's not in the tree, this will add a new unconnected node
        // the final function will still return None
        let start_node = self.node(from);
        let mut results = vec![];

        // Start traversal
        let mut trav = &self.object_arena[start_node];
        // Explore all parents, then hop up one
        while let Some(inner) = trav.orbits {
            for res in self.hops_to_target_parent(inner, target) {
                results.push(res);
            }
            trav = &self.object_arena[inner];
        }
        results.iter().fold(None, |acc, res| match acc {
            Some(x) => Some(x.min(*res)),
            None => Some(*res),
        })
    }
}

pub fn run() {
    let mut system = OrbitSystem::from_str(&get_puzzle_string(6).unwrap()).unwrap();
    println!(
        "{}\n{:?}",
        system.num_orbits(),
        system.minimal_orbit_distance("YOU", "SAN")
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
    #[test]
    fn test_minimal_orbit_distance() {
        assert_eq!(
            OrbitSystem::from_str(
                "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN"
            )
            .unwrap()
            .minimal_orbit_distance("YOU", "SAN"),
            Some(4)
        );
    }
    #[test]
    fn test_solutions() {
        assert_eq!(
            OrbitSystem::from_str(&get_puzzle_string(6).unwrap())
                .unwrap()
                .num_orbits(),
            142497
        )
    }
}
