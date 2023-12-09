fn predict_backwards(history: Vec<isize>) -> isize {
    println!("Predicting for {history:?}");

    let differences  = {
        let mut result = Vec::new();

        for i in 0..history.len() - 1 {
            result.push(history[i + 1] - history[i])
        }

        result
    };

    let first_value = history[0];

    // If all differences are zero, we're done
    if differences.iter().filter(|number| **number != 0).count() == 0 {
        return first_value;
    }

    return first_value - predict_backwards(differences);
}

fn part2(input: String) -> isize {
    let mut result = 0;

    for line in input.lines() {
        let history = line.split(" ").map(|number| number.parse().expect("Not a number")).collect();

        result += predict_backwards(history);
    }

    return result
}

fn main() {
    let filename = "final.txt";
    let input = std::fs::read_to_string(filename).expect("Invalid filename ya dummy");

    println!("Result: {}", part2(input));
}
