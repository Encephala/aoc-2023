use itertools::Itertools;

fn part1(input: String) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        // Escape blank line at end of file
        if line == "" {
            continue;
        }

        let card_info = line.split(":").last().expect("Line was invalid");

        // println!("{line}");

        // let parts: Vec<&str> = card_info.split("|").map(|part| part.trim()).flat_map(|part| part.split(" ")).collect();
        // dbg!(parts);
        // panic!();

        let (winning_numbers, card_numbers): (Vec<u32>, Vec<u32>) = {
            let parts = card_info.split("|").map(|part| part.trim());

            // Disgusting, but okay
            parts.map(|part| part.split(" ")
                                       .filter(|substring| substring.len() > 0) // Account for double space occurring
                                       .map(|string| string.parse().expect("Invalid number found"))
                                       .collect())
                 .collect_tuple()
                 .expect("Ya dummy")
        };

        let num_winning_numbers: u32 = card_numbers.iter()
                                              .map(|number| winning_numbers.contains(number) as u32)
                                              .sum();


        result += {
            if num_winning_numbers > 0 {
                (2 as u32).pow(num_winning_numbers - 1)
            } else {
                0
            }
        }
    }

    return result
}

fn main() {
    let filename: &str = "final.txt";
    let input = std::fs::read_to_string(filename).expect("Problem while reading file");

    println!("Result: {}", part1(input));
}
