use itertools::Itertools;
use std::collections::HashSet;
use rand::seq::SliceRandom;
use std::io;

const PARTICLES: [&str; 6] = ["gas", "light", "gate", "keep", "girl", "boss"];

fn main() {
    let combinations = PARTICLES.iter()
        .permutations(2)
        .permutations(3)
        .filter(|i| i.iter().flatten().collect::<HashSet<_>>().len() == 6)
        .collect::<Vec<_>>();

    loop {
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        println!(">>> {}", format_combination(random_combination(&combinations)));
    }
}

fn random_combination<'a>(combination: &'a Vec<Vec<Vec<&&'a str>>>) -> &'a Vec<Vec<&'a &'a str>> {
    combination.choose(&mut rand::thread_rng()).unwrap()
}

fn format_combination(c: &Vec<Vec<&&str>>) -> String {
    format!("{}{} {}{} {}{}", c[0][0], c[0][1], c[1][0], c[1][1], c[2][0], c[2][1])
}
