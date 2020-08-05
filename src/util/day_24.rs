use crate::util::Part;
use std::collections::{HashSet, HashMap};

pub fn solve(input:String, part:Part) -> String {

    let result = match part {
        Part::Part1 => part1(input),
        Part::Part2 => part2(input,200)
    };

    format!("{}",result)
}


fn part1(input:String) -> usize {
    let mut state = parse(input.as_str());
    let mut set = HashSet::new();
    set.insert(state);

    loop {
        state = next_state(state);
        if  set.contains(&state) {
            return state as usize;
        } else {
            set.insert(state);
        }
    }

}

fn part2(input:String, iterations:i32) -> usize {
    let state = parse(input.as_str());
    let level_min = -iterations ;
    let level_max = iterations;
    let mut map:HashMap<i32,u32> = HashMap::new();

    // Init states
    let mut i:i32 = level_min;
    while i <= level_max {
        map.insert(i,0);
        i += 1;
    }

    // init state
    map.insert(0, state);

    // Mutate state
    for _ in 0..iterations {
        let mut next_state = map.clone();

        i = level_min+1;
        while i < level_max as i32 {

            let state = *map.get(&i).unwrap();
            let outer_state = *map.get(&(i-1)).unwrap();
            let inner_state = *map.get( &(i+1)).unwrap();

            let next = next_state_rec(inner_state, outer_state, state);

            // Update new map
            next_state.insert( i , next);

            i+=1;
        }

        // Update states
        map = next_state;
    }

    map.iter().map(|(_,v)| count_bits(*v)).sum()
}

fn count_bits(state:u32) -> usize {
    let mut res = 0;
    for n in 0..32 {
        if n != 12 && (1 << n & state) > 0 {
            res += 1;
        }
    }
    res
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

        let adjacent = [up,down,left,right].iter().filter(|item| **item > 0).count();
        let has_bug = (state & 1 << n) > 0;

        //println!("Pos:{}, up:{},down:{},left:{},right:{}", n, up>0,down>0,left>0,right>0);
        //println!("pos {} has {} adjacent bugs",n,sum);
        if adjacent != 1 && has_bug {
            // Has bug that should vanish
            next_state &= !(state & (1 << n));
            //println!("pos {} Bug should die, mask = {:#b}",n,!(state & (1 << n)));
        } else if !has_bug && (adjacent == 1 || adjacent == 2){
            // No bug, new bug should pop up
            next_state |= 1 << n;
            //println!("pos {} Should have NEW bug, next state={:#b}, mask={:#b}",n,next_state,state & (1 << n));
        } else if adjacent == 1 && has_bug {
            //println!("pos {} Should LIVE",n);
            // Bug should live
            next_state |= 1 << n;
        }
    }

    next_state
}

fn next_state_rec(inner:u32,outer:u32,state:u32) -> u32 {
    let mut next_state = 0;
    for n in 0..25 {
        if n == 12 {
            continue;
        }

        // Up ?
        let up = if n > 4 && n != 17 {
            vec![(state & 1 << (n-5)) > 0]
        } else if n < 5 {
            get_inner_up(outer)
        } else {
            get_outer_bottom(inner)
        };

        // Right
        let right = if (n+1) % 5 == 0 && n != 0 {
            get_inner_right(outer)
        } else if n == 11 {
            get_outer_left(inner)
        } else {
            vec![(state & 1 << (n+1)) > 0]
        };

        // Left
        let left = if n % 5 == 0 {
            get_inner_left(outer)
        } else if n == 13 {
            get_outer_right(inner)
        } else {
            vec![(state & 1 << (n-1))>0]
        };

        // Down
        let down = if n < 20 && n != 7{
            vec![ (state & 1 << (n+5)) > 0]
        } else if n == 7 {
            get_outer_top(inner)
        } else {
            get_inner_down(outer)
        };

        let adjacent = [up,down,left,right].iter().flatten().filter(|p| **p).count();
        let has_bug = (state & 1 << n) > 0;

        if adjacent != 1 && has_bug {
            // Has bug that should vanish
            next_state &= !(state & (1 << n));
            //println!("pos {} Bug should die, mask = {:#b}",n,!(state & (1 << n)));
        } else if !has_bug && (adjacent == 1 || adjacent == 2){
            // No bug, new bug should pop up
            next_state |= 1 << n;
            //println!("pos {} Should have NEW bug, next state={:#b}, mask={:#b}",n,next_state,state & (1 << n));
        } else if adjacent == 1 && has_bug {
            //println!("pos {} Should LIVE",n);
            // Bug should live
            next_state |= 1 << n;
        }
    }
    next_state
}

fn get_outer_left(state:u32) -> Vec<bool> {
    let n = [0,5,10,15,20];
    n.iter().map( |bit_no| (state & 1 << *bit_no) > 0).collect()
}

fn get_outer_right(state:u32) -> Vec<bool> {
    let n = [4,9,14,19,24];
    n.iter().map( |bit_no| (state & 1 << *bit_no) > 0).collect()
}

fn get_outer_top(state:u32) -> Vec<bool> {
    let n = [0,1,2,3,4];
    n.iter().map( |bit_no| (state & 1 << *bit_no) > 0).collect()
}

fn get_outer_bottom(state:u32) -> Vec<bool> {
    let n = [20,21,22,23,24];
    n.iter().map( |bit_no| (state & 1 << *bit_no) > 0).collect()
}

fn get_inner_left(state:u32) -> Vec<bool> {
    vec![(state & 1 << 11) > 0]
}

fn get_inner_up(state:u32) -> Vec<bool> {
    vec![(state & 1 << 7) > 0]
}

fn get_inner_down(state:u32) -> Vec<bool> {
    vec![(state & 1 << 17) > 0]
}

fn get_inner_right(state:u32) -> Vec<bool> {
    vec![(state & 1 << 13) > 0]
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

    #[test]
    fn test_part2_test1() {
        let input = "....#
#..#.
#..##
..#..
#....";

        let res = part2(input.to_string(),10);
        println!("{}",res);
        assert_eq!(res,99)
    }

    #[test]
    fn test_part2() {
        let input = "#.#..
.#.#.
#...#
.#..#
##.#.";

        let res = part2(input.to_string(),200);
        println!("{}",res);
        assert_eq!(res,1916)
    }
}
