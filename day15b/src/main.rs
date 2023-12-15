struct Lens {
    label: String,
    focal_length: usize,
}

fn hash(text: &str) -> usize {
    text.as_bytes()
        .iter()
        .fold(0, |hash: usize, char| ((hash + *char as usize) * 17) % 256)
}

fn solution(input: &str) -> usize {
    let mut boxes: Vec<Vec<Lens>> = (0..256).map(|_| Vec::new()).collect();

    input.split(',').for_each(|text| {
        let label: String = text.chars().take_while(|c| c.is_alphabetic()).collect();
        let box_idx = hash(&label);

        if text.ends_with('-') {
            boxes[box_idx].retain(|lens| lens.label != label);
        } else {
            let focal_length = text
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse()
                .unwrap();

            let mut found = false;
            for lens in boxes[box_idx].iter_mut() {
                if lens.label == label {
                    lens.focal_length = focal_length;
                    found = true;
                    break;
                }
            }

            if !found {
                boxes[box_idx].push(Lens {
                    label,
                    focal_length,
                });
            }
        }
    });

    boxes
        .iter()
        .enumerate()
        .flat_map(|(i, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(move |(j, lens)| (i + 1) * (j + 1) * lens.focal_length)
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

        assert_eq!(res, 145);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 239484);
    }
}
