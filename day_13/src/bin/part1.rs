use std::cmp::min;

fn parse_input(input: &String) -> Vec<Vec<&str>> {
    let mut result = Vec::new();

    result.push(Vec::new());
    let mut i = 0;
    for line in input.lines() {
        // New block
        if line == "" {
            i += 1;
            result.push(Vec::new());

            continue;
        }

        result[i].push(line);
    }

    return result
}

fn strings_are_equal(string: &str, other_string: &str) -> bool {
    for (char, other_char) in string.chars().zip(other_string.chars()) {
        if char != other_char {
            return false;
        }
    }

    return true;
}


fn block_is_horizontally_mirrored_at(block: &Vec<&str>, index: usize) -> bool {
    let max_offset = min(index, block.len() - index);

    for offset in 1..=max_offset {
        if !strings_are_equal(block[index - offset], block[index + offset - 1]) {
            return false;
        }
    }

    return true;
}

// Returns index i if there is a mirror between row i - 1 and row i
fn horizontal_mirror_index(block: &Vec<&str>) -> Option<usize> {
    for i in 1..block.len() {
        if block_is_horizontally_mirrored_at(block, i) {
            return Some(i);
        }
    }

    None
}


fn block_is_vertically_mirrored_at(block: &Vec<&str>, index: usize) -> bool {
    let max_offset = min(index, block[0].len() - index);

    for offset in 1..=max_offset {
        let column: String = block.iter().map(
            |row| row.chars().nth(index - offset).expect("Invalid index ya dummy")
        ).collect();

        let other_column: String = block.iter().map(
            |row| row.chars().nth(index + offset - 1).expect("Invalid index ya dummy")
        ).collect();


        if !strings_are_equal(&column, &other_column) {
            return false;
        }
    }

    return true;
}

// Returns index i if there is a mirror between column i - 1 and column i
// Asumes inputs are same length
fn vertical_mirror_index(block: &Vec<&str>) -> Option<usize> {
    for i in 1..block[0].len() {
        if block_is_vertically_mirrored_at(block, i) {
            return Some(i);
        }
    }

    None
}

fn solution(blocks: &Vec<Vec<&str>>) -> usize {
    let mut result = 0;

    for block in blocks {
        if let Some(horizontal_index) = horizontal_mirror_index(&block) {
            result += 100 * horizontal_index;
        }

        if let Some(vertical_index) = vertical_mirror_index(&block) {
            result += vertical_index;
        }
    }

    return result;
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let blocks = parse_input(&input);

    println!("Part 1: {}", solution(&blocks));
}
