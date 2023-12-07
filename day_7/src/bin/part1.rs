use std::{collections::HashMap, cmp::Ordering};
use itertools::Itertools;

struct Hand<'a> {
    value: &'a str,
    bid: usize,
}

impl Hand<'_> {
    fn hand_type(&self) -> usize {
        let char_counts = self.to_char_counts();

        // Five of kind?
        for char_count in char_counts.values() {
            if char_count == &5 {
                return 6
            }
        }

        // Four of kind?
        for char_count in char_counts.values() {
            if char_count == &4 {
                return 5
            }
        }

        // Full House?
        let mut any_three = false;
        let mut any_two = false;
        for char_count in char_counts.values() {
            if char_count == &2 {
                any_two = true;
            } else if char_count == &3 {
                any_three = true;
            }
        }

        if any_two && any_three {
            return 4
        }

        // Three of kind?
        for char_count in char_counts.values() {
            if char_count == &3 {
                return 3
            }
        }

        // Two pair/one pair?
        let mut pair_count: usize = 0;
        for char_count in char_counts.values() {
            if char_count == &2 {
                pair_count += 1
            }
        }

        // Can only be zero, one or two pairs
        if pair_count == 2 {
            return 2
        }

        if pair_count == 1 {
            return 1
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
            ('2', 2),
            ('3', 3),
            ('4', 4),
            ('5', 5),
            ('6', 6),
            ('7', 7),
            ('8', 8),
            ('9', 9),
            ('T', 10),
            ('J', 11),
            ('Q', 12),
            ('K', 13),
            ('A', 14),
        ]);

        let chars = self.value.chars().map(|char| char_to_value.get(&char).expect("Invalid char"));

        let other_chars = other_hand.value.chars().map(|char| char_to_value.get(&char).expect("Invalid char"));

        for (char, other_char) in chars.zip(other_chars) {
            if char > other_char {
                return Ordering::Greater
            } else if char < other_char {
                return Ordering::Less
            } else {
                // If equal, continue comparing
            }
        }

        return Ordering::Equal
    }
}





fn part1(input: String) -> usize {
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

    println!("Result: {}", part1(input));
}
