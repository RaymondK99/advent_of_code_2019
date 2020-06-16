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
    find_max(opcodes)
}

fn part2(opcodes:Vec<i64>) -> i64 {

    1
}


fn run_thrusters(phase_settings:Vec<i64>, opcodes:&Vec<i64>, input:Option<i64>) -> Option<i64> {
    let mut last_output = input.or(Some(0));
    let amps = vec!['A','B','C','D','E'];

    for i in 0..5 {
        println!("=> Running amp:{}, with setting:{}, and input:{:?}", amps[i], phase_settings[i], last_output);
        let mut program = Program::new(opcodes.clone(), Some(vec![phase_settings[i], last_output.unwrap()]));
        program.run();
        last_output = program.get_last_output();
    }

    last_output
}

fn find_max(opcodes:Vec<i64>) -> i64 {
    let mut max_value = 0;
    let initial_permutation = vec![0,1,2,3,4];

    for phase_setting in permute::permute(initial_permutation) {
        let value = run_thrusters(phase_setting, &opcodes, None);
        max_value = std::cmp::max( value.unwrap(), max_value);
    }

    max_value
}


fn run_thruster_w_feedback(opcodes:Vec<i64>,phase_setting:Vec<i64>) {
    let amps = vec!['A','B','C','D','E'];
    let mut next_input = Some(0);

    let mut programs = vec![];
    for _ in 0..5 {
        programs.push(Program::new(opcodes.clone(),None));
    }

    // Load phase setting
    while next_input.is_some() {
        for i in 0..5 {
            let program = &mut programs[i];
            let phase = phase_setting[i];

            println!(" => Run amp:{} with setting:{} and input:{}", amps[i], phase, next_input.unwrap());

            // Load phase
            program.add_input(phase);

            // Load input
            program.add_input(next_input.unwrap());

            // Run until halted or blocked
            while !program.is_halted() && !program.needs_input() {
                program.run_instruction();
            }

            if program.is_halted() {
                println!("HALTED!!");
            } else if program.needs_input() {
                println!("Needs input...");
            }

            // Get output
            next_input = program.get_last_output();

            if next_input.is_none() || (i == 4 && program.is_halted()) {
                break;
            }

            println!("Produced output:{}", next_input.unwrap());
        }
    }

}

fn find_max_w_feedback(opcodes:Vec<i64>) -> i64 {
    let max_value = 0;
    let initial_permutation = vec![5,6,7,8,9];
    let amps = vec!['A','B','C','D','E'];


    for phase_setting in permute::permute(initial_permutation) {
        let mut programs = vec![];
        let mut next_input = Some(0);

        for i in 0..5 {
            let input:Vec<i64> = vec![];
            programs.push(Program::new(opcodes.clone(),None));
        }

        // Load phase setting
        for i in 0..5 {
            let program = &mut programs[i];
            let phase = phase_setting[i];

            println!(" => Run amp:{} with setting:{} and input:{}",amps[i],phase,next_input.unwrap());

            // Load phase
            program.add_input(phase);

            // Load input
            program.add_input(next_input.unwrap());

            // Run until halted or blocked
            while !program.is_halted() && !program.needs_input() {
                program.run_instruction();
            }

            if program.is_halted() {
                println!("HALTED!!");
            } else if program.needs_input() {
                println!("Needs input...");
            }

            // Get output
            next_input = program.get_last_output();

            if next_input.is_none() {
                break;
            }

            println!("Produced output:{}", next_input.unwrap());
        }

        if next_input.is_none() {
            break;
        }

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
        let res = find_max(opcodes);
        println!("max:{}",res);
        assert_eq!(res,338603);
    }

    //#[test]
    fn test_part_2() {
        let opcodes = vec![3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 38, 59,84,97,110,191,272,353,434,99999,3,9,1002,9,2,9,101,4,9,9,1002,9,2,9,4,9,99,3,9,102,5,9,9,1001,9,3,9,1002,9,5,9,101,5,9,9,4,9,99,3,9,102,5,9,9,101,5,9,9,1002,9,3,9,101,2,9,9,1002,9,4,9,4,9,99,3,9,101,3,9,9,1002,9,3,9,4,9,99,3,9,102,5,9,9,1001,9,3,9,4,9,99,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,99];
        let res = find_max_w_feedback(opcodes.clone());
        println!("max:{}",res);
        assert_eq!(res,338603);
    }


    //#[test]
    fn test41() {
        let opcodes = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
                           27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];

        run_thruster_w_feedback(opcodes,vec![9,8,7,6,5]);
    }
}