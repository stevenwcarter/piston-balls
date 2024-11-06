use ball::{collide_balls, Ball};
use opengl_graphics::GlGraphics;
use piston_window::*;

pub mod ball;
pub mod config;

pub struct App {
    pub gl: GlGraphics,
    pub balls: Vec<Ball>,
}

impl App {
    pub fn new(gl: GlGraphics, ball_count: Option<usize>) -> Self {
        let balls: Vec<Ball> = (0..ball_count.unwrap_or(5))
            .map(|_| Ball::default())
            .collect();

        Self { balls, gl }
    }
}

impl App {
    pub fn render(&mut self, e: &RenderArgs) {
        self.gl.draw(e.viewport(), |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            self.balls.iter().for_each(|b| b.draw(c, g));
        });

        self.balls.sort_by(|a, b| b.y.partial_cmp(&a.y).unwrap());

        // Update ball positions and velocities
        self.balls.iter_mut().for_each(|b| {
            b.update();
        });

        let mut indices_to_update: Vec<(usize, usize)> = Vec::new();
        (0..self.balls.len()).for_each(|i| {
            (i + 1..self.balls.len()).for_each(|ball_i| {
                let b1 = self.balls.get(i).unwrap();
                let other = self.balls.get(ball_i).unwrap();
                if b1.check_collision(other) {
                    indices_to_update.push((i, ball_i));
                }
            });
        });

        indices_to_update.iter().for_each(|(i, other_i)| {
            collide_balls(&mut self.balls, *i, *other_i);
            // let b1_copy = *balls.get(*i).unwrap();
            // {
            //     let b2 = *balls.get(*other_i).unwrap();
            //     let b1 = balls.get_mut(*i).unwrap();

            //     b1.handle_collision(&b2);
            // }
            // {
            //     let b2 = balls.get_mut(*other_i).unwrap();

            //     b2.handle_collision(&b1_copy);
            // }
        });

        self.balls.iter_mut().for_each(|ball| {
            ball.check_bounds();
        });
    }
}
