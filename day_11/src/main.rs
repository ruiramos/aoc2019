use day_11::{Computer, InputMode};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Debug)]
enum Direction {
    L,
    R,
    U,
    D,
}

fn turn_left(d: Direction) -> Direction {
    match d {
        Direction::L => Direction::D,
        Direction::R => Direction::U,
        Direction::U => Direction::L,
        Direction::D => Direction::R,
    }
}

fn turn_right(d: Direction) -> Direction {
    match d {
        Direction::L => Direction::U,
        Direction::R => Direction::D,
        Direction::U => Direction::R,
        Direction::D => Direction::L,
    }
}
fn move_forward(state: ((isize, isize), Direction)) -> ((isize, isize), Direction) {
    let pos = state.0;
    let d = state.1;
    (
        match d {
            Direction::L => (pos.0 - 1, pos.1),
            Direction::R => (pos.0 + 1, pos.1),
            Direction::U => (pos.0, pos.1 - 1),
            Direction::D => (pos.0, pos.1 + 1),
        },
        d,
    )
}

fn main() {
    let map = get_painted_map(0);
    let count = map.values().count();
    println!("01. {}", count);
    assert_eq!(count, 2539);

    let map2 = get_painted_map(1);
    output(map2);
}

fn write(c: char) {
    print!("{}", c);
}

fn output(map: HashMap<(isize, isize), char>) {
    for y in 0..8 {
        for x in 0..50 {
            let pixel = *map.get(&(x, y)).unwrap_or(&'b');
            if pixel == 'b' {
                write(' ');
            } else {
                write('#');
            }
        }
        write('\n');
    }
    io::stdout().flush().unwrap();
}

fn get_painted_map(start: usize) -> HashMap<(isize, isize), char> {
    let data = get_program();

    let mut map: HashMap<(isize, isize), char> = HashMap::new();

    let mut robot_state = ((0, 0), Direction::U);
    let mut c = Computer::new(&data, InputMode::Args, vec![start as isize]);

    while !c.is_terminated() {
        c.execute();

        match c.get_output() {
            Some(n) if n == 0 => {
                map.insert(robot_state.0, 'b');
            }
            Some(n) if n == 1 => {
                map.insert(robot_state.0, 'w');
            }
            _ => panic!("got weird output"),
        }

        c.execute();

        match c.get_output() {
            Some(n) if n == 0 => {
                robot_state.1 = turn_left(robot_state.1);
                robot_state = move_forward(robot_state);
            }
            Some(n) if n == 1 => {
                robot_state.1 = turn_right(robot_state.1);
                robot_state = move_forward(robot_state);
            }
            _ => panic!("got weird output"),
        }

        match map.get(&robot_state.0) {
            Some(col) if col == &'w' => c.send_input(1),
            _ => c.send_input(0),
        }
    }

    map
}

fn run_second() {}

fn get_program() -> String {
    let mut f = File::open("./data.txt").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    String::from(buffer.trim())
}
