use crate::util::Part;
use std::f64::consts::PI;
use std::collections::*;
use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Equal, Less};

pub fn solve(input:String, part:Part) -> String {

    let result = match part {
        Part::Part1 => part1(create_asteroid_list(input)),
        Part::Part2 => part2(create_asteroid_list(input), 200)
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

#[derive(Debug,PartialEq,Copy, Clone)]
struct Asteroid {
    x:i64,
    y:i64,
}

impl Asteroid {
    fn get_angle(&self, other:&Asteroid) -> f64 {
        let dy = self.y - other.y;
        let dx = self.x - other.x;
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

fn check_visible(asteroids:Vec<Asteroid>) -> Option<HashMap<i64, Vec<Asteroid>>> {
    let mut angle_map:Option<HashMap<i64, Vec<Asteroid>>> = None;

    for asteroid in &asteroids {
        let mut angle_cnt:HashMap<i64, Vec<Asteroid>> = HashMap::new();
        for other in &asteroids {
            if asteroid.x == other.x && asteroid.y == other.y {
                continue;
            } else {
                let angle = asteroid.get_angle(other) as i64;
                let mut asteroid_list = match angle_cnt.get_mut(&angle) {
                    None => vec![],
                    Some(list) => list.to_vec()
                };

                asteroid_list.push( other.clone());
                // Sort by distance
                asteroid_list.sort_by( |a,b| {
                    let mut dist_a = ((asteroid.x - a.x).abs().pow(2) + (asteroid.x - a.y).abs().pow(2)) as f64;
                    let mut dist_b = ((asteroid.x - b.x).abs().pow(2) + (asteroid.y - b.y).abs().pow(2)) as f64;
                    dist_a = dist_a.sqrt();
                    dist_b = dist_b.sqrt();

                    match dist_a.partial_cmp(&dist_b) {
                        Some(Ordering::Greater) => Greater,
                        Some(Ordering::Less) => Less,
                        Some(Ordering::Equal) => Equal,
                        _ => panic!("N.A")
                    }
                });

                angle_cnt.insert(angle, asteroid_list);

            }
        }

        // Update max value
        if angle_map.as_ref().is_none() || angle_map.as_ref().unwrap().len() < angle_cnt.len() {
            angle_map = Some(angle_cnt);
        }
    }

    angle_map
}


fn part1(asteroids:Vec<Asteroid>) -> usize {
   check_visible(asteroids).unwrap().len()
}


fn part2(asteroids:Vec<Asteroid>, target:i64) -> usize {
    let num_asteroids = asteroids.len();
    let mut angle_map = check_visible(asteroids).unwrap();
    let mut num_destroyed = 0;
    let mut angles : Vec<i64> = angle_map.keys().map(|k| *k).collect::<Vec<i64>>();

    // Sort so that starting point is 90 degrees and increasing
    angles.sort_by( |a,b| {
        if *a >= 270000 && *b < 270000  {
            return Ordering::Less;
        } else if *b >= 270000 && *a < 270000 {
            return Ordering::Greater;
        }
        a.cmp(&b)
    });

    while num_destroyed < target {
        for angle in &angles {
            let mut cnt =  angle_map.get_mut(angle).unwrap();
            if cnt.len() > 0 {
                // Destroy one asteroid
                let asteroid = cnt.remove(0);
                num_destroyed += 1;
                println!("[{}] Destroyed {},{} at angle:{}, left:{}",num_destroyed,asteroid.x,asteroid.y, *angle,cnt.len());

                if num_destroyed == target {
                    return (asteroid.x * 100 + asteroid.y) as usize;
                }
            }
        }
    }

    panic!("Not enough ateroids to destroy!")
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
        let res = check_visible(asteroids).unwrap().len();

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
        let res = check_visible(asteroids).unwrap().len();

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
        let res = check_visible(asteroids).unwrap().len();

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
        let res = check_visible(asteroids).unwrap().len();

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
        let res = check_visible(asteroids).unwrap().len();

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
        let res = check_visible(asteroids).unwrap().len();

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


    #[test]
    fn test_part2_1() {
        let indata = ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##";

        let asteroids = create_asteroid_list(indata.to_string());
        println!("{:?}",asteroids);
        let res = part2(asteroids,33);

        println!("res = {:?}", res);
        assert_eq!(res, 1201);
    }


    #[test]
    fn part2_test2() {
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
        let res = part2(asteroids,200);

        println!("res = {:?}", res);
        assert_eq!(res, 802);
    }

    //        let indata = include_str!("../../input_08.txt");

    #[test]
    fn test_part2() {
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
        let res = part2(asteroids,200);

        println!("res = {:?}", res);
        assert_eq!(res, 1919);
    }
}

