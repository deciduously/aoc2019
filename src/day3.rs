use super::get_puzzle_string;
use std::{
    io::{self, ErrorKind::*},
    ops::AddAssign,
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
}

impl FromStr for Direction {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        match s {
            "D" => Ok(Down),
            "L" => Ok(Left),
            "R" => Ok(Right),
            "U" => Ok(Up),
            _ => Err(Self::Err::new(InvalidInput, "Unknown direction in input")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Step {
    direction: Direction,
    length: usize,
}

impl FromStr for Step {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = Direction::from_str(&s.chars().nth(0).unwrap().to_string())?;
        let length = s[1..].parse::<usize>().unwrap();
        Ok(Self { direction, length })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct Position(i32, i32);

impl Position {
    fn manhattan_distance(self, other: Position) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

impl From<Position> for (f64, f64) {
    fn from(p: Position) -> Self {
        (f64::from(p.0), f64::from(p.1))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Line {
    origin: Position,
    destination: Position,
}

impl Line {
    fn new(origin: Position, destination: Position) -> Self {
        Self {
            origin,
            destination,
        }
    }
    // https://www.geeksforgeeks.org/program-for-point-of-intersection-of-two-lines/
    fn ax_by_eqc(self) -> (f64, f64, f64) {
        let a = self.destination.1 - self.origin.1;
        let b = self.origin.0 - self.destination.0;
        let c = a * self.origin.0 + b * self.origin.1;
        (a.into(), b.into(), c.into())
    }
    fn contains(self, point: Position) -> bool {
        // in this grid, all lines are parallel
        let orig = self.origin;
        let dest = self.destination;
        if orig.0 == dest.0 {
            // vertical
            if orig.1 < dest.1 {
                point.0 == orig.0 && point.1 <= dest.1 && point.1 >= orig.1
            } else {
                point.0 == orig.0 && point.1 >= dest.1 && point.1 <= orig.1
            }
        } else {
            // horizontal
            if orig.0 < dest.0 {
                point.1 == orig.1 && point.0 <= dest.0 && point.0 >= orig.0
            } else {
                point.1 == orig.1 && point.0 >= dest.0 && point.1 <= orig.0
            }
        }
    }
    fn intersection(self, other: Self) -> Option<Position> {
        // get both as ax + by = c
        let (a1, b1, c1) = self.ax_by_eqc();
        let (a2, b2, c2) = other.ax_by_eqc();
        // check if parallel
        let determinant = a1 * b2 - a2 * b1;
        if determinant == 0.0 {
            None
        } else {
            // find intersection
            let x = (b2 * c1 - b1 * c2) / determinant;
            let y = (a1 * c2 - a2 * c1) / determinant;
            if x.fract() == 0.0 && y.fract() == 0.0 {
                Some(Position(x.floor() as i32, y.floor() as i32))
            } else {
                None
            }
        }
    }
    fn len(self) -> u32 {
        if self.origin.0 == self.destination.0 {
            // vertical
            (self.origin.1 - self.destination.1).abs() as u32
        } else {
            // horizontal
            (self.origin.0 - self.destination.0).abs() as u32
        }
    }
}

impl AddAssign<Step> for Position {
    fn add_assign(&mut self, rhs: Step) {
        use Direction::*;
        let offset = rhs.length as i32;
        match rhs.direction {
            Down => {
                self.1 -= offset;
            }
            Left => {
                self.0 -= offset;
            }
            Right => {
                self.0 += offset;
            }
            Up => {
                self.1 += offset;
            }
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
struct WirePath(Vec<Step>);

impl FromStr for WirePath {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret = vec![];
        let steps = s.split(',');
        for step in steps {
            ret.push(Step::from_str(step)?);
        }
        Ok(Self(ret))
    }
}

impl WirePath {
    fn get_lines(&self) -> Vec<Line> {
        let mut origin = Position::default();
        let mut destination = origin;
        let mut ret = Vec::new();
        for step in &self.0 {
            // set destination
            destination += *step;
            // add line
            ret.push(Line::new(origin, destination));
            // reset origin
            origin = destination;
        }
        ret
    }
    fn distance_to_pos(&self, pos: Position) -> Option<u32> {
        // if line contains destination, take delta, otherwise just take whole line length
        // If we read the end of the path without finding the point, return None
        let mut ret = 0;
        let mut found = false;
        for line in self.get_lines() {
            if line.contains(pos) {
                if pos.0 == line.origin.0 {
                    ret += (pos.1 - line.origin.1).abs();
                } else {
                    ret += (pos.0 - line.origin.0).abs();
                }
                found = true;
                break;
            } else {
                ret += line.len() as i32;
            }
        }
        if found {
            Some(ret as u32)
        } else {
            None
        }
    }
}

#[derive(Debug, Default)]
struct Grid {
    wires: Vec<WirePath>,
}

impl Grid {
    fn get_intersections(&self) -> Vec<Position> {
        // Get all the lines between nodes
        let mut path_lines = Vec::new();
        for wire in &self.wires {
            path_lines.push(wire.get_lines());
        }
        // Find any points where they cross
        // for each vec in lines, check against each other vec

        let mut ret = Vec::new();
        for current_path_idx in 0..path_lines.len() {
            let current_path = &path_lines[current_path_idx];
            for other_path_idx in 0..path_lines.len() {
                if current_path_idx == other_path_idx {
                    // Only check against other paths, not lines in same wire
                    continue;
                } else {
                    // Check each line in path against each line in each other path
                    let other_path = &path_lines[other_path_idx];
                    for line in current_path {
                        for other_line in other_path {
                            if let Some(intersection) = line.intersection(*other_line) {
                                if line.contains(intersection) && other_line.contains(intersection)
                                {
                                    ret.push(intersection);
                                }
                            }
                        }
                    }
                }
            }
        }
        ret.sort();
        ret.dedup();
        ret
    }
    fn get_closest_intersection_distance(&self) -> Option<u32> {
        // Traverse nodes, skipping the origin
        // collect distance from origin of any that have more than one wire
        self.get_intersections().iter().fold(None, |acc, el| {
            let distance = Position::default().manhattan_distance(*el) as u32;
            if distance > 0 {
                match acc {
                    None => Some(distance),
                    Some(ret) => Some(ret.min(distance)),
                }
            } else {
                None
            }
        })
    }
    fn get_path_length(&self, pos: Position) -> Option<u32> {
        self.wires.iter().fold(None, |acc, wire| {
            let dist = wire.distance_to_pos(pos);
            match acc {
                None => dist,
                Some(x) => Some(x + dist.unwrap_or(0)),
            }
        })
    }
    fn get_optimal_path_length(&self) -> Option<u32> {
        self.get_intersections().iter().fold(None, |acc, pos| {
            let len = self.get_path_length(*pos);
            println!("Checking {:?} (length {:?}) against {:?}", pos, len, acc);
            if len == Some(0) {
                return acc;
            }
            match acc {
                None => len,
                Some(x) => match len {
                    None => acc,
                    Some(y) => {
                        Some(x.min(y))
                    }
                },
            }
        })
    }
}

impl FromStr for Grid {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret = Self::default();
        for path in s.split('\n') {
            ret.wires.push(WirePath::from_str(path)?);
        }
        println!("g: {:?}", ret);
        Ok(ret)
    }
}

fn closest_intersection_distance(input: &str) -> Result<Option<u32>, io::Error> {
    let grid = Grid::from_str(input)?;
    Ok(grid.get_closest_intersection_distance())
}

fn optimal_path_length(input: &str) -> Result<Option<u32>, io::Error> {
    let grid = Grid::from_str(input)?;
    Ok(grid.get_optimal_path_length())
}

pub fn run() {
    println!(
        "{}",
        closest_intersection_distance(&get_puzzle_string(3).unwrap())
            .unwrap()
            .unwrap()
    );
    println!(
        "{}",
        optimal_path_length(&get_puzzle_string(3).unwrap())
            .unwrap()
            .unwrap()
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_manhattan_distance() {
        assert_eq!(
            Position::default().manhattan_distance(Position::default().into()),
            0
        );
        assert_eq!(
            Position::default().manhattan_distance(Position(1, 0).into()),
            1
        );
        assert_eq!(
            Position::default().manhattan_distance(Position(0, 1).into()),
            1
        );
        assert_eq!(
            Position::default().manhattan_distance(Position(1, 1).into()),
            2
        );
        assert_eq!(
            Position::default().manhattan_distance(Position(1, -1).into()),
            2
        );
        assert_eq!(
            Position::default().manhattan_distance(Position(3, 2).into()),
            5
        );
    }
    #[test]
    fn test_lines_from_wirepath() {
        assert_eq!(
            WirePath::from_str("R8,U5,L5,D3").unwrap().get_lines(),
            vec![
                Line::new(Position(0, 0), Position(8, 0)),
                Line::new(Position(8, 0), Position(8, 5)),
                Line::new(Position(8, 5), Position(3, 5)),
                Line::new(Position(3, 5), Position(3, 2)),
            ]
        );
    }
    #[test]
    fn test_line_contains_position() {
        let line = Line::new(Position::default(), Position(0, 8));
        assert_eq!(line.contains(Position(0, 5)), true);
        assert_eq!(line.contains(Position(0, 9)), false);
        assert_eq!(line.contains(Position(1, 5)), false);
    }
    #[test]
    fn test_intersection() {
        let line_a = Line::new(Position(1, 1), Position(4, 4));
        let line_b = Line::new(Position(1, 8), Position(2, 4));
        assert_eq!(line_a.intersection(line_b), None);
        let line_c = Line::new(Position(3, 2), Position(3, 5));
        let line_d = Line::new(Position(2, 3), Position(6, 3));
        assert_eq!(line_c.intersection(line_d), Some(Position(3, 3)));
    }
    #[test]
    fn test_step_from_str() {
        assert_eq!(
            Step::from_str("D344").unwrap(),
            Step {
                direction: Direction::Down,
                length: 344
            }
        );
    }
    #[test]
    fn test_wirepath_from_str() {
        use Direction::*;
        assert_eq!(
            WirePath::from_str("R8,U5,L5,D3").unwrap(),
            WirePath(vec![
                Step {
                    direction: Right,
                    length: 8
                },
                Step {
                    direction: Up,
                    length: 5
                },
                Step {
                    direction: Left,
                    length: 5
                },
                Step {
                    direction: Down,
                    length: 3
                }
            ])
        )
    }
    #[test]
    fn test_closest_intersection_distance() {
        assert_eq!(
            closest_intersection_distance("R8,U5,L5,D3\nU7,R6,D4,L4").unwrap(),
            Some(6)
        );
        assert_eq!(
            closest_intersection_distance(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )
            .unwrap(),
            Some(135)
        );
        assert_eq!(
            closest_intersection_distance(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            )
            .unwrap(),
            Some(159)
        );
    }
    #[test]
    fn test_walk_to_point() {
        assert_eq!(
            WirePath::from_str("R8,U5,L5,D3")
                .unwrap()
                .distance_to_pos(Position(3, 3)),
            Some(20)
        );
    }
    #[test]
    fn test_optimal_path_length() {
        assert_eq!(
            optimal_path_length("R8,U5,L5,D3\nU7,R6,D4,L4").unwrap(),
            Some(30)
        );
        assert_eq!(
            optimal_path_length(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )
            .unwrap(),
            Some(410)
        );
        assert_eq!(
            optimal_path_length(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            )
            .unwrap(),
            Some(610)
        );
    }
    #[test]
    fn test_solutions() {
        assert_eq!(
            closest_intersection_distance(&get_puzzle_string(3).unwrap())
                .unwrap()
                .unwrap(),
            248
        );
    }
}
