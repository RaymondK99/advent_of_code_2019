use crate::util::Part;

pub fn solve(input:String, part:Part) -> String {

    let result = match part {
        Part::Part1 => part1(input.trim().to_string(), 25, 6),
        Part::Part2 => part2(input.trim().to_string(), 25 , 6)
    };

    format!("{}",result)
}

fn part1(input:String, pic_width:usize, pic_height:usize) -> String {
    let layers:Vec<[u32;10]>  = input.as_bytes()
        .chunks(pic_width*pic_height).map(
        |layer_str| {
            let mut count_digits = [0;10];
            for ch in layer_str {
                let digit = (ch - 0x30) as usize;
                count_digits[digit] += 1;
            }
            count_digits
        }
    ).collect();

    // Find smallest element
    let min_zero_layer = layers.iter()
        .min_by_key(|&item| item[0])
        .unwrap();

    format!("{}",min_zero_layer[1] * min_zero_layer[2])
}

fn part2(input:String, pic_width:usize, pic_height:usize) -> String {
    let layers:Vec<&[u8]> = input.as_bytes()
        .chunks(pic_height*pic_width)
        .collect();


    let mut decoded_picture = String::new();
    let pic_size = pic_height*pic_width;
    for pix_no in 0..pic_size {
        if pix_no > 0 && pix_no % pic_width == 0 {
            //decoded_picture.push('\n');
        }

        let mut final_color = '2';

        // For each layer
        for &layer in layers.iter() {
            let color = layer[pix_no] as char;
            if color == '0' || color == '1' {
                final_color = color;
                break;
            }
        }

        decoded_picture.push( final_color);
    }
    decoded_picture
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::util::Part::*;

    #[test]
    fn test1() {
        let indata = include_str!("../../input_08.txt");
        let res = solve(indata.trim().to_string(), Part1);
        println!("res = {}", res);
        assert_eq!(res, "2904");

    }

    #[test]
    fn test2() {
        let indata = "0222112222120000";
        let res = part2(indata.to_string(), 2,2);
        println!("res = {}", res);
        assert_eq!(res, "0110");

    }

    #[test]
    fn test21() {
        let indata = "2222212222122000";
        let res = part2(indata.to_string(), 2,2);
        println!("res = {}", res);
        assert_eq!(res, "2110");

    }

    #[test]
    fn test23() {
        let indata = "222222222000111222000111000";
        let res = part2(indata.to_string(), 3,3);
        println!("res = {}", res);
        //assert_eq!(res, "21\n10");

    }

    #[test]
    fn test3() {
        let indata = include_str!("../../input_08.txt");
        let res = solve(indata.trim().to_string(), Part2);
        println!("------------");
        println!("{}", res);
        //assert_eq!(res, "2904");

    }


}
