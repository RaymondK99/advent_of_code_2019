
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let opcodes:Vec<u32> = input.split(',')
        .map(|op| op.trim().parse().unwrap())
        .collect();

    let result = match part {
        Part::Part1 => part1(opcodes),
        Part::Part2 => part2(opcodes)
    };

    format!("{}",result)
}


fn part1(opcodes:Vec<u32>) -> u32 {
    run_int_codes(12, 2, opcodes)[0]
}

fn part2(opcodes:Vec<u32>) -> u32  {
    const RESULT:u32 = 19690720;

    let mut i = 0;
    let mut output = 0;

    while i < 100 {
        let mut j = 0;
        while j < 100 {
            let opcode_input = opcodes.clone();
            let result = run_int_codes(i,j,opcode_input);
            //println!("i={}, j={}, result={}", i,j,result[0]);
            if result[0] == RESULT {
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

fn run_int_codes(pos1:u32, pos2:u32, mut opcodes : Vec<u32>) -> Vec<u32> {
    *(&mut opcodes[1]) = pos1;
    *(&mut opcodes[2]) = pos2;


    int_codes(opcodes)
}



fn int_codes(mut opcodes : Vec<u32>) -> Vec<u32> {

    let mut i = 0;
    while i < opcodes.len() {
        let opcode = opcodes[i];

        //println!("opcode = {}", opcode);

        if opcode == 1 || opcode == 2 {
            let value1_index = opcodes[i+1] as usize;
            let value2_index = opcodes[i+2] as usize;
            let res_index = opcodes[i+3] as usize;

            let value1 = opcodes[value1_index];
            let value2 = opcodes[value2_index];
            let res = &mut opcodes[res_index];

            //println!("vec[{}] = {} op({}) {}",res_index, value1, opcode, value2 );


            *res = match opcode {
                1 => value1 + value2,
                2 => value1 * value2,
                _ => panic!("Something went wrong...")
            };

            i+=4;

        } else if opcode == 99  {
            break;
        } else {
            println!("Something went wrong!!")
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

        let input = vec![2,3,0,3,99];

        println!("{:?}", int_codes(input));
    }

    #[test]
    fn test2() {

        let input = vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,9,19,1,10,19,23,2,9,23,27,1,6,27,31,2,31,9,35,1,5,35,39,1,10,39,43,1,10,43,47,2,13,47,51,1,10,51,55,2,55,10,59,1,9,59,63,2,6,63,67,1,5,67,71,1,71,5,75,1,5,75,79,2,79,13,83,1,83,5,87,2,6,87,91,1,5,91,95,1,95,9,99,1,99,6,103,1,103,13,107,1,107,5,111,2,111,13,115,1,115,6,119,1,6,119,123,2,123,13,127,1,10,127,131,1,131,2,135,1,135,5,0,99,2,14,0,0];

        println!("{:?}", part1(input));
    }

    #[test]
    fn test3() {

        let input = vec![2,4,4,5,99,0];

        println!("{:?}", int_codes(input));
    }

    #[test]
    fn test4() {

        let input = vec![1,1,1,4,99,5,6,0,99];

        println!("{:?}", int_codes(input));
    }


    #[test]
    fn test5() {

        let input = vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,9,19,1,10,19,23,2,9,23,27,1,6,27,31,2,31,9,35,1,5,35,39,1,10,39,43,1,10,43,47,2,13,47,51,1,10,51,55,2,55,10,59,1,9,59,63,2,6,63,67,1,5,67,71,1,71,5,75,1,5,75,79,2,79,13,83,1,83,5,87,2,6,87,91,1,5,91,95,1,95,9,99,1,99,6,103,1,103,13,107,1,107,5,111,2,111,13,115,1,115,6,119,1,6,119,123,2,123,13,127,1,10,127,131,1,131,2,135,1,135,5,0,99,2,14,0,0];

        println!("{:?}", part2(input));
    }
}
