use itertools::Itertools;
use std::iter::repeat;
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn part1(input: String) -> usize {
    let mut lines = input.lines();

    let instructions: Vec<char> = lines.next().expect("No first line found").chars().collect();

    // Skip blankline
    lines.next();

    let mut map: HashMap<String, Node> = HashMap::new();

    for line in lines {
        let mut parts = line.split(" = ");

        let value = parts.next().expect("No value found");

        let elements = parts.next().expect("No elements found").replace("(", "").replace(")", "");

        let (left, right) = elements.split(", ").collect_tuple().expect("Invalid elements");

        map.insert(
            value.to_string(),
            Node {
            left: left.to_string(),
            right: right.to_string(),
        });
    }

    let mut location: &str = "AAA";

    let mut count: usize = 0;
    'outer: for instructions in repeat(instructions) {
        for char in instructions {
            if location == "ZZZ" {
                break 'outer;
            }

            if char == 'L' {
                location = &map.get(location).expect("Location not in map").left;
            } else {
                location = &map.get(location).expect("Location not in map").right;
            }

            count += 1;
            println!("{count} ({location})");
        }

    }

    return count
}

fn main() {
    let filename = "final.txt";
    let input = std::fs::read_to_string(filename).expect("Invalid file ya dummy");

    println!("Result: {}", part1(input));
}
