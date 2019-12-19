use day_15::computer::*;
use day_15::stateless_computer::*;
use day_15::vis::App;
use day_15::ComputerState;
use day_15::PositionType;
use rand::Rng;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::GlyphCache;
use opengl_graphics::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

type Map = HashMap<(isize, isize), usize>;

fn main() {
    let program = get_program()
        .split(',')
        .enumerate()
        .fold(HashMap::new(), |mut map, (i, s)| {
            map.insert(i, s.parse().unwrap());
            map
        });

    let i = 0;
    let r = 0;
    let pos = (50, 50);
    let mut map: Map = HashMap::new();
    roam(program, i, r, pos, &mut map, 1);

    let goal = (30, 64);
    let mut memory: HashMap<(isize, isize), usize> = HashMap::new();
    let result = get_max_dist(&mut map, goal, &mut memory, 0);

    println!("{}", result);

    //println!("{:?}", map);
    init_graphics(map.clone());
}

fn get_max_dist(
    map: &mut Map,
    position: (isize, isize),
    memory: &mut HashMap<(isize, isize), usize>,
    dist: usize,
) -> usize {
    let positions = get_adjacent(&position)
        .into_iter()
        .filter(|pos| map.get(pos).unwrap() == &1 && memory.get(pos).is_none())
        .collect::<Vec<(isize, isize)>>();

    usize::max(
        positions.iter().fold(0, |acc, p| {
            memory.insert(*p, 1);
            usize::max(get_max_dist(map, *p, memory, dist + 1), acc)
        }),
        dist,
    )
}

fn get_unexplored_directions(position: &(isize, isize), map: &Map) -> Vec<usize> {
    let directions = get_adjacent(position);
    directions
        .iter()
        .enumerate()
        .flat_map(|(i, pos)| match map.get(pos) {
            None => vec![i + 1],
            _ => vec![],
        })
        .collect::<Vec<usize>>()
}

fn get_adjacent(position: &(isize, isize)) -> Vec<(isize, isize)> {
    (1..=4)
        .collect::<Vec<usize>>()
        .iter()
        .map(|d| add_position_direction(position, *d))
        .collect::<Vec<(isize, isize)>>()
}

fn add_position_direction(position: &(isize, isize), direction: usize) -> (isize, isize) {
    match direction {
        1 => (position.0, position.1 - 1),
        2 => (position.0, position.1 + 1),
        3 => (position.0 - 1, position.1),
        4 => (position.0 + 1, position.1),
        _ => panic!("wrong direction provided"),
    }
}

fn roam(program: Program, i: usize, r: isize, position: (isize, isize), map: &mut Map, it: usize) {
    let directions = get_unexplored_directions(&position, map);

    let results = directions
        .iter()
        .map(|d| go_direction(&mut program.clone(), i, r, Some(*d as isize)))
        .collect::<Vec<(isize, ComputerState, Program, usize, isize)>>();

    for (i, result) in results.into_iter().enumerate() {
        let direction = directions[i];
        let (output, state, program, new_i, new_r) = result;

        match output {
            1 => {
                let new_position = add_position_direction(&position, direction);
                map.insert(new_position, 1);
                //println!("exploring {:?}", new_position);
                roam(program, new_i, new_r, new_position, map, it + 1);
            }
            2 => {
                let new_position = add_position_direction(&position, direction);
                map.insert(new_position, 2);
                println!(" - > found it {:?} {}", new_position, it);
                roam(program, new_i, new_r, new_position, map, it + 1);
            }
            0 => {
                let new_position = add_position_direction(&position, direction);
                map.insert(new_position, 0);
                //println!("wall at {:?}", new_position);
                // wall
            }
            _ => panic!("wrong result?"),
        };
    }
}

fn go_direction(
    program: &mut Program,
    i: usize,
    r: isize,
    input: Option<isize>,
) -> (isize, ComputerState, Program, usize, isize) {
    let (output, state, program, i, r) = StatelessComputer::execute(program, i, r, input);
    if output.is_none() {
        go_direction(program, i, r, input)
    } else {
        (output.unwrap(), state, program.clone(), i, r)
    }
}

fn init_graphics(map: Map) {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let size = [1000, 1000];
    let ref mut window: Window = WindowSettings::new("opengl_graphics", size)
        .graphics_api(opengl)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        map: map,
        glyph: GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new()).unwrap(),
        position: None,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            //let p = rx.try_recv().ok().unwrap();
            //app.update(&args, map, None);
        }
    }
}

fn get_program() -> String {
    let mut f = File::open("./data.txt").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    String::from(buffer.trim())
}
