use std::collections::{HashMap, HashSet};
use std::env;
use std::iter::FromIterator;

fn build_orbit_map(input: &str) -> HashMap<String, String> {
    input
        .lines()
        .map(|line| {
            let mut orbit_parts = line.split(')').take(2);

            let from = orbit_parts
                .next()
                .expect("Expected orbit of format FROM)TO")
                .to_string();
            let to = orbit_parts
                .next()
                .expect("Expected orbit of format FROM)TO")
                .to_string();

            // there can be multiple FROM but only one TO
            (to, from)
        })
        .collect()
}

fn tree_path_recurse(
    map: &HashMap<String, String>,
    key: &String,
    terminal: Option<&String>,
    count: u32,
) -> u32 {
    if let Some(terminal_key) = terminal {
        if terminal_key == key {
            return count;
        }
    }

    match map.get(key) {
        Some(k2) => tree_path_recurse(map, k2, terminal, count + 1),
        None => count,
    }
}

fn build_path(map: &HashMap<String, String>, start: &String) -> Vec<String> {
    let mut out = vec![start.clone()];
    let mut key = start;

    while let Some(next) = map.get(key) {
        key = next;
        out.push(next.clone());
    }

    out
}

fn calculate_transfer_distance(map: &HashMap<String, String>) -> u32 {
    let from = map
        .get(&"YOU".to_string())
        .expect("Could not find YOU in orbit map")
        .clone();
    let to = map
        .get(&"SAN".to_string())
        .expect("Could not find SAN in orbit map")
        .clone();

    let from_path = build_path(map, &from);
    let to_path = build_path(map, &to);

    HashSet::<String>::from_iter(from_path.iter().cloned())
        .intersection(&HashSet::from_iter(to_path.iter().cloned()))
        .map(|node| {
            tree_path_recurse(map, &from, Some(node), 0)
                + tree_path_recurse(map, &to, Some(node), 0)
        })
        .min()
        .expect("No intersections!")
}

fn sum_orbits(orbit_map: &HashMap<String, String>) -> u32 {
    orbit_map
        .keys()
        .map(|k| tree_path_recurse(orbit_map, k, None, 0))
        .sum()
}

fn get_transfer_from_to(orbit_map: &HashMap<String, String>) -> (String, String) {
    (
        orbit_map
            .get(&"YOU".to_string())
            .expect("Could not find YOU in orbit map")
            .clone(),
        orbit_map
            .get(&"SAN".to_string())
            .expect("Could not find SAN in orbit map")
            .clone(),
    )
}

fn part1(input: &str) {
    let orbit_map = build_orbit_map(input);

    println!(
        "Total number of direct and indirect orbits: {}",
        sum_orbits(&orbit_map)
    );
}

fn part2(input: &str) {
    let orbit_map = build_orbit_map(input);

    println!(
        "Minimum number of orbital transfers: {}",
        calculate_transfer_distance(&orbit_map)
    );
}

fn main() {
    let input = include_str!("../../input/day06/input");
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
    fn aoc19_day6_part1_example_1() {
        let input = r"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
";

        let orbit_map = build_orbit_map(input);

        assert_eq!(sum_orbits(&orbit_map), 42);
    }

    #[test]
    fn aoc19_day6_part2_example_1() {
        let input = r"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
";

        let orbit_map = build_orbit_map(input);

        assert_eq!(calculate_transfer_distance(&orbit_map), 4);
    }
}
