use std::collections::HashMap;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use opengl_graphics::GlGraphics;
use opengl_graphics::GlyphCache;
use piston::input::{RenderArgs, UpdateArgs};

#[derive(Debug)]
pub struct Pixel {
    x: isize,
    y: isize,
    op: usize,
}

impl Pixel {
    pub fn new(x: isize, y: isize, op: usize) -> Pixel {
        Pixel { x, y, op }
    }
}

pub struct App<'a> {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub map: HashMap<(isize, isize), usize>,
    pub glyph: GlyphCache<'a>,
    pub score: usize,
}

impl<'a> App<'a> {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLOCK: [f32; 4] = [0.8, 0.7, 0.8, 1.0];
        const BG: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
        const GRAY: [f32; 4] = [0.9, 0.9, 0.9, 1.0];

        let map = &self.map;
        let gc = &mut self.glyph;
        let score = &mut self.score;
        let (max_x, max_y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            text(
                WHITE,
                20,
                &format!("Score: {}", score),
                gc,
                c.transform.trans(20., 25.),
                gl,
            )
            .expect("Error rendering text");

            for (coords, op) in map.iter() {
                if coords == &(-1, 0) {
                    *score = *op;
                    continue;
                }

                let transform = c.transform.trans(20., 35.);
                let square = rectangle::square(coords.0 as f64 * 10., coords.1 as f64 * 10., 10.);

                match op {
                    0 => {
                        rectangle(BLACK, square, transform, gl);
                    }
                    1 => {
                        rectangle(GRAY, square, transform, gl);
                    }
                    2 => rectangle(BLOCK, square, transform, gl),
                    4 => {
                        ellipse(
                            WHITE,
                            [
                                coords.0 as f64 * 10. + 2.,
                                coords.1 as f64 * 10. + 2.,
                                6.,
                                6.,
                            ],
                            transform,
                            gl,
                        );
                    }
                    3 => {
                        rectangle(
                            RED,
                            [coords.0 as f64 * 10., coords.1 as f64 * 10. + 6., 10., 4.],
                            transform,
                            gl,
                        );
                    }
                    _ => panic!("what the color"),
                }
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs, p: Option<Pixel>) {
        // Rotate 2 radians per second.
        if let Some(p) = p {
            self.map.insert((p.x, p.y), p.op);
        };
    }
}
