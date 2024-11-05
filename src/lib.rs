use ball::{collide_balls, Ball};
use piston_window::*;

pub mod ball;

pub fn render() {
    let mut window: PistonWindow = WindowSettings::new("Bouncing Balls", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut balls: Vec<Ball> = (0..10).map(|_| Ball::default()).collect();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);

            balls.iter().for_each(|b| b.draw(c, g));
        });

        // Update ball positions and velocities
        balls.iter_mut().for_each(|b| {
            b.update();
        });

        let mut indices_to_update: Vec<(usize, usize)> = Vec::new();
        (0..balls.len() - 1).for_each(|i| {
            (i..balls.len() - 1).for_each(|ball_i| {
                let b1 = balls.get(i).unwrap();
                let other = balls.get(ball_i).unwrap();
                if b1.check_collision(other) {
                    indices_to_update.push((i, ball_i));
                }
            });
        });

        indices_to_update.iter().for_each(|(i, other_i)| {
            collide_balls(&mut balls, *i, *other_i);
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
        })
    }
}
