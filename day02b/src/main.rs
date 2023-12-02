use std::collections::HashMap;
use regex::Regex;

struct Draw {
    r: usize,
    g: usize,
    b: usize
}

fn main() {
    let input = include_str!("../input.txt");

    let mut games: HashMap<usize, Vec<Draw>> = HashMap::new();

    let red_re = Regex::new(r"(\d+) red").unwrap();
    let green_re = Regex::new(r"(\d+) green").unwrap();
    let blue_re = Regex::new(r"(\d+) blue").unwrap();

    for l in input.lines() {
        let parts: Vec<&str> = l.split(':').collect();
        let game_id: usize = parts[0][5..].parse().unwrap();

        let draws: Vec<Draw> = parts[1].trim().split(';').map(|text| {
            let r = red_re.captures(text).map_or(0, |c |c[1].parse().unwrap());
            let g = green_re.captures(text).map_or(0, |c| c[1].parse().unwrap());
            let b = blue_re.captures(text).map_or(0, |c| c[1].parse().unwrap());

            Draw {
                r,
                g,
                b
            }
        }).collect();

        games.insert(game_id, draws);
    }

    let res: usize = games.values().map(|draws| {
        // Figure out the minimum number of cubes required to play each game
        let min_r = draws.iter().map(|d| d.r).max().unwrap();
        let min_g = draws.iter().map(|d| d.g).max().unwrap();
        let min_b = draws.iter().map(|d| d.b).max().unwrap();

        // Compute the power of th game
        min_r * min_g * min_b
    }).sum();

    println!("Result: {}", res);
}
