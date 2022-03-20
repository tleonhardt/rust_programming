#![warn(clippy::all, clippy::pedantic)]

fn main() {
    if let Err(e) = minigrep::get_args().and_then(minigrep::run) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
