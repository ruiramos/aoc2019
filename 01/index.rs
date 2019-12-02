use std::fs::File;
use std::io;
use std::io::prelude::*;

fn fuel(m: usize) -> usize {
    m / 3 - 2
}

fn sum_of_vec(v: Vec<usize>) -> usize {
    v.iter().fold(0, |e, acc| acc + e)
}

fn main() -> io::Result<()> {
    let mut f = File::open("fuel.txt")?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    let mass: Vec<usize> = buffer
        .split('\n')
        .map(|m| {
            let value = m.parse::<usize>();
            if let Ok(value) = value {
                fuel(value)
            } else {
                0
            }
        })
        .collect();

    println!("{}", sum_of_vec(mass));

    Ok(())
}
