fn part2(input: String) -> usize {
    let mut line_iterator = input.lines().peekable();


    let t =
        line_iterator.nth(0).expect("Faulty input ya dummy")
        .split(":").last().expect("Input didn't have semicolon ya dummy")
        .trim().replace(" ", "").parse::<usize>().expect("Not a number ya dummy");

    let d =
        line_iterator.nth(0).expect("Faulty input ya dummy")
        .split(":").last().expect("Input didn't have semicolon ya dummy")
        .trim().replace(" ", "").parse::<usize>().expect("Not a number ya dummy");

    let a: usize = 1;

    let (low_solution, high_solution) = {
        let discriminant: f64 = (t.pow(2) * a.pow(2) - 4 * a * d) as f64;

        (((t * a) as f64 - discriminant.sqrt()) / (2 * a) as f64, (((t * a) as f64 + discriminant.sqrt()) / (2 * a) as f64))
    };

    (low_solution.ceil() as usize..=high_solution.floor() as usize).count()
}

fn main() {
    let filename = "final.txt";
    let input = std::fs::read_to_string(filename).expect("Problem while reading file");

    println!("Result: {}", part2(input));
}
