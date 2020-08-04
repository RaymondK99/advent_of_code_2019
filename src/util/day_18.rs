use crate::util::Part;
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::fmt::Debug;
use std::cmp::{Reverse};

pub fn solve(input:String, part:Part) -> String {
    let result = match part {
        Part::Part1 => part1(input.as_str()),
        Part::Part2 => part2(input.as_str())
    };

    format!("{}",result)
}

#[derive(Eq,Hash,PartialEq,Debug,Copy, Clone,Ord, PartialOrd)]
struct Pos {
    x:u32,
    y:u32,
}

#[derive(Eq,Hash,PartialEq,Debug,Ord, PartialOrd)]
struct Node {
    ch:char,
    pos:Pos,
    keys:Vec<char>,
}

impl Node {
    fn make_copy(&self) -> Node {
        Node{ch:self.ch, pos:self.pos, keys:self.keys.clone()}
    }
}


#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct QueueElement {
    // This struct will by default be sorted by shortest distance and collected keys
    distance:i32,
    missing_keys:usize,
}


fn parse_input(input:&str) -> HashMap<Pos,char> {
    let num_lines = input.lines().count() as u32;
    let keys: Vec<char> = input.chars().filter(|ch| ch.is_ascii_lowercase()).collect();
    let mut key_set = HashSet::new();
    for x in keys.iter() {
        key_set.insert(*x);
    }

    let mut map: HashMap<Pos, char> = HashMap::new();
    input.lines().enumerate().for_each(|(line_no, line)| {
        line.chars().enumerate().for_each(|(x_pos, ch)| {
            map.insert(Pos { x: x_pos as u32, y: num_lines - line_no as u32 - 1 }, ch);
        })
    });

    map
}


fn dijkstras(map:HashMap<Pos,char>) -> i32 {
    // Find start pos
    let start_pos = *map.iter().find(|&(_,v)| *v == '@').unwrap().0;
    let num_keys = map.iter().filter(|&(_,v)| v.is_ascii_lowercase()).count();

    // Add first state
    let start_node = Node{ch:'@',pos:start_pos, keys:Vec::new()};

    let mut queue = BinaryHeap::new();
    let mut distances = HashMap::new();

    distances.insert(start_node.make_copy(), 0);
    queue.push(Reverse((QueueElement{missing_keys:num_keys,distance:0},start_node)));

    while !queue.is_empty() {

        // Pop first item
        let (elem,node)= queue.pop().unwrap().0;
        let (x,y) = (node.pos.x, node.pos.y);
        let dist = elem.distance;

        // Check if we reach exit criteria
        if node.keys.len() == num_keys {
            return dist;
        }

        // Generate adjacent nodes
        let adjacent_pos = vec![Pos{x:x+1,y:y},Pos{x:x-1,y:y},Pos{x:x,y:y-1},Pos{x:x,y:y+1}];
        let mut next_nodes = vec![];
        let next_dist = dist + 1;
        for neighbor_pos in adjacent_pos {
            let item = *map.get(&neighbor_pos).unwrap();
            if item == '.' || item == '@' || (item.is_ascii_uppercase() && node.keys.contains(&item.to_ascii_lowercase())) {
                // Open
                next_nodes.push(Node{ch:item, pos: neighbor_pos, keys:node.keys.clone()});
            } else if item == '#' {
                // Wall
                continue;
            } else if item.is_ascii_lowercase() {
                // Key
                let mut next_keys = node.keys.clone();
                if !next_keys.contains(&item) {
                    next_keys.push(item);
                    next_keys.sort();
                }
                next_nodes.push( Node{ch:item,pos: neighbor_pos,keys:next_keys});
            }  else if item.is_ascii_uppercase() {
                // continue, locked door
                continue;
            }
            else {
                panic!("Item {}",item);
            }
        }

        // Push non visited or nodes with lower distance to queue.
        for next_node in next_nodes {
            let prev_dist = distances.get(&next_node);
            if prev_dist.is_none() || (*prev_dist.unwrap() > next_dist )   {
                // Evaluate node
                queue.push(Reverse((QueueElement{distance:next_dist, missing_keys:num_keys - next_node.keys.len()},next_node.make_copy())));
                distances.insert(next_node, next_dist);
            }
        }
    }

    panic!("No solution!")
}

fn part1(input:&str) -> i32 {
    let map = parse_input(input);
    dijkstras(map)
}

fn part2(_input:&str) -> i32 {
    2
}




#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() {
        let input =
            "#########
#b.A.@.a#
#########";

        let m = part1(input);
        assert_eq!(m,8);
        println!("{:?}", m);

    }

    #[test]
    fn test2() {
        let input =
            "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";

        let m = part1(input);
        assert_eq!(m, 86);
        println!("{:?}", m);

    }

    #[test]
    fn test3() {
        let input =
            "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";

        let m = part1(input);
        assert_eq!(m,136);
        println!("{:?}", m);

    }

}