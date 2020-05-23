
use super::Part;
use crate::util::int_code_computer::*;

pub fn solve(input : String, part: Part) -> String {

    let opcodes:Vec<i32> = input.split(',')
        .map(|op| op.trim().parse().unwrap())
        .collect();

    let result = match part {
        Part::Part1 => part1(opcodes),
        Part::Part2 => part2(opcodes)
    };

    format!("{}",result)
}


fn part1(mut opcodes:Vec<i32>) -> i32 {

    *(&mut opcodes[1]) = 12;
    *(&mut opcodes[2]) = 2;

    let mut program = Program::new(opcodes, None);
    program.run();
    program.get_first_opcode()
}

fn part2(opcodes:Vec<i32>) -> i32  {
    const RESULT:i32 = 19690720;

    let mut i = 0;
    let mut output = 0;

    while i < 100 {
        let mut j = 0;
        while j < 100 {
            let opcode_input = opcodes.clone();
            let result = run_int_codes(i,j,opcode_input);
            //println!("i={}, j={}, result={}", i,j,result);
            if result == RESULT {
                output = 100 * i + j;
                //println!("Found the answer");
                return output;
            }

            j +=1;
        }
        i+=1;
    };

    output
}

fn run_int_codes(pos1:i32, pos2:i32, mut opcodes : Vec<i32>) -> i32 {
    *(&mut opcodes[1]) = pos1;
    *(&mut opcodes[2]) = pos2;


    let mut prog = Program::new(opcodes, None);
    prog.run();
    prog.get_first_opcode()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;



    #[test]
    fn test2() {

        let input = vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,9,19,1,10,19,23,2,9,23,27,1,6,27,31,2,31,9,35,1,5,35,39,1,10,39,43,1,10,43,47,2,13,47,51,1,10,51,55,2,55,10,59,1,9,59,63,2,6,63,67,1,5,67,71,1,71,5,75,1,5,75,79,2,79,13,83,1,83,5,87,2,6,87,91,1,5,91,95,1,95,9,99,1,99,6,103,1,103,13,107,1,107,5,111,2,111,13,115,1,115,6,119,1,6,119,123,2,123,13,127,1,10,127,131,1,131,2,135,1,135,5,0,99,2,14,0,0];

        println!("{:?}", part1(input));
    }


    #[test]
    fn test5() {

        let opcodes = vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,9,19,1,10,19,23,2,9,23,27,1,6,27,31,2,31,9,35,1,5,35,39,1,10,39,43,1,10,43,47,2,13,47,51,1,10,51,55,2,55,10,59,1,9,59,63,2,6,63,67,1,5,67,71,1,71,5,75,1,5,75,79,2,79,13,83,1,83,5,87,2,6,87,91,1,5,91,95,1,95,9,99,1,99,6,103,1,103,13,107,1,107,5,111,2,111,13,115,1,115,6,119,1,6,119,123,2,123,13,127,1,10,127,131,1,131,2,135,1,135,5,0,99,2,14,0,0];

        for i in 0..99 {
            for j in 0..99 {
                let res = run_int_codes(i,j, opcodes.clone());
                if res == 19690720 {
                    break;
                }
            }
        }
        //println!("{:?}", part2(input));
    }
}
