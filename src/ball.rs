use piston_window::*;
use rand::Rng;

fn random_color() -> types::Color {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0.0..1.0);
    let g = rng.gen_range(0.0..1.0);
    let b = rng.gen_range(0.0..1.0);

    [r, g, b, 1.0]
}

#[derive(Debug, Clone, Copy)]
pub struct Ball {
    x: f64,              // x coordinate
    y: f64,              // y coordinate
    vx: f64,             // velocity in x direction
    vy: f64,             // velocity in y direction
    radius: f64,         // radius of circle
    color: types::Color, // color of circle
}
impl Default for Ball {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let color = random_color();
        let x: f64 = rng.gen_range(50.0..600.0);
        let y: f64 = rng.gen_range(50.0..380.0);
        let vx: f64 = rng.gen_range(0.5..3.0);
        let vy: f64 = rng.gen_range(0.5..3.0);
        let radius: f64 = rng.gen_range(10.0..30.0);

        Self {
            x,
            y,
            vx,
            vy,
            radius,
            color,
        }
    }
}
impl Ball {
    pub fn draw(&self, c: Context, g: &mut G2d) {
        ellipse(
            self.color,
            graphics::ellipse::circle(self.x, self.y, self.radius),
            c.transform,
            g,
        );
    }
    pub fn update(&mut self) {
        self.x += self.vx;
        self.y += self.vy;

        if self.x - self.radius < 0.0 || self.x + self.radius > 640.0 {
            self.vx *= -1.0;
        }
        if self.y - self.radius < 0.0 || self.y + self.radius > 480.0 {
            self.vy *= -1.0;
        }
    }

    pub fn check_collision(&self, other: &Ball) -> bool {
        let distance = f64::powi(self.x - other.x, 2) + f64::powi(self.y - other.y, 2);
        let radii = f64::powi(self.radius + other.radius, 2);

        distance <= radii
    }
}

pub fn collide(ball1: &mut Ball, ball2: &mut Ball) {
    // Calculate the difference in x and y coordinates
    let dx = ball2.x - ball1.x;
    let dy = ball2.y - ball1.y;

    // Calculate the distance and ensure we avoid division by zero
    let distance = (dx * dx + dy * dy).sqrt();
    if distance == 0.0 {
        return;
    }

    // Calculate the minimum distance needed to avoid overlap
    let min_distance = ball1.radius + ball2.radius;

    // Check if balls are overlapping
    if distance < min_distance {
        // Calculate the overlap amount
        let overlap = min_distance - distance;

        // Move each ball along the collision normal by half the overlap amount
        let correction_x = dx / distance * overlap / 2.0;
        let correction_y = dy / distance * overlap / 2.0;

        ball1.x -= correction_x;
        ball1.y -= correction_y;
        ball2.x += correction_x;
        ball2.y += correction_y;
    }

    // Calculate the angle of the collision normal
    let collision_angle = dy.atan2(dx);

    // Calculate the speed and direction of each ball
    let speed1 = (ball1.vx * ball1.vx + ball1.vy * ball1.vy).sqrt();
    let speed2 = (ball2.vx * ball2.vx + ball2.vy * ball2.vy).sqrt();

    let direction1 = ball1.vy.atan2(ball1.vx);
    let direction2 = ball2.vy.atan2(ball2.vx);

    // Decompose the velocities into components along the collision normal
    let velocity1_normal = speed1 * (direction1 - collision_angle).cos();
    let velocity2_normal = speed2 * (direction2 - collision_angle).cos();

    // Swap the normal components (elastic collision response)
    let new_velocity1_normal = velocity2_normal;
    let new_velocity2_normal = velocity1_normal;

    // Calculate the final velocity components for each ball after the collision
    let velocity1_x = speed1
        * (direction1 - collision_angle).sin()
        * (collision_angle + std::f64::consts::PI / 2.0).cos()
        + new_velocity1_normal * collision_angle.cos();
    let velocity1_y = speed1
        * (direction1 - collision_angle).sin()
        * (collision_angle + std::f64::consts::PI / 2.0).sin()
        + new_velocity1_normal * collision_angle.sin();

    let velocity2_x = speed2
        * (direction2 - collision_angle).sin()
        * (collision_angle + std::f64::consts::PI / 2.0).cos()
        + new_velocity2_normal * collision_angle.cos();
    let velocity2_y = speed2
        * (direction2 - collision_angle).sin()
        * (collision_angle + std::f64::consts::PI / 2.0).sin()
        + new_velocity2_normal * collision_angle.sin();

    // Update the velocities of each ball
    ball1.vx = velocity1_x;
    ball1.vy = velocity1_y;
    ball2.vx = velocity2_x;
    ball2.vy = velocity2_y;
}

pub fn collide_balls(balls: &mut [Ball], index1: usize, index2: usize) {
    if index1 == index2 {
        return;
        // panic!("Cannot get mutable references to the same element twice");
    }

    // Ensure index1 < index2 for safe split
    let (index1, index2) = if index1 < index2 {
        (index1, index2)
    } else {
        (index2, index1)
    };

    // Split at the second index, then split the first part at the first index
    let (left, right) = balls.split_at_mut(index2);
    let ball1 = &mut left[index1];
    let ball2 = &mut right[0]; // index2 is 0 in the second part of the slice

    // Now we can safely call the collide function with two distinct mutable references
    collide(ball1, ball2);
}