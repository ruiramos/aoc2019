use day_13::vis::{App, Pixel};
use day_13::{Computer, InputMode};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::rc::Rc;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use opengl_graphics::GlyphCache;
use opengl_graphics::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use sdl2_window::Sdl2Window;

fn main() {
    run_first();

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let size = [500, 300];
    let ref mut window: Sdl2Window = WindowSettings::new("opengl_graphics", size)
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let (tx, rx): (Sender<Pixel>, Receiver<Pixel>) = mpsc::channel();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        map: HashMap::new(),
        glyph: GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new()).unwrap(),
        score: 0,
    };

    let processing = thread::spawn(move || {
        run_second(tx);
    });

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(window) {
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

fn run_second(tx: Sender<Pixel>) {
    let mut data = get_program();
    data.replace_range(0..1, "2");

    let mut computer = Computer::new(&data, InputMode::Args, vec![]);

    let ball_x = Rc::new(RefCell::new(0));
    let paddle_x = Rc::new(RefCell::new(0));

    let mut instruction = vec![];

    while !computer.is_terminated() {
        computer.execute(
            //output fn
            &mut |output| {
                instruction.push(output);
                if instruction.len() == 3 {
                    let (x, y, op) = (instruction[0], instruction[1], instruction[2]);

                    tx.send(Pixel::new(x, y, op as usize)).unwrap();

                    if op == 3 {
                        *paddle_x.borrow_mut() = x;
                    } else if op == 4 {
                        *ball_x.borrow_mut() = x;
                    } else if x == -1 {
                        println!("score: {}", op);
                    }

                    instruction = vec![];
                }
            },
            // input fn
            &mut || -> isize {
                /* play via stdin
                let stdin = io::stdin();
                let line1 = stdin.lock().lines().next().unwrap().unwrap();
                print!("> ");
                io::stdout().flush();
                return line1.parse().expect("Expected a number");
                */

                let bx = *ball_x.borrow();
                let px = *paddle_x.borrow();
                if px > bx {
                    -1
                } else if px < bx {
                    1
                } else {
                    0
                }
            },
        );
    }
}

fn output(inst: &HashMap<(isize, isize), usize>) {
    println!("score {:?}", inst.get(&(-1, 0)));
    println!("blocks {:?}", inst.values().filter(|v| *v == &2).count());
    for y in 0..30 {
        for x in 0..80 {
            let cell = inst.get(&(x, y));
            let output = match cell {
                None => ' ',
                Some(n) if *n == 0 => ' ',
                Some(n) if *n == 1 => '#',
                Some(n) if *n == 2 => 'x',
                Some(n) if *n == 3 => '_',
                Some(n) if *n == 4 => 'o',
                _ => {
                    println!("{:?}", cell);
                    panic!("unknown cell type");
                }
            };
            print!("{}", output);
        }
        print!("\n");
    }
}

fn run_first() {
    let mut inst: HashMap<usize, usize> = HashMap::new();
    let mut instruction = vec![];

    let mut computer = Computer::new(&get_program(), InputMode::Args, vec![]);

    while !computer.is_terminated() {
        computer.execute(
            //output fn
            &mut |output| {
                instruction.push(output);
                if instruction.len() == 3 {
                    let (x, y, op) = (instruction[0], instruction[1], instruction[2]);

                    *inst.entry(op as usize).or_insert(0) += 1;

                    instruction = vec![];
                }
            },
            // input fn
            &mut || -> isize { 0 },
        );
    }

    println!("{:?}", inst.get(&2));
}

fn get_program() -> String {
    let mut f = File::open("./data.txt").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    String::from(buffer.trim())
}
