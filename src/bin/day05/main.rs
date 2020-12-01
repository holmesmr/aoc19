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

fn main() {
    let input = include_str!("../../../input/day05/input");

    let mut program = load_initial_program_state(input);

    let mut cpu = IntcodeCPU::new(program);
    cpu.run().expect("Should not have excepted at runtime");

    println!("Program finished");
}
