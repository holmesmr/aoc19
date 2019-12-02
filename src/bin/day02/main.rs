use std::env;
use std::str::FromStr;

mod intcode;

use intcode::IntcodeCPU;

fn load_initial_program_state(input: &str) -> Vec<u32> {
    input
        .split(',')
        .enumerate()
        .map(|(i, pos)| {
            u32::from_str(pos.trim()).unwrap_or_else(|_| {
                panic!("Could not interpret '{}' at position {} as u32", pos, i)
            })
        })
        .collect()
}

fn set_inputs(state: &mut [u32], noun: u32, verb: u32) {
    state[1] = noun;
    state[2] = verb;
}

fn part1(input: &str) {
    let mut program = load_initial_program_state(input);
    set_inputs(&mut program, 12, 2);

    let mut cpu = IntcodeCPU::new(program);
    cpu.run().expect("Should not have excepted at runtime");

    println!("Value at position 0: {}", cpu.output());
}

const PART2_NOUN_MIN: u32 = 0;
const PART2_NOUN_MAX: u32 = 100;
const PART2_VERB_MIN: u32 = 0;
const PART2_VERB_MAX: u32 = 100;
const PART2_TARGET_OUTPUT: u32 = 19690720;

fn part2(input: &str) {
    let mut program = load_initial_program_state(input);

    let program_ref = &program;

    // Search input space
    for noun in PART2_NOUN_MIN..PART2_NOUN_MAX {
        for verb in PART2_VERB_MIN..PART2_VERB_MAX {
            let mut program = program_ref.clone();
            set_inputs(&mut program, noun, verb);

            let mut cpu = IntcodeCPU::new(program);
            let res = cpu.run();

            if let Err(ex) = res {
                eprintln!("WARNING: CPU exception {:?} at position {} while running with inputs (noun = {}, verb = {}). Skipping", ex, cpu.pc(), noun, verb);
                continue;
            }

            if cpu.output() == PART2_TARGET_OUTPUT {
                println!(
                    "Solution found (noun = {}, verb = {}). Answer is {}",
                    noun,
                    verb,
                    (100 * noun + verb)
                );
                return;
            }
        }
    }

    println!("ERROR: Could not find suitable answer in solution space.");
    std::process::exit(2);
}

fn main() {
    let input = include_str!("../../../input/day02/input");
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
    fn aoc19_day2_part1_example_1() {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50";
        let expected_state: &[u32] = &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50][..];

        let prog = load_initial_program_state(input);
        let mut cpu = IntcodeCPU::new(prog);

        cpu.run().expect("Should not have excepted at runtime");

        assert_eq!(cpu.inspect_state(), expected_state);
    }

    #[test]
    fn aoc19_day2_part1_example_2() {
        let input = "1,0,0,0,99";
        let expected_state: &[u32] = &[2, 0, 0, 0, 99][..];

        let prog = load_initial_program_state(input);
        let mut cpu = IntcodeCPU::new(prog);

        cpu.run().expect("Should not have excepted at runtime");

        assert_eq!(cpu.inspect_state(), expected_state);
    }

    #[test]
    fn aoc19_day2_part1_example_3() {
        let input = "2,3,0,3,99";
        let expected_state: &[u32] = &[2, 3, 0, 6, 99][..];

        let prog = load_initial_program_state(input);
        let mut cpu = IntcodeCPU::new(prog);

        cpu.run().expect("Should not have excepted at runtime");

        assert_eq!(cpu.inspect_state(), expected_state);
    }

    #[test]
    fn aoc19_day2_part1_example_4() {
        let input = "2,4,4,5,99,0";
        let expected_state: &[u32] = &[2, 4, 4, 5, 99, 9801][..];

        let prog = load_initial_program_state(input);
        let mut cpu = IntcodeCPU::new(prog);

        cpu.run().expect("Should not have excepted at runtime");

        assert_eq!(cpu.inspect_state(), expected_state);
    }

    #[test]
    fn aoc19_day2_part1_example_5() {
        let input = "1,1,1,4,99,5,6,0,99";
        let expected_state: &[u32] = &[30, 1, 1, 4, 2, 5, 6, 0, 99][..];

        let prog = load_initial_program_state(input);
        let mut cpu = IntcodeCPU::new(prog);

        cpu.run().expect("Should not have excepted at runtime");

        assert_eq!(cpu.inspect_state(), expected_state);
    }
}
