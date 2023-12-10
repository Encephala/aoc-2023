use std::collections::{HashMap, HashSet};
use glam::i64::I64Vec2 as Vec2;

const OFFSETS: [Vec2; 4] = [Vec2::NEG_X, Vec2::NEG_Y, Vec2::X, Vec2::Y];

trait Position2 {
    fn can_move_to(&self, other: Vec2, tiles: &HashMap<Vec2, char>, visited_pos: &HashSet<Vec2>) -> Result<bool, String>;

    fn movable_positions(&self, tiles: &HashMap<Vec2, char>, visited_pos: &HashSet<Vec2>) -> Vec<Vec2>;
}


impl Position2 for Vec2 {
    fn can_move_to(&self, other: Vec2, tiles: &HashMap<Vec2, char>, visited_pos: &HashSet<Vec2>) -> Result<bool, String> {
        let diff_as_array = (other - *self).to_array();
        if visited_pos.contains(&other) {
            return Ok(false)
        }

        match tiles.get(self).unwrap() {
            '|' => {
                if let [0, _] = diff_as_array {
                    return Ok(true)
                }
            },

            '-' => {
                if let [_, 0] = diff_as_array {
                    return Ok(true)
                }
            },

            'L' => {
                if [[1, 0], [0, -1]].contains(&diff_as_array) {
                    return Ok(true)
                }
            }

            'J' => {
                if [[-1, 0], [0, -1]].contains(&diff_as_array) {
                    return Ok(true)
                }
            }

            '7' => {
                if [[-1, 0], [0, 1]].contains(&diff_as_array) {
                    return Ok(true)
                }
            }

            'F' => {
                if [[1, 0], [0, 1]].contains(&diff_as_array) {
                    return Ok(true)
                }
            }

            // Starting position can move to any tile that can move to it
            'S' => {
                return other.can_move_to(*self, tiles, visited_pos)
            }

            other => return Err(format!("Character {other} not found")),
        }

        return Ok(false)
    }

    fn movable_positions(&self, tiles: &HashMap<Vec2, char>, visited_pos: &HashSet<Vec2>) -> Vec<Vec2> {
        let mut result: Vec<Vec2> = Vec::new();

        for offset in OFFSETS {
            let destination = *self + offset;

            // If not out of bounds and not a '.' character
            if let Some(character) = tiles.get(&destination) {
                if *character == '.' { continue }

                if self.can_move_to(destination, tiles, visited_pos).unwrap() {
                    result.push(destination);
                }
            }
        }

        result
    }
}


fn starting_pos(tiles: &HashMap<Vec2, char>) -> Result<&Vec2, &'static str> {
    for (key, value) in tiles {
        if *value == 'S' {
            return Ok(key)
        }
    }

    Err("'S' not found in input")
}

fn input_to_tiles(input: &String) -> HashMap<Vec2, char> {
    let mut tiles = HashMap::new();

    // (0, 0) is top left
    for (row, line) in input.lines().enumerate() {
        for (col, character) in line.chars().enumerate() {
            tiles.insert(Vec2 {x: col as i64, y: row as i64}, character);
        }
    }

    tiles
}



fn part1(input: &String) -> usize {
    let tiles = &input_to_tiles(input);

    let starting_pos = starting_pos(&tiles).expect("Invalid input");

    let movable_positions_start = starting_pos.movable_positions(tiles, &HashSet::new());

    // "left" and "right" search path
    let mut pos_left = *movable_positions_start.first().unwrap();
    let mut pos_right = *movable_positions_start.last().unwrap();

    let mut visited_pos: HashSet<Vec2> = HashSet::new();
    visited_pos.insert(*starting_pos);
    visited_pos.insert(pos_left);
    visited_pos.insert(pos_right);


    let mut distance = 1; // Already made one step by finding pos_left and pos_right
    loop {
        distance += 1;

        // There will only ever be one movable position
        pos_left = pos_left.movable_positions(tiles, &visited_pos)[0];
        pos_right = pos_right.movable_positions(tiles, &visited_pos)[0];

        visited_pos.insert(pos_left);
        visited_pos.insert(pos_right);

        if pos_left == pos_right {
            break
        }

    }

    return distance;
}

fn part2(input: &String) -> usize {
    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().chars().count();

    let tiles = &input_to_tiles(input);

    let starting_pos = starting_pos(&tiles).expect("Invalid input");

    let movable_positions_start = starting_pos.movable_positions(tiles, &HashSet::new());

    // "left" and "right" search path
    let mut pos_left = *movable_positions_start.first().unwrap();
    let mut pos_right = *movable_positions_start.last().unwrap();


    // Infer character at starting position
    let offset_left = pos_left - *starting_pos;
    let offset_right = pos_right - *starting_pos;

    let starting_char = {
        if offset_left == Vec2::X {
            if offset_right == Vec2::Y {
                'F'
            }

            else if offset_right == Vec2::NEG_Y {
                'L'
            }

            else { // Vec2::NEG_X
                '-'
            }
        } else if offset_left == Vec2::Y {
            if offset_right == Vec2::X {
                'F'
            }

            else if offset_right == Vec2::NEG_X {
                '7'
            }

            else { // Vec2::NEG_Y
                '|'
            }
        } else if offset_left == Vec2::NEG_X {
            if offset_right == Vec2::X {
                '-'
            }

            else if offset_right == Vec2::Y {
                '7'
            }

            else { //Vec2::NEG_Y
                'J'
            }
        } else { // Vec2::NEG_Y
            if offset_right == Vec2::Y {
                '|'
            }

            else if offset_right == Vec2::X {
                'L'
            }

            else { // Vec2::NEG_X
                'J'
            }
        }
    };


    let mut visited_pos: HashSet<Vec2> = HashSet::new();
    visited_pos.insert(*starting_pos);
    visited_pos.insert(pos_left);
    visited_pos.insert(pos_right);


    loop {
        // There will only ever be one movable position
        pos_left = pos_left.movable_positions(tiles, &visited_pos)[0];
        pos_right = pos_right.movable_positions(tiles, &visited_pos)[0];

        visited_pos.insert(pos_left);
        visited_pos.insert(pos_right);

        if pos_left == pos_right {
            break
        }
    }

    // Create grid but with all non-loop elements simplified to '.'
    let mut tiles_simplified: Vec<Vec<char>> = Vec::new();

    for row in 0..num_rows {
        tiles_simplified.push(Vec::new());
        for _ in 0..num_cols {
            tiles_simplified[row].push('.')
        }
    }

    for pos in visited_pos.iter() {
        let Vec2{ x, y } = pos;
        tiles_simplified[*y as usize][*x as usize] = *tiles.get(&pos).unwrap();
    }

    tiles_simplified[starting_pos.y as usize][starting_pos.x as usize] = starting_char;


    let mut inside_count = 0;

    for (y, row) in tiles_simplified.iter().enumerate() {
        let mut inside = false;

        for (x, character) in row.iter().enumerate() {
            if ['|', 'F', '7'].contains(character) {
                inside = !inside;
            }

            else if inside && !visited_pos.contains(&Vec2 { x: x as i64, y: y as i64}) {
                inside_count += 1;
            }
        }
    }

    return inside_count
}

fn main() {
    let filename = "final.txt";
    println!("Using {filename}");

    let input = std::fs::read_to_string(filename).expect("Invalid file ya dummy");

    println!("Result part 1: {}", part1(&input));

    println!("Result part 2: {}", part2(&input));
}
