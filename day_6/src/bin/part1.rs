fn part1(input: String) -> usize {
    let mut line_iterator = input.lines().peekable();


    let time_info = line_iterator.nth(0).expect("Faulty input ya dummy")
        .split(":").last().expect("Input didn't have semicolon ya dummy")
        .trim().split_whitespace().map(|time| time.parse::<usize>().expect("Not a number ya dummy"));

    let distance_info = line_iterator.nth(0).expect("Faulty input ya dummy")
        .split(":").last().expect("Input didn't have semicolon ya dummy")
        .trim().split_whitespace().map(|time| time.parse::<usize>().expect("Not a number ya dummy"));

    let a = 1;

    let mut options: Vec<usize> = Vec::new();
    for (time, distance_to_beat) in time_info.zip(distance_info) {
        options.push({
            (1..time).map(|press_time| {
                let distance = (time - press_time) * (a * press_time);
                (distance > distance_to_beat) as usize
            }).sum()
        })
    }

    return options.iter().product()
}

fn main() {
    let filename = "final.txt";
    let input = std::fs::read_to_string(filename).expect("Problem while reading file");

    println!("Result: {}", part1(input));
}
