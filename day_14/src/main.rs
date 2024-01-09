// Converts input to 2D array in column-major order
fn parse_input(input: &String) -> Vec<Vec<char>> {
    let mut result = Vec::new();

    let num_columns = input.find('\n').unwrap();

    let lines = input.lines();

    for _ in 0..num_columns {
        result.push(Vec::new());
    }

    for line in lines {
        for (i, char) in line.chars().enumerate() {
            result[i].push(char);
        }
    }

    return result;
}

fn print_game(game: &Vec<Vec<char>>) {
    println!("Game:");

    for i in 0..game[0].len() {
        for j in 0..game.len() {
            print!("{}", game[j][i])
        }
        println!();
    }

    println!();
}


#[allow(non_snake_case)]
fn shift_rocks_north(column: &mut Vec<char>, up_to: usize, last_hash_index: usize, current_O_count: usize) {
    if last_hash_index == usize::MAX {
        for i in 0..current_O_count {
            column[i] = 'O';
        }

        for i in current_O_count..up_to {
            column[i] = '.'
        }
    } else {
        for i in last_hash_index + 1..last_hash_index + 1 + current_O_count {
            column[i] = 'O';
        }

        for i in last_hash_index + 1 + current_O_count..up_to {
            column[i] = '.'
        }
    }

    // column[up_to] is '#' or end of string,
    // so don't need to change it
}

#[allow(non_snake_case)]
fn tilt_column_north(column: &mut Vec<char>) {
    let mut current_O_count = 0;
    let mut last_hash_index = usize::MAX;

    for j in 0..column.len() {
        if column[j] == 'O' {
            current_O_count += 1;

            continue;
        }

        if column[j] == '#' {
            shift_rocks_north(column, j, last_hash_index, current_O_count);

            current_O_count = 0;
            last_hash_index = j;

            continue;
        }

        if column[j] == '.' {
            // do nothing
        }
    }

    shift_rocks_north(column, column.len(), last_hash_index, current_O_count);
}

fn tilt_north(game: &mut Vec<Vec<char>>) {
    for column in game {
        tilt_column_north(column);
    }
}


fn game_to_load(game: &Vec<Vec<char>>) -> usize {
    let mut result = 0;

    let column_length = game[0].len();

    for j in 0..game.len() {
        for i in 1..=column_length {
            if game[j][column_length - i] == 'O' {
                result += i
            }
        }
    }

    return result;
}


fn part1(mut game: Vec<Vec<char>>) -> usize {
    tilt_north(&mut game);

    return game_to_load(&game);
}


fn tilt_column_south(column: &mut Vec<char>) {
    column.reverse();
    tilt_column_north(column);
    column.reverse();
}

fn tilt_south(game: &mut Vec<Vec<char>>) {
    for column in game {
        tilt_column_south(column);
    }
}


#[allow(non_snake_case)]
fn shift_rocks_west(game: &mut Vec<Vec<char>>, row_idx: usize, up_to: usize, last_hash_index: usize, current_O_count: usize) {
    if last_hash_index == usize::MAX {
        for j in up_to..up_to + current_O_count {
            game[j][row_idx] = 'O';
        }

        for j in up_to + current_O_count..game.len() {
            game[j][row_idx] = '.'
        }
    } else {
        let last_hash_offset_from_end = game.len() - 1 - last_hash_index;

        for j in up_to..up_to + current_O_count {
            game[j][row_idx] = 'O';
        }

        for j in up_to + current_O_count..game.len() - last_hash_offset_from_end - 1 {
            game[j][row_idx] = '.'
        }
    }

    // game[up_to][row_idx] is '#' or end of string,
    // so don't need to change it
}

#[allow(non_snake_case)]
fn tilt_west(game: &mut Vec<Vec<char>>) {
    for i in 0..game[0].len() {
        let mut current_O_count = 0;
        let mut last_hash_index = usize::MAX;

        for j in (0..game.len()).rev() {
            if game[j][i] == 'O' {
                current_O_count += 1;

                continue;
            }

            if game[j][i] == '#' {
                shift_rocks_west(game, i, j + 1, last_hash_index, current_O_count);

                current_O_count = 0;
                last_hash_index = j;

                continue;
            }

            if game[j][i] == '.' {
                // do nothing
            }
        }

        shift_rocks_west(game, i, 0, last_hash_index, current_O_count);
    }
}


#[allow(non_snake_case)]
fn shift_rocks_east(game: &mut Vec<Vec<char>>, row_idx: usize, up_to: usize, last_hash_index: usize, current_O_count: usize) {
    if last_hash_index == usize::MAX {
        for j in up_to - current_O_count..up_to {
            game[j][row_idx] = 'O';
        }

        for j in 0..up_to - current_O_count {
            game[j][row_idx] = '.'
        }
    } else {
        for j in up_to - current_O_count..up_to {
            game[j][row_idx] = 'O';
        }

        for j in last_hash_index + 1..up_to -current_O_count {
            game[j][row_idx] = '.'
        }
    }

    // game[up_to][row_idx] is '#' or end of string,
    // so don't need to change it
}

#[allow(non_snake_case)]
fn tilt_east(game: &mut Vec<Vec<char>>) {
    for i in 0..game[0].len() {
        let mut current_O_count = 0;
        let mut last_hash_index = usize::MAX;

        for j in 0..game.len() {
            if game[j][i] == 'O' {
                current_O_count += 1;

                continue;
            }

            if game[j][i] == '#' {
                shift_rocks_east(game, i, j, last_hash_index, current_O_count);

                current_O_count = 0;
                last_hash_index = j;

                continue;
            }

            if game[j][i] == '.' {
                // do nothing
            }
        }

        shift_rocks_east(game, i, game.len(), last_hash_index, current_O_count);
    }
}


fn games_are_equal(game: &Vec<Vec<char>>, other_game: &Vec<Vec<char>>) -> bool {
    for (column, other_column) in game.iter().zip(other_game.iter()) {
        for (char, other_char) in column.iter().zip(other_column.iter()) {
            if char != other_char {
                return false;
            }
        }
    }

    true
}


fn part2(mut game: Vec<Vec<char>>) -> usize {
    let mut last_loads = Vec::<usize>::new();

    let mut i = 0;

    loop {
        tilt_north(&mut game);
        // println!("After north");
        // print_game(&game);
        tilt_west(&mut game);
        // println!("After west");
        // print_game(&game);
        tilt_south(&mut game);
        // println!("After south");
        // print_game(&game);
        tilt_east(&mut game);
        // println!("After east");
        // print_game(&game);

        // Not quite sure what the logic is to break here;
        // Game can get stuck in infinite loop, so can't just check if game == last_game
        // I'm sure there's some solution to finding which game in that loop will be the one billionth,
        // but eh
    }

    0
}

fn main() {
    let input = std::fs::read_to_string("test.txt").unwrap();

    let game = parse_input(&input);

    println!("Result part 1 {}", part1(game.clone()));

    println!("Result part 2 {}", part2(game.clone()));
}
