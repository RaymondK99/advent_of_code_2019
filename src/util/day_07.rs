use crate::util::Part;
use crate::util::int_code_computer::*;
use permute;

pub fn solve(input:String, part:Part) -> String {

    let opcodes:Vec<i32> = input.split(',')
        .map(|op| op.trim().parse().unwrap())
        .collect();

    let result = match part {
        Part::Part1 => part1(opcodes),
        Part::Part2 => part2(opcodes)
    };

    format!("{}",result)
}

fn part1(opcodes:Vec<i32>) -> i32 {
    find_max(opcodes)
}

fn part2(opcodes:Vec<i32>) -> i32 {

    1
}


fn run_thrusters(phase_settings:Vec<i32>, opcodes:&Vec<i32>) -> i32 {
    let mut last_output = 0;
    let amps = vec!['A','B','C','D','E'];

    for i in 0..5 {
        //println!("=> Running amp:{}, with setting:{}, and input:{}", amps[i], phase_settings[i], last_output);
        let mut program = Program::new( opcodes.clone(), Some(vec![phase_settings[i], last_output]));
        program.run();
        last_output = program.get_last_output();
    }

    last_output
}

fn find_max(opcodes:Vec<i32>) -> i32 {
    let mut max_value = 0;
    let initial_permutation = vec![0,1,2,3,4];

    for phase_setting in permute::permute(initial_permutation) {
        let value = run_thrusters(phase_setting, &opcodes);
        max_value = std::cmp::max( value, max_value);
    }

    max_value
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() {
        let opcodes = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        assert_eq!(run_thrusters(vec![4,3,2,1,0], &opcodes),43210);
    }

    #[test]
    fn test2() {
        let opcodes = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,
                           101,5,23,23,1,24,23,23,4,23,99,0,0];
        assert_eq!(run_thrusters(vec![0,1,2,3,4], &opcodes),54321);
    }

    #[test]
    fn test3() {
        let opcodes = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
                           1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        assert_eq!(run_thrusters(vec![1,0,4,3,2], &opcodes),65210);
    }



    #[test]
    fn test_part_1() {
        let opcodes = vec![3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 38, 59,84,97,110,191,272,353,434,99999,3,9,1002,9,2,9,101,4,9,9,1002,9,2,9,4,9,99,3,9,102,5,9,9,1001,9,3,9,1002,9,5,9,101,5,9,9,4,9,99,3,9,102,5,9,9,101,5,9,9,1002,9,3,9,101,2,9,9,1002,9,4,9,4,9,99,3,9,101,3,9,9,1002,9,3,9,4,9,99,3,9,102,5,9,9,1001,9,3,9,4,9,99,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,99];
        let res = find_max(opcodes);
        println!("max:{}",res);
        assert_eq!(res,338603);
    }
}