extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct App {
    canvas: GlGraphics, // OpenGL drawing backend.
    rate: f64, // rate to increase multiplication factor
    npoints: u32, // number of points in the circle
    circle: Vec<[f64; 4]>,
    lines: Vec<(u32, u32)>,
    factor: f64,
}

impl App {

    // colors
    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 0.0];
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    const BLUE:  [f32; 4] = [0.0, 0.0, 1.0, 1.0];

    fn new(canvas: GlGraphics, rate: f64, npoints: u32) -> App {
        App {
            canvas: canvas,
            rate: rate,
            npoints: npoints,
            circle: Vec::new(),
            lines: Vec::new(),
            factor: 0.0,
        }
    }

    fn make_circle(&self, inc: f64, radius: f64, offset: f64) -> Vec<[f64; 4]> {
        let mut ret = Vec::new();
        for i in 0..(self.npoints) {
            let i = i as f64;
            ret.push([radius * f64::sin(i * inc) + offset, radius * f64::cos(i * inc) + offset, 15.0, 15.0]);
        }
        ret
    }

    fn make_lines(&mut self) {
        let mut ret = Vec::new();
        for a in 1..self.npoints {
            // a -> (a * factor) % n
            ret.push((a, (f64::round((a as f64) * (self.factor)) as u32) % self.npoints));
        }
        self.lines = ret;
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let inc = 6.28 / (self.npoints as f64);
        let radius = 700.0;
        let offset = 800.0;
        self.circle = self.make_circle(inc, radius, offset);
        let circle_points = &self.circle;
        let lines = &self.lines;
        let factor = &self.factor;

        self.canvas.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(App::WHITE, gl);

            // render points
            for p in circle_points {
                ellipse(App::BLACK, *p, c.transform, gl);
            }

            // draw cursor
            ellipse(App::BLUE, [(radius + 20.) * f64::sin(factor * inc) + offset, (radius + 20.) * f64::cos(factor * inc) + offset, 15.0, 15.0], c.transform, gl);

            // render lines
            for pair in lines {
                let p1 = circle_points[pair.0 as usize];
                let p2 = circle_points[pair.1 as usize];
                line(App::BLUE, 1.0, [p1[0] + (p1[2] / 2.), p1[1] + (p1[3] / 2.), p2[0] + (p2[2] / 2.), p2[1] + (p2[3] / 2.)], c.transform, gl);
            }

        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.factor += self.rate * (args.dt);
        self.factor %= self.npoints as f64;
        self.make_lines();
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "Times Table",
            [1600, 1600]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(GlGraphics::new(opengl), 1.0, 200);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
