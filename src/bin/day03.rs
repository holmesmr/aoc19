use itertools::Itertools;
use std::collections::HashSet;
use std::env;
use std::iter::FromIterator;
use std::str::FromStr;

fn parse_path(path_str: &str) -> Vec<(i32, i32)> {
    let mut point = (0, 0);

    path_str
        .split(',')
        .flat_map(|dir| {
            println!("started line {} at {:?}", dir, point);

            let current = point;
            let (dir, count) = dir.split_at(1);
            let count = i32::from_str(count).expect("Couldn't parse count path");
            let ret = match dir {
                "U" => {
                    point.1 += count;
                    (current.1..current.1 + count)
                        .map(|y| (current.0, y + 1))
                        .collect::<Vec<(i32, i32)>>()
                }
                "D" => {
                    point.1 -= count;
                    (current.1 - count..current.1)
                        .rev()
                        .map(|y| (current.0, y + 1))
                        .collect::<Vec<(i32, i32)>>()
                }
                "L" => {
                    point.0 -= count;
                    (current.0 - count..current.0)
                        .rev()
                        .map(|x| (x + 1, current.1))
                        .collect::<Vec<(i32, i32)>>()
                }
                "R" => {
                    point.0 += count;
                    (current.0..current.0 + count)
                        .map(|x| (x + 1, current.1))
                        .collect::<Vec<(i32, i32)>>()
                }
                c => panic!("Unknown direction {} encountered", c),
            };

            println!("finished line {} at {:?}", dir, point);
            ret
        })
        .collect()
}

fn find_smallest_intersection_distance(
    path1: Vec<(i32, i32)>,
    path2: Vec<(i32, i32)>,
) -> Option<i32> {
    HashSet::<(i32, i32)>::from_iter(path1)
        .intersection(&HashSet::<(i32, i32)>::from_iter(path2))
        .map(|(x, y)| x + y)
        .min()
}

fn part1(input: &str) {
    let mut input_lines = input.lines();

    let path1 = parse_path(input_lines.next().expect("First path missing from input"));
    let path2 = parse_path(input_lines.next().expect("Second path missing from input"));

    let dist =
        find_smallest_intersection_distance(path1, path2).expect("could not find an intersection");

    println!("Smallest intersection is {}", dist);
}

fn part2(input: &str) {
    let mut input_lines = input.lines();

    let path1 = parse_path(input_lines.next().expect("First path missing from input"));
    let path2 = parse_path(input_lines.next().expect("Second path missing from input"));

    //    let dist = find_smallest_intersection_path_length(&path1, &path2)
    //        .expect("could not find an intersection");
    //
    //    println!("Shortest sum of distances to an intersection is {}", dist);
}

fn main() {
    let input = include_str!("../../input/day03/input");
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
    fn aoc19_day3_part1_example_1() {
        let path1 = parse_path("R8,U5,L5,D3");
        let path2 = parse_path("U7,R6,D4,L4");

        println!("Path 1: {:?}, Path 2: {:?}", &path1, &path2);

        let dist = find_smallest_intersection_distance(path1, path2)
            .expect("could not find an intersection");

        assert_eq!(dist, 6);
        panic!("bad");
    }

    #[test]
    fn aoc19_day3_part1_example_2() {
        let path1 = parse_path("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let path2 = parse_path("U62,R66,U55,R34,D71,R55,D58,R83");

        let dist = find_smallest_intersection_distance(path1, path2)
            .expect("could not find an intersection");

        assert_eq!(dist, 159);
    }

    #[test]
    fn aoc19_day3_part1_example_3() {
        let path1 = parse_path("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let path2 = parse_path("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

        let dist = find_smallest_intersection_distance(path1, path2)
            .expect("could not find an intersection");

        assert_eq!(dist, 135);
    }

    #[test]
    fn aoc19_day3_part2_example_1() {
        let path1 = parse_path("R8,U5,L5,D3");
        let path2 = parse_path("U7,R6,D4,L4");

        //        let dist = find_smallest_intersection_path_length(&path1, &path2)
        //            .expect("could not find an intersection");
        //
        //        assert_eq!(dist, 30);
    }

    #[test]
    fn aoc19_day3_part2_example_2() {
        let path1 = parse_path("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let path2 = parse_path("U62,R66,U55,R34,D71,R55,D58,R83");

        //        let dist = find_smallest_intersection_path_length(&path1, &path2)
        //            .expect("could not find an intersection");
        //
        //        assert_eq!(dist, 610);
    }

    #[test]
    fn aoc19_day3_part2_example_3() {
        let path1 = parse_path("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let path2 = parse_path("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

        //        let dist = find_smallest_intersection_path_length(&path1, &path2)
        //            .expect("could not find an intersection");
        //
        //        assert_eq!(dist, 410);
    }
}
