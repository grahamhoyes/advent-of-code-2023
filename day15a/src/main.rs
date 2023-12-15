fn solution(input: &str) -> usize {
    input
        .split(',')
        .map(|x| {
            x.as_bytes()
                .iter()
                .fold(0, |hash: usize, char| ((hash + *char as usize) * 17) % 256)
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let res = solution(input);

    println!("Result: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 1320);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 511257);
    }
}
