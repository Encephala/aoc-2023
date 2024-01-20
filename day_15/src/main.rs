use regex::Regex;

fn part1(input: &str) -> usize {
    input.split(',').map(|label| hash(label)).sum()
}

fn hash(label: &str) -> usize {
    label.chars()
        .filter(|character| character != &'\n')
        .fold(0, |acc, value| (acc + value as usize) * 17 % 256 )
}


struct Lens {
    label: String,
    focal: usize,
}

fn part2(input: &str) -> usize {
    let mut boxes: Vec<Vec<Lens>> = Vec::new();

    for _ in 0..256 {
        boxes.push(Vec::new());
    }

    let operation_regex = Regex::new("[-=]").unwrap();

    for mut instruction in input.split(',') {
        // Remove newline char
        if instruction.chars().last().unwrap() == '\n' {
            instruction = &instruction[0..instruction.len() - 1]
        }

        let index_of_operation = operation_regex.find(instruction).expect("Invalid instruction").start();

        let label = &instruction[0..index_of_operation];
        let operation = &instruction[index_of_operation..];

        let the_box = &mut boxes[hash(label)];


        if operation.chars().next().unwrap() == '-' {
            if let Some(index) = the_box.iter().position(|lens| lens.label == label) {
                the_box.remove(index);
            }

            continue;
        }

        // Operation is "=x"
        let new_focal = operation.chars().last().expect("Invalid '=' operation")
            .to_digit(10).expect("Not a digit");

        if let Some(lens) = the_box.iter_mut().find(|lens| lens.label == label) {
            // Update lens focal length
            lens.focal = new_focal as usize;

            continue;
        }

        // Append new lens to box
        the_box.push(Lens{ label: label.into(), focal: new_focal as usize });
    }

    let mut result = 0;

    for (box_idx, the_box) in boxes.iter().enumerate() {
        for (lens_idx, lens) in the_box.iter().enumerate() {
            result += focusing_power(box_idx, lens_idx, lens)
        }
    }

    return result;
}

fn focusing_power(box_idx: usize, lens_idx: usize, lens: &Lens) -> usize {
    return (1 + box_idx) * (1 + lens_idx) * lens.focal;
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}
