use crate::util::Part;
use std::f64::consts::PI;
use std::collections::HashMap;
use std::fs::read;

pub fn solve(input:String, part:Part) -> String {

    let result = match part {
        Part::Part1 => part1(create_asteroid_list(input)),
        Part::Part2 => part2(create_asteroid_list(input))
    };

    format!("{}",result)
}

fn create_asteroid_list(input:String) -> Vec<Asteroid> {
    let mut asteroids = vec![];

    input.lines().enumerate().for_each( | (y, line)| {
        line.chars().enumerate()
            .filter(|(_,ch)| *ch == '#')
            .for_each( |(x, ch)| {
            asteroids.push(Asteroid{x:x as i64, y:y as i64});
        })
    });

    asteroids
}

#[derive(Debug,PartialEq)]
struct Asteroid {
    x:i64,
    y:i64,
}

impl Asteroid {
    fn get_angle(&self, other:&Asteroid) -> f64 {
        let dy = (self.y - other.y);
        let dx = (self.x - other.x);
        let mut angle;

        if dy == 0 && dx > 0 {
            angle = 180_f64;
        } else if dy == 0 && dx < 0 {
            angle = 0_f64;
        } else if dx == 0 && dy < 0  {
            angle = 90_f64;
        } else if dx == 0 && dy > 0  {
            angle = -90_f64;

        } else {
            angle = dy as f64/dx as f64;
            angle = angle.atan() * 180_f64 / PI;
            if dx > 0 {
                // left quandrants
                angle += 180_f64;
            }
        }

        while angle < 00_f64 {
            angle += 360.00_f64;
        }

        while angle > 360.00_f64 {
            angle -= 360.00_f64;
        }

        angle * 1000_f64
    }
}

fn check_visible(asteroids:Vec<Asteroid>) -> usize {
    let mut max:usize = 0;
    for asteroid in &asteroids {
        let mut angle_cnt:HashMap<i64, usize> = HashMap::new();
        for other in &asteroids {
            if asteroid.x == other.x && asteroid.y == other.y {
                continue;
            } else {
                let angle = asteroid.get_angle(other) as i64;
                let cnt = match angle_cnt.contains_key(&angle) {
                    true => angle_cnt.get(&angle).unwrap().clone() + 1,
                    false => 1,
                };
                angle_cnt.insert(angle, cnt);
            }
        }
        max = std::cmp::max(max, angle_cnt.len());
        //println!("Asteroid {},{} has {} visible, {:?}",asteroid.x, asteroid.y, angle_cnt.len(), angle_cnt);
    }
    max
}


fn part1(asteroids:Vec<Asteroid>) -> usize {
   check_visible(asteroids)
}


fn part2(asteroids:Vec<Asteroid>) -> usize {
    2
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() {
        let indata = ".#..#
.....
#####
....#
...##";

        let asteroids = create_asteroid_list(indata.to_string());
        println!("{:?}",asteroids);
        let res = check_visible(asteroids);

        println!("res = {:?}", res);
        assert_eq!(res, 8);
    }

    #[test]
    fn test2() {
        let indata = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";

        let asteroids = create_asteroid_list(indata.to_string());
        println!("{:?}",asteroids);
        let res = check_visible(asteroids);

        println!("res = {:?}", res);
        assert_eq!(res, 33);
    }

    #[test]
    fn test3() {
        let indata = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";

        let asteroids = create_asteroid_list(indata.to_string());
        println!("{:?}",asteroids);
        let res = check_visible(asteroids);

        println!("res = {:?}", res);
        assert_eq!(res, 35);
    }

    #[test]
    fn test4() {
        let indata = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";

        let asteroids = create_asteroid_list(indata.to_string());
        println!("{:?}",asteroids);
        let res = check_visible(asteroids);

        println!("res = {:?}", res);
        assert_eq!(res, 41);
    }

    #[test]
    fn test5() {
        let indata = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

        let asteroids = create_asteroid_list(indata.to_string());
        println!("{:?}",asteroids);
        let res = check_visible(asteroids);

        println!("res = {:?}", res);
        assert_eq!(res, 210);
    }
    #[test]
    fn test_part1() {
        let indata = "#..#.#.###.#...##.##....
.#.#####.#.#.##.....##.#
##..#.###..###..#####..#
####.#.#..#....#..##.##.
.#######.#####...#.###..
.##...#.#.###..###.#.#.#
.######.....#.###..#....
.##..##.#..#####...###.#
#######.#..#####..#.#.#.
.###.###...##.##....##.#
##.###.##.#.#..####.....
#.#..##..#..#.#..#####.#
#####.##.#.#.#.#.#.#..##
#...##.##.###.##.#.###..
####.##.#.#.####.#####.#
.#..##...##..##..#.#.##.
###...####.###.#.###.#.#
..####.#####..#####.#.##
..###..###..#..##...#.#.
##.####...##....####.##.
####..#..##.#.#....#..#.
.#..........#..#.#.####.
###..###.###.#.#.#....##
########.#######.#.##.##";

        let asteroids = create_asteroid_list(indata.to_string());
        println!("{:?}",asteroids);
        let res = check_visible(asteroids);

        println!("res = {:?}", res);
        assert_eq!(res, 247);
    }





    #[test]
    fn test31() {
        let a1 = Asteroid{x:0,y:0};
        let a_0 = Asteroid{x:2,y:0};
        let a_45 = Asteroid{x:2,y:2};
        let a_90 = Asteroid{x:0,y:2};
        let a_135 = Asteroid{x:-2,y:2};
        let a_180 = Asteroid{x:-2,y:0};
        let a_225 = Asteroid{x:-2,y:-2};
        let a_270 = Asteroid{x:0,y:-2};
        let a_315 = Asteroid{x:2,y:-2};

        println!("{}",a1.get_angle(&a_0));
        println!("{}",a1.get_angle(&a_45));
        println!("{}",a1.get_angle(&a_90));
        println!("{}",a1.get_angle(&a_135));
        println!("{}",a1.get_angle(&a_180));
        println!("{}",a1.get_angle(&a_225));
        println!("{}",a1.get_angle(&a_270));
        println!("{}",a1.get_angle(&a_315));
    }
}

