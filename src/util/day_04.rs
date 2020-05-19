use crate::util::Part::*;
use crate::util::Part;

pub fn solve(input:String, part:Part) -> String {

    let lines : Vec<String> = input.split('-')
        .map(|line| line.trim().to_string())
        .collect();

    let from = (&lines[0]).parse().unwrap();
    let to = (&lines[1]).parse().unwrap();

    match part {
        Part1 => count_valid(from, to, is_valid).to_string(),
        Part2 => count_valid(from, to, is_valid2).to_string(),
    }
}

fn count_valid(from:u32, to:u32, is_valid: fn(u32) -> bool) -> u32 {
    let mut i = from;
    let mut sum = 0;

    while i <= to {
        //println!("{}",i);
        if is_valid(i) {
            sum += 1;
            //println!("{}", i);
        }

        i +=1;
    }

    sum
}

fn get_digits(num:u32) -> Vec<u32> {
    let num_str = num.to_string();

    let digits = num_str.chars()
        .map( |ch| ch.to_digit(10).unwrap())
        .collect();

    digits
}


fn is_valid(num:u32) -> bool {
    let mut i = 1;
    let digits = get_digits(num);
    let mut has_pair = false;
    let mut not_decreasing = true;
    while i < digits.len() {
        let prev = digits[i-1];
        let cur = digits[i];

        has_pair = has_pair || prev == cur;
        not_decreasing = not_decreasing && cur >= prev;

        //println!("{}",prev);

        i +=1;
    }

    has_pair && not_decreasing
}

fn is_valid2(num:u32) -> bool {
    let mut i = 1;
    let digits = get_digits(num);
    let mut has_pair = false;
    let mut not_decreasing = true;
    let mut streak = 1;
    while i < digits.len() {
        let prev = digits[i-1];
        let cur = digits[i];


        not_decreasing = not_decreasing && cur >= prev;



        // Did we have a pair?
        if prev != cur && streak == 2 {
            // Pair...
            has_pair = true;
            streak = 1;
        } else if prev == cur && streak == 1 && i == (digits.len() - 1) {
            // Last 2 digits are a pair
            has_pair = true;
        } else if prev == cur {
            streak += 1;
        } else if prev != cur {
            streak = 1;
        }

        //println!("{}",prev);

        i +=1;
    }

    has_pair && not_decreasing
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() {
        is_valid2(888999);
    }

    #[test]
    fn test2() {

        println!("{}",count_valid(357253,892942, is_valid));
        assert_eq!(530, count_valid(357253,892942, is_valid));

    }

    #[test]
    fn test3() {

        println!("{}",count_valid(357253,892942, is_valid2));
        assert_eq!(324, count_valid(357253,892942, is_valid2));
    }
}
