use itertools::Itertools;

#[derive(Debug)]
struct Game {
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
    count: usize,
}

fn winning_number_count(game: &Game) -> usize{
    let mut count = 0;

    // Explicitly call iter() because Rust defaults to into_iter() but I don't want to move
    for number in game.card_numbers.iter() {
        if game.winning_numbers.contains(&number) {
            count += 1;
        }
    }

    return count;
}

fn part2(input: String) -> usize {
    let mut all_games: Vec<Game> = Vec::new();

    // Gather all games into Vector
    for line in input.lines() {
        // Escape blank line at end of file
        if line == "" {
            continue;
        }

        let card_info = line.split(":").last().expect("Line was invalid");

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

        // Index of game in vector is (game_number - 1)
        all_games.push(Game {
            winning_numbers,
            card_numbers,
            count: 1,
        });
    }


    // Add current game count to winning_number_count of next games
    for i in 0..all_games.len() {
        for offset in 1..=winning_number_count(&all_games[i]) {
            all_games[i + offset].count += all_games[i].count;
        }
    }

    return all_games.iter().map(|game| game.count).sum();
}

fn main() {
    let filename: &str = "final.txt";
    let input = std::fs::read_to_string(filename).expect("Problem while reading file");

    println!("Result: {}", part2(input));
}
