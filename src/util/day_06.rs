use crate::util::Part;
use std::collections::HashMap;


pub fn solve(input:String, part:Part) -> String {
    let mut planets:HashMap<String, Vec<String> > = HashMap::new();

    // Build up system
    for pair in input.lines().map(|l| l.trim().to_string()) {
        let pars:Vec<String> = pair.split(')').map(|p| p.to_string()).collect();
        let planet_name = pars[0].clone();
        let orbitor = pars[1].clone();

        // Add parent planet
        if planets.contains_key( &orbitor) {
            planets.get_mut(&orbitor).unwrap().push(planet_name.clone());
        } else {
            planets.insert(orbitor.clone(), vec![planet_name.clone()]);
        }

        // Add child planets
        if planets.contains_key(&planet_name) {
            planets.get_mut(&planet_name).unwrap().push(orbitor);
        } else {
            planets.insert(planet_name, vec![orbitor]);
        }
    }

    match part {
        Part::Part1 => part1(planets).to_string(),
        Part::Part2 => part2(planets).to_string(),
    }
}

fn part1(planets:HashMap<String, Vec<String>>) -> u32 {
    calc_dist(&String::from("COM"), &String::from("COM"), 0,&planets)
}

fn calc_dist(origin:&String, name:&String, dist:u32, planets:&HashMap<String, Vec<String>>) -> u32 {
    match planets.get(name) {
        Some(neighbors) =>  dist + neighbors.iter()
            .filter( |p| (*p).to_string().ne(origin))
            .map( |o| calc_dist(name,o, dist+1, planets)).
            sum::<u32>(),
        None => dist
    }
}

fn part2(planets:HashMap<String, Vec<String>>) -> u32 {
   find_santa(&planets, &String::from("YOU"),&String::from("YOU"),0) - 2
}

fn find_santa(planets:&HashMap<String, Vec<String>>, name:&String, origin:&String, dist:u32) -> u32 {
    // Is this Santa?
    if name.as_str().eq("SAN") {
        return dist;
    }

    match planets.get(name) {
        Some(neighbors) => {
            neighbors.iter().filter( |p| (*p).ne(origin))
                .map( |p| find_santa(planets, p, name, dist+1))
                .sum()
        },
        None => 0
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test1() {
        let inputs = "COM)B
                            B)C
                            C)D
                            D)E
                            E)F
                            B)G
                            G)H
                            D)I
                            E)J
                            J)K
                            K)L";

        let res = solve(inputs.to_string(), Part::Part1);
        println!("{}",res);
        assert_eq!("42", res);
    }

    #[test]
    fn test2() {
        let inputs = "COM)B
                            B)C
                            C)D
                            D)E
                            E)F
                            B)G
                            G)H
                            D)I
                            E)J
                            J)K
                            K)L
                            K)YOU
                            I)SAN";

        let res = solve(inputs.to_string(), Part::Part2);
        println!("{}",res);
        assert_eq!("4", res);
    }
}
