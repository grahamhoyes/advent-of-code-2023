fn main() {
    let input = include_str!("../input.txt");

    let res: u32 = input
        .lines()
        .map(|l| {
            let first_num_idx = l.find(char::is_numeric).unwrap();
            let first_num = l.chars().nth(first_num_idx).unwrap();

            let last_num_idx = l.rfind(char::is_numeric).unwrap();
            let last_num = l.chars().nth(last_num_idx).unwrap();

            let calibration: u32 = format!("{}{}", first_num, last_num).parse().unwrap();
            calibration
        })
        .sum();

    println!("Result: {}", res);
}
