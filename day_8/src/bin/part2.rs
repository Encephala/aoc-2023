use itertools::Itertools;
use std::iter::repeat;
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn gcd(a: usize, b: usize ) -> usize {
    println!("gcd {a}, {b}");

    // Ensure a > b
    let (a, b) = (a.max(b), a.min(b));

    // Base case
    if b == 0 {
        return a
    }

    return gcd(a % b, b)
}

fn lcm(a: usize, b:usize) -> usize {
    // Brackets avoid potential overflow
    // Integer division is fine because gcd will always divide b
    return a * (b / gcd(a, b))
}

fn part2(input: String) -> usize {
    let mut lines = input.lines();

    let instructions: Vec<char> = lines.next().expect("No first line found").chars().collect();

    // Skip blankline
    lines.next();

    let mut map = HashMap::<String, Node>::new();

    let mut locations = Vec::<String>::new();

    for line in lines {
        let mut parts = line.split(" = ");

        let value = parts.next().expect("No value found");

        let elements = parts.next().expect("No elements found").replace("(", "").replace(")", "");

        let (left, right) = elements.split(", ").collect_tuple().expect("Invalid elements");

        if value.chars().last().expect("Invalid left location") == 'A' {
            locations.push(value.to_string());
        }

        map.insert(
            value.to_string(),
            Node {
            left: left.to_string(),
            right: right.to_string(),
        });
    }

    println!("Starting locations: {:?}", &locations);

    let mut count: usize = 0;

    let mut path_lengths = Vec::<usize>::new();

    'outer: for instructions in repeat(instructions) {
        for char in instructions {
            // Remove all finished locations
            let mut finished = Vec::<usize>::new();
            for i in 0..locations.len() {
                if locations[i].chars().last().expect("Invalid location") == 'Z' {
                    finished.push(i);
                }
            }

            // Remove in reverse order to maintain indices
            for index in finished.iter().rev() {
                path_lengths.push(count);
                locations.remove(*index);
            }

            if locations.len() == 0 {
                break 'outer;
            }


            for i in 0..locations.len() {
                if char == 'L' {
                    locations[i] = map.get(&locations[i]).expect("Location not in map").left.clone();
                } else {
                    locations[i] = map.get(&locations[i]).expect("Location not in map").right.clone();
                }
            }

            count += 1;
        }
    }

    dbg!(&path_lengths);
    return path_lengths.iter().fold(1, |acc, length| lcm(acc, *length))
}

fn main() {
    let filename = "final.txt";
    let input = std::fs::read_to_string(filename).expect("Invalid file ya dummy");

    println!("Result: {}", part2(input));
}
