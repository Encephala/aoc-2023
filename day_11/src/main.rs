#[derive(Debug)]
struct Sky {
    value: Vec<Vec<bool>>,
    num_rows: usize,
    num_cols: usize,
}

impl From<&String> for Sky {
    fn from(input: &String) -> Self {
        let mut result: Vec<Vec<bool>> = Vec::new();

        for (row_idx, line) in input.lines().enumerate() {
            result.push(Vec::new());
            for char in line.chars() {
                result[row_idx].push(char == '#');
            }
        }

        Sky {
            value: result,
            num_rows: input.lines().count(),
            num_cols: input.lines().next().unwrap().len(),
        }
    }
}

impl Sky {
    fn empty_rows(&self) -> Vec<usize> {
        let mut result = Vec::new();

        for (row_idx, row) in self.value.iter().enumerate() {
            if row.iter().filter(|is_galaxy| **is_galaxy).count() == 0 {
                result.push(row_idx);
            }
        }

        return result
    }

    fn empty_cols(&self) -> Vec<usize> {
        let mut result = Vec::new();

        for col_idx in 0..self.num_cols {
            let mut is_empty = true;
            for row_idx in 0..self.num_rows {
                if self.value[row_idx][col_idx] {
                    is_empty = false;
                }
            }

            if is_empty {
                result.push(col_idx);
            }
        }

        return result
    }

    fn clone_row(&mut self, row_idx: usize) {
        self.value.insert(row_idx, self.value[row_idx].clone());

        self.num_rows += 1;
    }

    fn clone_col(&mut self, col_idx: usize) {
        for row in self.value.iter_mut() {
            row.insert(col_idx, row[col_idx]);
        }

        self.num_cols += 1;
    }
}


fn part1(input: &String) -> i64 {
    let mut sky: Sky = input.into();

    let empty_rows = sky.empty_rows();
    let empty_cols = sky.empty_cols();

    for row_idx in empty_rows.iter().rev() {
        sky.clone_row(*row_idx);
    }

    for col_idx in empty_cols.iter().rev() {
        sky.clone_col(*col_idx);
    }

    let galaxies: Vec<_> = {
        sky.value.iter().enumerate().flat_map(|(row_idx, row)|
            row.iter().enumerate().filter(|(_, is_galaxy)| **is_galaxy).map(move |(col_idx, _)| (row_idx as i64, col_idx as i64))
        ).collect()
    };

    let mut sum = 0;
    for (i, galaxy) in galaxies.iter().enumerate() {
        for other_galaxy in galaxies.iter().skip(i + 1) {
            sum += (galaxy.0 - other_galaxy.0).abs();
            sum += (galaxy.1 - other_galaxy.1).abs();
        }
    }

    return sum
}


fn part2(input: &String) -> i64 {
    let sky: Sky = input.into();

    let empty_rows = sky.empty_rows();
    let empty_cols = sky.empty_cols();

    let growth_rate = 1_000_000;

    let galaxies: Vec<_> = {
        sky.value.iter().enumerate().flat_map(|(row_idx, row)|
            row.iter().enumerate().filter(|(_, is_galaxy)| **is_galaxy).map(move |(col_idx, _)| (row_idx as i64, col_idx as i64))
        ).collect()
    };

    let mut sum = 0;
    for (i, galaxy) in galaxies.iter().enumerate() {
        for other_galaxy in galaxies.iter().skip(i + 1) {
            // Distance in rows
            let (min_x, max_x) = (galaxy.0.min(other_galaxy.0), galaxy.0.max(other_galaxy.0));
            sum += max_x - min_x;

            let num_empty_rows_in_between = empty_rows.iter().filter(|row_idx| (min_x + 1..max_x).contains(&(**row_idx as i64))).count() as i64;
            sum += growth_rate * num_empty_rows_in_between - num_empty_rows_in_between;

            // Distance in cols
            let (min_y, max_y) = (galaxy.1.min(other_galaxy.1), galaxy.1.max(other_galaxy.1));
            sum += (max_y - min_y).abs();

            let num_empty_cols_in_between = empty_cols.iter().filter(|row_idx| (min_y + 1..max_y).contains(&(**row_idx as i64))).count() as i64;
            sum += growth_rate * num_empty_cols_in_between - num_empty_cols_in_between;
        }
    }

    return sum
}

fn main() {
    let filename = "final.txt";
    let input = std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("Result part 1: {}", part1(&input));

    println!("Result part 2: {}", part2(&input));
}
