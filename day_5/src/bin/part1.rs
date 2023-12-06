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

fn part1(input: String) -> usize {
    let mut lines = input.lines().peekable();


    let seeds: Vec<usize> = lines.nth(0).expect("No input given ya dummy")
                                 .split(":").nth(1).expect("Wrong line ya dummy")
                                 .trim()
                                 .split_whitespace()
                                 .map(|substring| substring.parse().expect("Not a number ya dummy"))
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

    let mut locations: Vec<usize> = Vec::new();

    // Run each seed through all mappings
    for seed in seeds {
        let mut intermediate_result = seed as i64;

        // Run through all mappings
        for mapping_list in mappings.iter() {
            for mapping in mapping_list {
                if mapping.maps(intermediate_result as usize) {
                    intermediate_result += mapping.offset;
                    break; // In the Almanac, only one mapping ever exists for a number
                }
            }

            // If no mapping maps it, we don't do anything to the result
        }

        locations.push(intermediate_result as usize);
    }

    return locations.into_iter().min().expect("You processed the seeds wrong ya dummy");
}

fn main() {
    let filename: &str = "final.txt";
    let input = std::fs::read_to_string(filename).expect("Problem while reading file");

    println!("Result: {}", part1(input));
}
