use crate::util::Part;
use regex::Regex;


pub fn solve(input:String, part:Part) -> String {
    let planets = create_planets(input.as_str());
    let result = match part {
        Part::Part1 => part1(planets),
        Part::Part2 => part2(input.as_str())
    };

    format!("{}",result)
}

fn create_planets(input:&str) -> Vec<Planet> {
    let mut inc = 0;
    input.lines().map( |line| { inc +=1; Planet::new(line, inc) }).collect()
}



fn part1(mut planets:Vec<Planet>) -> u64 {
    run_steps(&mut planets, 1000)
}

fn part2(input:&str) -> u64 {
    let pos_vector:Vec<(i32,i32,i32)> =  input.lines().map( |s| parse_line(s)).collect();
    let mut x_pos: [i32;4] = [0;4];
    let mut y_pos: [i32;4]=[0;4];
    let mut z_pos :[i32;4]=[0;4];
    let max = 4000_0000_000_000_000;

    for i in 0..4 {
        x_pos[i] = pos_vector[i].0;
        y_pos[i] = pos_vector[i].1;
        z_pos[i] = pos_vector[i].2;
    }

    let period_x = check_period(x_pos, max);
    let period_y = check_period(y_pos, max);
    let period_z = check_period(z_pos, max);

    let mut gcd = period_x;
    while gcd % period_y != 0 || gcd % period_z != 0 {
        gcd += period_x;
    }

    gcd
}

fn update_velocity(planets:&mut Vec<Planet>) {
    let mut new_vel:Vec<Point> = vec![];

    for planet in planets.iter() {
        let mut delta_vel = planet.vel;

        for other  in planets.iter() {
            if planet != other {
                let vel = planet.update_velocity(other);
                delta_vel.x += vel.x;
                delta_vel.y += vel.y;
                delta_vel.z += vel.z;
            }
        }
        new_vel.push(delta_vel);
    }

    // Update vel
    for i in 0..new_vel.len() {
        planets[i].vel = new_vel[i];
    }
}

fn update_pos(planets:&mut Vec<Planet>) {
    planets.iter_mut().for_each(|p| p.update_pos());
}

fn run_steps(planets:&mut Vec<Planet>,steps:i64) -> u64 {
    for step in 0..steps {
        update_velocity(planets);
        update_pos(planets);
    }

    planets.iter().map(|p|p.total_energy()).sum()
}


fn parse_line(line:&str) -> (i32,i32,i32) {

    // ex: <x=-1, y=0, z=2>
    let r = Regex::new(r"<x=(-?\d+), y=(-?[0-9]*), z=(-?[0-9]*)>").unwrap();

    let caps = r.captures(line).unwrap();
    let x = caps.get(1).unwrap().as_str().parse().unwrap();
    let y = caps.get(2).unwrap().as_str().parse().unwrap();
    let z = caps.get(3).unwrap().as_str().parse().unwrap();

    (x,y,z)
}

fn check_period(mut pos_x:[i32;4],max:u64) -> u64 {
    let mut vel_x:[i32;4] = [0;4];
    let first_pos_x = pos_x.clone();
    let mut n = 0;
    while n < max {
        // Calc vel
        for i in 0..4 {

            for j in 0..4 {
                if i == j {
                    continue;
                }

                if pos_x[i] < pos_x[j] {
                    vel_x[i] += 1;
                } else if pos_x[i] > pos_x[j] {
                    vel_x[i] -=1;
                }
            }
        }

        // Move planets
        for i in 0..4 {
            pos_x[i] += vel_x[i];
        }

        // Next step
        n += 1;

        if first_pos_x.eq(&pos_x) && vel_x.eq(&[0;4]) {
            println!("Found at step {}", n);
            return n;
        }

        if n % 1000_000 == 0 {
            println!("Steps = {}", n);
        }
    }

    panic!("No period found!")
}


#[derive(Debug,Clone,Copy,PartialEq,Eq)]
struct Point {
    x:i64,
    y:i64,
    z:i64,
}

impl Point {
    fn energy(&self) -> u64 {
        self.x.abs() as u64 + self.y.abs() as u64 + self.z.abs() as u64
    }
}

#[derive(Debug,PartialEq,Eq,Copy, Clone)]
struct Planet {
    id:u32,
    pos:Point,
    vel:Point,
}


impl Planet {
    fn new(line:&str,id:u32) -> Planet {

        // ex: <x=-1, y=0, z=2>
        let r = Regex::new(r"<x=(-?\d+), y=(-?[0-9]*), z=(-?[0-9]*)>").unwrap();

        let caps = r.captures(line).unwrap();
        let x = caps.get(1).unwrap().as_str().parse().unwrap();
        let y = caps.get(2).unwrap().as_str().parse().unwrap();
        let z = caps.get(3).unwrap().as_str().parse().unwrap();

        let vel = Point{x:0,y:0,z:0};
        let pos = Point{x:x,y:y,z:z};
        Planet{id:id,pos:pos, vel:vel}
    }

    fn update_velocity(&self, other:&Planet) -> Point {
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;
        if self.pos.x < other.pos.x {
            x = 1;
        } else if self.pos.x > other.pos.x {
            x = -1;
        }

        if self.pos.y < other.pos.y {
            y = 1;
        } else if self.pos.y > other.pos.y {
            y = -1;
        }

        if self.pos.z < other.pos.z {
            z = 1;
        } else if self.pos.z > other.pos.z {
            z = -1;
        }

        Point{x:x,y:y,z:z}
    }

    fn update_pos(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }

    fn total_energy(&self) -> u64 {
        self.pos.energy() * self.vel.energy()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() {
        println!("Test");

        let line1 = "<x=-1, y=0, z=-200>";
        let p1 = Planet::new(line1,1);

        println!("{:?}", p1);

        assert_eq!(p1.pos.x, -1);
        assert_eq!(p1.pos.y, 0);
        assert_eq!(p1.pos.z, -200);

    }

    #[test]
    fn test2() {
        println!("Test");
        let input = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";


        let planets = create_planets(input);
        let p1 = &planets[0];
        let p4 = &planets[3];
        println!("{:?}", planets);

        assert_eq!(p1.pos.x, -1);
        assert_eq!(p1.pos.y, 0);
        assert_eq!(p1.pos.z, 2);
        assert_eq!(p1.id, 1);

        assert_eq!(p4.pos.x, 3);
        assert_eq!(p4.pos.y, 5);
        assert_eq!(p4.pos.z, -1);
        assert_eq!(p4.id, 4);
    }

    #[test]
    fn test3() {
        println!("Test");
        let input = "<x=-1, y=0, z=2>
<x=1, y=2, z=3>
<x=1, y=2, z=3>
<x=3, y=5, z=-1>";

        let planets = create_planets(input);
        let p1 = &planets[0];
        let p2 = &planets[1];
        let p3 = &planets[2];
        let p4 = &planets[3];
        println!("{:?}", planets);

        assert_ne!(p1,p4);
        assert_eq!(p1,p1);
        assert_ne!(p2,p3);

    }

    #[test]
    fn test4() {
        println!("Test");
        let input = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

        let mut planets = create_planets(input);

        update_velocity(&mut planets);
        println!("{:?}", planets);

        let p1 = &planets[0];
        let p2 = &planets[1];

        assert_eq!(Point{x:3,y:-1,z:-1},p1.vel);
        assert_eq!(Point{x:1,y:3,z:3},p2.vel);

    }

    #[test]
    fn test_example1() {
        println!("Test");
        let input = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

        let mut planets = create_planets(input);

        let res = run_steps(&mut planets,10);
        println!("res={}",res);
        assert_eq!(179, res);
    }

    #[test]
    fn test_example2() {
        println!("Test");
        let input = "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";

        let mut planets = create_planets(input);

        let res = run_steps(&mut planets,100);
        println!("res={}",res);
        assert_eq!(1940, res);
    }

    #[test]
    fn test_part2_test1() {
        println!("Test");
        let input = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

        let res = part2(input);
        println!("res={}",res);

    }

    #[test]
    fn test_part2_test2() {
        let in1 = "<x=3, y=15, z=8>
<x=5, y=-1, z=-2>
<x=-10, y=8, z=2>
<x=8, y=4, z=-5>";

        let input = "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";

        let res = part2(input);
        println!("res={}",res);
    }

    #[test]
    fn test_part2_test3() {
        let input = "<x=3, y=15, z=8>
<x=5, y=-1, z=-2>
<x=-10, y=8, z=2>
<x=8, y=4, z=-5>";


        let res = part2(input);
        println!("res={}",res);
    }
}