#[derive(Debug, Clone)]
struct Location {
    is_galaxy: bool,
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Galaxy {
    value: Vec<Vec<Location>>
}

impl From<&String> for Galaxy {
    fn from(input: &String) -> Self {
        let mut result: Vec<Vec<Location>> = Vec::new();

        let mut galaxy_index: usize = 0;

        for (row_idx, line) in input.lines().enumerate() {
            result.push(Vec::new());

            for (col_idx, char) in line.chars().enumerate() {
                if char == '#' {
                    galaxy_index += 1;
                    result[row_idx].push(Location { is_galaxy: true, row: row_idx, col: col_idx });
                } else {
                    result[row_idx].push(Location { is_galaxy: false, row: row_idx, col: col_idx });
                }
            }
        }

        Galaxy { value: result }
    }
}

impl Galaxy {
    fn empty_rows(&self) -> Vec<usize> {
        let mut result = Vec::new();

        for (row_idx, row) in self.value.iter().enumerate() {
            if row.iter().filter(|location| location.is_galaxy == false).count() == 0 {
                result.push(row_idx)
            }
        }

        return result
    }

    fn empty_cols(&self) -> Vec<usize> {
        let mut result = Vec::new();

        for col_idx in 0..self.value[0].len() {
            let mut col_is_empty = true;
            for row_idx in 0..self.value.len() {
                if self.value[row_idx][col_idx].is_galaxy == false {
                    col_is_empty = false;
                }
            }

            if col_is_empty {
                result.push(col_idx)
            }
        }

        return result
    }

    fn clone_empty_row(&mut self, row_idx: usize) {
        self.value.insert(row_idx, self.value[row_idx].clone());
        // TODO: Update index of all rows after
    }

    fn clone_empty_column(&mut self, col_idx: usize) {
        for row_idx in 0..self.value.len() {
            self.value[row_idx].insert(col_idx, Location { is_galaxy: false, row: 0, col: 0 } )
        }
        // TODO: Update index of all cols after
    }

    fn distance(&self, other: &Galaxy) {

    }

    fn galaxy_pairs(&self) -> Vec<Vec<Location>> {
        let mut result = Vec::new();

        for (gal_idx, galaxy) in self.value.iter().flat_map(|row| row).enumerate() {
            for other_galaxy in self.value.iter().flat_map(|row| row).skip(gal_idx) {

            }
        }

        return result
    }
}

fn part1(input: &String) -> usize {
    let mut galaxy: Galaxy = input.into();

    let empty_rows = galaxy.empty_rows();
    let empty_cols = galaxy.empty_cols();

    for row_idx in empty_rows.iter().rev() {
        galaxy.clone_empty_row(*row_idx);
    }

    for col_idx in empty_cols.iter().rev() {
        galaxy.clone_empty_column(*col_idx);
    }


    0
}

fn main() {
    let filename = "test1.txt";
    let input = std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("Result part 1: {}", part1(&input));
}
