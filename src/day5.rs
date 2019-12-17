use crate::*;

pub fn run() {
    println!(
        "{}",
        intcode::intcode(&get_puzzle_string(5).unwrap(), true).0
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
}
