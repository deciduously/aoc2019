use super::get_puzzle_string;

fn fuel(n: i32) -> i32 {
    (n as f64 / 3.0).floor() as i32 - 2
}

// Almost identical to @marisalynn's python...
// def all_the_fuel(mass):
//     dividend_fuel = fuel(mass)
//     return_fuel = dividend_fuel
//     while True:
//         dividend_fuel = fuel(dividend_fuel)
//         if dividend_fuel < 0:
//             break
//         return_fuel += dividend_fuel
//     return return_fuel

fn fuel_with_dividends(n: i32) -> i32 {
    let mut dividend = fuel(n);
    let mut ret = dividend;
    loop {
        dividend = fuel(dividend);
        if dividend < 0 {
            break;
        }
        ret += dividend;
    }
    ret
}

fn sum_fuels(fuels_str: &str, fuel_fn: Box<dyn Fn(i32) -> i32>) -> i32 {
    fuels_str
        .split('\n')
        .map(|s| s.parse::<i32>().unwrap())
        .fold(0, |acc, el| fuel_fn(el) + acc)
}

pub fn run() {
    println!(
        "{}",
        sum_fuels(&get_puzzle_string(1).unwrap(), Box::new(fuel))
    );
    println!(
        "{}",
        sum_fuels(
            &get_puzzle_string(1).unwrap(),
            Box::new(fuel_with_dividends)
        )
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;
    #[test]
    fn test_sum_fuels() {
        let fuels = String::from_str("12\n14\n1969\n100756").unwrap();
        assert_eq!(sum_fuels(&fuels, Box::new(fuel)), 34241);
    }
    #[test]
    fn test_sum_dividends() {
        let fuels = String::from_str("14\n1969\n100756").unwrap();
        assert_eq!(sum_fuels(&fuels, Box::new(fuel_with_dividends)), 51314);
    }
    #[test]
    fn test_solutions() {
        assert_eq!(
            sum_fuels(&get_puzzle_string(1).unwrap(), Box::new(fuel)),
            3301059
        );
        assert_eq!(
            sum_fuels(
                &get_puzzle_string(1).unwrap(),
                Box::new(fuel_with_dividends)
            ),
            4948732
        );
    }
}
