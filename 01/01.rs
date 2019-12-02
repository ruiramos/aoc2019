use std::convert::TryInto;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn fuel(m: usize) -> isize {
    (m / 3 - 2) as isize
}

fn fuel_rec(m: usize) -> isize {
    let result = fuel(m);
    match result {
        n if n > 0 => n + fuel_rec(n.try_into().unwrap()),
        _ => result,
    }
}

fn main() -> io::Result<()> {
    let mut f = File::open("fuel.txt")?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    let sum_of_mass: isize = buffer
        .split('\n')
        .map(|m| {
            let value = m.parse::<usize>();
            if let Ok(value) = value {
                fuel(value)
            } else {
                0
            }
        })
        .sum();

    println!("01. {}", sum_of_mass);

    let sum_of_mass: isize = buffer
        .split('\n')
        .map(|m| {
            let value = m.parse::<usize>();
            if let Ok(value) = value {
                fuel_rec(value)
            } else {
                0
            }
        })
        .sum();

    println!("02. {}", sum_of_mass);

    Ok(())
}
