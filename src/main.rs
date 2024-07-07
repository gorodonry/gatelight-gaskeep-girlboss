use itertools::Itertools;
use std::collections::HashSet;

const PARTICLES: [&str; 6] = ["gas", "light", "gate", "keep", "girl", "boss"];

fn main() {
    let combinations = PARTICLES.iter()
        .permutations(2)
        .permutations(3)
        .filter(|i| i.iter().flatten().collect::<HashSet<_>>().len() == 6)
        .collect::<Vec<_>>();

    println!("{:?}", combinations);
    println!("{}", combinations.len());

    for c in combinations.iter() {
        println!("{}{} {}{} {}{}", c[0][0], c[0][1], c[1][0], c[1][1], c[2][0], c[2][1]);
    }
}
