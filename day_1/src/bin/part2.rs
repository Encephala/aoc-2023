use std::collections::HashMap;

fn part2(input: String) -> u32 {
    let digit_map: HashMap<&str, u32> = HashMap::from([
        ("1", 1),
        ("one", 1),
        ("2", 2),
        ("two", 2),
        ("3", 3),
        ("three", 3),
        ("4", 4),
        ("four", 4),
        ("5", 5),
        ("five", 5),
        ("6", 6),
        ("six", 6),
        ("7", 7),
        ("seven", 7),
        ("8", 8),
        ("eight", 8),
        ("9", 9),
        ("nine", 9),
    ]);

    let mut first_digits = Vec::<u32>::new();
    let mut last_digits = Vec::<u32>::new();

    // For each line, find the first and last character
    for line in input.split("\n") {
        // Escape the newline at the end of the file
        if line == "" { break }

        let mut indices_for_min: [(usize, &str); 18] = [(std::usize::MAX, ""); 18];
        let mut indices_for_max: [(usize, &str); 18] = [(std::usize::MIN, ""); 18];

        for (i, key) in digit_map.keys().enumerate() {
            match line.find(key) {
                Some(index) => {
                    indices_for_min[i] = (index, key);
                },
                None => ()
            }


            // Prevent usize underflow
            if key.len() > line.len() {
                continue;
            }

            for char_index in (0..line.len() - key.len() + 1).rev() {
                if &&line[char_index..char_index + key.len()] == key {
                    indices_for_max[i] = (char_index, key);
                    break;
                }
            }
        }

        let first_digit = digit_map[indices_for_min.iter().filter(|tuple| tuple.1 != "")
            .min_by_key(|tuple| tuple.0).expect("You passed an empty iterator you dummy").1];
        let last_digit = digit_map[indices_for_max.iter().filter(|tuple| tuple.1 != "")
            .max_by_key(|tuple| tuple.0).expect("You passed an empty iterator you dummy").1];


        first_digits.push(first_digit);
        last_digits.push(last_digit);
    }


    let mut result: Vec<u32> = Vec::new();
    for (ldigit, rdigit) in first_digits.iter().zip(last_digits.iter()) {
        let concatenated = format!("{ldigit}{rdigit}");
        let parsed: u32 = concatenated.parse().expect("Something went wrong with finding digits");
        result.push(parsed);
    }

    return result.iter().sum();
}

fn main() {
    let filename: &str = "final.txt";
    let input = std::fs::read_to_string(filename).expect("Problem while reading file");

    println!("Result: {}", part2(input));
}
