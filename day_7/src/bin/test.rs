use std::collections::HashMap;

#[derive(Debug)]
struct Hand<'a> {
    value: &'a str,
}

impl Hand<'_> {
    fn hand_type(&self) -> usize {
        let mut char_counts = self.to_char_counts();

        let num_jokers = *char_counts.entry('J').or_insert(0);

        // Five of kind?
        for char_count in char_counts.values() {
            if char_count + num_jokers >= 5 {
                return 6
            }
        }

        // Four of a kind?
        for (char, char_count) in char_counts.clone() {
            if char == 'J' {
                continue
            }

            if char_count + num_jokers >= 4 {
                return 5
            }
        }

        // Full House?
        let iter: HashMap<char, usize> = char_counts.clone();
        for (outer_char, count) in iter.clone() {
            if outer_char == 'J' {
                continue
            }

            let mut jokers_remaining = num_jokers;

            if count + jokers_remaining >= 2 {
                if count < 2 {
                    jokers_remaining -= 2 - count;
                }

                // If a remaining character has at least three
                for (char, count) in iter.clone() {
                    if char == 'J' {
                        continue
                    }

                    if count + jokers_remaining >= 3 {
                        if char != outer_char {
                            return 4
                        }
                    }
                }
            }

            else if count + jokers_remaining >= 3 {
                if count < 3 {
                    jokers_remaining -= 3 - count;
                }

                // If a remaining character has at least two
                for (char, count) in iter.clone() {
                    if char == 'J' {
                        continue
                    }

                    if count + jokers_remaining >= 2 {
                        if char != outer_char {
                            return 4
                        }
                    }
                }
            }

        }


        // Three of kind?
        for (char, char_count) in char_counts.clone() {
            if char == 'J' {
                continue
            }

            if char_count + num_jokers >= 3 {
                return 3
            }
        }

        // Two pair?
        for (outer_char, count) in char_counts.clone() {
            if outer_char == 'J' {
                continue
            }

            let mut jokers_remaining = num_jokers;

            if count + jokers_remaining >= 2 {
                if count < 2 {
                    jokers_remaining -= 2 - count;
                }

                for (char, count) in char_counts.clone() {
                    if char == 'J' {
                        continue
                    }

                    if count + jokers_remaining >= 2 {
                        if char != outer_char {
                            return 2
                        }
                    }
                }
            }
        }

        // One pair?
        for (char, count) in char_counts.clone() {
            if char == 'J' {
                continue
            }

            // Actually, won't ever be >2 but okay
            if count + num_jokers >= 2 {
                return 1
            }
        }

        return 0
    }

    fn to_char_counts(&self) -> HashMap<char, usize> {
        let mut result = HashMap::new();

        for char in self.value.chars() {
            result.entry(char).and_modify(|val| *val += 1).or_insert(1);
        }

        return result;
    }


}



fn main() {
    let test = Hand { value: "9J4AJ" };

    println!("{test:?}");
    println!("{:?}", test.to_char_counts());
    println!("{:?}", test.hand_type());
}
