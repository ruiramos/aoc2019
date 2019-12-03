use std::convert::TryInto;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn fuel(m: &usize) -> isize {
    (m / 3 - 2) as isize
}

fn fuel_rec(m: &usize) -> isize {
    let result = fuel(m);
    match result {
        n if n > 0 => n + fuel_rec(&(n as usize)),
        _ => result,
    }
}

fn main() {
    let mut f = File::open("fuel.txt").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    let input: Vec<usize> = buffer.lines().map(|m| m.parse().unwrap()).collect();

    let sum_of_mass: isize = input.iter().map(fuel).sum();
    println!("01. {}", sum_of_mass);

    let sum_of_mass: isize = input.iter().map(fuel_rec).sum();
    println!("02. {}", sum_of_mass);
}
