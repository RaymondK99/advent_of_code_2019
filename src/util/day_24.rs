use crate::util::Part;
use std::collections::{HashSet};

pub fn solve(input:String, part:Part) -> String {

    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input)
    };

    format!("{}",result)
}


fn part1(input:String) -> u32 {
    let mut state = parse(input.as_str());
    let mut set = HashSet::new();
    set.insert(state);

    loop {
        state = next_state(state);
        if  set.contains(&state) {
            return state;
        } else {
            set.insert(state);
        }
    }

}

fn part2(input:String) -> u32 {
    2
}

fn parse(input:&str) -> u32 {
    let state:u32 = input.chars().filter(|ch| *ch == '.' || *ch == '#')
        .enumerate()
        .map(|(n, ch)| {
            match ch {
                '#' => 1 << n,
                _ => 0,
            }
        }).sum();

    //println!("State = {:#025b}",state);
    //println!("Value = {}", state);
    state
}

fn next_state(state:u32) -> u32 {
    let mut next_state:u32 = 0;
    for n in 0..25 {
        // Up
        let up = if n > 4 {
            state & 1 << (n-5)
        } else {
            0
        };

        let right = if (n+1) % 5 == 0 && n != 0 {
            0
        } else {
            state & 1 << (n+1)
        };

        let left = if n % 5 == 0 {
            0
        } else {
            state & 1 << (n-1)
        };

        let down = if n < 20 {
            state & 1 << (n+5)
        } else {
            0
        };

        let mut sum = 0;
        [up,down,left,right].iter().for_each(|item| {
            if *item > 0 {
                sum += 1;
            }
        });
        //println!("Pos:{}, up:{},down:{},left:{},right:{}", n, up>0,down>0,left>0,right>0);
        //println!("pos {} has {} adjacent bugs",n,sum);
        if sum != 1 && (state & 1 << n) > 0 {
            // Has bug that should vanish
            next_state &= !(state & (1 << n));
            //println!("pos {} Bug should die, mask = {:#b}",n,!(state & (1 << n)));
        } else if (state & 1 << n) == 0 && (sum == 1 || sum == 2){
            // No bug, new bug should pop up
            next_state |= 1 << n;
            //println!("pos {} Should have NEW bug, next state={:#b}, mask={:#b}",n,next_state,state & (1 << n));
        } else if sum == 1 && (state & 1 << n) > 0 {
            //println!("pos {} Should LIVE",n);
            // Bug should live
            next_state |= 1 << n;
        }
    }

    next_state
}

fn to_string(state:u32) -> String {
    let mut s = String::new();
    for n in 0..25 {
        if n % 5 == 0 && n > 0 {
            s.push('\n');
        }

        if state & 1 << n == 0 {
            s.push('.');
        } else {
            s.push('#');
        }
    }
    s
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() {
        let input = "....#
#..#.
#..##
..#..
#....";

        let res = parse(input);
        println!("res={}",res);
    }

    #[test]
    fn test2() {
        let input = ".....
.....
.....
#....
.#...";

        assert_eq!(2129920,parse(input));
    }

    #[test]
    fn test3() {
        let input = "....#
#..#.
#..##
..#..
#....";

        let state = parse(input);
        let state1 = next_state(state);
        let state2 = next_state(state1);
        let state3 = next_state(state2);
        let state4 = next_state(state3);

        let state1_str = to_string(state1);
        let state2_str = to_string(state2);
        let state3_str = to_string(state3);
        let state4_str = to_string(state4);

        assert_eq!(state1_str,"#..#.
####.
###.#
##.##
.##..");

        assert_eq!(state2_str,"#####
....#
....#
...#.
#.###");

        assert_eq!(state3_str,"#....
####.
...##
#.##.
.##.#");

        assert_eq!(state4_str,"####.
....#
##..#
.....
##...");

    }

    #[test]
    fn test4() {
        let input = "....#
#..#.
#..##
..#..
#....";

        let res = part1(input.to_string());
        println!("{}",res);
        assert_eq!(res,2129920)
    }

    #[test]
    fn test_part1() {
        let input = "#.#..
.#.#.
#...#
.#..#
##.#.";

        let res = part1(input.to_string());
        println!("{}",res);
        assert_eq!(res,25719471)
    }
}
