use regex::Regex;

struct Draw {
    r: usize,
    g: usize,
    b: usize,
}

fn solution(input: &str) -> usize {
    let red_re = Regex::new(r"(\d+) red").unwrap();
    let green_re = Regex::new(r"(\d+) green").unwrap();
    let blue_re = Regex::new(r"(\d+) blue").unwrap();

    input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(':').collect();

            parts[1]
                .trim()
                .split(';')
                .map(|text| {
                    let r = red_re.captures(text).map_or(0, |c| c[1].parse().unwrap());
                    let g = green_re.captures(text).map_or(0, |c| c[1].parse().unwrap());
                    let b = blue_re.captures(text).map_or(0, |c| c[1].parse().unwrap());

                    Draw { r, g, b }
                })
                .collect::<Vec<Draw>>()
        })
        .map(|draws| {
            // Figure out the minimum number of cubes required to play each game
            let min_r = draws.iter().map(|d| d.r).max().unwrap();
            let min_g = draws.iter().map(|d| d.g).max().unwrap();
            let min_b = draws.iter().map(|d| d.b).max().unwrap();

            // Compute the power of the game
            min_r * min_g * min_b
        })
        .sum()
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

    assert_eq!(res, 70950);
}
