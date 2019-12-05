use std::env;
use std::str::FromStr;

mod intcode;

use intcode::IntcodeCPU;

fn load_initial_program_state(input: &str) -> Vec<i32> {
    input
        .split(',')
        .enumerate()
        .map(|(i, pos)| {
            i32::from_str(pos.trim()).unwrap_or_else(|_| {
                panic!("Could not interpret '{}' at position {} as u32", pos, i)
            })
        })
        .collect()
}

fn part1(input: &str) {
    let mut program = load_initial_program_state(input);

    let mut cpu = IntcodeCPU::new(program);
    cpu.run().expect("Should not have excepted at runtime");

    println!("Program finished");
}

fn part2(input: &str) {

}

fn main() {
    let input = include_str!("../../../input/day05/input");
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
