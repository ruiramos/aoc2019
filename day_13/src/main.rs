use day_13::vis::{App, Pixel};
use day_13::{Computer, InputMode};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
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

    processing.join().unwrap();

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
}

fn run_second(tx: Sender<Pixel>) {
    let mut data = get_program();
    data.replace_range(0..1, "2");

    //let mut inst: HashMap<(isize, isize), usize> = HashMap::new();

    let mut computer = Computer::new(&data, InputMode::Args, vec![]);

    let mut ball_x = None;
    let mut paddle_x = None;

    while !computer.is_terminated() {
        computer.execute();
        let x = computer.get_output().unwrap();
        computer.execute();
        let y = computer.get_output().unwrap();
        computer.execute();
        let op = computer.get_output().unwrap();

        tx.send(Pixel::new(x, y, op as usize)).unwrap();

        if op == 3 {
            paddle_x = Some(x);
        } else if op == 4 {
            ball_x = Some(x);
        }

        if paddle_x.is_some() && ball_x.is_some() {
            let bx = ball_x.unwrap();
            let px = paddle_x.unwrap();
            if px > bx {
                computer.set_input(-1);
            } else if px < bx {
                computer.set_input(1);
            } else {
                computer.set_input(0);
            }
        } else {
            computer.set_input(0);
        }

        //inst.insert((x, y), op as usize);

        /*
        if inst.values().find(|op| **op == 4).is_some()
            && inst.values().find(|op| **op == 3).is_some()
        {
            output(&inst);
        }
        */
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

    let mut computer = Computer::new(&get_program(), InputMode::Args, vec![]);

    loop {
        computer.execute();
        let x = computer.get_output();
        computer.execute();
        let y = computer.get_output();
        computer.execute();
        let op = computer.get_output();

        *inst.entry(op.unwrap() as usize).or_insert(0) += 1;

        if computer.is_terminated() {
            break;
        }
    }

    println!("{:?}", inst.get(&2));
}

fn get_program() -> String {
    let mut f = File::open("./data.txt").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    String::from(buffer.trim())
}
