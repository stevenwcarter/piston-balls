use bouncing_balls::render;

fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    render();
}
