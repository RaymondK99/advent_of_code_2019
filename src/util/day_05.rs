use crate::util::Part;

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

fn part1(opcodes:Vec<i32>) -> String {
    //
    let mut inputs = vec![1];
    let mut outputs = vec![];

    int_codes(&mut inputs, &mut outputs, opcodes);

    outputs.last().unwrap().to_string()
}

fn part2(opcodes:Vec<i32>) -> String {
    //
    //
    let mut inputs = vec![5];
    let mut outputs = vec![];

    int_codes(&mut inputs, &mut outputs, opcodes);

    outputs.last().unwrap().to_string()
}


fn is_pos_mode(instr:i32, par_no:u32) -> bool {
    !is_immediate_mode(instr, par_no)
}

fn is_immediate_mode(instruction:i32, par_no:u32) -> bool {
    let mut par_args = (instruction - instruction % 100) / 100;
    let mut i = 0;

    while i < par_no && par_args > 0 {
        if (i+1) == par_no && par_args % 2 == 1 {
            return true;
        }

        par_args /= 10;
        i += 1;
    }

    false
}

fn int_codes(inputs:&mut Vec<i32>, outputs:&mut Vec<i32>, mut opcodes : Vec<i32>) -> Vec<i32> {

    let mut i = 0;
    while i < opcodes.len() {
        let instruction = opcodes[i];
        let opcode = instruction % 100;

        println!("opcode = {}", opcode);

        if opcode == 1 || opcode == 2 {
            let value1_index = opcodes[i+1] as usize;
            let value2_index = opcodes[i+2] as usize;
            let res_index = opcodes[i+3] as usize;

            // Fetch value at position or the actual value
            let value1 = match is_pos_mode(instruction, 1) {
                true => opcodes[value1_index],
                false => value1_index as i32,
            };

            // Fetch valiue at position or the actual value
            let value2 = match is_pos_mode(instruction, 2 ) {
                true => opcodes[value2_index],
                false => value2_index as i32,
            };

            let res = &mut opcodes[res_index];

            //println!("vec[{}] = {} op({}) {}",res_index, value1, opcode, value2 );

            *res = match opcode {
                1 => value1 + value2,
                2 => value1 * value2,
                _ => panic!("Something went wrong...")
            };

            println!(" => OP {}: Arg: {} and Arg: {} and store result {} at {}", opcode, value1, value2, res, res_index);

            i+=4;

        } else if opcode == 3 {
            // input
            let value1_index = opcodes[i+1] as usize;
            let input_arg = inputs.remove(0);
            // Store value
            opcodes[value1_index] = input_arg;

            println!(" => OP 3: Read {} from input and store at {}", input_arg, value1_index);
            // Advance program counter
            i+=2;
        } else if opcode == 4 {
            // output
            let value1_index =  opcodes[i+1] as usize;
            let output_value = match is_pos_mode(instruction, 1) {
                true => opcodes[value1_index],
                false => value1_index as i32,
            };

            outputs.push( output_value);

            println!(" => OP 4: Push value {} to output.", output_value);
            // Advance program counter
            i+=2;
        } else if opcode == 5 || opcode == 6 {
            // Jump if true/false
            let param1 = match is_pos_mode(instruction, 1) {
                true => opcodes[opcodes[i+1] as usize],
                false => opcodes[i+1],
            };

            let param2 = match is_pos_mode(instruction, 2) {
                true => opcodes[opcodes[i+2] as usize],
                false => opcodes[i+2],
            };

            let jump = if opcode == 5 {
                if param1 > 0 {
                    println!(" => OP 5: {} > 0, set PC to {}", param1, param2);
                    true
                } else {
                    println!(" => OP 5: {} !> 0", param1);
                    false
                }
            } else if opcode == 6 {
                if param1 == 0 {
                    println!(" => OP 6: {} == 0, set PC to {}", param1, param2);
                    true
                } else {
                    println!(" => OP 6: {} != 0", param1);
                    false
                }
            } else { panic!("Uknown op..")};

            if jump {
                // Jump
                i = param2 as usize;
            } else {
                i += 3;
            }

        } else if opcode == 7 || opcode == 8 {
            // Jump if true/false
            let param1 = match is_pos_mode(instruction, 1) {
                true => opcodes[opcodes[i+1] as usize],
                false => opcodes[i+1],
            };

            let param2 = match is_pos_mode(instruction, 2) {
                true => opcodes[opcodes[i+2] as usize],
                false => opcodes[i+2],
            };

            let res_index= opcodes[i+3] as usize;
            let mut result = &mut opcodes[res_index];

            if opcode == 7 && param1 < param2 {
                println!(" => OP 7: {} < {}. Set 1 to index {}", param1, param2, res_index);
                *result = 1;
            } else if opcode == 7 {
                println!(" => OP 7: {} !< {}. Set 0 to index {}", param1, param2, res_index);
                *result = 0;
            }

            if opcode == 8 && param1 == param2 {
                println!(" => OP 8: {} == {}. Set 1 to index {}", param1, param2, res_index);
                *result = 1;
            } else if opcode == 8 {
                println!(" => OP 8: {} != {}. Set 0 to index {}", param1, param2, res_index);
                *result = 0;
            }

            i += 4;
        }

        else if opcode == 99  {
            break;
        } else {
            panic!("Something went wrong!!")

        }

    }

    opcodes

}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test1() {
        let opcodes = vec![2, 3, 0, 3, 99];
        let mut inputs = vec![];
        let mut outputs = vec![];

        println!("{:?}", int_codes(&mut inputs, &mut outputs, opcodes));
    }

    #[test]
    fn test11() {
        let opcodes = vec![3,0,4,0,99];
        let mut inputs = vec![123];
        let mut outputs = vec![];

        println!("{:?}", int_codes(&mut inputs, &mut outputs, opcodes));
        println!("outputs = {:?}", outputs);
    }


    #[test]
    fn test13() {
        println!("{:?}", is_pos_mode(11102,1));
        println!("{:?}", is_pos_mode(11102,2));
        println!("{:?}", is_pos_mode(11102,3));

        println!("{:?}", is_pos_mode(02,1));
        println!("{:?}", is_pos_mode(02,2));
        println!("{:?}", is_pos_mode(02,3));
    }

    #[test]
    fn test14() {
        println!("{:?}", is_pos_mode(01002,1));
        println!("{:?}", is_pos_mode(01002,2));
        println!("{:?}", is_pos_mode(01002,3));

        println!("{:?}", is_pos_mode(110002,1));
        println!("{:?}", is_pos_mode(1110002,2));
        println!("{:?}", is_pos_mode(1111110002,3));
    }

    #[test]
    fn test2() {
        let opcodes = vec![1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 9, 19, 1, 10, 19, 23, 2, 9, 23, 27, 1, 6, 27, 31, 2, 31, 9, 35, 1, 5, 35, 39, 1, 10, 39, 43, 1, 10, 43, 47, 2, 13, 47, 51, 1, 10, 51, 55, 2, 55, 10, 59, 1, 9, 59, 63, 2, 6, 63, 67, 1, 5, 67, 71, 1, 71, 5, 75, 1, 5, 75, 79, 2, 79, 13, 83, 1, 83, 5, 87, 2, 6, 87, 91, 1, 5, 91, 95, 1, 95, 9, 99, 1, 99, 6, 103, 1, 103, 13, 107, 1, 107, 5, 111, 2, 111, 13, 115, 1, 115, 6, 119, 1, 6, 119, 123, 2, 123, 13, 127, 1, 10, 127, 131, 1, 131, 2, 135, 1, 135, 5, 0, 99, 2, 14, 0, 0];

        let mut inputs = vec![];
        let mut outputs = vec![];

        println!("{:?}", int_codes(&mut inputs, &mut outputs, opcodes));
    }


    #[test]
    fn test5() {
        let opcodes = vec![3,225,1,225,6,6,1100,1,238,225,104,0,1101,72,36,225,1101,87,26,225,2,144,13,224,101,-1872,224,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1102,66,61,225,1102,25,49,224,101,-1225,224,224,4,224,1002,223,8,223,1001,224,5,224,1,223,224,223,1101,35,77,224,101,-112,224,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1002,195,30,224,1001,224,-2550,224,4,224,1002,223,8,223,1001,224,1,224,1,224,223,223,1102,30,44,225,1102,24,21,225,1,170,117,224,101,-46,224,224,4,224,1002,223,8,223,101,5,224,224,1,224,223,223,1102,63,26,225,102,74,114,224,1001,224,-3256,224,4,224,102,8,223,223,1001,224,3,224,1,224,223,223,1101,58,22,225,101,13,17,224,101,-100,224,224,4,224,1002,223,8,223,101,6,224,224,1,224,223,223,1101,85,18,225,1001,44,7,224,101,-68,224,224,4,224,102,8,223,223,1001,224,5,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,7,677,226,224,102,2,223,223,1005,224,329,101,1,223,223,8,677,226,224,1002,223,2,223,1005,224,344,1001,223,1,223,1107,677,677,224,102,2,223,223,1005,224,359,1001,223,1,223,1107,226,677,224,102,2,223,223,1005,224,374,101,1,223,223,7,226,677,224,102,2,223,223,1005,224,389,101,1,223,223,8,226,677,224,1002,223,2,223,1005,224,404,101,1,223,223,1008,226,677,224,1002,223,2,223,1005,224,419,1001,223,1,223,107,677,677,224,102,2,223,223,1005,224,434,101,1,223,223,1108,677,226,224,1002,223,2,223,1006,224,449,101,1,223,223,1108,677,677,224,102,2,223,223,1006,224,464,101,1,223,223,1007,677,226,224,102,2,223,223,1006,224,479,101,1,223,223,1008,226,226,224,102,2,223,223,1006,224,494,101,1,223,223,108,226,226,224,1002,223,2,223,1006,224,509,101,1,223,223,107,226,226,224,102,2,223,223,1006,224,524,101,1,223,223,1107,677,226,224,102,2,223,223,1005,224,539,1001,223,1,223,108,226,677,224,1002,223,2,223,1005,224,554,101,1,223,223,1007,226,226,224,102,2,223,223,1005,224,569,101,1,223,223,8,226,226,224,102,2,223,223,1006,224,584,101,1,223,223,1008,677,677,224,1002,223,2,223,1005,224,599,1001,223,1,223,107,226,677,224,1002,223,2,223,1005,224,614,1001,223,1,223,1108,226,677,224,102,2,223,223,1006,224,629,101,1,223,223,7,677,677,224,1002,223,2,223,1005,224,644,1001,223,1,223,108,677,677,224,102,2,223,223,1005,224,659,101,1,223,223,1007,677,677,224,102,2,223,223,1006,224,674,101,1,223,223,4,223,99,226];
        let mut inputs = vec![1];
        let mut outputs = vec![];

        println!("{:?}", int_codes(&mut inputs, &mut outputs, opcodes));
        println!("inputs = {:?}", inputs);
        println!("outputs = {:?}", outputs);

    }

    #[test]
    fn test6() {
        let opcodes= vec![3,9,8,9,10,9,4,9,99,-1,8];
        let mut inputs = vec![9];
        let mut outputs = vec![];

        println!("{:?}", int_codes(&mut inputs, &mut outputs, opcodes));
        println!("outputs = {:?}", outputs);
    }

    #[test]
    fn test61() {
        let opcodes= vec![3,9,7,9,10,9,4,9,99,-1,8];
        let mut inputs = vec![7];
        let mut outputs = vec![];

        println!("{:?}", int_codes(&mut inputs, &mut outputs, opcodes));
        println!("outputs = {:?}", outputs);
    }

    #[test]
    fn test62() {
        let opcodes= vec![3,3,1108,-1,8,3,4,3,99];
        let mut inputs = vec![8];
        let mut outputs = vec![];

        println!("{:?}", int_codes(&mut inputs, &mut outputs, opcodes));
        println!("outputs = {:?}", outputs);
    }

    #[test]
    fn test63() {
        let opcodes= vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                          1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                          999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];

        let mut inputs = vec![8];
        let mut outputs = vec![];

        int_codes(&mut inputs, &mut outputs, opcodes);
        assert_eq!(1000, *outputs.last().unwrap());

    }

    #[test]
    fn test64() {
        let opcodes= vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                          1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                          999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];

        let mut inputs = vec![7];
        let mut outputs = vec![];

        int_codes(&mut inputs, &mut outputs, opcodes);
        assert_eq!(999, *outputs.last().unwrap());


    }


    #[test]
    fn test65() {
        let opcodes= vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                          1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                          999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];

        let mut inputs = vec![10];
        let mut outputs = vec![];

        int_codes(&mut inputs, &mut outputs, opcodes);
        assert_eq!(1001, *outputs.last().unwrap());
    }

    #[test]
    fn test66() {
        let opcodes = vec![3,225,1,225,6,6,1100,1,238,225,104,0,1101,72,36,225,1101,87,26,225,2,144,13,224,101,-1872,224,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1102,66,61,225,1102,25,49,224,101,-1225,224,224,4,224,1002,223,8,223,1001,224,5,224,1,223,224,223,1101,35,77,224,101,-112,224,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1002,195,30,224,1001,224,-2550,224,4,224,1002,223,8,223,1001,224,1,224,1,224,223,223,1102,30,44,225,1102,24,21,225,1,170,117,224,101,-46,224,224,4,224,1002,223,8,223,101,5,224,224,1,224,223,223,1102,63,26,225,102,74,114,224,1001,224,-3256,224,4,224,102,8,223,223,1001,224,3,224,1,224,223,223,1101,58,22,225,101,13,17,224,101,-100,224,224,4,224,1002,223,8,223,101,6,224,224,1,224,223,223,1101,85,18,225,1001,44,7,224,101,-68,224,224,4,224,102,8,223,223,1001,224,5,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,7,677,226,224,102,2,223,223,1005,224,329,101,1,223,223,8,677,226,224,1002,223,2,223,1005,224,344,1001,223,1,223,1107,677,677,224,102,2,223,223,1005,224,359,1001,223,1,223,1107,226,677,224,102,2,223,223,1005,224,374,101,1,223,223,7,226,677,224,102,2,223,223,1005,224,389,101,1,223,223,8,226,677,224,1002,223,2,223,1005,224,404,101,1,223,223,1008,226,677,224,1002,223,2,223,1005,224,419,1001,223,1,223,107,677,677,224,102,2,223,223,1005,224,434,101,1,223,223,1108,677,226,224,1002,223,2,223,1006,224,449,101,1,223,223,1108,677,677,224,102,2,223,223,1006,224,464,101,1,223,223,1007,677,226,224,102,2,223,223,1006,224,479,101,1,223,223,1008,226,226,224,102,2,223,223,1006,224,494,101,1,223,223,108,226,226,224,1002,223,2,223,1006,224,509,101,1,223,223,107,226,226,224,102,2,223,223,1006,224,524,101,1,223,223,1107,677,226,224,102,2,223,223,1005,224,539,1001,223,1,223,108,226,677,224,1002,223,2,223,1005,224,554,101,1,223,223,1007,226,226,224,102,2,223,223,1005,224,569,101,1,223,223,8,226,226,224,102,2,223,223,1006,224,584,101,1,223,223,1008,677,677,224,1002,223,2,223,1005,224,599,1001,223,1,223,107,226,677,224,1002,223,2,223,1005,224,614,1001,223,1,223,1108,226,677,224,102,2,223,223,1006,224,629,101,1,223,223,7,677,677,224,1002,223,2,223,1005,224,644,1001,223,1,223,108,677,677,224,102,2,223,223,1005,224,659,101,1,223,223,1007,677,677,224,102,2,223,223,1006,224,674,101,1,223,223,4,223,99,226];
        let mut inputs = vec![5];
        let mut outputs = vec![];

        println!("{:?}", int_codes(&mut inputs, &mut outputs, opcodes));
        println!("inputs = {:?}", inputs);
        println!("outputs = {:?}", outputs);

    }
}