fn part1(input: String) -> u32 {
    let mut first_digits = Vec::<u32>::new();
    let mut last_digits = Vec::<u32>::new();
    // For each line, find the first and last character
    for line in input.split("\n") {
        for char in line.chars() {
            match char.to_digit(10) {
                Some(number) => {
                    first_digits.push(number);
                    break;
                },
                None => (),
            }
        }

        for char in line.chars().rev() {
            match char.to_digit(10) {
                Some(number) => {
                    last_digits.push(number);
                    break;
                },
                None => (),
            }
        }
    }

    let mut result: Vec<u32> = Vec::new();
    for (ldigit, rdigit) in first_digits.iter().zip(last_digits.iter()) {
        let concatenated = format!("{ldigit}{rdigit}");
        let parsed = concatenated.parse().expect("Something went wrong with finding digits");
        result.push(parsed);
    }

    return result.iter().sum();
}

fn main() {
    let filename: &str = "final.txt";
    let input = std::fs::read_to_string(filename).expect("Problem while reading file");

    println!("Result: {}", part1(input));
}
