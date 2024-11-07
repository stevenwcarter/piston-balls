use opengl_graphics::GlGraphics;
use piston_window::*;

pub mod ball;
mod ballset;
pub mod config;

pub(crate) use ballset::BallSet;

pub struct App {
    pub gl: GlGraphics,
    pub ballset: BallSet,
}

impl App {
    pub fn new(gl: GlGraphics, ball_count: Option<usize>) -> Self {
        let ballset = BallSet::new(ball_count);

        Self { ballset, gl }
    }
}

impl App {
    pub fn render(&mut self, e: &RenderArgs) {
        self.gl.draw(e.viewport(), |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            self.ballset.balls().iter().for_each(|b| b.draw(c, g));
        });

        self.ballset.update_loop();
    }
}
