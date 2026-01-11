use rand::seq::SliceRandom;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Default for Card {
    fn default() -> Self {
        Card::placeholder()
    }
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card { rank, suit }
    }

    pub fn placeholder() -> Self {
        Card {
            rank: Rank::Two,
            suit: Suit::Hearts,
        }
    }

    pub fn is_placeholder(&self) -> bool {
        self.rank == Rank::Two && self.suit == Suit::Hearts
    }

    pub fn rank_str(&self) -> &'static str {
        match self.rank {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        }
    }

    pub fn suit_str(&self) -> &'static str {
        match self.suit {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        }
    }

    pub fn is_red(&self) -> bool {
        matches!(self.suit, Suit::Hearts | Suit::Diamonds)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank_str(), self.suit_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for &suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for &rank in &[
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Ten,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
                Rank::Ace,
            ] {
                cards.push(Card::new(rank, suit));
            }
        }
        cards.shuffle(&mut rand::thread_rng());
        Deck { cards }
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Deck::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PokerRound {
    #[default]
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandRank {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluatedHand {
    pub hand_rank: HandRank,
    pub primary_values: Vec<Rank>,
    pub kickers: Vec<Rank>,
}

impl EvaluatedHand {
    pub fn score(&self) -> (HandRank, Vec<Rank>) {
        (self.hand_rank.clone(), self.primary_values.clone())
    }
}

pub fn evaluate_hand(cards: &[Card]) -> EvaluatedHand {
    let mut cards: Vec<Card> = cards
        .iter()
        .filter(|c| !c.is_placeholder())
        .cloned()
        .collect();
    if cards.len() < 5 {
        return EvaluatedHand {
            hand_rank: HandRank::HighCard,
            primary_values: Vec::new(),
            kickers: Vec::new(),
        };
    }

    cards.sort_by_key(|c| c.rank);

    let is_flush = cards
        .windows(5)
        .any(|window| window.iter().all(|c| c.suit == window[0].suit));

    let ranks: Vec<Rank> = cards.iter().map(|c| c.rank).collect();
    let mut unique_ranks: Vec<Rank> = ranks.clone();
    unique_ranks.dedup();

    let is_straight = if unique_ranks.len() >= 5 {
        let has_ace_low = unique_ranks.contains(&Rank::Two) && unique_ranks.contains(&Rank::Ace);
        let has_wheel = has_ace_low
            && unique_ranks.contains(&Rank::Three)
            && unique_ranks.contains(&Rank::Four)
            && unique_ranks.contains(&Rank::Five);

        if has_wheel {
            true
        } else {
            let mut found_straight = false;
            for i in 0..=unique_ranks.len() - 5 {
                let window = &unique_ranks[i..i + 5];
                let is_consecutive = window.windows(2).all(|w| (w[1] as u8) - (w[0] as u8) == 1);
                if is_consecutive {
                    found_straight = true;
                    break;
                }
            }
            found_straight
        }
    } else {
        false
    };

    let rank_counts: Vec<(Rank, usize)> = {
        let mut counts: Vec<(Rank, usize)> = Vec::new();
        for &r in &ranks {
            if let Some(pos) = counts.iter().position(|(rank, _)| *rank == r) {
                counts[pos].1 += 1;
            } else {
                counts.push((r, 1));
            }
        }
        counts.sort_by_key(|(_, count)| *count);
        counts
    };

    let four_of_kind = rank_counts
        .iter()
        .find(|(_, count)| *count == 4)
        .map(|(rank, _)| *rank);
    let three_of_kind = rank_counts
        .iter()
        .find(|(_, count)| *count == 3)
        .map(|(rank, _)| *rank);
    let pairs: Vec<Rank> = rank_counts
        .iter()
        .filter(|(_, count)| *count == 2)
        .map(|(rank, _)| *rank)
        .collect();

    if is_flush && is_straight {
        let has_wheel =
            unique_ranks == vec![Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Ace];
        let straight_high = if has_wheel {
            Rank::Five
        } else {
            unique_ranks.iter().max().copied().unwrap_or(Rank::Ace)
        };
        return EvaluatedHand {
            hand_rank: HandRank::StraightFlush,
            primary_values: vec![straight_high],
            kickers: Vec::new(),
        };
    }

    if let Some(four) = four_of_kind {
        let kicker = ranks
            .iter()
            .find(|&&r| r != four)
            .copied()
            .unwrap_or(Rank::Two);
        return EvaluatedHand {
            hand_rank: HandRank::FourOfAKind,
            primary_values: vec![four],
            kickers: vec![kicker],
        };
    }

    if let Some(three) = three_of_kind {
        if !pairs.is_empty() {
            let pair = pairs[0];
            return EvaluatedHand {
                hand_rank: HandRank::FullHouse,
                primary_values: vec![three, pair],
                kickers: Vec::new(),
            };
        }
    }

    if is_flush {
        let flush_values: Vec<Rank> = cards.iter().map(|c| c.rank).rev().collect();
        return EvaluatedHand {
            hand_rank: HandRank::Flush,
            primary_values: flush_values,
            kickers: Vec::new(),
        };
    }

    if is_straight {
        let has_wheel =
            unique_ranks == vec![Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Ace];
        let straight_high = if has_wheel {
            Rank::Five
        } else {
            unique_ranks.iter().max().copied().unwrap_or(Rank::Ace)
        };
        return EvaluatedHand {
            hand_rank: HandRank::Straight,
            primary_values: vec![straight_high],
            kickers: Vec::new(),
        };
    }

    if let Some(three) = three_of_kind {
        let kickers: Vec<Rank> = ranks
            .iter()
            .filter(|&&r| r != three)
            .copied()
            .rev()
            .take(2)
            .collect();
        return EvaluatedHand {
            hand_rank: HandRank::ThreeOfAKind,
            primary_values: vec![three],
            kickers,
        };
    }

    if pairs.len() >= 2 {
        let mut sorted_pairs = pairs.clone();
        sorted_pairs.sort();
        let kicker: Vec<Rank> = ranks
            .iter()
            .filter(|&&r| !sorted_pairs.contains(&r))
            .copied()
            .rev()
            .take(1)
            .collect();
        return EvaluatedHand {
            hand_rank: HandRank::TwoPair,
            primary_values: sorted_pairs,
            kickers: kicker,
        };
    }

    if pairs.len() == 1 {
        let pair = pairs[0];
        let kickers: Vec<Rank> = ranks
            .iter()
            .filter(|&&r| r != pair)
            .copied()
            .rev()
            .take(3)
            .collect();
        return EvaluatedHand {
            hand_rank: HandRank::Pair,
            primary_values: vec![pair],
            kickers,
        };
    }

    let high_cards: Vec<Rank> = ranks.iter().copied().rev().collect();
    EvaluatedHand {
        hand_rank: HandRank::HighCard,
        primary_values: high_cards.clone(),
        kickers: Vec::new(),
    }
}

pub fn determine_winner(
    p1_hole: &[Card; 2],
    p2_hole: &[Card; 2],
    community_cards: &[Card; 5],
) -> (i32, bool) {
    let player1_hand: Vec<Card> = p1_hole
        .iter()
        .chain(community_cards.iter())
        .cloned()
        .collect();
    let player2_hand: Vec<Card> = p2_hole
        .iter()
        .chain(community_cards.iter())
        .cloned()
        .collect();

    let eval1 = evaluate_hand(&player1_hand);
    let eval2 = evaluate_hand(&player2_hand);

    let score1 = eval1.score();
    let score2 = eval2.score();

    if score1 > score2 {
        (0, true)
    } else if score2 > score1 {
        (1, true)
    } else {
        (-1, false)
    }
}
