use crate::util::Part;
use crate::util::int_code_computer::*;


pub fn solve(input:String, part:Part) -> String {

    let opcodes:Vec<i64> = input.split(',')
        .map(|op| op.trim().parse().unwrap())
        .collect();

    let result = match part {
        Part::Part1 => part1(opcodes),
        Part::Part2 => part2(opcodes)
    };

    format!("{}",result)
}

fn part1(opcodes:Vec<i64>) -> i64 {
    let inputs = vec![1];
    let mut program = Program::new(opcodes, Some(inputs));
    program.run();
    program.get_last_output().unwrap()
}

fn part2(opcodes:Vec<i64>) -> i64 {
    let inputs = vec![2];
    let mut program = Program::new(opcodes, Some(inputs));
    program.run();
    program.get_last_output().unwrap()
}
