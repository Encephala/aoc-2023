use std::collections::{HashSet, HashMap};

use itertools::Itertools;
use glam::i64::I64Vec2 as Vec2;

#[derive(Debug)]
enum Element {
    Empty,
    MirrorR,
    MirrorL,
    SplitH,
    SplitV,
}

const DOWN: Vec2 = Vec2{x: 0, y: 1};
const UP: Vec2 = Vec2{x: 0, y: -1};
const LEFT: Vec2 = Vec2{x: -1, y: 0};
const RIGHT: Vec2 = Vec2{x: 1, y: 0};


fn parse_input(input: &String) -> HashMap<Vec2, Element> {
    let mut result = HashMap::new();

    for (y, row) in input.split('\n').enumerate() {
        for (x, char) in row.chars().enumerate() {
            result.insert(Vec2{ x: x as i64, y: y as i64 },
                match char {
                    '.' => Element::Empty,
                    '/' => Element::MirrorR,
                    '\\' => Element::MirrorL,
                    '|' => Element::SplitV,
                    '-' => Element::SplitH,
                    _ => { panic!("Invalid character in input") },
                }
            );
        }
    }

    return result;
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Beam {
    position: Vec2,
    direction: Vec2,
}

fn new_beams(beam: &Beam, grid: &HashMap<Vec2, Element>) -> Vec<Beam> {
    let new_pos = beam.position + beam.direction;

    let grid_element = grid.get(&new_pos);

    if grid_element.is_none() {
        return Vec::new();
    }

    let result = match grid_element.unwrap() {
        Element::Empty => {
            Vec::from([Beam{ position: new_pos, direction: beam.direction }])
        },

        Element::MirrorL => {
            match beam.direction {
                LEFT => Vec::from([Beam{ position: new_pos, direction: UP }]),
                RIGHT => Vec::from([Beam{ position: new_pos, direction: DOWN }]),
                UP => Vec::from([Beam{ position: new_pos, direction: LEFT }]),
                DOWN => Vec::from([Beam{ position: new_pos, direction: RIGHT }]),
                _ => panic!("Invalid direction"),
            }
        },

        Element::MirrorR => {
            match beam.direction {
                LEFT => Vec::from([Beam{ position: new_pos, direction: DOWN }]),
                RIGHT => Vec::from([Beam{ position: new_pos, direction: UP }]),
                UP => Vec::from([Beam{ position: new_pos, direction: RIGHT }]),
                DOWN => Vec::from([Beam{ position: new_pos, direction: LEFT }]),
                _ => panic!("Invalid direction"),
            }
        },

        Element::SplitH => {
            match beam.direction {
                DOWN | UP => Vec::from([Beam{ position: new_pos, direction: LEFT }, Beam{ position: new_pos, direction: RIGHT }]),
                other => Vec::from([Beam{ position: new_pos, direction: other }]),
            }
        },

        Element::SplitV => {
            match beam.direction {
                LEFT | RIGHT => Vec::from([Beam{ position: new_pos, direction: DOWN }, Beam{ position: new_pos, direction: UP }]),
                other => Vec::from([Beam{ position: new_pos, direction: other }]),
            }
        },
    };

    return result;
}

fn push_new_beams(beam: &Beam, grid: &HashMap<Vec2, Element>, previous_beams: &mut HashSet<Beam>, beams: &mut Vec<Beam>) {
    for new_beam in new_beams(beam, grid).into_iter() {
        if previous_beams.contains(&new_beam) {
            continue;
        }

        previous_beams.insert(new_beam.clone());
        beams.push(new_beam);
    }
}

fn part1(input: &String) -> usize {
    let grid = parse_input(input);

    let mut previous_beams: HashSet<Beam> = HashSet::new();

    let mut beams: Vec<Beam> = Vec::new();

    beams.push(Beam{ position: Vec2{x: -1, y: 0}, direction: RIGHT });

    while let Some(beam) = beams.pop() {
        push_new_beams(&beam, &grid, &mut previous_beams, &mut beams);
    }

    return previous_beams.iter().unique_by(|beam| beam.position).count();
}

fn tiles_energised(grid: &HashMap<Vec2, Element>, start_beam: Beam) -> usize {
    let mut previous_beams: HashSet<Beam> = HashSet::new();

    let mut beams: Vec<Beam> = Vec::new();

    beams.push(start_beam);

    while let Some(beam) = beams.pop() {
        push_new_beams(&beam, grid, &mut previous_beams, &mut beams);
    }

    return previous_beams.iter().unique_by(|beam| beam.position).count();


}


fn part2(input: &String) -> usize {
    let grid = parse_input(input);

    let num_cols = input.chars().filter(|char| char == &'\n').count() + 1; // No newline at end of file
    let num_rows = input.chars().position(|char| char == '\n').unwrap();

    let mut result: Vec<usize> = Vec::new();

    for start_x in 0..num_cols {
        result.push(tiles_energised(&grid, Beam{
            position: Vec2{ x: start_x as i64, y: -1},
            direction: DOWN,
        }));

        result.push(tiles_energised(&grid, Beam{
            position: Vec2{ x: start_x as i64, y: num_rows as i64},
            direction: UP,
        }));
    }

    for start_y in 0..num_rows {
        result.push(tiles_energised(&grid, Beam{
            position: Vec2{ x: -1, y: start_y as i64},
            direction: RIGHT,
        }));

        result.push(tiles_energised(&grid, Beam{
            position: Vec2{ x: num_cols as i64, y: start_y as i64},
            direction: LEFT,
        }));
    }

    return *result.iter().max().unwrap();
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}
