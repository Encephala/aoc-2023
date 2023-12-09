fn predict(history: Vec<isize>) -> isize {
    println!("Predicting for {history:?}");

    let differences  = {
        let mut result = Vec::new();

        for i in 0..history.len() - 1 {
            result.push(history[i + 1] - history[i])
        }

        result
    };

    let last_value = history[history.len() - 1];

    // If all differences are zero, we're done
    if differences.iter().filter(|number| **number != 0).count() == 0 {
        return last_value;
    }

    return last_value + predict(differences);
}

fn part1(input: String) -> isize {
    let mut result = 0;

    for line in input.lines() {
        let history = line.split(" ").map(|number| number.parse().expect("Not a number")).collect();

        result += predict(history);
    }

    return result
}

fn main() {
    let filename = "final.txt";
    let input = std::fs::read_to_string(filename).expect("Invalid filename ya dummy");

    println!("Result: {}", part1(input));
}
