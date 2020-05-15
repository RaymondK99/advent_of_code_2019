use std::io;
use std::io::prelude::*;

fn main() {


    let stdin = io::stdin();

    let mut masses:Vec<u64> = vec!();

    for line in stdin.lock().lines() {
        masses.push( line.unwrap().trim().parse().unwrap());
    }

    println!("{}",sum_fuel(masses));
}

fn sum_fuel(masses : Vec<u64>) -> u64 {
    masses.iter().map(|mass|  (mass / 3 - 2)).sum()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test1() {

        let masses = vec![12,14,1969,100756,22,121,23323,232];

        println!("{}", sum_fuel(masses));
    }

}
