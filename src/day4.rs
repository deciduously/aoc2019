use std::{io, str::FromStr};

const PUZZLE: &str = "278384-824795";

fn char_strs(s: &str) -> Vec<&str> {
    s.split("")
        .skip(1)
        .take_while(|c| *c != "")
        .collect::<Vec<&str>>()
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Id(u64);

impl Id {
    fn validate(id: &str, hardcore_rules: bool) -> bool {
        // need at least a double pair and is all in ascending order
        let mut hardcore_sentinel = false;
        let (found_a_double, _, all_ascending, _) = char_strs(id).iter().fold(
            (false, 0, true, None),
            |(found_a_double, streak, ret_2, previous_dig), dig| match previous_dig {
                None => (found_a_double, streak, ret_2, Some(*dig)),
                Some(s) => {
                    let n = s.parse::<u64>().unwrap();
                    let m = dig.parse::<u64>().unwrap();
                    let streak_check = if hardcore_rules {
                        if streak == 1 && n != m {
                            // we found an exact twofer
                            hardcore_sentinel = true;
                        }
                        hardcore_sentinel || (streak == 0 && n == m)
                    } else {
                        n == m || found_a_double
                    };
                    (
                        streak_check,
                        if n == m { streak + 1 } else { 0 },
                        m >= n && ret_2,
                        Some(*dig),
                    )
                }
            },
        );
        let double_ck = if hardcore_rules {
            hardcore_sentinel || found_a_double
        } else {
            found_a_double
        };
        double_ck && all_ascending
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct IdRange {
    lower: u64,
    higher: u64,
}

impl FromStr for IdRange {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split('-');
        let lower = nums.next().unwrap().parse::<u64>().unwrap();
        let higher = nums.next().unwrap().parse::<u64>().unwrap();
        Ok(Self { lower, higher })
    }
}

impl IdRange {
    fn total_inputs(&self, hardcore_rules: bool) -> u32 {
        let mut ret = 0;
        for i in self.lower..=self.higher {
            if Id::validate(&format!("{}", i), hardcore_rules) {
                ret += 1;
            }
        }
        ret
    }
}

pub fn run() {
    println!("{}", IdRange::from_str(PUZZLE).unwrap().total_inputs(false));
    println!("{}", IdRange::from_str(PUZZLE).unwrap().total_inputs(true));
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_validate_part1() {
        assert_eq!(Id::validate("122345", false), true);
        assert_eq!(Id::validate("111123", false), true);
        assert_eq!(Id::validate("135679", false), false);
        assert_eq!(Id::validate("111111", false), true);
        assert_eq!(Id::validate("223450", false), false);
        assert_eq!(Id::validate("123789", false), false);
    }
    #[test]
    fn test_validate_part2() {
        assert_eq!(Id::validate("112233", true), true);
        assert_eq!(Id::validate("123444", true), false);
        assert_eq!(Id::validate("111122", true), true);
    }
    #[test]
    fn test_range_from_str() {
        assert_eq!(
            IdRange::from_str(PUZZLE).unwrap(),
            IdRange {
                lower: 278384,
                higher: 824795
            }
        )
    }
    #[test]
    fn test_solutions() {
        // Maybe omit - this runs really fast in release mode but slow in test
        assert_eq!(IdRange::from_str(PUZZLE).unwrap().total_inputs(false), 921);
        assert_eq!(IdRange::from_str(PUZZLE).unwrap().total_inputs(true), 603);
    }
}
