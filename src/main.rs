use opengl_graphics::GlGraphics;
use piston_window::*;
use std::env;

use bouncing_balls::{config::CONFIG, App};

fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let opengl = OpenGL::V3_2;

    let args: Vec<String> = env::args().collect();
    let count = match args.get(1) {
        Some(arg) => Some(arg.parse::<usize>().unwrap_or(CONFIG.ball_count)),
        _ => Some(CONFIG.ball_count),
    };

    let mut window: PistonWindow =
        WindowSettings::new("Bouncing Balls", [CONFIG.width, CONFIG.height])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut app = App::new(GlGraphics::new(opengl), count);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
    }
}
