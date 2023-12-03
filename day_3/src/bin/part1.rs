#[derive(Debug)]
struct Element {
    is_numeric: bool,
    value: Option<i32>,
    _row: usize,
    column: usize,
    length: usize,
}

impl Element {
    fn has_adjacent_symbol_in(&self, elements: Vec<&Vec<Element>>) -> bool {
        let (start_of_number, end_of_number) = (self.column, self.column + self.length - 1);

        let start_search_index = {
            if start_of_number == 0 {
                start_of_number
            } else {
                start_of_number - 1
            }
        };

        let stop_search_index = end_of_number + 1;

        let allowed_indices = start_search_index..stop_search_index + 1;

        for row in elements {
            for other_element in row {
                if !other_element.is_numeric && allowed_indices.contains(&other_element.column) {
                    return true;
                }
            }
        }

        return false;
    }
}


fn part1(input: String) -> i32 {
    let mut elements: Vec<Vec<Element>> = Vec::new();

    for (i, line) in input.split("\n").enumerate() {
        // Escape empty line at end of file
        if line == "\n" {
            continue
        }


        // Parse input into vector of elements
        elements.push(Vec::new());


        let mut iterator = line.chars().enumerate();

        while let Some((index, char)) = iterator.next() {
            if char == '.' {
                continue
            }

            // If it's a digit, scan string for the whole number
            // and skip parsing the digits found
            else if char.is_digit(10) {
                let mut value = String::from("");

                for char in line[index..].chars() {
                    if char.is_digit(10) {
                        value.push(char);
                    } else {
                        break
                    }
                }

                elements[i].push(Element{
                    is_numeric: true,
                    value: Some(value.parse().expect("Wrongly detected a number ya dumm")),
                    _row: i,
                    column: index,
                    length: value.len(),
                });

                // Move iterator forward
                // -1 because we are at the first character of the number already,
                // and another -1 because nth(i) moves i + 1 elements forward
                if value.len() != 1 {
                    iterator.nth(value.len() - 2);
                } else {
                    // Don't need to skip anything
                }
            }

            // Otherwise, it's a symbol
            else {
                elements[i].push(Element{
                    is_numeric: false,
                    value: None,
                    _row: i,
                    column: index,
                    length: 1,
                })
            }
        }
    }


    // Find the elements that have an adjacent symbol
    let mut result = 0;

    for (i, row) in elements.iter().enumerate() {
        for element in row.iter() {
            if element.is_numeric {
                // Get previous, current and next row
                let mut relevant_rows: Vec<&Vec<Element>> = Vec::new();

                if i > 0 { relevant_rows.push(&elements[i - 1]); }
                relevant_rows.push(&elements[i]);
                if i < elements.len() - 1 { relevant_rows.push(&elements[i + 1]); }

                if element.has_adjacent_symbol_in(relevant_rows) {
                    result += element.value.expect("This value doesn't exist ya dummy");
                }
            }
        }
    }

    return result;
}

fn main() {
    let filename: &str = "final.txt";
    let input = std::fs::read_to_string(filename).expect("Problem while reading file");

    println!("Result: {}", part1(input));
}
