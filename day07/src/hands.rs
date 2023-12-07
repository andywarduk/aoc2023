use std::cmp::{Ordering, Reverse};

// Card ranks for part 1
pub const CARDSP1: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

// Card ranks for part 2
pub const CARDSP2: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

// Part 1 hand

#[derive(Debug, Clone)]
pub struct HandP1 {
    pub cards: Vec<char>,
    pub hand_type: HandStrength,
    pub bid: u64,
}

impl PartialEq for HandP1 {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl Eq for HandP1 {}

impl Ord for HandP1 {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp_hand(
            &self.hand_type,
            &other.hand_type,
            &self.cards,
            &other.cards,
            &CARDSP1,
        )
    }
}

impl PartialOrd for HandP1 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Part 2 hand

#[derive(Debug, Clone)]
pub struct HandP2 {
    pub cards: Vec<char>,
    pub hand_type: HandStrength,
    pub bid: u64,
}

impl PartialEq for HandP2 {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl Eq for HandP2 {}

impl Ord for HandP2 {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp_hand(
            &self.hand_type,
            &other.hand_type,
            &self.cards,
            &other.cards,
            &CARDSP2,
        )
    }
}

impl PartialOrd for HandP2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Hand strength

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandStrength {
    High,
    Pair1,
    Pair2,
    Three,
    FullHouse,
    Four,
    Five,
}

impl HandStrength {
    pub fn from_cards(cards: &[char], ranks: &[char]) -> Self {
        // Get counts of each card
        let mut counts = ranks
            .iter()
            .map(|c| cards.iter().filter(|hc| **hc == *c).count())
            .collect::<Vec<usize>>();

        // Sort count descending
        counts.sort_by_key(|c| Reverse(*c));

        // Calculate hand strength
        match counts[0] {
            5 => Self::Five,
            4 => Self::Four,
            3 => {
                // Could be 3 of a kind or full house
                if counts[1] == 2 {
                    Self::FullHouse
                } else {
                    Self::Three
                }
            }
            2 => {
                // Could be one or two pairs
                if counts[1] == 2 {
                    Self::Pair2
                } else {
                    Self::Pair1
                }
            }
            1 => Self::High,
            _ => panic!("Invalid count"),
        }
    }

    pub fn from_cards_with_jokers(cards: &[char], ranks: &[char]) -> Self {
        // Calculate result with jokers
        let mut result = Self::from_cards(cards, ranks);

        // Find joker positions
        let jokers = cards
            .iter()
            .enumerate()
            .filter_map(|(i, c)| if *c == 'J' { Some(i) } else { None })
            .collect::<Vec<usize>>();

        // Got any jokers?
        if !jokers.is_empty() {
            // Yes - call first replace iteration
            Self::replace_joker(cards, &jokers, 0, &mut result);
        }

        result
    }

    fn replace_joker(cards: &[char], jokers: &[usize], elem: usize, result: &mut Self) {
        // Clone to new cards vector
        let mut cards = cards.to_vec();

        // Iterate cards
        for c in CARDSP2 {
            if c == 'J' {
                continue;
            }

            // Replace this joker with the card
            cards[jokers[elem]] = c;

            // Calculate hand strength
            let new_result = Self::from_cards(&cards, &CARDSP2);

            // Better?
            if new_result > *result {
                *result = new_result;
            }

            // End of joker list?
            if elem < jokers.len() - 1 {
                // No - iterate
                Self::replace_joker(&cards, jokers, elem + 1, result)
            }
        }
    }
}

fn cmp_hand(
    type1: &HandStrength,
    type2: &HandStrength,
    cards1: &[char],
    cards2: &[char],
    ranks: &[char],
) -> Ordering {
    // Compare hand types first
    let cmp_res = type1.cmp(type2);

    #[cfg(test)]
    println!("{:?} vs {:?} -> {:?}", type1, type2, cmp_res);

    match cmp_res {
        // Equal - compare cards
        Ordering::Equal => cards1
            .iter()
            .zip(cards2)
            .find_map(|(c1, c2)| {
                let cmp_res = ranks
                    .iter()
                    .position(|c| *c == *c2)
                    .unwrap()
                    .cmp(&ranks.iter().position(|c| *c == *c1).unwrap());

                #[cfg(test)]
                println!("  {:?} vs {:?} -> {:?}", c1, c2, cmp_res);

                match cmp_res {
                    Ordering::Equal => None,
                    _ => Some(cmp_res),
                }
            })
            .unwrap(),
        _ => cmp_res,
    }
}
