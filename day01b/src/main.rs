struct Replacement(&'static str, &'static str);

const REPLACEMENTS: &[Replacement] = &[
    Replacement("one", "1"),
    Replacement("two", "2"),
    Replacement("three", "3"),
    Replacement("four", "4"),
    Replacement("five", "5"),
    Replacement("six", "6"),
    Replacement("seven", "7"),
    Replacement("eight", "8"),
    Replacement("nine", "9"),
];

fn main() {
    let input = include_str!("../input.txt");

    let res: u32 = input.lines().map(|l| {
        // Need to account for things like "eightwo", which should be considered "82"

        // Tuple is (position, value)
        let mut numbers: Vec<(usize, String)> = vec![];

        for r in REPLACEMENTS {
            // Find the first and last spelled out digits, if there are any
            if let Some(idx) = l.find(r.0) {
                numbers.push((idx, r.1.into()));
            }

            if let Some(idx) = l.rfind(r.0) {
                numbers.push((idx, r.1.into()));
            }
        }

        // Find regular digits
        if let Some(idx) = l.find(char::is_numeric) {
            let num = l.chars().nth(idx).unwrap().to_string();
            numbers.push((idx, num));
        }

        if let Some(idx) = l.rfind(char::is_numeric) {
            let num = l.chars().nth(idx).unwrap().to_string();
            numbers.push((idx, num));
        }

        // Sort the numbers by index
        numbers.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let first_num = numbers.first().unwrap().clone().1;
        let last_num = numbers.last().unwrap().clone().1;

        let calibration = format!("{}{}", first_num, last_num).parse::<u32>().unwrap();
        calibration
    }).sum();

    println!("Result: {}", res);
}
