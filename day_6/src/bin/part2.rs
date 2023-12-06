fn part2(input: String) -> usize {
    let mut line_iterator = input.lines().peekable();


    let time =
        line_iterator.nth(0).expect("Faulty input ya dummy")
        .split(":").last().expect("Input didn't have semicolon ya dummy")
        .trim().replace(" ", "").parse::<usize>().expect("Not a number ya dummy");

    let distance_to_beat =
        line_iterator.nth(0).expect("Faulty input ya dummy")
        .split(":").last().expect("Input didn't have semicolon ya dummy")
        .trim().replace(" ", "").parse::<usize>().expect("Not a number ya dummy");

    let a = 1;

    (1..time).map(|press_time| {
        let distance = (time - press_time) * a * press_time;
        (distance > distance_to_beat) as usize
    }).sum()
}

fn main() {
    let filename = "final.txt";
    let input = std::fs::read_to_string(filename).expect("Problem while reading file");

    println!("Result: {}", part2(input));
}
