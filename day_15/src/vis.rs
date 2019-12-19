use crate::PositionType;
use std::collections::HashMap;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use opengl_graphics::GlGraphics;
use opengl_graphics::GlyphCache;
use piston::input::{RenderArgs, UpdateArgs};

pub struct App<'a> {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub map: HashMap<(isize, isize), usize>,
    pub glyph: GlyphCache<'a>,
    pub position: Option<(isize, isize)>,
}

impl<'a> App<'a> {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const ROBOT: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const BLOCK: [f32; 4] = [0.8, 0.7, 0.8, 1.0];
        const BG: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
        const GRAY: [f32; 4] = [0.9, 0.9, 0.9, 1.0];

        let map = &self.map;
        let gc = &mut self.glyph;
        let position = &self.position;
        let (max_x, max_y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let transform = c.transform.trans(0., 0.);

            for (coords, pos) in map.iter() {
                let square = rectangle::square(coords.0 as f64 * 10., coords.1 as f64 * 10., 10.);
                match pos {
                    0 => rectangle(BLOCK, square, transform, gl),
                    1 => rectangle(WHITE, square, transform, gl),
                    2 => rectangle(GREEN, square, transform, gl),
                    _ => panic!("what the color"),
                }
            }

            if let Some((x, y)) = position {
                let square = rectangle::square(*x as f64 * 10., *y as f64 * 10., 10.);
                rectangle(ROBOT, square, transform, gl);
            };
        });
    }

    pub fn update(
        &mut self,
        args: &UpdateArgs,
        map: HashMap<(isize, isize), usize>,
        p: Option<(isize, isize)>,
    ) {
        self.map = map;
        self.position = p;
    }
}
