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

fn strings_are_off_by_one(string: &str, other_string: &str) -> bool {
    let mut have_found_difference = false;

    for (char, other_char) in string.chars().zip(other_string.chars()) {
        if !have_found_difference {
            if char == other_char {
                continue;
            }

            have_found_difference = true;

            continue;
        }

        // If we've found the difference, all subsequent chars must be equal,
        // or we're off by at least two
        if char != other_char {
            return false
        }
    }

    if have_found_difference {
            return true
    }

    return false;
}


fn block_is_horizontally_mirrored_at(block: &Vec<&str>, index: usize) -> bool {
    let max_offset = min(index, block.len() - index);

    let mut have_found_smudge = false;
    for offset in 1..=max_offset {
        let row = block[index - offset];
        let other_row = block[index + offset - 1];

        if strings_are_equal(row, other_row) {
            continue;
        }

        // If rows not equal and we've already found smudge, not a mirror here
        if have_found_smudge {
            return false;
        }

        if strings_are_off_by_one(row, other_row) {
            have_found_smudge = true;

            continue;
        }

        // If rows are off by more than one, not a mirror here
        return false;
    }

    // Must have found smudge, or it isn't a valid mirror for part 2
    if !have_found_smudge {
        return false;
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

    let mut have_found_smudge = false;
    for offset in 1..=max_offset {
        let column: String = block.iter().map(
            |row| row.chars().nth(index - offset).expect("Invalid index ya dummy")
        ).collect();

        let other_column: String = block.iter().map(
            |row| row.chars().nth(index + offset - 1).expect("Invalid index ya dummy")
        ).collect();

        if strings_are_equal(&column, &other_column) {
            continue;
        }

        if have_found_smudge {
            return false;
        }

        if strings_are_off_by_one(&column, &other_column) {
            have_found_smudge = true;

            continue;
        }

        return false;
    }

    // Must have found smudge, or it isn't a valid mirror for part 2
    if !have_found_smudge {
        return false;
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

    println!("Part 2: {}", solution(&blocks));
}
