use std::{collections::HashMap, cmp::Ordering};
use itertools::Itertools;

struct Hand<'a> {
    value: &'a str,
    bid: usize,
}

impl Hand<'_> {
    fn hand_type(&self) -> usize {
        let char_counts = self.to_char_counts();

        let num_jokers = *char_counts.get(&'J').unwrap_or(&0);

        // Five of kind?
        for (char, char_count) in char_counts.clone() {
            if char == 'J' {
                if char_count == 5 {
                    return 6
                }

                continue
            }

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

    // Only compares value, not type
    fn is_greater_than(&self, other_hand: &Hand) -> Ordering {
        let char_to_value: HashMap<char, usize> = HashMap::from([
            ('J', 1),
            ('2', 2),
            ('3', 3),
            ('4', 4),
            ('5', 5),
            ('6', 6),
            ('7', 7),
            ('8', 8),
            ('9', 9),
            ('T', 10),
            ('Q', 12),
            ('K', 13),
            ('A', 14),
        ]);

        let char_counts = self.value.chars().map(|char| char_to_value.get(&char).expect("Invalid char"));

        let other_char_counts = other_hand.value.chars().map(|char| char_to_value.get(&char).expect("Invalid char"));

        for (count, other_count) in char_counts.zip(other_char_counts) {
            if count > other_count {
                return Ordering::Greater
            } else if count < other_count {
                return Ordering::Less
            } else {
                // If equal, continue comparing
            }
        }

        return Ordering::Equal
    }
}





fn part2(input: String) -> usize {
    let mut hands_by_type: Vec<Vec<Hand>> = Vec::new();
    for _i in 0..7 {
        hands_by_type.push(Vec::new());
    }

    // Gather all hands
    for line in input.lines() {
        let (value, bid) = line.split_whitespace().collect_tuple().expect("Invalid line ya dummy");

        let hand = Hand { value, bid: bid.parse().expect("Not a number ya dummy") };

        let hand_type = hand.hand_type();

        hands_by_type[hand_type].push(hand);
    }



    // Sort all sets of hands,
    // in ascending order of value
    for set_of_hands in hands_by_type.iter_mut() {
        set_of_hands.sort_unstable_by(|hand, other_hand| hand.is_greater_than(other_hand) );
    }

    // Calculate combined value of hands
    let mut result = 0;

    let mut rank: usize = 0;

    for (i, set_of_hands) in hands_by_type.iter().enumerate() {
        println!("Type {i}");
        for hand in set_of_hands {
            rank += 1;

            result += rank * hand.bid;
            println!("Hand: {} - rank: {}, bid: {}", hand.value, rank, hand.bid);
        }
    }

    return result;
}

fn main() {
    let filename = "final.txt";
    let input = std::fs::read_to_string(filename).expect("Problem while reading file");

    println!("Result: {}", part2(input));
}
