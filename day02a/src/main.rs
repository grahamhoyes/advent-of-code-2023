use regex::Regex;
use std::collections::HashMap;

struct Draw {
    r: usize,
    g: usize,
    b: usize,
}

fn solution(input: &str) -> usize {
    let mut games: HashMap<usize, Vec<Draw>> = HashMap::new();

    let red_re = Regex::new(r"(\d+) red").unwrap();
    let green_re = Regex::new(r"(\d+) green").unwrap();
    let blue_re = Regex::new(r"(\d+) blue").unwrap();

    for l in input.lines() {
        let parts: Vec<&str> = l.split(':').collect();
        let game_id: usize = parts[0][5..].parse().unwrap();

        let draws: Vec<Draw> = parts[1]
            .trim()
            .split(';')
            .map(|text| {
                let r = red_re.captures(text).map_or(0, |c| c[1].parse().unwrap());
                let g = green_re.captures(text).map_or(0, |c| c[1].parse().unwrap());
                let b = blue_re.captures(text).map_or(0, |c| c[1].parse().unwrap());

                Draw { r, g, b }
            })
            .collect();

        games.insert(game_id, draws);
    }

    // Possible games with 12 red, 13 green, and 14 blue
    let possible_games = games.iter().filter_map(|(k, draws)| {
        if draws.iter().all(|d| d.r <= 12 && d.g <= 13 && d.b <= 14) {
            Some(k)
        } else {
            None
        }
    });

    possible_games.sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let res = solution(input);

    println!("Result: {}", res);
}

#[test]
fn test_solution() {
    let input = include_str!("../input.txt");
    let res = solution(input);

    assert_eq!(res, 2076);
}
