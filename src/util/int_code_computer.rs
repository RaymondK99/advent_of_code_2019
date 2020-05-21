use crate::util::int_code_computer::Operation::{ADD, MULT, GET_INPUT, PUSH_OUTPUT, UNKNOWN, HALT_PROGRAM, JMP_IF_EQUAL_TO_ZERO, JMP_IF_GREATER_THAN_ZERO, SET_IF_EQUAL, SET_IF_LESS_THAN};
use std::io::{self, Write};


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

pub struct Program {
    pc:usize,
    opcodes:Vec<i32>,
    inputs:Vec<i32>,
    outputs:Vec<i32>,
}


impl Program {
    pub fn new(opcodes:Vec<i32>, inputs:Option<Vec<i32>>) -> Program {

        Program{pc:0, opcodes:opcodes,
            inputs: match inputs {
                Some(in_vec) => in_vec,
                None => vec![],
            },
            outputs:vec![]}
    }



    pub fn run(&mut self) {
        let mut cont_program = true;
        while cont_program {
            cont_program = exec_op(&mut self.pc, &mut self.opcodes, &mut self.inputs, &mut self.outputs);
        }
    }

    pub fn get_last_output(&self) -> i32 {
        *self.outputs.last().unwrap()
    }

    pub fn get_first_opcode(&self) -> i32 {
        *self.opcodes.first().unwrap()
    }

    pub fn print_opcodes(&self) {
        println!("{:?}",self.opcodes);
    }
    pub fn print_outputs(&self) {
        println!("{:?}",self.outputs);
    }
}


#[derive(Debug)]
enum Operation {
    ADD{param_mask:i32, arg1:i32,arg2:i32,pos_out:i32},
    MULT{param_mask:i32, arg1:i32,arg2:i32,pos_out:i32},
    GET_INPUT{pos_out:i32},
    PUSH_OUTPUT{param_mask:i32,pos_out:i32},
    JMP_IF_GREATER_THAN_ZERO{param_mask:i32,arg1:i32,arg2:i32},
    JMP_IF_EQUAL_TO_ZERO{param_mask:i32,arg1:i32,arg2:i32},
    SET_IF_LESS_THAN{param_mask:i32, arg1:i32,arg2:i32,pos_out:i32},
    SET_IF_EQUAL{param_mask:i32, arg1:i32,arg2:i32,pos_out:i32},
    HALT_PROGRAM,
    UNKNOWN,
}

fn new_op(pc:usize, program:&Vec<i32>) -> Operation {
    let opcode = program[pc] % 100;
    let mask = (program[pc] - opcode) / 100;

    match opcode {
        1 => ADD{param_mask:mask, arg1:program[pc+1], arg2:program[pc+2],pos_out:program[pc+3]},
        2 => MULT{param_mask:mask, arg1:program[pc+1], arg2:program[pc+2],pos_out:program[pc+3]},
        3 => GET_INPUT{pos_out:program[pc+1]},
        4 => PUSH_OUTPUT{param_mask:mask, pos_out:program[pc+1]},
        5 => JMP_IF_GREATER_THAN_ZERO{param_mask:mask,arg1:program[pc+1],arg2:program[pc+2]},
        6 => JMP_IF_EQUAL_TO_ZERO{param_mask:mask,arg1:program[pc+1],arg2:program[pc+2]},
        7 => SET_IF_LESS_THAN{param_mask:mask, arg1:program[pc+1], arg2:program[pc+2],pos_out:program[pc+3]},
        8 => SET_IF_EQUAL{param_mask:mask, arg1:program[pc+1], arg2:program[pc+2],pos_out:program[pc+3]},
        99 => HALT_PROGRAM,
        _ => UNKNOWN,
    }
}

fn get_param_value(pc:&usize, mask:i32, par_num:usize, program:&Vec<i32>) -> i32 {
    let par_value = program[(pc + par_num) as usize];
    match is_pos_mode2(mask, par_num) {
        true => program[par_value as usize],
        false => par_value,
    }
}

fn is_pos_mode2(mask:i32, num:usize) -> bool {
    let is_set = match num {
        1 => mask % 2 == 1,
        2 => (mask / 10) % 2 == 1,
        3 => (mask / 100) % 2 == 1,
        _ => panic!("unknown par num"),
    };

    !is_set
}

fn exec_op(pc:&mut usize, program:&mut Vec<i32>, inpusts:&mut Vec<i32>, outputs:&mut Vec<i32>) -> bool {
    let mut cont_execute = true;
    let op = new_op(*pc, program);
    println!("{:?}",op);
    match op {
        ADD{param_mask, arg1,arg2,pos_out} => {
            let arg1_val = get_param_value(pc, param_mask, 1, program);
            let arg2_val = get_param_value(pc, param_mask, 2, program);
            *(&mut program[pos_out as usize]) = arg1_val + arg2_val;
            *pc += 4;

        }
        MULT{param_mask, arg1,arg2,pos_out} => {
            let arg1_val = get_param_value(pc, param_mask, 1, program);
            let arg2_val = get_param_value(pc, param_mask, 2, program);

            *(&mut program[pos_out as usize]) = arg1_val * arg2_val;
            *pc += 4;
        }
        GET_INPUT{pos_out} => {
            *(&mut program[pos_out as usize]) = inpusts.remove(0);
            *pc += 2;
        }
        JMP_IF_GREATER_THAN_ZERO{param_mask,arg1,arg2} => {
            let arg1_val = get_param_value(pc, param_mask, 1, program);
            let arg2_val = get_param_value(pc, param_mask, 2, program);

            if arg1_val > 0 {
                *pc = arg2_val as usize;
            } else {
                *pc += 3;
            }
        },
        JMP_IF_EQUAL_TO_ZERO{param_mask,arg1,arg2} => {
            let arg1_val = get_param_value(pc, param_mask, 1, program);
            let arg2_val = get_param_value(pc, param_mask, 2, program);

            if arg1_val == 0 {
                *pc = arg2_val as usize;
            } else {
                *pc += 3;
            }
        },
        SET_IF_LESS_THAN{param_mask,arg1,arg2, pos_out} => {
            let arg1_val = get_param_value(pc, param_mask, 1, program);
            let arg2_val = get_param_value(pc, param_mask, 2, program);
            let res = &mut program[pos_out as usize];
            if arg1_val < arg2_val {
                *res = 1;
            } else {
                *res = 0;
            }
            *pc += 4;
        },
        SET_IF_EQUAL{param_mask,arg1,arg2, pos_out} => {
            let arg1_val = get_param_value(pc, param_mask, 1, program);
            let arg2_val = get_param_value(pc, param_mask, 2, program);
            let res = &mut program[pos_out as usize];
            if arg1_val == arg2_val {
                *res = 1;
            } else {
                *res = 0;
            }
            *pc += 4;
        },

        PUSH_OUTPUT{param_mask, pos_out} => {
            let out_value = get_param_value(pc, param_mask, 1, program);
            outputs.push(out_value);
            *pc += 2;
        }
        HALT_PROGRAM => {
            cont_execute = false;
        }

        _ => {
            panic!("Uknown op...");
            // Do nothing
        }
    }

    cont_execute
}




#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test_add1() {
        let opcodes = vec![1, 5, 6, 3,99,10,20];

        let mut program = Program::new(opcodes, None);
        program.run();
        program.print_opcodes();

        assert_eq!(30,program.opcodes[3]);
    }

    #[test]
    fn test_add2() {
        let opcodes = vec![1101, 1, 1,3,1102,2,3,7, 99];

        let mut program = Program::new(opcodes, None);
        program.run();
        program.print_opcodes();

        assert_eq!(2,program.opcodes[3]);
        assert_eq!(6,program.opcodes[7]);
    }

    #[test]
    fn test_mask1() {

        println!("{:?}", is_pos_mode2(111,1));
        println!("{:?}", is_pos_mode2(111,2));
        println!("{:?}", is_pos_mode2(111,3));

        println!("{:?}", is_pos_mode2(0,1));
        println!("{:?}", is_pos_mode2(0,2));
        println!("{:?}", is_pos_mode2(0,3));

        println!("{:?}", is_pos_mode2(101,1));
        println!("{:?}", is_pos_mode2(101,2));
        println!("{:?}", is_pos_mode2(101,3));

        println!("{:?}", is_pos_mode2(10,1));
        println!("{:?}", is_pos_mode2(10,2));
        println!("{:?}", is_pos_mode2(10,3));

    }

    #[test]
    fn test_mult1() {
        let opcodes = vec![2, 3, 3, 3, 99];

        let mut program = Program::new(opcodes, None);
        program.run();
        program.print_opcodes();

        assert_eq!(9,program.opcodes[3]);
    }

    #[test]
    fn test_read_input1() {
        let opcodes = vec![3,9,8,9,10,9,4,9,99,-1,8];
        let mut inputs = vec![0];
        let mut program = Program::new(opcodes.clone(), Option::Some(vec![0]));
        program.run();
        program.print_outputs();
        assert_eq!(0,program.get_last_output());

        // Equal to 8
        let mut program2 = Program::new(opcodes.clone(), Option::Some(vec![8]));
        program2.run();
        program2.print_outputs();

        assert_eq!(1,program2.get_last_output());
    }

    #[test]
    fn test_read_input2() {
        let opcodes = vec![3,3,1108,-1,8,3,4,3,99];
        let mut inputs = vec![0];
        let mut program = Program::new(opcodes.clone(), Option::Some(vec![0]));
        program.run();
        program.print_outputs();
        assert_eq!(0,program.get_last_output());

        // Equal to 8
        let mut program2 = Program::new(opcodes.clone(), Option::Some(vec![8]));
        program2.run();
        program2.print_outputs();

        assert_eq!(1,program2.get_last_output());
    }

    #[test]
    fn test_read_input3() {
        let opcodes = vec![3,9,7,9,10,9,4,9,99,-1,8];
        let mut inputs = vec![0];
        let mut program = Program::new(opcodes.clone(), Option::Some(vec![0]));
        program.run();
        program.print_outputs();
        assert_eq!(1,program.get_last_output());

        // Less than 8
        let mut program2 = Program::new(opcodes.clone(), Option::Some(vec![8]));
        program2.run();
        program2.print_outputs();

        assert_eq!(0,program2.get_last_output());
    }

    #[test]
    fn test_read_input4() {
        let opcodes = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        let mut program = Program::new(opcodes.clone(), Option::Some(vec![0]));
        program.run();
        assert_eq!(0,program.get_last_output());

        let mut program2 = Program::new(opcodes.clone(), Option::Some(vec![99]));
        program2.run();
        assert_eq!(1,program2.get_last_output());


        let mut program3 = Program::new(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], Option::Some(vec![0]));
        program3.run();
        program3.print_outputs();

        assert_eq!(0,program3.get_last_output());
    }


    #[test]
    fn day_05_1() {
        let opcodes = vec![3,225,1,225,6,6,1100,1,238,225,104,0,1101,72,36,225,1101,87,26,225,2,144,13,224,101,-1872,224,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1102,66,61,225,1102,25,49,224,101,-1225,224,224,4,224,1002,223,8,223,1001,224,5,224,1,223,224,223,1101,35,77,224,101,-112,224,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1002,195,30,224,1001,224,-2550,224,4,224,1002,223,8,223,1001,224,1,224,1,224,223,223,1102,30,44,225,1102,24,21,225,1,170,117,224,101,-46,224,224,4,224,1002,223,8,223,101,5,224,224,1,224,223,223,1102,63,26,225,102,74,114,224,1001,224,-3256,224,4,224,102,8,223,223,1001,224,3,224,1,224,223,223,1101,58,22,225,101,13,17,224,101,-100,224,224,4,224,1002,223,8,223,101,6,224,224,1,224,223,223,1101,85,18,225,1001,44,7,224,101,-68,224,224,4,224,102,8,223,223,1001,224,5,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,7,677,226,224,102,2,223,223,1005,224,329,101,1,223,223,8,677,226,224,1002,223,2,223,1005,224,344,1001,223,1,223,1107,677,677,224,102,2,223,223,1005,224,359,1001,223,1,223,1107,226,677,224,102,2,223,223,1005,224,374,101,1,223,223,7,226,677,224,102,2,223,223,1005,224,389,101,1,223,223,8,226,677,224,1002,223,2,223,1005,224,404,101,1,223,223,1008,226,677,224,1002,223,2,223,1005,224,419,1001,223,1,223,107,677,677,224,102,2,223,223,1005,224,434,101,1,223,223,1108,677,226,224,1002,223,2,223,1006,224,449,101,1,223,223,1108,677,677,224,102,2,223,223,1006,224,464,101,1,223,223,1007,677,226,224,102,2,223,223,1006,224,479,101,1,223,223,1008,226,226,224,102,2,223,223,1006,224,494,101,1,223,223,108,226,226,224,1002,223,2,223,1006,224,509,101,1,223,223,107,226,226,224,102,2,223,223,1006,224,524,101,1,223,223,1107,677,226,224,102,2,223,223,1005,224,539,1001,223,1,223,108,226,677,224,1002,223,2,223,1005,224,554,101,1,223,223,1007,226,226,224,102,2,223,223,1005,224,569,101,1,223,223,8,226,226,224,102,2,223,223,1006,224,584,101,1,223,223,1008,677,677,224,1002,223,2,223,1005,224,599,1001,223,1,223,107,226,677,224,1002,223,2,223,1005,224,614,1001,223,1,223,1108,226,677,224,102,2,223,223,1006,224,629,101,1,223,223,7,677,677,224,1002,223,2,223,1005,224,644,1001,223,1,223,108,677,677,224,102,2,223,223,1005,224,659,101,1,223,223,1007,677,677,224,102,2,223,223,1006,224,674,101,1,223,223,4,223,99,226];
        let mut inputs = vec![1];

        let mut program = Program::new(opcodes.clone(), Option::Some(inputs));
        program.run();
        program.print_outputs();

        assert_eq!(program.get_last_output(), 5577461);
    }

    #[test]
    fn day_05_2() {
        let opcodes = vec![3,225,1,225,6,6,1100,1,238,225,104,0,1101,72,36,225,1101,87,26,225,2,144,13,224,101,-1872,224,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1102,66,61,225,1102,25,49,224,101,-1225,224,224,4,224,1002,223,8,223,1001,224,5,224,1,223,224,223,1101,35,77,224,101,-112,224,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1002,195,30,224,1001,224,-2550,224,4,224,1002,223,8,223,1001,224,1,224,1,224,223,223,1102,30,44,225,1102,24,21,225,1,170,117,224,101,-46,224,224,4,224,1002,223,8,223,101,5,224,224,1,224,223,223,1102,63,26,225,102,74,114,224,1001,224,-3256,224,4,224,102,8,223,223,1001,224,3,224,1,224,223,223,1101,58,22,225,101,13,17,224,101,-100,224,224,4,224,1002,223,8,223,101,6,224,224,1,224,223,223,1101,85,18,225,1001,44,7,224,101,-68,224,224,4,224,102,8,223,223,1001,224,5,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,7,677,226,224,102,2,223,223,1005,224,329,101,1,223,223,8,677,226,224,1002,223,2,223,1005,224,344,1001,223,1,223,1107,677,677,224,102,2,223,223,1005,224,359,1001,223,1,223,1107,226,677,224,102,2,223,223,1005,224,374,101,1,223,223,7,226,677,224,102,2,223,223,1005,224,389,101,1,223,223,8,226,677,224,1002,223,2,223,1005,224,404,101,1,223,223,1008,226,677,224,1002,223,2,223,1005,224,419,1001,223,1,223,107,677,677,224,102,2,223,223,1005,224,434,101,1,223,223,1108,677,226,224,1002,223,2,223,1006,224,449,101,1,223,223,1108,677,677,224,102,2,223,223,1006,224,464,101,1,223,223,1007,677,226,224,102,2,223,223,1006,224,479,101,1,223,223,1008,226,226,224,102,2,223,223,1006,224,494,101,1,223,223,108,226,226,224,1002,223,2,223,1006,224,509,101,1,223,223,107,226,226,224,102,2,223,223,1006,224,524,101,1,223,223,1107,677,226,224,102,2,223,223,1005,224,539,1001,223,1,223,108,226,677,224,1002,223,2,223,1005,224,554,101,1,223,223,1007,226,226,224,102,2,223,223,1005,224,569,101,1,223,223,8,226,226,224,102,2,223,223,1006,224,584,101,1,223,223,1008,677,677,224,1002,223,2,223,1005,224,599,1001,223,1,223,107,226,677,224,1002,223,2,223,1005,224,614,1001,223,1,223,1108,226,677,224,102,2,223,223,1006,224,629,101,1,223,223,7,677,677,224,1002,223,2,223,1005,224,644,1001,223,1,223,108,677,677,224,102,2,223,223,1005,224,659,101,1,223,223,1007,677,677,224,102,2,223,223,1006,224,674,101,1,223,223,4,223,99,226];
        let mut inputs = vec![5];

        let mut program = Program::new(opcodes.clone(), Option::Some(inputs));
        program.run();
        program.print_outputs();

        assert_eq!(program.get_last_output(), 7161591);
    }

    #[test]
    fn day_02_part1() {

        let opcodes = vec![2,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,9,19,1,10,19,23,2,9,23,27,1,6,27,31,2,31,9,35,1,5,35,39,1,10,39,43,1,10,43,47,2,13,47,51,1,10,51,55,2,55,10,59,1,9,59,63,2,6,63,67,1,5,67,71,1,71,5,75,1,5,75,79,2,79,13,83,1,83,5,87,2,6,87,91,1,5,91,95,1,95,9,99,1,99,6,103,1,103,13,107,1,107,5,111,2,111,13,115,1,115,6,119,1,6,119,123,2,123,13,127,1,10,127,131,1,131,2,135,1,135,5,0,99,2,14,0,0];
        let mut program = Program::new(opcodes.clone(), None);
        program.run();
        println!("pos 0:{}", program.opcodes[0]);
        assert_eq!(3760627, program.opcodes[0]);
    }

}
