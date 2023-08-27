use std::env;

use genpass::config::Config;
use genpass::{generate_password, generate_strong_password};

fn main() {
    let config = Config::build(env::args().collect()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    let password = if config.strong {
        generate_strong_password(&config)
    } else {
        generate_password(&config)
    };

    println!("{}", password);
}
