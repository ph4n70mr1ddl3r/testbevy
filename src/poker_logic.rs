use crate::constants::MIN_CARDS_FOR_HAND_EVALUATION;
use rand::{seq::SliceRandom, thread_rng};
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
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
    pub is_placeholder: bool,
}

impl Default for Card {
    fn default() -> Self {
        Self {
            rank: Rank::Two,
            suit: Suit::Hearts,
            is_placeholder: true,
        }
    }
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card {
            rank,
            suit,
            is_placeholder: false,
        }
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

const SUITS: [Suit; 4] = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
const RANKS: [Rank; 13] = [
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
];

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
        for &suit in &SUITS {
            for &rank in &RANKS {
                cards.push(Card::new(rank, suit));
            }
        }
        cards.shuffle(&mut thread_rng());
        Deck { cards }
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Returns the number of cards remaining in the deck.
    pub fn cards_remaining(&self) -> usize {
        self.cards.len()
    }
}

/// Finds the highest straight in a set of ranks using bit manipulation.
/// Returns the high card of the straight (e.g., for A-K-Q-J-10, returns Ace).
/// Also handles the wheel straight (A-2-3-4-5) where 5 is the high card.
/// Uses bit masking to efficiently check for consecutive ranks.
fn find_straight_high(ranks: &HashSet<Rank>) -> Option<Rank> {
    if ranks.len() < 5 {
        return None;
    }

    // Bit pattern for wheel straight: A,2,3,4,5 (Ace=14, Five=5)
    const WHEEL_BITS: u16 = (1 << 14) | (1 << 2) | (1 << 3) | (1 << 4) | (1 << 5);
    // Mask for 5 consecutive bits
    const STRAIGHT_MASK: u16 = 0b11111;

    let mut rank_bits: u16 = 0;
    for &rank in ranks {
        let idx = rank as u8;
        if (2..=14).contains(&idx) {
            rank_bits |= 1 << idx;
        }
    }

    let has_wheel = (rank_bits & WHEEL_BITS) == WHEEL_BITS;

    const STRAIGHT_HIGH_MAP: [(u16, Rank); 10] = [
        (STRAIGHT_MASK << 10, Rank::Ace),
        (STRAIGHT_MASK << 9, Rank::King),
        (STRAIGHT_MASK << 8, Rank::Queen),
        (STRAIGHT_MASK << 7, Rank::Jack),
        (STRAIGHT_MASK << 6, Rank::Ten),
        (STRAIGHT_MASK << 5, Rank::Nine),
        (STRAIGHT_MASK << 4, Rank::Eight),
        (STRAIGHT_MASK << 3, Rank::Seven),
        (STRAIGHT_MASK << 2, Rank::Six),
        (STRAIGHT_MASK << 1, Rank::Five),
    ];

    for (mask, rank) in STRAIGHT_HIGH_MAP.iter() {
        if (rank_bits & mask) == *mask {
            return Some(*rank);
        }
    }

    if has_wheel {
        Some(Rank::Five)
    } else {
        None
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
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

/// Represents the ranking of a poker hand.
/// The derived `Ord` implementation follows standard poker hand rankings:
/// HighCard < Pair < TwoPair < ThreeOfAKind < Straight < Flush < FullHouse < FourOfAKind < StraightFlush
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    pub fn score(&self) -> (HandRank, &[Rank]) {
        (self.hand_rank, &self.primary_values)
    }
}

/// Evaluates a poker hand and returns its ranking and relevant card values.
/// Uses standard poker hand ranking rules with proper tie-breaking logic.
///
/// # Algorithm Overview
/// 1. Filter out placeholder cards and validate minimum hand size
/// 2. Count suits and ranks to identify flush and pair possibilities
/// 3. Check for straight patterns using bit manipulation
/// 4. Evaluate hands in ranking order (straight flush → four of a kind → etc.)
/// 5. For each hand type, extract primary values and kickers for tie-breaking
///
/// # Tie-breaking Rules
/// - Higher rank cards break ties
/// - Kickers are used when primary ranks are equal
/// - For flushes and straights, the highest card in the pattern wins
pub fn evaluate_hand(cards: &[Card]) -> EvaluatedHand {
    let non_placeholder_count = cards.iter().filter(|c| !c.is_placeholder).count();
    if non_placeholder_count < MIN_CARDS_FOR_HAND_EVALUATION {
        return EvaluatedHand {
            hand_rank: HandRank::HighCard,
            primary_values: Vec::new(),
            kickers: Vec::new(),
        };
    }

    let mut cards_vec: Vec<Card> = cards
        .iter()
        .filter(|c| !c.is_placeholder)
        .cloned()
        .collect();
    cards_vec.sort_by_key(|c| c.rank);

    let suit_counts: HashMap<Suit, usize> = {
        let mut counts = HashMap::new();
        for c in cards_vec.iter() {
            *counts.entry(c.suit).or_insert(0) += 1;
        }
        counts
    };
    let is_flush = suit_counts.values().any(|&count| count >= 5);

    let ranks: Vec<Rank> = cards_vec.iter().map(|c| c.rank).collect();
    let unique_ranks: HashSet<Rank> = ranks.iter().copied().collect();

    let straight_high = find_straight_high(&unique_ranks);
    let is_straight = straight_high.is_some();

    let rank_counts: HashMap<Rank, usize> = {
        let mut counts = HashMap::new();
        for &r in &ranks {
            *counts.entry(r).or_insert(0) += 1;
        }
        counts
    };

    let mut rank_counts_vec: Vec<(Rank, usize)> = rank_counts.into_iter().collect();
    rank_counts_vec.sort_by_key(|(rank, count)| (*count, Reverse(*rank)));

    let four_of_kind = rank_counts_vec
        .iter()
        .find(|(_, count)| *count == 4)
        .map(|(rank, _)| *rank);
    let three_of_kind = rank_counts_vec
        .iter()
        .find(|(_, count)| *count == 3)
        .map(|(rank, _)| *rank);
    let pairs: Vec<Rank> = rank_counts_vec
        .iter()
        .filter(|(_, count)| *count == 2)
        .map(|(rank, _)| *rank)
        .collect();

    // Check for straight flush: must be both a straight and a flush
    if is_flush && is_straight {
        // Find the suit that has 5+ cards (the flush suit)
        let flush_suit = suit_counts
            .iter()
            .find(|(_, &count)| count >= 5)
            .map(|(suit, _)| *suit);

        if let Some(flush_suit) = flush_suit {
            let flush_cards: Vec<Card> = cards_vec
                .iter()
                .filter(|c| c.suit == flush_suit)
                .cloned()
                .collect();
            let flush_ranks: Vec<Rank> = flush_cards.iter().map(|c| c.rank).collect();
            let flush_unique: HashSet<Rank> = flush_ranks.iter().copied().collect();

            if let Some(straight_high) = find_straight_high(&flush_unique) {
                return EvaluatedHand {
                    hand_rank: HandRank::StraightFlush,
                    primary_values: vec![straight_high],
                    kickers: Vec::new(),
                };
            }
        }
    }

    if let Some(four) = four_of_kind {
        let kicker: Vec<Rank> = ranks
            .iter()
            .filter(|&&r| r != four)
            .copied()
            .max()
            .map(|r| vec![r])
            .unwrap_or_default();
        return EvaluatedHand {
            hand_rank: HandRank::FourOfAKind,
            primary_values: vec![four],
            kickers: kicker,
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
        let flush_suit = suit_counts
            .iter()
            .find(|(_, &count)| count >= 5)
            .map(|(suit, _)| *suit)
            .expect("Flush suit should exist when is_flush is true");
        let flush_values: Vec<Rank> = cards_vec
            .iter()
            .filter(|c| c.suit == flush_suit)
            .map(|c| c.rank)
            .rev()
            .collect();
        return EvaluatedHand {
            hand_rank: HandRank::Flush,
            primary_values: flush_values,
            kickers: Vec::new(),
        };
    }

    if is_straight {
        return EvaluatedHand {
            hand_rank: HandRank::Straight,
            primary_values: vec![straight_high.unwrap()],
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
        let mut sorted_pairs = pairs;
        sorted_pairs.sort_by_key(|&r| Reverse(r));
        let top_two_pairs: Vec<Rank> = sorted_pairs.iter().take(2).copied().collect();
        let kicker: Vec<Rank> = ranks
            .iter()
            .filter(|&&r| !top_two_pairs.contains(&r))
            .copied()
            .rev()
            .take(1)
            .collect();
        return EvaluatedHand {
            hand_rank: HandRank::TwoPair,
            primary_values: top_two_pairs,
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
        primary_values: high_cards,
        kickers: Vec::new(),
    }
}

pub fn determine_winner(
    p1_hole: &[Card; 2],
    p2_hole: &[Card; 2],
    community_cards: &[Card; 5],
) -> i32 {
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

    match score1.cmp(&score2) {
        std::cmp::Ordering::Greater => 0,
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(rank: Rank, suit: Suit) -> Card {
        Card::new(rank, suit)
    }

    #[test]
    fn test_high_card() {
        let hand = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Spades),
            card(Rank::Ten, Suit::Diamonds),
            card(Rank::Five, Suit::Clubs),
            card(Rank::Three, Suit::Hearts),
            card(Rank::Two, Suit::Spades),
            card(Rank::Eight, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::HighCard);
        assert_eq!(eval.primary_values[0], Rank::Ace);
    }

    #[test]
    fn test_pair() {
        let hand = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Diamonds),
            card(Rank::Ten, Suit::Clubs),
            card(Rank::Five, Suit::Hearts),
            card(Rank::Two, Suit::Spades),
            card(Rank::Eight, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::Pair);
        assert_eq!(eval.primary_values[0], Rank::Ace);
    }

    #[test]
    fn test_two_pair() {
        let hand = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Diamonds),
            card(Rank::King, Suit::Clubs),
            card(Rank::Ten, Suit::Hearts),
            card(Rank::Two, Suit::Spades),
            card(Rank::Eight, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::TwoPair);
    }

    #[test]
    fn test_three_of_a_kind() {
        let hand = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Diamonds),
            card(Rank::King, Suit::Clubs),
            card(Rank::Ten, Suit::Hearts),
            card(Rank::Two, Suit::Spades),
            card(Rank::Eight, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::ThreeOfAKind);
        assert_eq!(eval.primary_values[0], Rank::Ace);
    }

    #[test]
    fn test_flush() {
        let hand = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Hearts),
            card(Rank::Ten, Suit::Hearts),
            card(Rank::Five, Suit::Hearts),
            card(Rank::Three, Suit::Hearts),
            card(Rank::Two, Suit::Spades),
            card(Rank::Eight, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::Flush);
        assert_eq!(eval.primary_values[0], Rank::Ace);
    }

    #[test]
    fn test_full_house() {
        let hand = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Diamonds),
            card(Rank::King, Suit::Clubs),
            card(Rank::King, Suit::Hearts),
            card(Rank::Two, Suit::Spades),
            card(Rank::Eight, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::FullHouse);
        assert_eq!(eval.primary_values[0], Rank::Ace);
        assert_eq!(eval.primary_values[1], Rank::King);
    }

    #[test]
    fn test_four_of_a_kind() {
        let hand = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Diamonds),
            card(Rank::Ace, Suit::Clubs),
            card(Rank::King, Suit::Hearts),
            card(Rank::Two, Suit::Spades),
            card(Rank::Eight, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::FourOfAKind);
        assert_eq!(eval.primary_values[0], Rank::Ace);
    }

    #[test]
    fn test_straight_flush() {
        let hand = [
            card(Rank::Seven, Suit::Hearts),
            card(Rank::Three, Suit::Hearts),
            card(Rank::Four, Suit::Hearts),
            card(Rank::Five, Suit::Hearts),
            card(Rank::Six, Suit::Hearts),
            card(Rank::King, Suit::Spades),
            card(Rank::Eight, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::StraightFlush);
        assert_eq!(eval.primary_values[0], Rank::Seven);
    }

    #[test]
    fn test_wheel_straight_flush() {
        let hand = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Two, Suit::Spades),
            card(Rank::Three, Suit::Hearts),
            card(Rank::Four, Suit::Hearts),
            card(Rank::Five, Suit::Hearts),
            card(Rank::King, Suit::Spades),
            card(Rank::Eight, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        // This is a straight (wheel), not a straight flush (only 4 hearts)
        assert_eq!(eval.hand_rank, HandRank::Straight);
        assert_eq!(eval.primary_values[0], Rank::Five);
    }

    #[test]
    fn test_determine_winner() {
        let p1 = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Spades),
        ];
        let p2 = [
            card(Rank::King, Suit::Hearts),
            card(Rank::Queen, Suit::Spades),
        ];
        let community = [
            card(Rank::Ten, Suit::Diamonds),
            card(Rank::Jack, Suit::Clubs),
            card(Rank::Two, Suit::Hearts),
            card(Rank::Five, Suit::Spades),
            card(Rank::Eight, Suit::Diamonds),
        ];

        let result = determine_winner(&p1, &p2, &community);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_determine_winner_split() {
        let p1 = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Spades),
        ];
        let p2 = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
        ];
        let community = [
            card(Rank::Ten, Suit::Diamonds),
            card(Rank::Jack, Suit::Clubs),
            card(Rank::Two, Suit::Hearts),
            card(Rank::Five, Suit::Spades),
            card(Rank::Eight, Suit::Diamonds),
        ];

        let result = determine_winner(&p1, &p2, &community);
        assert_eq!(result, -1);
    }

    #[test]
    fn test_royal_flush() {
        let hand = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Hearts),
            card(Rank::Queen, Suit::Hearts),
            card(Rank::Jack, Suit::Hearts),
            card(Rank::Ten, Suit::Hearts),
            card(Rank::Two, Suit::Spades),
            card(Rank::Eight, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::StraightFlush);
        assert_eq!(eval.primary_values[0], Rank::Ace);
    }

    #[test]
    fn test_wheel_straight() {
        let hand = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Two, Suit::Spades),
            card(Rank::Three, Suit::Hearts),
            card(Rank::Four, Suit::Diamonds),
            card(Rank::Five, Suit::Clubs),
            card(Rank::King, Suit::Spades),
            card(Rank::Eight, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::Straight);
        assert_eq!(eval.primary_values[0], Rank::Five);
    }

    #[test]
    fn test_broadway_straight() {
        let hand = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Hearts),
            card(Rank::Jack, Suit::Diamonds),
            card(Rank::Ten, Suit::Clubs),
            card(Rank::Two, Suit::Spades),
            card(Rank::Eight, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::Straight);
        assert_eq!(eval.primary_values[0], Rank::Ace);
    }

    #[test]
    fn test_straight_detects_highest() {
        let hand = [
            card(Rank::Two, Suit::Hearts),
            card(Rank::Three, Suit::Spades),
            card(Rank::Four, Suit::Hearts),
            card(Rank::Five, Suit::Diamonds),
            card(Rank::Six, Suit::Clubs),
            card(Rank::King, Suit::Spades),
            card(Rank::Eight, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::Straight);
        assert_eq!(eval.primary_values[0], Rank::Six);
    }

    #[test]
    fn test_full_house_with_three_of_kind_beats_two_pair() {
        let p1 = [card(Rank::Ace, Suit::Hearts), card(Rank::Ace, Suit::Spades)];
        let p2 = [
            card(Rank::King, Suit::Hearts),
            card(Rank::King, Suit::Spades),
        ];
        let community = [
            card(Rank::Ace, Suit::Diamonds),
            card(Rank::Two, Suit::Clubs),
            card(Rank::Two, Suit::Hearts),
            card(Rank::Five, Suit::Spades),
            card(Rank::Eight, Suit::Diamonds),
        ];

        let result = determine_winner(&p1, &p2, &community);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_four_of_kind_beats_full_house() {
        let p1 = [card(Rank::Ace, Suit::Hearts), card(Rank::Ace, Suit::Spades)];
        let p2 = [
            card(Rank::King, Suit::Hearts),
            card(Rank::King, Suit::Spades),
        ];
        let community = [
            card(Rank::Ace, Suit::Diamonds),
            card(Rank::King, Suit::Clubs),
            card(Rank::Two, Suit::Hearts),
            card(Rank::Two, Suit::Spades),
            card(Rank::Eight, Suit::Diamonds),
        ];

        let result = determine_winner(&p1, &p2, &community);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_straight_flush_beats_four_of_kind() {
        let p1 = [
            card(Rank::Seven, Suit::Hearts),
            card(Rank::Eight, Suit::Hearts),
        ];
        let p2 = [card(Rank::Ace, Suit::Hearts), card(Rank::Ace, Suit::Spades)];
        let community = [
            card(Rank::Ace, Suit::Diamonds),
            card(Rank::Ace, Suit::Clubs),
            card(Rank::Nine, Suit::Hearts),
            card(Rank::Ten, Suit::Hearts),
            card(Rank::Jack, Suit::Hearts),
        ];

        let result = determine_winner(&p1, &p2, &community);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_kickers_matter() {
        let p1 = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Spades),
        ];
        let p2 = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::Queen, Suit::Hearts),
        ];
        let community = [
            card(Rank::Ten, Suit::Diamonds),
            card(Rank::Jack, Suit::Clubs),
            card(Rank::Two, Suit::Hearts),
            card(Rank::Five, Suit::Spades),
            card(Rank::Eight, Suit::Diamonds),
        ];

        let result = determine_winner(&p1, &p2, &community);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_flush_beats_straight() {
        let p1 = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Hearts),
        ];
        let p2 = [
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Diamonds),
        ];
        let community = [
            card(Rank::Ten, Suit::Spades),
            card(Rank::Jack, Suit::Clubs),
            card(Rank::Two, Suit::Hearts),
            card(Rank::Five, Suit::Spades),
            card(Rank::Eight, Suit::Diamonds),
        ];

        let result = determine_winner(&p1, &p2, &community);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_deck_cards_remaining() {
        let mut deck = Deck::new();
        assert_eq!(deck.cards_remaining(), 52);

        for _ in 0..5 {
            deck.draw().unwrap();
        }
        assert_eq!(deck.cards_remaining(), 47);
    }

    #[test]
    fn test_card_is_placeholder() {
        let placeholder = Card::default();
        assert!(placeholder.is_placeholder);

        let real_card = Card::new(Rank::Ace, Suit::Spades);
        assert!(!real_card.is_placeholder);
    }

    #[test]
    fn test_three_of_kind_kickers() {
        let hand = [
            card(Rank::Seven, Suit::Hearts),
            card(Rank::Seven, Suit::Spades),
            card(Rank::Seven, Suit::Diamonds),
            card(Rank::Four, Suit::Clubs),
            card(Rank::Three, Suit::Hearts),
            card(Rank::Two, Suit::Spades),
            card(Rank::Nine, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::ThreeOfAKind);
        assert_eq!(eval.primary_values[0], Rank::Seven);
        assert_eq!(eval.kickers.len(), 2);
    }

    #[test]
    fn test_two_pair_kicker() {
        let hand = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Diamonds),
            card(Rank::King, Suit::Clubs),
            card(Rank::Ten, Suit::Hearts),
            card(Rank::Two, Suit::Spades),
            card(Rank::Eight, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::TwoPair);
        assert_eq!(eval.primary_values[0], Rank::Ace);
        assert_eq!(eval.primary_values[1], Rank::King);
        assert_eq!(eval.kickers[0], Rank::Ten);
    }

    #[test]
    fn test_pair_with_three_kickers() {
        let hand = [
            card(Rank::Queen, Suit::Hearts),
            card(Rank::Queen, Suit::Spades),
            card(Rank::Jack, Suit::Diamonds),
            card(Rank::Nine, Suit::Clubs),
            card(Rank::Seven, Suit::Hearts),
            card(Rank::Five, Suit::Spades),
            card(Rank::Three, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::Pair);
        assert_eq!(eval.primary_values[0], Rank::Queen);
        assert_eq!(eval.kickers.len(), 3);
    }

    #[test]
    fn test_ace_low_straight() {
        let hand = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Two, Suit::Spades),
            card(Rank::Three, Suit::Diamonds),
            card(Rank::Four, Suit::Clubs),
            card(Rank::Five, Suit::Hearts),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::Straight);
        assert_eq!(eval.primary_values[0], Rank::Five);
    }

    #[test]
    fn test_straight_with_gap() {
        let hand = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Diamonds),
            card(Rank::Jack, Suit::Clubs),
            card(Rank::Nine, Suit::Hearts),
            card(Rank::Ten, Suit::Spades),
            card(Rank::Eight, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::Straight);
        assert_eq!(eval.primary_values[0], Rank::Ace);
    }

    #[test]
    fn test_high_card_values() {
        let hand = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Spades),
            card(Rank::Jack, Suit::Diamonds),
            card(Rank::Nine, Suit::Clubs),
            card(Rank::Seven, Suit::Hearts),
            card(Rank::Five, Suit::Spades),
            card(Rank::Three, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::HighCard);
        assert_eq!(eval.primary_values[0], Rank::Ace);
        assert_eq!(eval.primary_values[1], Rank::King);
        assert_eq!(eval.primary_values[2], Rank::Jack);
    }

    #[test]
    fn test_duplicate_ranks_in_full_house() {
        let hand = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Diamonds),
            card(Rank::Ace, Suit::Clubs),
            card(Rank::King, Suit::Hearts),
            card(Rank::King, Suit::Spades),
            card(Rank::Eight, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::FourOfAKind);
        assert_eq!(eval.primary_values[0], Rank::Ace);
    }

    #[test]
    fn test_evaluate_hand_empty() {
        let hand: [Card; 0] = [];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::HighCard);
        assert!(eval.primary_values.is_empty());
    }

    #[test]
    fn test_determine_winner_full_house_vs_flush() {
        let p1 = [card(Rank::Ace, Suit::Hearts), card(Rank::Ace, Suit::Spades)];
        let p2 = [
            card(Rank::King, Suit::Diamonds),
            card(Rank::King, Suit::Clubs),
        ];
        let community = [
            card(Rank::Ace, Suit::Diamonds),
            card(Rank::King, Suit::Hearts),
            card(Rank::Ten, Suit::Hearts),
            card(Rank::Jack, Suit::Hearts),
            card(Rank::Queen, Suit::Hearts),
        ];

        let result = determine_winner(&p1, &p2, &community);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_determine_winner_four_of_kind() {
        let p1 = [
            card(Rank::Seven, Suit::Hearts),
            card(Rank::Seven, Suit::Spades),
        ];
        let p2 = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Spades),
        ];
        let community = [
            card(Rank::Seven, Suit::Diamonds),
            card(Rank::Seven, Suit::Clubs),
            card(Rank::Two, Suit::Hearts),
            card(Rank::Five, Suit::Spades),
            card(Rank::Eight, Suit::Diamonds),
        ];

        let result = determine_winner(&p1, &p2, &community);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_determine_winner_straight_vs_flush() {
        let p1 = [
            card(Rank::Six, Suit::Hearts),
            card(Rank::Seven, Suit::Diamonds),
        ];
        let p2 = [
            card(Rank::King, Suit::Hearts),
            card(Rank::Queen, Suit::Hearts),
        ];
        let community = [
            card(Rank::Eight, Suit::Hearts),
            card(Rank::Nine, Suit::Hearts),
            card(Rank::Ten, Suit::Hearts),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Two, Suit::Clubs),
        ];

        let result = determine_winner(&p1, &p2, &community);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_determine_winner_three_of_kind_beats_two_pair() {
        let p1 = [
            card(Rank::Seven, Suit::Hearts),
            card(Rank::Seven, Suit::Diamonds),
        ];
        let p2 = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Hearts),
        ];
        let community = [
            card(Rank::Seven, Suit::Clubs),
            card(Rank::King, Suit::Diamonds),
            card(Rank::King, Suit::Spades),
            card(Rank::Two, Suit::Hearts),
            card(Rank::Three, Suit::Clubs),
        ];

        let result = determine_winner(&p1, &p2, &community);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_determine_winner_high_card_wins() {
        let p1 = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Two, Suit::Diamonds),
        ];
        let p2 = [
            card(Rank::King, Suit::Hearts),
            card(Rank::Queen, Suit::Diamonds),
        ];
        let community = [
            card(Rank::Ten, Suit::Spades),
            card(Rank::Eight, Suit::Clubs),
            card(Rank::Six, Suit::Hearts),
            card(Rank::Four, Suit::Diamonds),
            card(Rank::Three, Suit::Clubs),
        ];

        let result = determine_winner(&p1, &p2, &community);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_straight_flush_royal() {
        let hand = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Spades),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Ten, Suit::Spades),
            card(Rank::Two, Suit::Hearts),
            card(Rank::Three, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::StraightFlush);
        assert_eq!(eval.primary_values[0], Rank::Ace);
    }

    #[test]
    fn test_five_cards_only() {
        let hand = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Diamonds),
            card(Rank::Jack, Suit::Clubs),
            card(Rank::Ten, Suit::Hearts),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::Straight);
        assert_eq!(eval.primary_values[0], Rank::Ace);
    }

    #[test]
    fn test_three_pair_not_possible() {
        let hand = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Diamonds),
            card(Rank::King, Suit::Clubs),
            card(Rank::Queen, Suit::Hearts),
            card(Rank::Queen, Suit::Spades),
            card(Rank::Jack, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::TwoPair);
        assert_eq!(eval.primary_values.len(), 2);
        assert_eq!(eval.primary_values[0], Rank::Ace);
        assert_eq!(eval.primary_values[1], Rank::King);
    }

    #[test]
    fn test_full_house_with_aces_over_kings() {
        let hand = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Diamonds),
            card(Rank::King, Suit::Clubs),
            card(Rank::King, Suit::Hearts),
            card(Rank::Two, Suit::Spades),
            card(Rank::Three, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::FullHouse);
        assert_eq!(eval.primary_values[0], Rank::Ace);
        assert_eq!(eval.primary_values[1], Rank::King);
    }

    #[test]
    fn test_full_house_with_kings_over_aces() {
        let hand = [
            card(Rank::King, Suit::Hearts),
            card(Rank::King, Suit::Spades),
            card(Rank::King, Suit::Diamonds),
            card(Rank::Ace, Suit::Clubs),
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Two, Suit::Spades),
            card(Rank::Three, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::FullHouse);
        assert_eq!(eval.primary_values[0], Rank::King);
        assert_eq!(eval.primary_values[1], Rank::Ace);
    }

    #[test]
    fn test_straight_flush_low_end() {
        let hand = [
            card(Rank::Two, Suit::Hearts),
            card(Rank::Three, Suit::Hearts),
            card(Rank::Four, Suit::Hearts),
            card(Rank::Five, Suit::Hearts),
            card(Rank::Six, Suit::Hearts),
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::StraightFlush);
        assert_eq!(eval.primary_values[0], Rank::Six);
    }

    #[test]
    fn test_four_of_kind_kicker_order() {
        let hand = [
            card(Rank::Queen, Suit::Hearts),
            card(Rank::Queen, Suit::Spades),
            card(Rank::Queen, Suit::Diamonds),
            card(Rank::Queen, Suit::Clubs),
            card(Rank::King, Suit::Hearts),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Ace, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::FourOfAKind);
        assert_eq!(eval.primary_values[0], Rank::Queen);
        assert_eq!(eval.kickers[0], Rank::Ace);
    }

    #[test]
    fn test_full_house_aces_over_kings() {
        let hand = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Diamonds),
            card(Rank::King, Suit::Clubs),
            card(Rank::King, Suit::Hearts),
            card(Rank::Two, Suit::Spades),
            card(Rank::Three, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::FullHouse);
        assert_eq!(eval.primary_values[0], Rank::Ace);
        assert_eq!(eval.primary_values[1], Rank::King);
    }

    #[test]
    fn test_straight_with_duplicates() {
        let hand = [
            card(Rank::Six, Suit::Hearts),
            card(Rank::Six, Suit::Spades),
            card(Rank::Seven, Suit::Hearts),
            card(Rank::Eight, Suit::Diamonds),
            card(Rank::Nine, Suit::Clubs),
            card(Rank::Ten, Suit::Hearts),
            card(Rank::Five, Suit::Spades),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::Straight);
        assert_eq!(eval.primary_values[0], Rank::Ten);
    }

    #[test]
    fn test_no_straight_with_gap() {
        let hand = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Diamonds),
            card(Rank::Jack, Suit::Clubs),
            card(Rank::Nine, Suit::Hearts),
            card(Rank::Eight, Suit::Spades),
            card(Rank::Six, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::HighCard);
    }

    #[test]
    fn test_straight_6_high() {
        let hand = [
            card(Rank::Two, Suit::Hearts),
            card(Rank::Three, Suit::Spades),
            card(Rank::Four, Suit::Diamonds),
            card(Rank::Five, Suit::Clubs),
            card(Rank::Six, Suit::Hearts),
            card(Rank::King, Suit::Spades),
            card(Rank::Eight, Suit::Diamonds),
        ];
        let eval = evaluate_hand(&hand);
        assert_eq!(eval.hand_rank, HandRank::Straight);
        assert_eq!(eval.primary_values[0], Rank::Six);
    }
}
