use day_11::{Computer, InputMode};
use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::{thread, time};

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

#[derive(Debug)]
pub struct Pixel {
    x: isize,
    y: isize,
    color: char,
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    map: HashMap<(isize, isize), char>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BG: [f32; 4] = [0.8, 0.8, 0.8, 1.0];

        let map = &self.map;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BG, gl);

            for (coords, color) in map.iter() {
                let transform = c.transform.trans(0., 0.);
                let square = rectangle::square(coords.0 as f64 * 10., coords.1 as f64 * 10., 10.);

                match color {
                    'b' => rectangle(BLACK, square, transform, gl),
                    'w' => rectangle(WHITE, square, transform, gl),
                    _ => panic!("what the color"),
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs, p: Option<Pixel>) {
        // Rotate 2 radians per second.
        if let Some(p) = p {
            println!("got a pixel! {:?}", p);
            self.map.insert((p.x, p.y), p.color);
        };
    }
}
#[derive(Debug)]
enum Direction {
    L,
    R,
    U,
    D,
}

fn main() {
    let mut map: HashMap<(isize, isize), char> = HashMap::new();
    get_painted_map(&mut map, 0, None);
    let count = map.values().count();
    println!("01. {}", count);
    assert_eq!(count, 2539);

    let mut map2: HashMap<(isize, isize), char> = HashMap::new();

    let (tx, rx): (Sender<Pixel>, Receiver<Pixel>) = mpsc::channel();

    //output(&map2);

    let processing = thread::spawn(move || {
        get_painted_map(&mut map2, 1, Some(tx));
        /*
        let mut rng = rand::thread_rng();
        if rng.gen_range(0, 100) >= 99 {
            let x = rng.gen_range(0, 30);
            let y = rng.gen_range(0, 30);
            let color = 'b';
            let p = Pixel { x, y, color };
            println!("{:?}", p);
            thread::sleep(time::Duration::from_secs(1));
            tx.send(p).unwrap();
        }
        */
    });

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("painted-map", [500, 100])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        map: HashMap::new(),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            let p = rx.try_recv().ok();
            app.update(&args, p);
        }
    }

    processing.join().unwrap();
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

fn write(c: char) {
    print!("{}", c);
}

fn output(map: &HashMap<(isize, isize), char>) {
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

fn get_painted_map(
    map: &mut HashMap<(isize, isize), char>,
    start: usize,
    tx: Option<Sender<Pixel>>,
) {
    let data = get_program();

    let mut robot_state = ((0, 0), Direction::U);
    let mut c = Computer::new(&data, InputMode::Args, vec![start as isize]);

    while !c.is_terminated() {
        c.execute();

        match c.get_output() {
            Some(n) if n == 0 => {
                map.insert(robot_state.0, 'b');
                if let Some(tx) = tx.as_ref() {
                    tx.send(Pixel {
                        x: (robot_state.0).0,
                        y: (robot_state.0).1,
                        color: 'b',
                    });
                    thread::sleep(time::Duration::from_millis(50));
                }
            }
            Some(n) if n == 1 => {
                map.insert(robot_state.0, 'w');
                if let Some(tx) = tx.as_ref() {
                    tx.send(Pixel {
                        x: (robot_state.0).0,
                        y: (robot_state.0).1,
                        color: 'w',
                    });
                    thread::sleep(time::Duration::from_millis(50));
                }
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
}

fn get_program() -> String {
    let mut f = File::open("./data.txt").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    String::from(buffer.trim())
}
