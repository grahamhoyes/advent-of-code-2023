#[derive(Debug)]
struct Range {
    dest_start: usize,
    source_start: usize,
    length: usize,
}

fn solution(input: &str) -> usize {
    // Most of the input looks like:
    // <something> map:
    // x y z
    // \n
    //
    // The first line is special, and is seeds: <seeds>
    //
    // Start by splitting on lines that end in "map:". Note that by doing this,
    // the title of the map will actually be in the previous section, but we don't
    // need it.

    let mut sections = input.split(" map:\n");

    // Get the seeds out of the first line of the first section
    let seeds = sections.next().unwrap().lines().next().unwrap()[7..]
        .split(' ')
        .filter_map(|val| val.parse::<usize>().ok())
        .collect::<Vec<_>>();

    // Build maps from the rest of the sections (the sections iterator is already on
    // the second element)
    let maps = sections
        .map(|section| {
            section
                .lines()
                // Keep only the lines that contain numbers
                .filter(|line| !line.is_empty() && line.starts_with(char::is_numeric))
                .map(|line| {
                    let mut nums = line.split(' ').filter_map(|val| val.parse::<usize>().ok());

                    Range {
                        dest_start: nums.next().unwrap(),
                        source_start: nums.next().unwrap(),
                        length: nums.next().unwrap(),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    seeds
        .into_iter()
        .map(|seed| {
            let mut source_val = seed;

            // Go through each map, find where it goes to
            for map in maps.iter() {
                source_val = map
                    .iter()
                    .find_map(|range| {
                        if source_val >= range.source_start
                            && source_val < range.source_start + range.length
                        {
                            // Found our matching range, so update source_val for the next map
                            let offset = source_val - range.source_start;
                            return Some(range.dest_start + offset);
                        }
                        None
                    })
                    .unwrap_or(source_val);
            }

            source_val
        })
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("../input.txt");
    let res = solution(input);

    println!("Result: {}", res);
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");
    let res = solution(input);

    assert_eq!(res, 35);
}

#[test]
fn test_input() {
    let input = include_str!("../input.txt");
    let res = solution(input);

    assert_eq!(res, 322500873);
}
