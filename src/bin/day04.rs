use is_sorted::IsSorted;
use itertools::Itertools;
use std::str::FromStr;

fn is_number_valid_passcode_part1(&num: &i32) -> bool {
    num >= 100000 && num <= 999999 &&
        // All digits are bigger than the last, left to right
        (IsSorted::is_sorted(&mut (0..6).rev().map(|n| (num / 10_i32.pow(n)) % 10))) &&
        // Any two repeating digits
        ((0..5).map(|n| (num / 10_i32.pow(n)) % 100 % 11 == 0).any(|b| b))
}

fn is_number_also_valid_passcode_part2(&num: &i32) -> bool {
    num.to_string()
        .as_bytes()
        .iter()
        .group_by(|&&b| b)
        .into_iter()
        .any(|(_, c)| c.count() == 2)
}

fn parse_input(input: String) -> (i32, i32) {
    let mut inputs = input.split('-').take(2);
    let lower = i32::from_str(inputs.next().expect("Expected number-number as input"))
        .expect("Expected lower bound to be valid integer");
    let upper = i32::from_str(inputs.next().expect("Expected number-number as input"))
        .expect("Expected upper bound to be valid integer");
    (lower, upper)
}

fn part1((lower, upper): (i32, i32)) -> usize {
    (lower..upper + 1)
        .filter(is_number_valid_passcode_part1)
        .count()
}

fn part2((lower, upper): (i32, i32)) -> usize {
    (lower..upper + 1)
        .filter(is_number_valid_passcode_part1)
        .filter(is_number_also_valid_passcode_part2)
        .count()
}

fn main() {
    let mut args = std::env::args();

    let prog_name = args.next().expect("unable to get program name");

    let maybe_arg = args.next();
    let maybe_arg_str = maybe_arg.as_ref().map(String::as_str);

    let maybe_input = args.next();

    match (maybe_arg_str, maybe_input) {
        (Some("part1"), Some(input)) => {
            println!("Number of valid passwords: {}", part1(parse_input(input)))
        }
        (Some("part2"), Some(input)) => {
            println!("Number of valid passwords: {}", part2(parse_input(input)))
        }
        _ => {
            eprintln!("usage: {} (part1|part2) INPUT", prog_name);
            std::process::exit(1);
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn aoc19_day4_part1_example_1() {
        assert!(is_number_valid_passcode_part1(&111111));
    }

    #[test]
    fn aoc19_day4_part1_example_2() {
        assert!(!is_number_valid_passcode_part1(&223450));
    }

    #[test]
    fn aoc19_day4_part1_example_3() {
        assert!(!is_number_valid_passcode_part1(&123789));
    }

    #[test]
    fn aoc19_day4_part2_example_1() {
        assert!(is_number_valid_passcode_part1(&112233));
        assert!(is_number_also_valid_passcode_part2(&112233));
    }

    #[test]
    fn aoc19_day4_part2_example_2() {
        assert!(is_number_valid_passcode_part1(&123444));
        assert!(!is_number_also_valid_passcode_part2(&123444));
    }

    #[test]
    fn aoc19_day4_part2_example_3() {
        assert!(is_number_valid_passcode_part1(&111122));
        assert!(is_number_also_valid_passcode_part2(&111122));
    }
}
