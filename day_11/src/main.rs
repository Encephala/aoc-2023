#[derive(Debug)]
struct Galaxy {
    value: Vec<Vec<char>>
}

impl From<&String> for Galaxy {
    fn from(input: &String) -> Self {
        let mut result: Vec<Vec<char>> = Vec::new();

        for (row_idx, line) in input.lines().enumerate() {
            result.push(Vec::new());

            for char in line.chars() {
                result[row_idx].push(char);
            }
        }

        Galaxy { value: result }
    }
}

impl Galaxy {
    fn column_iterator(&self, col: usize) {

    }

    fn empty_rows(&self) -> Vec<usize> {
        let mut result = Vec::new();

        for (row_idx, row) in self.value.iter().enumerate() {
            if row.iter().filter(|char| char != &&'.').count() == 0 {
                result.push(row_idx)
            }
        }

        return result
    }

    fn empty_cols(&self) -> Vec<usize> {
        let mut result = Vec::new();

        for col_idx in 0..self.value[0].len() {
            for row_idx in 0..self.value.len() {
                if self.value[row_idx][col_idx] != '.' {
                    println!("Row {row_idx} col {col_idx} char {}", self.value[row_idx][col_idx]);
                    break
                }
            }

            // If we didn't break, add to result
            result.push(col_idx)
        }

        return result
    }
}

fn part1(input: &String) -> usize {
    let galaxy: Galaxy = input.into();

    println!("Rows:");
    let empty_rows = galaxy.empty_rows();
    println!("Columns:");
    let empty_cols = galaxy.empty_cols();

    dbg!(empty_rows);
    dbg!(empty_cols);

    0
}

fn main() {
    let filename = "test1.txt";
    let input = std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("Result part 1: {}", part1(&input));
}
