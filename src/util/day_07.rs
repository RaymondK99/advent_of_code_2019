use crate::util::Part;
use crate::util::int_code_computer::*;
use permute;

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
    let mut max_value = 0;
    let initial_permutation = vec![0,1,2,3,4];

    for phase_setting in permute::permute(initial_permutation) {
        let value = run_thrusters(phase_setting, &opcodes, None);
        max_value = std::cmp::max( value.unwrap(), max_value);
    }

    max_value
}

fn part2(opcodes:Vec<i64>) -> i64 {
    let mut max_value = 0;
    let initial_permutation = vec![5,6,7,8,9];

    for phase_setting in permute::permute(initial_permutation).iter() {
        let value = run_thrusters2(phase_setting.clone(), &opcodes);
        max_value = std::cmp::max( value, max_value);
        //println!("Permutation:{:?}, value={}", phase_setting, value);
    }

    max_value
}


fn run_thrusters(phase_settings:Vec<i64>, opcodes:&Vec<i64>, input:Option<i64>) -> Option<i64> {
    let mut last_output = input.or(Some(0));

    for i in 0..5 {
        let mut program = Program::new(opcodes.clone(), Some(vec![phase_settings[i], last_output.unwrap()]));
        program.run();
        last_output = program.get_last_output();
    }

    last_output
}

fn run_thrusters2(phase_settings:Vec<i64>, opcodes:&Vec<i64>) -> i64 {
    let mut next_input = 0;
    let mut last_produced_output = 0;
    let mut programs = vec![];
    let mut is_halted = false;

    // Allocate programs
    for i in 0..5 {
        let mut program = Program::new(opcodes.clone(), None);
        program.add_input(phase_settings[i]);
        programs.push(program);
    }

    while !is_halted {
        for i in 0..5 {
            let program = programs.get_mut(i).unwrap();

            // Push input
            program.add_input(next_input);

            // Run until output is produced
            while !program.is_halted() && program.get_output().len() == 0 {
                program.run_instruction();
            }

            // Check output if any
            if !program.get_output().is_empty() {
                // Fetch next input
                next_input = program.pop_output();
                if i == 4 {
                    // Store last produced output by thruster E
                    last_produced_output = next_input;
                }
            }

            if program.is_halted() && i == 4 {
                is_halted = true;
                break;
            }
        }
    }

    last_produced_output
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() {
        let opcodes = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        assert_eq!(run_thrusters(vec![4,3,2,1,0], &opcodes, None).unwrap(),43210);
    }

    #[test]
    fn test2() {
        let opcodes = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,
                           101,5,23,23,1,24,23,23,4,23,99,0,0];
        assert_eq!(run_thrusters(vec![0,1,2,3,4], &opcodes, None).unwrap(),54321);
    }

    #[test]
    fn test3() {
        let opcodes = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
                           1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        assert_eq!(run_thrusters(vec![1,0,4,3,2], &opcodes, None).unwrap(),65210);
    }



    #[test]
    fn test_part_1() {
        let opcodes = vec![3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 38, 59,84,97,110,191,272,353,434,99999,3,9,1002,9,2,9,101,4,9,9,1002,9,2,9,4,9,99,3,9,102,5,9,9,1001,9,3,9,1002,9,5,9,101,5,9,9,4,9,99,3,9,102,5,9,9,101,5,9,9,1002,9,3,9,101,2,9,9,1002,9,4,9,4,9,99,3,9,101,3,9,9,1002,9,3,9,4,9,99,3,9,102,5,9,9,1001,9,3,9,4,9,99,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,99];
        let res = part1(opcodes);
        println!("max:{}",res);
        assert_eq!(res,338603);
    }

    #[test]
    fn test_part2_1() {
        let opcodes = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
                           27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];

        let res = part2(opcodes);
        println!("res={}",res);
        assert_eq!(res,139629729);
    }

    #[test]
    fn test_part2_2() {
        let opcodes = vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
                           -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
                           53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];

        let res = part2(opcodes);
        println!("res={}",res);
        assert_eq!(res,18216);
    }

    #[test]
    fn test_part_2() {
        let opcodes = vec![3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 38, 59,84,97,110,191,272,353,434,99999,3,9,1002,9,2,9,101,4,9,9,1002,9,2,9,4,9,99,3,9,102,5,9,9,1001,9,3,9,1002,9,5,9,101,5,9,9,4,9,99,3,9,102,5,9,9,101,5,9,9,1002,9,3,9,101,2,9,9,1002,9,4,9,4,9,99,3,9,101,3,9,9,1002,9,3,9,4,9,99,3,9,102,5,9,9,1001,9,3,9,4,9,99,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,99];
        let res = part2(opcodes);
        println!("max:{}",res);
        assert_eq!(res,63103596);
    }

}