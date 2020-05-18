
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let masses:Vec<u32> = input.lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let result = match part {
        Part::Part1 => sum_fuel_part1(masses),
        Part::Part2 => sum_fuel_part2(masses)
    };

    format!("{}",result)
}

fn calc_fuel(mass:u32) -> u32 {
    let tmp = mass / 3;
    match tmp {
        0 => 0,
        1 => 0,
        2 => 0,
        _ => tmp - 2 + calc_fuel(tmp-2),
    }
}

fn sum_fuel_part2(masses : Vec<u32>) -> u32 {
    masses.iter().map( |mass| calc_fuel(*mass)).sum()
}

fn sum_fuel_part1(masses : Vec<u32>) -> u32 {
    masses.iter().map(|mass|  (mass / 3 - 2)).sum()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test1() {

        let masses = vec![12,14,1969,100756,22,121,23323,232];

        println!("{}", sum_fuel_part1(masses));
    }

    #[test]
    fn test12() {
        let masses = vec![1969];
        println!("{}", sum_fuel_part1(masses));
    }

    #[test]
    fn test2() {
        let masses = vec![1969];
        println!("{}", sum_fuel_part2(masses));
    }

    #[test]
    fn test21() {
        let masses = vec![100756];
        println!("{}", sum_fuel_part2(masses));
    }

}
