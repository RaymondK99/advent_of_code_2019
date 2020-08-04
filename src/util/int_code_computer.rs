use crate::util::int_code_computer::Operation::{Add, Mult, GetInput, PushOutput, Unknown, HaltProgram, JumpIfEqualToZero, JumpIfNotEqualToZero, SetIfEqual, SetIfLessThan, UpdateRelativeBase};
use crate::util::int_code_computer::ParameterMode::{ImmediateMode, RelativeMode, PositionMode};
use std::collections::HashMap;


pub struct Program {
    pc:usize,
    program_size:usize,
    memory:HashMap<usize,i64>,
    inputs:Vec<i64>,
    outputs:Vec<i64>,
    relative_base:i64,
}

#[derive(Debug)]
enum ParameterMode {
    PositionMode,
    ImmediateMode,
    RelativeMode,
}

impl ParameterMode {

    fn new(num:i64) -> ParameterMode {
        match num {
            1 => ImmediateMode,
            2 => RelativeMode,
            _ => PositionMode,
        }
    }
}

impl Program {
    pub fn new(opcodes:Vec<i64>, inputs:Option<Vec<i64>>) -> Program {
        let mut memory = HashMap::new();
        let mut i = 0;

        // populate instructions in memory
        for opcode in &opcodes {
            memory.insert(i as usize,*opcode);
            i+=1;
        }

        Program{pc:0,
            program_size:opcodes.len(),
            memory:memory,
            inputs: match inputs {
                Some(in_vec) => in_vec,
                None => vec![],
            },
            relative_base:0,
            outputs:vec![]}
    }

    fn get_param_address(&mut self, mask:i64, par_num:usize) -> i64 {
        let par_value = self.memory[&((self.pc + par_num) as usize)];
        match get_param_mode(mask, par_num) {
            RelativeMode => par_value + self.relative_base,
            _ => par_value,

        }
    }

    fn get_param_value(&mut self,mask:i64, par_num:usize) -> i64 {
        let par_value = self.memory[&((self.pc + par_num) as usize)];
        match get_param_mode(mask, par_num) {
            PositionMode =>  {
                self.get_memory(par_value as usize)
            },
            ImmediateMode => par_value,
            RelativeMode => {
                /*println!("    => Par no:{}, addr:{}, value={}",par_num, par_value + self.relative_base as i64,
                    self.get_memory((par_value + self.relative_base as i64) as usize));*/
                self.get_memory((par_value + self.relative_base as i64) as usize)
            }
        }
    }

    pub fn set_memory(&mut self,addr:usize, value:i64) {
        self.memory.insert(addr, value);
    }

    pub fn get_memory(&mut self,addr:usize) -> i64 {
        if !self.memory.contains_key(&addr) {
            self.memory.insert(addr, 0);
        }

        self.memory[&addr]
    }

    pub fn get_input_len(&self) -> usize {
        self.inputs.len()
    }

    pub fn get_input(&self) -> &Vec<i64> {
        self.inputs.as_ref()
    }

    pub fn run_until_output(&mut self, len:usize) -> Vec<i64> {
        let mut tmp = vec![];
        while tmp.len() < len {
            self.run_instruction();
            if self.outputs.len() > 0 {
                tmp.push(self.pop_output());
            }
        }

        tmp
    }

    fn next_op(&self) -> Operation {
        let program = &self.memory;
        let pc = &self.pc;
        let opcode = program[pc] % 100;
        let mask = (program[pc] - opcode) / 100;

        match opcode {
            1 => Add {param_mask:mask, arg1:program[&(pc+1)], arg2:program[&(pc+2)],pos_out:program[&(pc+3)]},
            2 => Mult {param_mask:mask, arg1:program[&(pc+1)], arg2:program[&(pc+2)],pos_out:program[&(pc+3)]},
            3 => GetInput {param_mask:mask,pos_out:program[&(pc+1)]},
            4 => PushOutput {param_mask:mask, pos_out:program[&(pc+1)]},
            5 => JumpIfNotEqualToZero {param_mask:mask,arg1:program[&(pc+1)],arg2:program[&(pc+2)]},
            6 => JumpIfEqualToZero {param_mask:mask,arg1:program[&(pc+1)],arg2:program[&(pc+2)]},
            7 => SetIfLessThan {param_mask:mask, arg1:program[&(pc+1)], arg2:program[&(pc+2)],pos_out:program[&(pc+3)]},
            8 => SetIfEqual {param_mask:mask, arg1:program[&(pc+1)], arg2:program[&(pc+2)],pos_out:program[&(pc+3)]},
            9 => UpdateRelativeBase{param_mask:mask, arg1:program[&(pc+1)]},
            99 => HaltProgram,
            _ => Unknown,
        }
    }

    fn exec_op(&mut self) -> bool {
        let mut cont_execute = true;
        let op = self.next_op();
        //println!("pc = {}, opcode={}, op = {:?}",self.pc, self.memory[&self.pc], op);
        match op {
            Add {param_mask, arg1,arg2,pos_out} => {
                let arg1_val = self.get_param_value(param_mask, 1);
                let arg2_val = self.get_param_value(param_mask, 2);
                let out_location = self.get_param_address(param_mask, 3);

                self.set_memory(out_location as usize,arg1_val + arg2_val);
                //println!(" => Set pos:{} to:{}",out_location, self.get_memory(out_location as usize));
                self.pc += 4;

            }
            Mult {param_mask, arg1,arg2,pos_out} => {
                let arg1_val = self.get_param_value(param_mask, 1);
                let arg2_val = self.get_param_value(param_mask, 2);
                let out_location = self.get_param_address(param_mask, 3);

                self.set_memory(out_location as usize,arg1_val * arg2_val);
                //println!(" => Set pos:{} to:{}",out_location, self.get_memory(out_location as usize));
                self.pc += 4;

            }
            GetInput {param_mask,pos_out} => {
                let out_location = self.get_param_address(param_mask, 1);

                let value = self.inputs.remove(0);
                self.set_memory(out_location as usize, value);
                self.pc += 2;
                //println!(" => Read to pos:{} as:{}",out_location, self.get_memory(out_location as usize));
            }

            JumpIfNotEqualToZero {param_mask,arg1,arg2} => {
                let arg1_val = self.get_param_value(param_mask, 1);
                let arg2_val = self.get_param_value(param_mask, 2);

                if arg1_val != 0 {
                    self.pc = arg2_val as usize;
                } else {
                    self.pc += 3;
                }

                //println!(" => Set PC to {}", self.pc);

            },
            JumpIfEqualToZero {param_mask,arg1,arg2} => {
                let arg1_val = self.get_param_value(param_mask, 1);
                let arg2_val = self.get_param_value(param_mask, 2);

                if arg1_val == 0 {
                    self.pc = arg2_val as usize;
                } else {
                    self.pc += 3;
                }
            },
            SetIfLessThan {param_mask,arg1,arg2, pos_out} => {
                let arg1_val = self.get_param_value(param_mask, 1);
                let arg2_val = self.get_param_value( param_mask, 2);
                let output_pos = self.get_param_address(param_mask, 3);

                if arg1_val < arg2_val {
                    self.set_memory(output_pos as usize, 1);
                } else {
                    self.set_memory(output_pos as usize, 0);
                }
                self.pc += 4;
            },
            SetIfEqual {param_mask,arg1,arg2, pos_out} => {
                let arg1_val = self.get_param_value( param_mask, 1);
                let arg2_val = self.get_param_value( param_mask, 2);
                let output_pos = self.get_param_address(param_mask, 3);
                if arg1_val == arg2_val {
                    self.set_memory(output_pos as usize, 1);
                } else {
                    self.set_memory(output_pos as usize, 0);
                }
                self.pc += 4;
            },

            PushOutput {param_mask, pos_out} => {
                let out_value = self.get_param_value(param_mask, 1);
                //println!(" => Push output as:{}", out_value);
                self.outputs.push(out_value);
                self.pc += 2;
            }

            UpdateRelativeBase {param_mask, arg1} => {
                let arg1_val = self.get_param_value(param_mask, 1);
                //println!(" => Update relative base from:{}, to:{}",self.relative_base, self.relative_base+arg1_val);
                self.relative_base += arg1_val;
                self.pc += 2;
            }
            HaltProgram => {
                cont_execute = false;
            }

            _ => {
                panic!("Uknown op...");
                // Do nothing
            }
        }

        cont_execute
    }

    pub fn run(&mut self) {
        let mut cont_program = true;
        while cont_program {
            cont_program = self.exec_op();
        }
    }

    pub fn needs_input(&self) -> bool {
        let next_op = self.next_op();
        match next_op {
            GetInput{param_mask,pos_out} => self.inputs.is_empty(),
            _ => false,
        }
    }

    pub fn is_blocked(&self) -> bool {
        self.inputs.is_empty() && self.needs_input()
    }

    pub fn add_input(&mut self, input:i64) {
        self.inputs.push(input);
    }

    pub fn is_halted(&self) -> bool {
        let next_op = self.next_op();
        match next_op {
            HaltProgram => true,
            _ => false,
        }
    }

    pub fn run_instruction(&mut self) {
        self.exec_op();
    }

    pub fn get_last_output(&self) -> Option<i64> {
        match self.outputs.is_empty() {
            true => None,
            false => Some(*self.outputs.last().unwrap()),
        }
    }

    pub fn get_output(&self) -> &Vec<i64> {
        self.outputs.as_ref()
    }

    pub fn pop_output(&mut self) -> i64 {
        self.outputs.pop().unwrap()
    }

    pub fn print_opcodes(&self) {
        let mut opcodes_str = String::new();
        for i in 0..self.program_size {
            if i > 0 {
                opcodes_str.push(',')
            }

            opcodes_str.push_str(&self.memory[&i].to_string());
        }
        println!("{}",opcodes_str);
    }
    pub fn print_outputs(&self) {
        println!("{:?}",self.outputs);
    }
}


#[derive(Debug)]
enum Operation {
    Add {param_mask:i64, arg1:i64,arg2:i64,pos_out:i64},
    Mult {param_mask:i64, arg1:i64,arg2:i64,pos_out:i64},
    GetInput {param_mask:i64, pos_out:i64},
    PushOutput {param_mask:i64,pos_out:i64},
    JumpIfNotEqualToZero {param_mask:i64,arg1:i64,arg2:i64},
    JumpIfEqualToZero {param_mask:i64,arg1:i64,arg2:i64},
    SetIfLessThan {param_mask:i64, arg1:i64,arg2:i64,pos_out:i64},
    SetIfEqual {param_mask:i64, arg1:i64,arg2:i64,pos_out:i64},
    UpdateRelativeBase{param_mask:i64, arg1:i64},
    HaltProgram,
    Unknown,
}




fn get_param_mode(mask:i64, num:usize) -> ParameterMode {
    let tmp = format!("{}",mask);

    if num > tmp.len() {
        return PositionMode;
    } else {
        let digit = tmp.as_bytes()[tmp.len() - num] - 0x30;
        ParameterMode::new(digit as i64)
    }
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

        assert_eq!(30,program.get_memory(3));
    }

    #[test]
    fn test_add2() {
        let opcodes = vec![1101, 1, 1,3,1102,2,3,7, 99];

        let mut program = Program::new(opcodes, None);
        program.run();
        program.print_opcodes();

        assert_eq!(2,program.get_memory(3));
        assert_eq!(6,program.get_memory(7));
    }

    #[test]
    fn test_mask1() {

        println!("{:?}", get_param_mode(111, 1));
        println!("{:?}", get_param_mode(111, 2));
        println!("{:?}", get_param_mode(111, 3));

        println!("{:?}", get_param_mode(0, 1));
        println!("{:?}", get_param_mode(0, 2));
        println!("{:?}", get_param_mode(0, 3));

        println!("{:?}", get_param_mode(101, 1));
        println!("{:?}", get_param_mode(101, 2));
        println!("{:?}", get_param_mode(101, 3));

        println!("{:?}", get_param_mode(10, 1));
        println!("{:?}", get_param_mode(10, 2));
        println!("{:?}", get_param_mode(10, 3));

        println!("{:?}", get_param_mode(1, 1));
        println!("{:?}", get_param_mode(1, 2));
        println!("{:?}", get_param_mode(1, 3));

        println!("{:?}", get_param_mode(202, 1));
        println!("{:?}", get_param_mode(202, 2));
        println!("{:?}", get_param_mode(202, 3));



    }

    #[test]
    fn test_mult1() {
        let opcodes = vec![2, 3, 3, 3, 99];

        let mut program = Program::new(opcodes, None);
        program.run();
        program.print_opcodes();

        assert_eq!(9,program.get_memory(3));
    }

    #[test]
    fn test_mult11() {
        let opcodes = vec![1102, 5, 6, 3, 99,4,5];

        let mut program = Program::new(opcodes, None);
        program.run();
        program.print_opcodes();

        assert_eq!(30,program.get_memory(3));
    }

    #[test]
    fn test_mult12() {
        let opcodes = vec![10002, 5, 6, 3, 99,4,5];

        let mut program = Program::new(opcodes, None);
        program.run();
        program.print_opcodes();

        assert_eq!(20,program.get_memory(3));
    }

    #[test]
    fn test_mult13() {
        let opcodes = vec![11002, 5, 6, 3, 99,4,5];

        let mut program = Program::new(opcodes, None);
        program.run();
        program.print_opcodes();

        assert_eq!(24,program.get_memory(3));
    }

    #[test]
    fn test_mult14() {
        let opcodes = vec![10102, 5, 6, 3, 99,4,5];

        let mut program = Program::new(opcodes, None);
        program.run();
        program.print_opcodes();

        assert_eq!(25,program.get_memory(3));
    }

    #[test]
    fn test_add_1() {
        let opcodes = vec![1, 5, 6, 3, 99,4,5];

        let mut program = Program::new(opcodes, None);
        program.run();
        program.print_opcodes();

        assert_eq!(9,program.get_memory(3));
    }

    #[test]
    fn test_add_2() {
        let opcodes = vec![11101, 1, 2, 3, 99,4,5];

        let mut program = Program::new(opcodes, None);
        program.run();
        program.print_opcodes();

        assert_eq!(3,program.get_memory(3));
    }

    #[test]
    fn test_rel_base_1() {
        let opcodes = vec![2201, 1, 2, 100, 99,4,5];

        let mut program = Program::new(opcodes, None);
        program.run();
        program.print_opcodes();

        assert_eq!(3,program.get_memory(100));
    }

    #[test]
    fn test_rel_base_2() {
        let opcodes = vec![109,6,2201, 1, 2, 100, 99,4,5];

        let mut program = Program::new(opcodes, None);
        program.run();
        program.print_opcodes();

        assert_eq!(9,program.get_memory(100));
    }

    #[test]
    fn test_rel_base_3() {
        let opcodes = vec![109,100,109,-92,2201, 1, 2, 100, 99,4,5];

        let mut program = Program::new(opcodes, None);
        program.run();
        program.print_opcodes();

        assert_eq!(9,program.get_memory(100));
    }

    #[test]
    fn test_rel_base_4() {
        let opcodes = vec![109,-100,2201, 107, 108, 100, 99,4,5];

        let mut program = Program::new(opcodes, None);
        program.run();
        program.print_opcodes();

        assert_eq!(9,program.get_memory(100));
    }

    #[test]
    fn test_rel_base_5() {
        let opcodes = vec![109,1000,1201, 0, 9, 1000, 99,4,5];

        let mut program = Program::new(opcodes, None);
        program.run();
        program.print_opcodes();

        assert_eq!(9,program.get_memory(1000));
    }


    #[test]
    fn test_read_input1() {
        let opcodes = vec![3,9,8,9,10,9,4,9,99,-1,8];
        let mut program = Program::new(opcodes.clone(), Option::Some(vec![0]));
        program.run();
        program.print_outputs();
        assert_eq!(0,program.get_last_output().unwrap());

        // Equal to 8
        let mut program2 = Program::new(opcodes.clone(), Option::Some(vec![8]));
        program2.run();
        program2.print_outputs();

        assert_eq!(1,program2.get_last_output().unwrap());
    }

    #[test]
    fn test_read_input2() {
        let opcodes = vec![3,3,1108,-1,8,3,4,3,99];
        let mut program = Program::new(opcodes.clone(), Option::Some(vec![0]));
        program.run();
        program.print_outputs();
        assert_eq!(0,program.get_last_output().unwrap());

        // Equal to 8
        let mut program2 = Program::new(opcodes.clone(), Option::Some(vec![8]));
        program2.run();
        program2.print_outputs();

        assert_eq!(1,program2.get_last_output().unwrap());
    }

    #[test]
    fn test_read_input3() {
        let opcodes = vec![3,9,7,9,10,9,4,9,99,-1,8];
        let mut program = Program::new(opcodes.clone(), Option::Some(vec![0]));
        program.run();
        program.print_outputs();
        assert_eq!(1,program.get_last_output().unwrap());

        // Less than 8
        let mut program2 = Program::new(opcodes.clone(), Option::Some(vec![8]));
        program2.run();
        program2.print_outputs();

        assert_eq!(0,program2.get_last_output().unwrap());
    }

    #[test]
    fn test_read_input4() {
        let opcodes = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        let mut program = Program::new(opcodes.clone(), Option::Some(vec![0]));
        program.run();
        assert_eq!(0,program.get_last_output().unwrap());

        let mut program2 = Program::new(opcodes.clone(), Option::Some(vec![99]));
        program2.run();
        assert_eq!(1,program2.get_last_output().unwrap());


        let mut program3 = Program::new(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], Option::Some(vec![0]));
        program3.run();
        program3.print_outputs();

        assert_eq!(0,program3.get_last_output().unwrap());
    }


    #[test]
    fn day_05_1() {
        let opcodes = vec![3,225,1,225,6,6,1100,1,238,225,104,0,1101,72,36,225,1101,87,26,225,2,144,13,224,101,-1872,224,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1102,66,61,225,1102,25,49,224,101,-1225,224,224,4,224,1002,223,8,223,1001,224,5,224,1,223,224,223,1101,35,77,224,101,-112,224,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1002,195,30,224,1001,224,-2550,224,4,224,1002,223,8,223,1001,224,1,224,1,224,223,223,1102,30,44,225,1102,24,21,225,1,170,117,224,101,-46,224,224,4,224,1002,223,8,223,101,5,224,224,1,224,223,223,1102,63,26,225,102,74,114,224,1001,224,-3256,224,4,224,102,8,223,223,1001,224,3,224,1,224,223,223,1101,58,22,225,101,13,17,224,101,-100,224,224,4,224,1002,223,8,223,101,6,224,224,1,224,223,223,1101,85,18,225,1001,44,7,224,101,-68,224,224,4,224,102,8,223,223,1001,224,5,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,7,677,226,224,102,2,223,223,1005,224,329,101,1,223,223,8,677,226,224,1002,223,2,223,1005,224,344,1001,223,1,223,1107,677,677,224,102,2,223,223,1005,224,359,1001,223,1,223,1107,226,677,224,102,2,223,223,1005,224,374,101,1,223,223,7,226,677,224,102,2,223,223,1005,224,389,101,1,223,223,8,226,677,224,1002,223,2,223,1005,224,404,101,1,223,223,1008,226,677,224,1002,223,2,223,1005,224,419,1001,223,1,223,107,677,677,224,102,2,223,223,1005,224,434,101,1,223,223,1108,677,226,224,1002,223,2,223,1006,224,449,101,1,223,223,1108,677,677,224,102,2,223,223,1006,224,464,101,1,223,223,1007,677,226,224,102,2,223,223,1006,224,479,101,1,223,223,1008,226,226,224,102,2,223,223,1006,224,494,101,1,223,223,108,226,226,224,1002,223,2,223,1006,224,509,101,1,223,223,107,226,226,224,102,2,223,223,1006,224,524,101,1,223,223,1107,677,226,224,102,2,223,223,1005,224,539,1001,223,1,223,108,226,677,224,1002,223,2,223,1005,224,554,101,1,223,223,1007,226,226,224,102,2,223,223,1005,224,569,101,1,223,223,8,226,226,224,102,2,223,223,1006,224,584,101,1,223,223,1008,677,677,224,1002,223,2,223,1005,224,599,1001,223,1,223,107,226,677,224,1002,223,2,223,1005,224,614,1001,223,1,223,1108,226,677,224,102,2,223,223,1006,224,629,101,1,223,223,7,677,677,224,1002,223,2,223,1005,224,644,1001,223,1,223,108,677,677,224,102,2,223,223,1005,224,659,101,1,223,223,1007,677,677,224,102,2,223,223,1006,224,674,101,1,223,223,4,223,99,226];
        let inputs = vec![1];

        let mut program = Program::new(opcodes.clone(), Option::Some(inputs));
        program.run();
        program.print_outputs();

        assert_eq!(program.get_last_output().unwrap(), 5577461);
    }

    #[test]
    fn day_05_2() {
        let opcodes = vec![3,225,1,225,6,6,1100,1,238,225,104,0,1101,72,36,225,1101,87,26,225,2,144,13,224,101,-1872,224,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1102,66,61,225,1102,25,49,224,101,-1225,224,224,4,224,1002,223,8,223,1001,224,5,224,1,223,224,223,1101,35,77,224,101,-112,224,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1002,195,30,224,1001,224,-2550,224,4,224,1002,223,8,223,1001,224,1,224,1,224,223,223,1102,30,44,225,1102,24,21,225,1,170,117,224,101,-46,224,224,4,224,1002,223,8,223,101,5,224,224,1,224,223,223,1102,63,26,225,102,74,114,224,1001,224,-3256,224,4,224,102,8,223,223,1001,224,3,224,1,224,223,223,1101,58,22,225,101,13,17,224,101,-100,224,224,4,224,1002,223,8,223,101,6,224,224,1,224,223,223,1101,85,18,225,1001,44,7,224,101,-68,224,224,4,224,102,8,223,223,1001,224,5,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,7,677,226,224,102,2,223,223,1005,224,329,101,1,223,223,8,677,226,224,1002,223,2,223,1005,224,344,1001,223,1,223,1107,677,677,224,102,2,223,223,1005,224,359,1001,223,1,223,1107,226,677,224,102,2,223,223,1005,224,374,101,1,223,223,7,226,677,224,102,2,223,223,1005,224,389,101,1,223,223,8,226,677,224,1002,223,2,223,1005,224,404,101,1,223,223,1008,226,677,224,1002,223,2,223,1005,224,419,1001,223,1,223,107,677,677,224,102,2,223,223,1005,224,434,101,1,223,223,1108,677,226,224,1002,223,2,223,1006,224,449,101,1,223,223,1108,677,677,224,102,2,223,223,1006,224,464,101,1,223,223,1007,677,226,224,102,2,223,223,1006,224,479,101,1,223,223,1008,226,226,224,102,2,223,223,1006,224,494,101,1,223,223,108,226,226,224,1002,223,2,223,1006,224,509,101,1,223,223,107,226,226,224,102,2,223,223,1006,224,524,101,1,223,223,1107,677,226,224,102,2,223,223,1005,224,539,1001,223,1,223,108,226,677,224,1002,223,2,223,1005,224,554,101,1,223,223,1007,226,226,224,102,2,223,223,1005,224,569,101,1,223,223,8,226,226,224,102,2,223,223,1006,224,584,101,1,223,223,1008,677,677,224,1002,223,2,223,1005,224,599,1001,223,1,223,107,226,677,224,1002,223,2,223,1005,224,614,1001,223,1,223,1108,226,677,224,102,2,223,223,1006,224,629,101,1,223,223,7,677,677,224,1002,223,2,223,1005,224,644,1001,223,1,223,108,677,677,224,102,2,223,223,1005,224,659,101,1,223,223,1007,677,677,224,102,2,223,223,1006,224,674,101,1,223,223,4,223,99,226];
        let inputs = vec![5];

        let mut program = Program::new(opcodes.clone(), Option::Some(inputs));
        program.run();
        program.print_outputs();

        assert_eq!(program.get_last_output().unwrap(), 7161591);
    }

    #[test]
    fn day_02_part1() {

        let opcodes = vec![2,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,9,19,1,10,19,23,2,9,23,27,1,6,27,31,2,31,9,35,1,5,35,39,1,10,39,43,1,10,43,47,2,13,47,51,1,10,51,55,2,55,10,59,1,9,59,63,2,6,63,67,1,5,67,71,1,71,5,75,1,5,75,79,2,79,13,83,1,83,5,87,2,6,87,91,1,5,91,95,1,95,9,99,1,99,6,103,1,103,13,107,1,107,5,111,2,111,13,115,1,115,6,119,1,6,119,123,2,123,13,127,1,10,127,131,1,131,2,135,1,135,5,0,99,2,14,0,0];
        let mut program = Program::new(opcodes.clone(), None);
        program.run();
        println!("pos 0:{}", program.get_memory(0));
        assert_eq!(3760627, program.get_memory(0));
    }


    #[test]
    fn day_09_test1() {
        let opcodes = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let mut program = Program::new(opcodes.clone(), None);
        program.run();

        program.print_outputs();
        assert_eq!(program.outputs,opcodes);
    }

    #[test]
    fn day_09_test2() {
        let opcodes = vec![1102,34915192,34915192,7,4,7,99,0];
        let mut program = Program::new(opcodes.clone(), None);
        program.run();

        program.print_outputs();
        assert_eq!(program.outputs,[1219070632396864]);
    }

    #[test]
    fn day_09_test3() {
        let opcodes = vec![104,1125899906842624,99];
        let mut program = Program::new(opcodes.clone(), None);
        program.run();

        program.print_outputs();
        assert_eq!(program.outputs,[1125899906842624]);
    }

    #[test]
    fn day_09_part1() {
        let opcodes = vec![1102,34463338,34463338,63,1007,63,34463338,63,1005,63,53,1101,3,0,1000,109,988,209,12,9,1000,209,6,209,3,203,0,1008,1000,1,63,1005,63,65,1008,1000,2,63,1005,63,904,1008,1000,0,63,1005,63,58,4,25,104,0,99,4,0,104,0,99,4,17,104,0,99,0,0,1101,37,0,1005,1101,30,0,1013,1102,1,33,1019,1102,1,25,1003,1102,1,28,1018,1101,26,0,1006,1102,1,866,1029,1101,760,0,1023,1102,39,1,1012,1102,23,1,1009,1101,281,0,1026,1102,1,20,1011,1102,1,34,1008,1101,0,36,1017,1101,38,0,1000,1102,0,1,1020,1102,278,1,1027,1101,21,0,1010,1102,875,1,1028,1101,0,212,1025,1102,1,1,1021,1102,1,24,1014,1102,763,1,1022,1101,0,31,1007,1102,1,221,1024,1101,0,32,1002,1102,1,29,1004,1102,1,35,1016,1102,22,1,1015,1101,0,27,1001,109,9,1207,-6,26,63,1005,63,199,4,187,1105,1,203,1001,64,1,64,1002,64,2,64,109,19,2105,1,-4,4,209,1001,64,1,64,1106,0,221,1002,64,2,64,109,-33,1207,5,37,63,1005,63,241,1001,64,1,64,1106,0,243,4,227,1002,64,2,64,109,16,2102,1,-2,63,1008,63,23,63,1005,63,269,4,249,1001,64,1,64,1106,0,269,1002,64,2,64,109,16,2106,0,0,1106,0,287,4,275,1001,64,1,64,1002,64,2,64,109,-11,21101,40,0,0,1008,1016,38,63,1005,63,311,1001,64,1,64,1105,1,313,4,293,1002,64,2,64,109,4,21107,41,40,-9,1005,1011,329,1105,1,335,4,319,1001,64,1,64,1002,64,2,64,109,-14,21108,42,42,5,1005,1011,353,4,341,1106,0,357,1001,64,1,64,1002,64,2,64,109,2,2107,33,0,63,1005,63,379,4,363,1001,64,1,64,1105,1,379,1002,64,2,64,109,-7,1201,2,0,63,1008,63,25,63,1005,63,401,4,385,1105,1,405,1001,64,1,64,1002,64,2,64,109,11,1201,-8,0,63,1008,63,28,63,1005,63,429,1001,64,1,64,1106,0,431,4,411,1002,64,2,64,109,-7,2108,26,1,63,1005,63,449,4,437,1105,1,453,1001,64,1,64,1002,64,2,64,109,9,1206,7,465,1105,1,471,4,459,1001,64,1,64,1002,64,2,64,109,4,21102,43,1,-3,1008,1015,42,63,1005,63,491,1106,0,497,4,477,1001,64,1,64,1002,64,2,64,109,7,21108,44,43,-7,1005,1018,517,1001,64,1,64,1105,1,519,4,503,1002,64,2,64,109,-28,2101,0,7,63,1008,63,29,63,1005,63,545,4,525,1001,64,1,64,1105,1,545,1002,64,2,64,109,11,2107,28,-7,63,1005,63,561,1105,1,567,4,551,1001,64,1,64,1002,64,2,64,109,-4,2101,0,-1,63,1008,63,26,63,1005,63,587,1105,1,593,4,573,1001,64,1,64,1002,64,2,64,109,9,1206,7,607,4,599,1105,1,611,1001,64,1,64,1002,64,2,64,109,-10,1208,1,27,63,1005,63,627,1106,0,633,4,617,1001,64,1,64,1002,64,2,64,109,26,1205,-9,649,1001,64,1,64,1106,0,651,4,639,1002,64,2,64,109,-20,1208,0,23,63,1005,63,669,4,657,1105,1,673,1001,64,1,64,1002,64,2,64,109,-7,2102,1,1,63,1008,63,28,63,1005,63,693,1105,1,699,4,679,1001,64,1,64,1002,64,2,64,109,18,21102,45,1,-6,1008,1014,45,63,1005,63,725,4,705,1001,64,1,64,1106,0,725,1002,64,2,64,109,-23,1202,6,1,63,1008,63,25,63,1005,63,751,4,731,1001,64,1,64,1106,0,751,1002,64,2,64,109,20,2105,1,6,1106,0,769,4,757,1001,64,1,64,1002,64,2,64,109,-22,2108,39,10,63,1005,63,789,1001,64,1,64,1106,0,791,4,775,1002,64,2,64,109,3,1202,6,1,63,1008,63,32,63,1005,63,815,1001,64,1,64,1105,1,817,4,797,1002,64,2,64,109,23,21107,46,47,-9,1005,1012,835,4,823,1106,0,839,1001,64,1,64,1002,64,2,64,109,1,1205,-1,853,4,845,1105,1,857,1001,64,1,64,1002,64,2,64,109,-2,2106,0,8,4,863,1001,64,1,64,1105,1,875,1002,64,2,64,109,-8,21101,47,0,-2,1008,1010,47,63,1005,63,897,4,881,1106,0,901,1001,64,1,64,4,64,99,21102,27,1,1,21101,0,915,0,1105,1,922,21201,1,27810,1,204,1,99,109,3,1207,-2,3,63,1005,63,964,21201,-2,-1,1,21102,1,942,0,1106,0,922,22101,0,1,-1,21201,-2,-3,1,21101,957,0,0,1106,0,922,22201,1,-1,-2,1106,0,968,22101,0,-2,-2,109,-3,2106,0,0];
        let mut program = Program::new(opcodes.clone(), Some([1].to_vec()));
        program.run();

        program.print_outputs();
        assert_eq!(program.outputs,[2775723069]);
    }


    #[test]
    fn day_09_part2() {
        let opcodes = vec![1102,34463338,34463338,63,1007,63,34463338,63,1005,63,53,1101,3,0,1000,109,988,209,12,9,1000,209,6,209,3,203,0,1008,1000,1,63,1005,63,65,1008,1000,2,63,1005,63,904,1008,1000,0,63,1005,63,58,4,25,104,0,99,4,0,104,0,99,4,17,104,0,99,0,0,1101,37,0,1005,1101,30,0,1013,1102,1,33,1019,1102,1,25,1003,1102,1,28,1018,1101,26,0,1006,1102,1,866,1029,1101,760,0,1023,1102,39,1,1012,1102,23,1,1009,1101,281,0,1026,1102,1,20,1011,1102,1,34,1008,1101,0,36,1017,1101,38,0,1000,1102,0,1,1020,1102,278,1,1027,1101,21,0,1010,1102,875,1,1028,1101,0,212,1025,1102,1,1,1021,1102,1,24,1014,1102,763,1,1022,1101,0,31,1007,1102,1,221,1024,1101,0,32,1002,1102,1,29,1004,1102,1,35,1016,1102,22,1,1015,1101,0,27,1001,109,9,1207,-6,26,63,1005,63,199,4,187,1105,1,203,1001,64,1,64,1002,64,2,64,109,19,2105,1,-4,4,209,1001,64,1,64,1106,0,221,1002,64,2,64,109,-33,1207,5,37,63,1005,63,241,1001,64,1,64,1106,0,243,4,227,1002,64,2,64,109,16,2102,1,-2,63,1008,63,23,63,1005,63,269,4,249,1001,64,1,64,1106,0,269,1002,64,2,64,109,16,2106,0,0,1106,0,287,4,275,1001,64,1,64,1002,64,2,64,109,-11,21101,40,0,0,1008,1016,38,63,1005,63,311,1001,64,1,64,1105,1,313,4,293,1002,64,2,64,109,4,21107,41,40,-9,1005,1011,329,1105,1,335,4,319,1001,64,1,64,1002,64,2,64,109,-14,21108,42,42,5,1005,1011,353,4,341,1106,0,357,1001,64,1,64,1002,64,2,64,109,2,2107,33,0,63,1005,63,379,4,363,1001,64,1,64,1105,1,379,1002,64,2,64,109,-7,1201,2,0,63,1008,63,25,63,1005,63,401,4,385,1105,1,405,1001,64,1,64,1002,64,2,64,109,11,1201,-8,0,63,1008,63,28,63,1005,63,429,1001,64,1,64,1106,0,431,4,411,1002,64,2,64,109,-7,2108,26,1,63,1005,63,449,4,437,1105,1,453,1001,64,1,64,1002,64,2,64,109,9,1206,7,465,1105,1,471,4,459,1001,64,1,64,1002,64,2,64,109,4,21102,43,1,-3,1008,1015,42,63,1005,63,491,1106,0,497,4,477,1001,64,1,64,1002,64,2,64,109,7,21108,44,43,-7,1005,1018,517,1001,64,1,64,1105,1,519,4,503,1002,64,2,64,109,-28,2101,0,7,63,1008,63,29,63,1005,63,545,4,525,1001,64,1,64,1105,1,545,1002,64,2,64,109,11,2107,28,-7,63,1005,63,561,1105,1,567,4,551,1001,64,1,64,1002,64,2,64,109,-4,2101,0,-1,63,1008,63,26,63,1005,63,587,1105,1,593,4,573,1001,64,1,64,1002,64,2,64,109,9,1206,7,607,4,599,1105,1,611,1001,64,1,64,1002,64,2,64,109,-10,1208,1,27,63,1005,63,627,1106,0,633,4,617,1001,64,1,64,1002,64,2,64,109,26,1205,-9,649,1001,64,1,64,1106,0,651,4,639,1002,64,2,64,109,-20,1208,0,23,63,1005,63,669,4,657,1105,1,673,1001,64,1,64,1002,64,2,64,109,-7,2102,1,1,63,1008,63,28,63,1005,63,693,1105,1,699,4,679,1001,64,1,64,1002,64,2,64,109,18,21102,45,1,-6,1008,1014,45,63,1005,63,725,4,705,1001,64,1,64,1106,0,725,1002,64,2,64,109,-23,1202,6,1,63,1008,63,25,63,1005,63,751,4,731,1001,64,1,64,1106,0,751,1002,64,2,64,109,20,2105,1,6,1106,0,769,4,757,1001,64,1,64,1002,64,2,64,109,-22,2108,39,10,63,1005,63,789,1001,64,1,64,1106,0,791,4,775,1002,64,2,64,109,3,1202,6,1,63,1008,63,32,63,1005,63,815,1001,64,1,64,1105,1,817,4,797,1002,64,2,64,109,23,21107,46,47,-9,1005,1012,835,4,823,1106,0,839,1001,64,1,64,1002,64,2,64,109,1,1205,-1,853,4,845,1105,1,857,1001,64,1,64,1002,64,2,64,109,-2,2106,0,8,4,863,1001,64,1,64,1105,1,875,1002,64,2,64,109,-8,21101,47,0,-2,1008,1010,47,63,1005,63,897,4,881,1106,0,901,1001,64,1,64,4,64,99,21102,27,1,1,21101,0,915,0,1105,1,922,21201,1,27810,1,204,1,99,109,3,1207,-2,3,63,1005,63,964,21201,-2,-1,1,21102,1,942,0,1106,0,922,22101,0,1,-1,21201,-2,-3,1,21101,957,0,0,1106,0,922,22201,1,-1,-2,1106,0,968,22101,0,-2,-2,109,-3,2106,0,0];
        let mut program = Program::new(opcodes.clone(), Some([2].to_vec()));
        program.run();

        program.print_outputs();
        assert_eq!(program.outputs,[49115]);
    }

    #[derive(Debug)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    #[test]
    fn day_11_test_1() {
        let opcodes = vec![3,8,1005,8,326,1106,0,11,0,0,0,104,1,104,0,3,8,1002,8,-1,10,101,1,10,10,4,10,108,0,8,10,4,10,101,0,8,28,2,1104,14,10,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,1,10,4,10,101,0,8,55,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,1,10,4,10,1001,8,0,77,2,103,7,10,3,8,102,-1,8,10,101,1,10,10,4,10,108,0,8,10,4,10,102,1,8,102,1006,0,76,1,6,5,10,1,1107,3,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,1,8,10,4,10,1001,8,0,135,1,1002,8,10,2,1101,3,10,1006,0,97,1,101,0,10,3,8,1002,8,-1,10,101,1,10,10,4,10,108,1,8,10,4,10,101,0,8,172,1006,0,77,1006,0,11,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,102,1,8,201,1006,0,95,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,1,10,4,10,1002,8,1,226,2,3,16,10,1,6,4,10,1006,0,23,1006,0,96,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,0,8,10,4,10,1001,8,0,261,1,3,6,10,2,1006,3,10,1006,0,78,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,101,0,8,295,1006,0,89,1,108,12,10,2,103,11,10,101,1,9,9,1007,9,1057,10,1005,10,15,99,109,648,104,0,104,1,21102,1,838365918100,1,21102,343,1,0,1106,0,447,21102,387365315476,1,1,21102,354,1,0,1106,0,447,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21101,0,179318254811,1,21102,401,1,0,1106,0,447,21102,1,97911876839,1,21101,0,412,0,1106,0,447,3,10,104,0,104,0,3,10,104,0,104,0,21101,838345577320,0,1,21101,435,0,0,1106,0,447,21102,1,838337188628,1,21101,0,446,0,1105,1,447,99,109,2,21202,-1,1,1,21101,40,0,2,21102,478,1,3,21101,0,468,0,1106,0,511,109,-2,2106,0,0,0,1,0,0,1,109,2,3,10,204,-1,1001,473,474,489,4,0,1001,473,1,473,108,4,473,10,1006,10,505,1102,1,0,473,109,-2,2106,0,0,0,109,4,2102,1,-1,510,1207,-3,0,10,1006,10,528,21101,0,0,-3,21202,-3,1,1,22101,0,-2,2,21101,1,0,3,21102,1,547,0,1106,0,552,109,-4,2106,0,0,109,5,1207,-3,1,10,1006,10,575,2207,-4,-2,10,1006,10,575,22102,1,-4,-4,1105,1,643,22102,1,-4,1,21201,-3,-1,2,21202,-2,2,3,21101,0,594,0,1105,1,552,21201,1,0,-4,21101,0,1,-1,2207,-4,-2,10,1006,10,613,21101,0,0,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,635,22102,1,-1,1,21101,635,0,0,106,0,510,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2106,0,0];
        let mut program = Program::new(opcodes.clone(), Some(vec![0]));

        let mut dir = Direction::Up;
        let mut pos = (0 , 0);
        let mut map =HashMap::new();

        while program.get_output().len() < 2 && !program.is_halted() {

            while program.get_output().len() < 2 && !program.is_halted() {
                program.run_instruction();
            }

            if program.is_halted() {
                break;
            }

            let turn = program.pop_output();
            let color = program.pop_output();

            println!("Pop turn={}, color={}",turn, color);

            if turn == 0 { // left
                dir = match &dir {
                    Direction::Up => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Down => Direction::Right,
                    Direction::Right => Direction::Up,
                }
            } else if turn == 1 { // right
                dir = match &dir {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                }
            }

            println!(" => New Dir={:?}",dir);

            // Paint
            map.insert( (pos.0, pos.1), color);

            // Move
            pos = match &dir {
                Direction::Up => (pos.0, pos.1 + 1),
                Direction::Down => (pos.0, pos.1 - 1),
                Direction::Left => (pos.0-1, pos.1),
                Direction::Right => (pos.0+1, pos.1),
            };

            println!("Paint color:{} and mov to {:?}",color, pos);

            program.add_input(color);
        }

        program.print_outputs();
        println!("Map size:{:?}", map);
    }


}
