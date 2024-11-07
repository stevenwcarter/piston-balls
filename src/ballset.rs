use crate::ball::{collide_balls, Ball};

pub struct BallSet {
    pub balls: Vec<Ball>,
}

impl BallSet {
    pub fn new(ball_count: Option<usize>) -> Self {
        let balls: Vec<Ball> = (0..ball_count.unwrap_or(5))
            .map(|_| Ball::default())
            .collect();

        Self { balls }
    }
    pub fn balls(&self) -> &[Ball] {
        &self.balls
    }
    pub fn update_loop(&mut self) {
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
        });

        self.balls.iter_mut().for_each(|ball| {
            ball.check_bounds();
        });
    }
    pub fn find_colliding_balls(&self) -> Vec<usize> {
        (0..self.balls.len())
            .flat_map(|i| {
                (i + 1..self.balls.len())
                    .filter(|ball_i| {
                        let b1 = self.balls.get(i).unwrap();
                        let other = self.balls.get(*ball_i).unwrap();
                        b1.check_collision(other)
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
    pub fn find_balls_colliding_with_ball(&self, search_index: usize) -> Vec<usize> {
        (search_index + 1..self.balls.len())
            .filter(|ball_i| {
                let b1 = self.balls.get(search_index).unwrap();
                let other = self.balls.get(*ball_i).unwrap();
                b1.check_collision(other)
            })
            .collect::<Vec<_>>()
    }
}
