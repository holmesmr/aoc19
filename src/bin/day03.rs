use itertools::Itertools;
use std::env;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Axis {
    X,
    Y,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn axis(self) -> Axis {
        use Axis::*;
        use Direction::*;

        match self {
            Up | Down => Y,
            Left | Right => X,
        }
    }

    fn parallel(self, other: Self) -> bool {
        self.axis() == other.axis()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    fn apply(self, movement: Movement) -> Self {
        use Direction::*;

        match movement.0 {
            Up => Coordinate {
                y: self.y + movement.1,
                ..self
            },
            Down => Coordinate {
                y: self.y - movement.1,
                ..self
            },
            Left => Coordinate {
                x: self.x - movement.1,
                ..self
            },
            Right => Coordinate {
                x: self.x + movement.1,
                ..self
            },
        }
    }

    fn taxicab_distance(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn taxicab_origin_distance(self) -> i32 {
        self.taxicab_distance(Coordinate::zero())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Movement(Direction, i32);

#[derive(Debug, Copy, Clone)]
struct ParseMovementError {}

impl FromStr for Movement {
    type Err = ParseMovementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, count) = s.split_at(1);

        let dir = match dir {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(ParseMovementError {}),
        };

        let count = i32::from_str(count).map_err(|_| ParseMovementError {})?;

        Ok(Movement(dir, count))
    }
}

type PathOwned = Vec<Movement>;
type Path<'a> = &'a [Movement];

#[derive(Debug, Clone, Eq, PartialEq)]
struct Segment {
    start_pos: Coordinate,
    end_pos: Coordinate,
    movement: Movement,
}

impl Segment {
    fn between_x(&self, x: i32) -> bool {
        let min_x = i32::min(self.start_pos.x, self.end_pos.x);
        let max_x = i32::max(self.start_pos.x, self.end_pos.x);

        x >= min_x && x <= max_x
    }

    fn between_y(&self, y: i32) -> bool {
        let min_y = i32::min(self.start_pos.y, self.end_pos.y);
        let max_y = i32::max(self.start_pos.y, self.end_pos.y);

        y >= min_y && y <= max_y
    }

    fn intersects_at(&self, other: &Self) -> Option<Coordinate> {
        if !self.movement.0.parallel(other.movement.0) {
            let (x_seg, y_seg) = match self.movement.0.axis() {
                Axis::X => (self, other),
                Axis::Y => (other, self),
            };

            // Y doesn't change on horizontal segments and vice versa
            let intersect_x = y_seg.start_pos.x;
            let intersect_y = x_seg.start_pos.y;
            if y_seg.between_y(intersect_y) && x_seg.between_x(intersect_x) {
                return Some(Coordinate {
                    x: intersect_x,
                    y: intersect_y,
                });
            }
        }

        None
    }
}

type PathSegmentsOwned = Vec<Segment>;
type PathSegments<'a> = &'a [Segment];

fn path_segments(start: Coordinate, path: Path) -> PathSegmentsOwned {
    let mut current = start;
    let mut segments = Vec::with_capacity(path.len());
    for &movement in path {
        let new = current.apply(movement);
        segments.push(Segment {
            start_pos: current,
            end_pos: new,
            movement,
        });
        current = new;
    }

    segments
}

fn parse_path(path_str: &str) -> PathOwned {
    path_str
        .split(',')
        .map(FromStr::from_str)
        .map(|r| r.expect("Failed to parse direction"))
        .collect()
}

fn path_length(path: PathSegments) -> i32 {
    path.iter()
        .fold(0, |acc, segment| acc + segment.movement.1.abs())
}

fn find_smallest_intersection_distance(path1: Path, path2: Path) -> Option<i32> {
    let segs1 = path_segments(Coordinate::zero(), path1);
    let segs2 = path_segments(Coordinate::zero(), path2);

    // Skip both of first here as while an intersection at origin is valid, it's not an intersection
    // if the origin start points have divergent axis (and it's impossible for multidimensional
    // movement in this problem space)
    segs1
        .into_iter()
        .skip(1)
        .cartesian_product(segs2.into_iter().skip(1))
        .filter_map(|(seg1, seg2)| seg1.intersects_at(&seg2))
        .map(Coordinate::taxicab_origin_distance)
        .min()
}

fn find_smallest_intersection_path_length(path1: Path, path2: Path) -> Option<i32> {
    let segs1 = path_segments(Coordinate::zero(), path1);
    let segs2 = path_segments(Coordinate::zero(), path2);

    let segs1_ref = &segs1;
    let segs2_ref = &segs2;

    segs1
        .iter()
        .skip(1)
        .cartesian_product(segs2.iter().skip(1))
        // Find intersection points
        .filter_map(|(seg1, seg2)| seg1.intersects_at(seg2).map(|pos| ((seg1, seg2), pos)))
        .map(|((seg1, seg2), pos)| {
            let segs1_path_end = segs1_ref
                .iter()
                .position(|seg| seg == seg1)
                .expect("Somehow cannot find referenced segment 1");
            let segs2_path_end = segs2_ref
                .iter()
                .position(|seg| seg == seg2)
                .expect("Somehow cannot find referenced segment 2");

            // We also have to factor in the distance from the start of the last segment to intersection
            let seg1_last_start = segs1_ref[segs1_path_end].start_pos;
            let seg2_last_start = segs2_ref[segs2_path_end].start_pos;

            path_length(&segs1_ref[0..segs1_path_end])
                + path_length(&segs2_ref[0..segs2_path_end])
                + pos.taxicab_distance(seg1_last_start)
                + pos.taxicab_distance(seg2_last_start)
        })
        .min()
}

fn part1(input: &str) {
    let mut input_lines = input.lines();

    let path1 = parse_path(input_lines.next().expect("First path missing from input"));
    let path2 = parse_path(input_lines.next().expect("Second path missing from input"));

    let dist = find_smallest_intersection_distance(&path1, &path2)
        .expect("could not find an intersection");

    println!("Smallest intersection is {}", dist);
}

fn part2(input: &str) {
    let mut input_lines = input.lines();

    let path1 = parse_path(input_lines.next().expect("First path missing from input"));
    let path2 = parse_path(input_lines.next().expect("Second path missing from input"));

    let dist = find_smallest_intersection_path_length(&path1, &path2)
        .expect("could not find an intersection");

    println!("Shortest sum of distances to an intersection is {}", dist);
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

        let dist = find_smallest_intersection_distance(&path1, &path2)
            .expect("could not find an intersection");

        assert_eq!(dist, 6);
    }

    #[test]
    fn aoc19_day3_part1_example_2() {
        let path1 = parse_path("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let path2 = parse_path("U62,R66,U55,R34,D71,R55,D58,R83");

        let dist = find_smallest_intersection_distance(&path1, &path2)
            .expect("could not find an intersection");

        assert_eq!(dist, 159);
    }

    #[test]
    fn aoc19_day3_part1_example_3() {
        let path1 = parse_path("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let path2 = parse_path("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

        let dist = find_smallest_intersection_distance(&path1, &path2)
            .expect("could not find an intersection");

        assert_eq!(dist, 135);
    }

    #[test]
    fn aoc19_day3_part2_example_1() {
        let path1 = parse_path("R8,U5,L5,D3");
        let path2 = parse_path("U7,R6,D4,L4");

        let dist = find_smallest_intersection_path_length(&path1, &path2)
            .expect("could not find an intersection");

        assert_eq!(dist, 30);
    }

    #[test]
    fn aoc19_day3_part2_example_2() {
        let path1 = parse_path("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let path2 = parse_path("U62,R66,U55,R34,D71,R55,D58,R83");

        let dist = find_smallest_intersection_path_length(&path1, &path2)
            .expect("could not find an intersection");

        assert_eq!(dist, 610);
    }

    #[test]
    fn aoc19_day3_part2_example_3() {
        let path1 = parse_path("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let path2 = parse_path("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

        let dist = find_smallest_intersection_path_length(&path1, &path2)
            .expect("could not find an intersection");

        assert_eq!(dist, 410);
    }
}
