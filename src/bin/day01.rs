use std::env;
use std::ops::Add;
use std::str::FromStr;

fn calculate_fuel_requirement_naively(mass: i64) -> i64 {
    (mass / 3) - 2
}

fn calculate_fuel_requirement_recursively(mass: i64) -> i64 {
    let fuel_requirement = calculate_fuel_requirement_naively(mass);

    if fuel_requirement > 0 {
        fuel_requirement + calculate_fuel_requirement_recursively(fuel_requirement)
    } else {
        0
    }
}

fn intify_lines(input: &str) -> Vec<i64> {
    input
        .lines()
        .enumerate()
        .map(|(n, line)| {
            i64::from_str(line).unwrap_or_else(|_| panic!("cannot parse input line {}", n))
        })
        .collect()
}

fn part1(input: &str) {
    use std::str::FromStr;

    let result = intify_lines(input)
        .into_iter()
        .map(calculate_fuel_requirement_naively)
        .fold(0i64, Add::add);

    println!("Total fuel required: {}", result);
}

fn part2(input: &str) {
    use std::str::FromStr;

    let result = intify_lines(input)
        .into_iter()
        .map(calculate_fuel_requirement_recursively)
        .fold(0i64, Add::add);

    println!("Total fuel required: {}", result);
}

fn main() {
    let input = include_str!("../../input/day01/input");
    let mut args = env::args();

    let prog_name = args.next().expect("unable to get program name");

    let maybe_arg = args.next();
    let maybe_arg_str = maybe_arg.as_ref().map(String::as_str);

    match maybe_arg_str {
        Some("part1") => part1(input),
        Some("part2") => part2(input),
        _ => {
            eprintln!("usage: {} (part1|part2)", prog_name);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc19_day1_part1_example_1() {
        assert_eq!(calculate_fuel_requirement_naively(12), 2);
    }

    #[test]
    fn aoc19_day1_part1_example_2() {
        assert_eq!(calculate_fuel_requirement_naively(14), 2);
    }

    #[test]
    fn aoc19_day1_part1_example_3() {
        assert_eq!(calculate_fuel_requirement_naively(1969), 654);
    }

    #[test]
    fn aoc19_day1_part1_example_4() {
        assert_eq!(calculate_fuel_requirement_naively(100756), 33583);
    }

    #[test]
    fn aoc19_day1_part2_example_1() {
        assert_eq!(calculate_fuel_requirement_naively(12), 2);
    }

    #[test]
    fn aoc19_day1_part2_example_2() {
        assert_eq!(calculate_fuel_requirement_recursively(1969), 966);
    }

    #[test]
    fn aoc19_day1_part2_example_3() {
        assert_eq!(calculate_fuel_requirement_recursively(100756), 50346);
    }
}
