use itertools::Itertools;

#[derive(Debug)]
struct MappingInfo {
    start: usize,
    length: usize,
    offset: i64,
}

impl MappingInfo {
    fn maps(&self, value: usize) -> bool {
        return self.start <= value && value < self.start + self.length;
    }
}

fn part2(input: String) -> usize {
    let mut lines = input.lines().peekable();


    let seeds: Vec<(usize, usize)> = lines.nth(0).expect("No input given ya dummy")
                                          .split(":").nth(1).expect("Wrong line ya dummy")
                                          .trim()
                                          .split_whitespace()
                                          .tuples()
                                          .map(|(start, length)| {
                                             let start: usize = start.parse().expect("Not a number ya dummy");
                                             let length: usize = length.parse().expect("Not a number ya dummy");

                                             (start, length)
                                          })
                                          .collect();

    let mut mappings: Vec<Vec<MappingInfo>> = Vec::new();

    // Stop when all blocks are parsed
    while let Some(_) = lines.peek() {
        // Skip blank line and title line
        lines.nth(1);

        mappings.push(Vec::new());

        // Loop over all lines in a block
        while let Some(line) = lines.peek() {
            // New block if blankline is found
            if line == &"" {
                break
            }

            let (dest, src, length): (usize, usize, usize) =
                lines.next().unwrap()
                .split_whitespace()
                .map(|substring| substring.parse().expect("Not a number ya dummy"))
                .collect_tuple()
                .expect("No numbers found in line ya dummy");

            mappings.last_mut().unwrap().push(
                MappingInfo {
                    start: src,
                    length,
                    offset: dest as i64 - src as i64,
                }
            );
        }
    }

    // Create inverse mappings
    let inverse_mappings: Vec<Vec<MappingInfo>> =
    mappings.iter()
            .map(|mappings_list| mappings_list.iter()
                                                                 // Invert each mapping
                                                                 .map(|mapping| {
                                                                    MappingInfo {
                                                                        start: (mapping.start as i64 + mapping.offset) as usize,
                                                                        length: mapping.length,
                                                                        offset: -mapping.offset,
                                                                    }
                                                                 })
                                                                 .collect())
            // Revert the order of the mappings
            .rev()
            .collect();

    // Start testing all possible locations if the corresponding seed is allowed
    let mut location: usize = 0;

    loop {
        if location % 1_000_000 == 0 {
            println!("Location {location:.2e}");
        }
        let mut intermediate_result = location as i64;

        // Run through all inverse mappings
        for mapping_list in inverse_mappings.iter() {
            for mapping in mapping_list {
                if mapping.maps(intermediate_result as usize) {
                    intermediate_result += mapping.offset;
                    break; // In the Almanac, only one mapping ever exists for a number
                }
            }

            // If no mapping maps it, we don't do anything to the result
        }

        // If the current location yields a seed that is allowed, return it
        for (start, length) in seeds.iter() {
            if (*start..start + length).contains(&(intermediate_result as usize)) {
                return location;
            }
        }

        location += 1;
    }
}

fn main() {
    let filename: &str = "final.txt";
let input = std::fs::read_to_string(filename).expect("Problem while reading file");

    println!("Result: {}", part2(input));
}
