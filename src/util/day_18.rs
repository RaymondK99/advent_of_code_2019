use crate::util::Part;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

pub fn solve(input:String, part:Part) -> String {
    let result = match part {
        Part::Part1 => part1(input.as_str()),
        Part::Part2 => part2(input.as_str())
    };

    format!("{}",result)
}

fn part1(input:&str) -> u32 {
    let mut m = parse_input(input);
    let mut best = 999999999;
    let mut map = HashMap::new();
    m.solve(&mut best, &mut map).unwrap()
}

fn part2(input:&str) -> u32 {
    2
}

#[derive(Eq,Hash,PartialEq,Debug,Copy, Clone)]
struct Pos {
    x:u32,
    y:u32,
}

impl Pos {
    fn new(x:u32,y:u32) -> Pos {
        Pos{x:x,y:y}
    }

    fn next(&self, dir:Direction) -> Pos {
        let new_pos = match dir {
            Direction::Up => Pos{x:self.x, y:self.y+1},
            Direction::Down=> Pos{x:self.x, y:self.y-1},
            Direction::Left=> Pos{x:self.x-1, y:self.y},
            Direction::Right => Pos{x:self.x+1, y:self.y},
        };

        new_pos
    }
}

#[derive(Debug,Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug,Clone, Hash, Eq, PartialEq)]
struct NodeKey {
    keys:Vec<char>,
    pos:Pos,
}

#[derive(Debug, Clone)]
struct Context {
    map:HashMap<Pos,char>,
    keys:HashSet<char>,
    keys_left:HashSet<char>,
    pos:Pos,
    height:u32,
    width:u32,
    steps:Vec<(char,u32)>,
    num_keys:u32,
    node:NodeKey,
}

impl Context {

    fn next_context(&self, new_pos:Pos, new_key:char, dist:u32) -> Context {
        let mut keys = self.keys.clone();
        keys.insert(new_key);
        let mut steps = self.steps.clone();
        steps.push( (new_key, dist));
        let mut key_set = self.keys_left.clone();
        key_set.remove(&new_key);

        let mut key_vec:Vec<char> = keys.iter().map(|c|*c).collect();
        key_vec.sort();
        let node = NodeKey{keys:key_vec, pos:new_pos};
        Context{node,keys_left:key_set, map:self.map.clone(), keys, pos:new_pos, steps, num_keys:self.num_keys, width:self.width, height:self.height}
    }

    fn get_item(&self, pos:Pos) -> char {
        *self.map.get(&pos).unwrap()
    }

    fn is_new_key(&self, pos:Pos) -> bool {
        let item = self.get_item(pos);
        item.is_ascii_lowercase() && !self.keys.contains(&item)
    }

    fn is_possible_move(&self, new_pos:Pos) -> bool {
        let item = self.map.get(&new_pos);
        //println!(" -> is possible: new_pos={:?}, item={:?}",new_pos,item);
        match item {
            Some(ch) => {
                is_passable(ch) || (is_door(ch) && self.keys.contains(&ch.to_ascii_lowercase()))
            },
            None => {
                false
            },
        }
    }

    fn bfs_keys(&self) -> Vec<(char,u32)>  {
        // Get possible moves
        let mut queue = vec![];
        let mut keys:Vec<(char,u32)> = vec![];
        let mut visited = HashMap::new();

        // Process starting point
        queue.push(self.pos);
        visited.insert(self.pos, 0 as u32);

        // While queue is not empty
        while !queue.is_empty() {
            // Remove first item
            let pos = queue.remove(0);
            let dist = *visited.get(&pos).unwrap();
            let item = self.get_item(pos);

            //println!("Visit item {} at {:?}", item, pos);
            // Is this a previously not found key in this search?
            if self.is_new_key(pos) {
                //println!("-> Found key {} at {:?}", item, pos);
                keys.push((item,dist));

                // Don't evaluate more from this position
                continue;
            }

            // Add possible moves
            for dir in &[Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
                let new_pos = pos.next(*dir);

                // Is this a possible move?
                if self.is_possible_move(new_pos) && !visited.contains_key(&new_pos) {
                    // Set new position as visited
                    visited.insert(new_pos, dist+1);

                    // push to queue
                    queue.push(new_pos);
                }
            }
        }

        // Print summary
        //println!("Found keys:{:?}", keys);
        keys
    }

    fn solve(&mut self,  best_solution:&mut u32, nodes:&mut HashMap<NodeKey, u32>) -> Option<u32> {
        //println!("Eval solution, steps:{}, {:?}", self.get_num_steps(),self.steps);

        // Check if we have a previous soultion to this
        if self.get_num_steps() > *best_solution {
            return None;
        }

        let prev_step_opt= nodes.get_mut(&self.node);
        if prev_step_opt.is_some() {
            let prev_step = *prev_step_opt.unwrap();

            if self.get_num_steps() < prev_step {
                nodes.insert(self.node.clone(), self.get_num_steps());
            } else {
                //println!("==> Already visited pos:{:?}, node:{:?}, prev steps:{}, curr step:{}",
                  //       self.pos, self.node, prev_step, self.get_num_steps());

                return None;
            }
        } else {
            nodes.insert(self.node.clone(), self.get_num_steps());
        }

        // Did we find a solution?
        if self.is_solved() {
            println!("=====> Found solution: {:?}, total steps: {}", self.steps, self.get_num_steps());
            *best_solution = self.get_num_steps();
            return Some(self.get_num_steps())
        }

        // Generate solutions
        let mut key_steps = self.bfs_keys();
        key_steps.sort_by(|(a1,b1),(a2,b2)| b1.cmp(b2));

        let mut results:Vec<u32> = vec![];

        for (item, dist) in key_steps.iter() {
            // Create new context
            let key_pos = self.get_key_pos(*item);
            let mut new_context = self.next_context(key_pos, *item, *dist );

            let solution = new_context.solve(best_solution, nodes);

            if solution.is_some() {
                results.push(solution.unwrap());
            }
        }

        match results.iter().min() {
            Some(steps) => Some(*steps),
            None => None,
        }
    }

    fn is_solved(&self) -> bool {
        self.keys.len() as u32 == self.num_keys
    }

    fn get_num_steps(&self) -> u32 {
        self.steps.iter().map(|(ch,dist)| dist).sum()
    }

    fn get_key_pos(&self, key:char) -> Pos {
        *self.map.iter().find(|(_,&b)| b == key).unwrap().0
    }

}


fn do_move(context:&mut Context, dir:Direction) -> char {
    let new_pos = match dir {
        Direction::Up => Pos{x:context.pos.x, y:context.pos.y+1},
        Direction::Down=> Pos{x:context.pos.x, y:context.pos.y-1},
        Direction::Left=> Pos{x:context.pos.x-1, y:context.pos.y},
        Direction::Right => Pos{x:context.pos.x+1, y:context.pos.y},
    };

    // Update position
    context.pos = new_pos;

    // Return new item
    *context.map.get(&new_pos).unwrap()
}



fn is_key(ch:&char) -> bool {
    ch.is_ascii_lowercase()
}

fn is_door(ch:&char) -> bool {
    ch.is_ascii_uppercase()
}

fn is_passable(ch:&char) -> bool {
    *ch == '.' || *ch == '@' || is_key(ch)
}

fn num_keys(map:&HashMap<Pos,char>) -> u32 {
    map.iter().fold(0,|acc, (key,item)| {
        if is_key(item) {
            acc + 1
        } else {
            acc
        }
    })
}

fn get_start_pos(map:&HashMap<Pos,char>) -> Pos {
    let item = map.iter().find( |(&pos,&ch)| ch == '@' ).unwrap();
    *item.0
}

fn parse_input(input:&str) -> Context {

    let num_lines = input.lines().count() as u32;
    let w = input.lines().last().unwrap().len() as u32;
    let num_keys = input.chars().filter(|ch| ch.is_ascii_lowercase()).count() as u32;
    let keys:Vec<char> = input.chars().filter(|ch| ch.is_ascii_lowercase()).collect();
    let mut key_set = HashSet::new();
    for x in keys.iter() {
        key_set.insert(*x);
    }

    let mut my_map:HashMap<Pos,char> = HashMap::new();
    input.lines().enumerate().for_each(|(line_no, line)| {
        line.chars().enumerate().for_each(|(x_pos, ch)| {
            my_map.insert( Pos{x:x_pos as u32, y:num_lines - line_no as u32 - 1}, ch);
        } )
    });

    let start_pos = get_start_pos(&my_map);
    let node = NodeKey{pos:start_pos, keys:vec![]};
    Context{node:node, keys_left:key_set, num_keys:num_keys, steps:vec![], map:my_map, pos:start_pos, height:num_lines, width:w, keys:HashSet::new()}
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::util::day_18::Direction::*;

    #[test]
    fn test1() {
        let input =
            "#########
#b.A.@.a#
#########";

        let m = parse_input(input);
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
        let mut m = parse_input(input);
        println!("start = {:?}",m.pos);

        assert_eq!(Pos::new(15,3), m.pos);


        assert_eq!(m.is_possible_move( m.pos.next(Up)),false);
        assert_eq!(m.is_possible_move(m.pos.next(Down)),false);
        assert_eq!(m.is_possible_move(m.pos.next(Left)),true);
        assert_eq!(m.is_possible_move(m.pos.next(Right)),true);

        //  Move left
        assert_eq!('.',do_move(&mut m, Left));
        assert_eq!(Pos::new(14,3), m.pos);
        assert_eq!(m.is_possible_move(m.pos.next(Left)),false);

        assert_eq!('@',do_move(&mut m, Right));
        assert_eq!('.',do_move(&mut m, Right));
        assert_eq!(true, m.is_possible_move(m.pos.next(Right)));
        assert_eq!('a',do_move(&mut m, Right));

        assert_eq!(6, num_keys(&m.map));

        // Check key pos
        let pos_a = m.get_key_pos('a');
        let pos_b = m.get_key_pos('b');

        assert_eq!( Pos::new(17,3), pos_a);
        assert_eq!( Pos::new(11,3), pos_b);

    }

    #[test]
    fn test3() {
        let input =
            "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
        let mut m = parse_input(input);

        let res = m.bfs_keys();
        assert_eq!(('a',2), *res.first().unwrap());

        // Add 'a' to found keys and move to that context
        m.keys.insert('a');
        m.pos.x = 17;

        // Redo BFS
        let res = m.bfs_keys();
        assert_eq!(('b',6), *res.first().unwrap());

    }

    #[test]
    fn test4() {
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
        let m = parse_input(input);

        let res = m.bfs_keys();
        println!("{:?}", res);
        assert_eq!(('b',3), res[0]);
        assert_eq!(('f',3), res[1]);
        assert_eq!(('a',3), res[2]);
        assert_eq!(('g',3), res[3]);

        let next_m = m.next_context( m.get_key_pos('b'),'b',3);

        let res = next_m.bfs_keys();
        println!("{:?}", res);
        println!("{:?}", next_m.steps);
    }

    #[test]
    fn test_solve_1() {
        let input =
            "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
        let mut m = parse_input(input);
        println!("start = {:?}", m.pos);
        let mut best = 99999999;
        let mut map = HashMap::new();
        let res = m.solve(&mut best, &mut map);

        println!("Solution: {}", res.unwrap() );
        assert_eq!(86, res.unwrap());

    }

    #[test]
    fn test_solve_2() {
        let input =
            "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";
        let mut m = parse_input(input);
        println!("start = {:?}", m.pos);
        let mut best = 99999999;
        let mut map = HashMap::new();
        let res = m.solve(&mut best, &mut map);

        println!("Solution: {}", res.unwrap() );
        assert_eq!(132, res.unwrap());

    }

    #[test]
    fn test_solve_3() {
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
        let mut m = parse_input(input);
        println!("start = {:?}", m.pos);
        let res = m.bfs_keys();

        println!("Solution: {:?}", res);

        let mut best = 9000;
        let mut map = HashMap::new();
        let res = m.solve(&mut best, &mut map);

        println!("Solution: {}", res.unwrap() );
        assert_eq!(136, res.unwrap());
    }


    #[test]
    fn test_solve_4() {
        let input =
            "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";
        let mut m = parse_input(input);
        println!("start = {:?}", m.pos);
        let res = m.bfs_keys();

        println!("Solution: {:?}", res);

        let mut best = 90000;
        let mut map = HashMap::new();
        let res = m.solve(&mut best, &mut map);

        println!("Solution: {}", res.unwrap() );
        assert_eq!(81, res.unwrap());

    }

    //#[test]
    fn test_solve_part1() {
        let input =
            "#################################################################################
#...#...#...#...........#.....#.......#.#.....#.........#...............#.......#
#.#.#.#.#.#.#####.#####.#.#T#.#.###.#.#.#.#.###.###.###.###P#########.###.#####.#
#.#...#.#.#v....#...#k..#.#.#.#.#.Z.#...#.#.#.....#.#l#.....#.........#.......#.#
#.#####.#.#####.#####.###.#.#.#.#.#####.#.###.#####.#.#######.#########.#######.#
#...#...#.#...#.#...#.....#.#.#.#.#...#.#.....#.#...#...........#.....#.#.#.S...#
#.#.###.#.#.#H#.#.#.#######.#.###.#.#.#.#.#####.#.###.###########.###.#.#.#.###.#
#.#...#...#.#.#.#.#.........#...#.#.#...#.#...#.#...#...#.........#.#.....#.#...#
#.###.#######.#.#.#####.#######.#.#####.#.#.#.#O###.###.#.#########.#######.#.###
#...#.#.....#.#.#.#.....#....b#.#.....#.#.#.#...#.#.#...#.#...#.......#.....#...#
###.#.#.###.#.#.#.#.#####.###.#.#####.#.#.#.###.#.#.#.###.#.###.#####.#.#######.#
#...#...#...#.#.#.#.....#.#...#.......#.#.#.#.#...#.#...#.#...#.#.#...#.#...#...#
#.#######.###.#.#.#######.#.###########.#.#.#.#.###.#.###.#.#.#.#E#.###.#.#.#.###
#.#...#.#.#.....#.#.......#.....#.#...A.#...#.#.#...#.#...#.#.#...#...#.#.#.....#
#.#.#.#.#.#####.#.#.###########.#.#.#########.#.#.#####.###.#.###.###.#.#######.#
#...#...#.#.Y.#.#.#...#.....#...#...#...#.......#...#...#...#.......#.#.......#.#
###.#####.#.#.#.#.#.###G###.#.###.###.#.#####.#####.#.#######.#######.#######.#.#
#.#.#.....#.#.#...#.#...#u#...#.#.....#.#...#.#.....#.......#.#...#.........#.#.#
#.#.#.###.#.#.#####.#.###.#####.#######.#.#.###.###########.###.#.#.#.#######.###
#...#.#...#.#......w#.#.#...#.........#.#.#...#.........#.......#...#.#.....#...#
#####.#####.#########.#.#.#.#.#.###.###.#.###.#.#######.#.###########.#.###.###.#
#..q#.....#.....#...#.#...#.#.#.#...#.F.#.#.#.#.......#.....#...#.....#.#.#.....#
#.#.#.###.#####.#.#.#######.#.#.#.###.###.#.#.#################.#.#####.#.#####.#
#.#.#...#.....#.#.#.........#.#.#...#...#...#.....#...#.........#.#.....#...#...#
#.#.#####.#####.###.###.#####D#.#######.###.#####.#.#.#.#########.#.#######.#.###
#.#.#.....#.....#...#...#.....#.......#.#.#.....#...#...#.........#.......#...#.#
#.#.#.#.###.#####.#######.#########.###.#.#####.###########.#############.#.###.#
#.#.#.#.#...#.........#...#.......#.....#.....#...#.......#.#.....#.......#...#.#
#.#.#.#.#.#####R#####.#.###.###.#########.###.###.#.#####.#.#####.#.#########.#.#
#.#.#.#.#.....#.#...#...#...#...#.......#...#...#.#...#.#.#.......#...#.....#.#.#
#.#L#.#######.#.#.#.#####.#######.#####.###.###.#.###.#.#.#####.#####.###.#.#.#.#
#.#.#.#...In..#.#.#...#.........#.....#.#...#.#.#.....#.#.#.#...#...#...#.#.#...#
#.#.#.#.#######.#.###X#.#######.#.#####.#.###.#.#######.#.#.#.###.#.###.#.#.###.#
#.#...#.#.....#...#...#...#...#.#.#.....#.#...#.........#.#...#...#...#.#.#.....#
#.###.#.###.#.#####.#####.#.#.#.#.#.#####.#.#.###.#######.###.#.#####.#.#.#######
#c#.#.#.....#...#.......#.#.#.#...#.....#.#j#...#.#.....#...#.#...#...#.#...#...#
#.#.#.#########.#######.#.#.#.#####.###.#.###.###.#.###.###.#.###.#.###.###.#.#.#
#.#.....#....m#.....#...#.#.#.....#...#.#.#...#...#.#.#...#.#...#.#...#.#.#.#.#.#
#.#######.#.#######.#.#####.#####.#####.#.#.#.#.###.#.###.#.#####.###.#.#.#.###.#
#.........#.........#.......#...............#.#.........#.........#.....#.......#
#######################################.@.#######################################
#.#...#...#...................#.....#.....................#.........#.......#...#
#.#.#.#.#.#.#######.#######.###J#.#.###.#.###.#############.###.###.#####.#.###.#
#...#.#.#...#.....#.#.#.....#...#.#...#.#.#...#...#.........#...#.#.......#r....#
#.###.#######.###.#.#.#.#####.###.###.#.#.#.###.#.#.#########.###.#############.#
#.#.#.......#.#.#.#...#.........#.#...#.#.#.....#...#x#.......#.......#..d......#
#.#.#######.#.#.#.###.###########.#.###.###########.#.#.#######.#####.#.#########
#...#.......#.#.#...#.#.......#...#.....#.........#...#.#.......#...#.#...#.....#
###.#.#####Q#.#.###.#.#.#.###.#.#.#######.#######.#####.#.#######.#.#####.#.###.#
#.#.#.#.....#.#.#.W.#.#.#...#.#.#.#.....#.#...........#.#.........#...#...#.#...#
#.#.#.#####.#.#.#.###.#####.#.#.###.###.#.###########.#.#.###########.#.###.#.###
#...#.#...#.#.#.#.#.........#.#.....#...#.....#...#.#...#.....#.......#.#...#...#
#.###.#.#.###.#.#.###########.#######.###.###.#.#.#.#########.#U#####.#.#####.#.#
#.#.C.#.#...#.#.#.....#.....#.#.....#...#...#...#.#...#.....#.#.#.....#.#...#.#.#
#.#.###.###.#.#.#####.#####.#.#.###.###.###.#####.#.#.#.#.###.#.#.#####.#.#.#.#.#
#.#.....#.#.#...#.#.B.#...#...#.#.#.....#.#...#.#.#.#...#.#...#.#...#...#.#.#.#.#
#.#####.#.#.###.#.#.###.#.#.###.#.#####.#.###.#.#.#.#####.#.###.#####.###.#.###.#
#.#...#...#...#.#.#...#.#.#...#.#...#...#...#...#.#.#...#...#.#.......#...#...#.#
#.#.#.#####.#.#.#.###.#.#.#####.#.###.#####.#.###.###.#.###.#.#########.#####.#.#
#.#.#...M...#.#.#...#.#.#...#...#.......#...#.#...#...#...#.......#.....#...#...#
#.#.#########.#.###.#.#.###.#.###########.###.#.#.#.#####.#.#####.#.#####.#.###.#
#.#.........#.#.#...#g#.#.#.#.....#.....#.....#.#.#.#...#.#.#...#...#.....#...#.#
#.#########.#.#.#.#.###.#.#.#####.#.###.#######.#.#.#.#.#.#####.#####.#####.#.#.#
#.......#...#.#.#.#.#...#...#f..#.....#.#.......#.#...#.#.....#...#...#.....#.#.#
#####.###.###.#.#.#.#.#####.#.#.#####.#.#.#######.#####.###.#####.###.#.#####.#.#
#.....#...#i..#y#.#...#...#...#.....#.#.#.#.....#.#...#.#.#.....#...#.#.#.....#.#
#.#####.###.###.#.###.#.#.#########.###.#.#.#.###.###.#.#.#####.###.#.#.#####.#.#
#.....#.#...#.N.#...#.#.#.......#.......#.#.#.........#.#...#.....#.#.#.....#.#.#
#####.#.#####.#####.#.#.#######.#######.#.#############.###.#.#####.#.#####.###.#
#...#.#.......#.....#.#...#...#.......#.#.......#a......#...#.....#.#.V.#...#...#
#.#.#.#########.###.#####.#.#########.#.#.#####.#.#######.#.#####.#.###.#.#.#.###
#.#.#.#..s....#.#...#.....#.........#.#.#.#...#.#.#.......#.#..t..#.#...#.#.#...#
#.#.#.#####.#.#.#####.#####.#######.#.#.#.#.###.#.#.#######.#.#####.#####.#####.#
#.#.#..p#...#...#...#...#.......#...#.#.#.#...#...#.......#.#.....#.....#.....#.#
#.#.###.#.#.#####.#.###.#.#######.###.#.#.###.###########.#######.#####.###.#.#.#
#.#o#...#.#.#.....#.....#...#...#...#.#.#...........#.....#.......#...#.#...#.#.#
#.###.###.###.#########.#####.#.###.#.#.###########.#.###.#.#######.#.#.#.#####.#
#...#...#.........#...#.......#.....#.#.#.......#...#.#.#.#.....#..h#.#.#...#...#
#.#.###.###########.#.###############.#.#.#####.#.###.#.#.#####K#.###.#.#.#.#.###
#.#.................#e................#.#z....#.......#.......#.....#...#.#.....#
#################################################################################";
        let mut m = parse_input(input);
        println!("start = {:?}", m.pos);
        let mut best = 99999999;
        let mut map = HashMap::new();
        let res = m.solve(&mut best, &mut map);

        println!("Solution: {}", res.unwrap() );
    }
}