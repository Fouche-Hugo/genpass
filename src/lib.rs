use std::ops::RangeInclusive;
use rand::seq::SliceRandom;

use rand;

pub mod config;

use rand::Rng;

pub fn generate_password(config: &config::Config) -> String {
    let mut rng = rand::thread_rng();
    let categories = generate_categories(config);
    let password: String = (0..config.size)
        .map(|_| {
            let category: usize = rng.gen_range(0..categories.len());
            rng.gen_range(categories[category].to_owned()) as char
        })
        .collect();
    password
}

pub fn generate_strong_password(config: &config::Config) -> String {
    let mut rng = rand::thread_rng();
    let categories = generate_categories(config);
    // in case of strong password, we need to have at least one character from each category
    let mut password: String = categories
        .iter()
        .map(|category| rng.gen_range(category.to_owned()) as char)
        .collect();

    // the rest of the password is generated randomly
    password.push_str(
        &(0..config.size as usize - categories.len())
            .map(|_| {
                let category: usize = rng.gen_range(0..categories.len());
                rng.gen_range(categories[category].to_owned()) as char
            })
            .collect::<String>(),
    );
    
    // shuffle the password
    let mut password: Vec<char> = password.chars().collect();
    password.shuffle(&mut rng);

    password.into_iter().collect()
}

fn generate_categories(config: &config::Config) -> Vec<RangeInclusive<u8>> {
    let mut categories: Vec<RangeInclusive<u8>> = vec![];

    if !config.without_letters {
        if !config.without_uppercase {
            categories.push(b'A'..=b'Z');
        }

        if !config.without_lowercase {
            categories.push(b'a'..=b'z');
        }
    }

    if !config.without_symbols {
        categories.push(b'!'..=b'/');
    }

    if !config.without_numbers {
        categories.push(b'0'..=b'9');
    }

    categories
}
