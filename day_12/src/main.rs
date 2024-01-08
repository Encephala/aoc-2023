use itertools::Itertools;

#[derive(Debug, Clone)]
struct Line {
    value: String,
    groups: Vec<usize>,
}

impl From<&str> for Line {
    fn from(input: &str) -> Self {
        let mut parts = input.split_whitespace();

        Line {
            value: parts.next().unwrap().into(),
            groups: parts.next().unwrap().split(",").map(|char| char.parse().expect("Not a number")).collect(),
        }
    }
}

impl Line {
    fn from_expanded(input: &str) -> Self {
        let mut parts = input.split_whitespace();

        let n = 5;

        let value = parts.next().unwrap();
        let groups = parts.next().unwrap().split(",").map(|char| char.parse().expect("Not a number")).collect::<Vec<usize>>();

        let value_repeated = Vec::from([value]).repeat(n).join("?");
        let groups_repeated = groups.repeat(n);

        Line {
            value: value_repeated,
            groups: groups_repeated,
        }
    }
}


// Check if a specific block matches the groups
fn text_matches_groups(block: String, groups: &Vec<usize>) -> bool {
    let mut lengths_of_hash_blocks: Vec<i64> = Vec::new();

    let mut current_char = block.chars().next().unwrap();
    let mut current_start_index: i64 = 0;

    for (i, character) in block.chars().enumerate().skip(1) {
        if character != current_char {
            if current_char == '#' {
                lengths_of_hash_blocks.push(i as i64 - current_start_index)
            }

            current_char = character;
            current_start_index = i as i64;
        }
    }

    // Add final block if it was hashes
    if block.chars().last().unwrap() == '#' {
        lengths_of_hash_blocks.push(block.len() as i64 - current_start_index);
    }


    let result: bool;

    if groups.len() != lengths_of_hash_blocks.len() {
        result = false;
    } else {
        result = groups.iter().zip(lengths_of_hash_blocks).filter(|(a, b)| **a != *b as usize).count() == 0;
    }

    return result
}

fn test() -> isize {
    let i = 0;

    i
}

// Recursive function, only ever goes to depth 2
// This can be done much cleaner but idk if I care to fix it
fn num_arrangements(input: &String, groups: &Vec<usize>) -> usize {
    let blocks: Vec<&str> = input.split(".").filter(|block| block.len() > 0).collect();

    // Base case
    if blocks.len() == 1 {
        let indices_of_unknown = blocks[0].chars().enumerate().filter(|(_, character)| character == &'?').map(|(index, _)| index).collect::<Vec<_>>();

        let mut valid_count: usize = 0;

        // Brute force solution within block hell yeah
        for n in 0..=indices_of_unknown.len() {
            println!("n = {n}");
            for indices in indices_of_unknown.iter().combinations(n) {
                let mut test_string = input.clone();

                for index in indices {
                    test_string.replace_range(index..&(index + 1), "#");
                }

                if text_matches_groups(test_string, groups) {
                    valid_count += 1;
                }
            }
        }
        return valid_count
    }


    // General case
    // Check all possible assignments of group sizes to contiguous blocks
    // Alternative: calculate number of options of non-repeated
    // then calculate how many new options are possible due to the repetition
    //

    return 0
}


fn part1(input: &String) -> usize {
    let lines: Vec<Line> = input.lines().map(|line| line.into()).collect();

    let mut result = 0;

    for line in lines {
        println!("{line:?}");
        let tmp = num_arrangements(&line.value, &line.groups);
        result += tmp;
        println!("\t{tmp} arrangements");
        println!();
    }

    return result
}

fn part2(input: &String) -> usize {
    let lines: Vec<Line> = input.lines().map(|line| Line::from_expanded(line)).collect();

    let mut result = 0;

    for (i, line) in lines.iter().enumerate() {
        println!("{i}");
        println!("{line:?}");
        let tmp = num_arrangements(&line.value, &line.groups);
        result += tmp;
        println!("\t{tmp} arrangements");
        println!();
    }

    return result
}

fn main() {
    let filename = "final.txt";
    let input = &std::fs::read_to_string(filename).expect("Invalid file ya dummy");

    println!("Result part 1: {}", part1(input));

    // println!("Result part 2: {}", part2(input));
}
