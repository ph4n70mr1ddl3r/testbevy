use crate::constants::*;
use crate::poker_logic::{determine_winner, Card, Deck, HandRank, PokerRound};
use bevy::prelude::*;

/// Configuration resource for game settings including display dimensions,
/// animation timing, betting amounts, and UI layout positions.
#[derive(Resource, Clone, Copy)]
pub struct UIPositioning {
    pub pot_display_y: f32,
    pub hand_number_x: f32,
    pub hand_number_y: f32,
    pub round_display_x: f32,
    pub round_display_y: f32,
    pub action_display_y: f32,
    pub player_label_offset: f32,
    pub chip_label_offset: f32,
}

impl Default for UIPositioning {
    fn default() -> Self {
        Self {
            pot_display_y: 130.0,
            hand_number_x: -160.0,
            hand_number_y: 360.0,
            round_display_x: 140.0,
            round_display_y: 360.0,
            action_display_y: -180.0,
            player_label_offset: 20.0,
            chip_label_offset: -5.0,
        }
    }
}

#[derive(Clone)]
pub struct AnimationConfig {
    pub card_deal_delay: f32,
    pub deal_duration: f32,
    pub community_delay_start: f32,
    pub community_delay_increment: f32,
    pub community_duration: f32,
    pub easing_power: i32,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            card_deal_delay: 0.2,
            deal_duration: 0.5,
            community_delay_start: 0.9,
            community_delay_increment: 0.15,
            community_duration: 0.4,
            easing_power: 3,
        }
    }
}

#[derive(Resource)]
pub struct GameConfig {
    pub card_width: f32,
    pub card_height: f32,
    pub card_offset_spacing: f32,
    pub community_card_scale: f32,
    pub card_target_y_offset: f32,
    pub animation_start_y: f32,
    pub community_card_start_y: f32,
    pub action_delay: f32,
    pub showdown_duration: f32,
    pub fold_showdown_duration: f32,
    pub starting_chips: u32,
    pub bet_amount: u32,
    pub raise_amount: u32,
    pub screen_width: f32,
    pub screen_height: f32,
    pub ui_positions: UIPositioning,
    pub animations: AnimationConfig,
    pub min_cards_for_reshuffle: usize,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            card_width: 55.0,
            card_height: 77.0,
            card_offset_spacing: 65.0,
            community_card_scale: 0.85,
            card_target_y_offset: 100.0,
            animation_start_y: 350.0,
            community_card_start_y: 280.0,
            action_delay: 2.5,
            showdown_duration: 5.0,
            fold_showdown_duration: 3.0,
            starting_chips: 1000,
            bet_amount: 50,
            raise_amount: 100,
            screen_width: 375.0,
            screen_height: 812.0,
            ui_positions: UIPositioning::default(),
            animations: AnimationConfig::default(),
            min_cards_for_reshuffle: 9,
        }
    }
}

/// Color palette resource for consistent styling across the game UI.
/// Contains all color values used for cards, table, text, and chips.
#[derive(Resource, Clone, Copy, PartialEq)]
pub struct ColorPalette {
    pub card_text_red: Color,
    pub card_text_black: Color,
    pub table_green_dark: Color,
    pub table_green_light: Color,
    pub face_up_white: Color,
    pub face_down_dark: Color,
    pub text_gray_dim: Color,
    pub text_gray_light: Color,
    pub text_gray_med: Color,
    pub text_white: Color,
    pub chip_gold: Color,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            card_text_red: Color::srgb(0.85, 0.0, 0.0),
            card_text_black: Color::srgb(0.1, 0.1, 0.1),
            table_green_dark: Color::srgb(0.12, 0.45, 0.18),
            table_green_light: Color::srgb(0.18, 0.55, 0.22),
            face_up_white: Color::srgb(0.98, 0.95, 0.95),
            face_down_dark: Color::srgb(0.2, 0.3, 0.2),
            text_gray_dim: Color::srgb(0.6, 0.6, 0.6),
            text_gray_light: Color::srgb(0.8, 0.8, 0.8),
            text_gray_med: Color::srgb(0.7, 0.7, 0.7),
            text_white: Color::srgb(0.9, 0.9, 0.9),
            chip_gold: Color::srgb(1.0, 0.85, 0.0),
        }
    }
}

#[derive(Component)]
pub struct CardEntity;

#[derive(Component)]
pub struct HandMarker;

#[derive(Component, Default)]
pub struct DealAnimation {
    pub start_pos: Vec3,
    pub target_pos: Vec3,
    pub start_time: f32,
    pub duration: f32,
    pub delay: f32,
}

#[derive(Resource, Default)]
/// Main game state resource tracking all game data including deck, pot,
/// current round, player states, chips, bets, and community cards.
pub struct GameStateResource {
    pub deck: Deck,
    pub pot: u32,
    pub pot_remainder: u32,
    pub current_round: PokerRound,
    pub dealer_position: usize,
    pub current_player: usize,
    pub last_action: String,
    pub showdown_timer: f32,
    pub action_tick: u32,
    pub hand_number: i32,
    pub animation_start_time: f32,
    pub player_chips: [u32; 2],
    pub player_bets: [u32; 2],
    pub current_bet: u32,
    pub needs_cleanup: bool,
    pub winner: Option<usize>,
    pub last_winner_message: String,
    pub p1_hole: [Card; 2],
    pub p2_hole: [Card; 2],
    pub community_cards: [Card; 5],
    pub needs_hand_restart: bool,
}

#[derive(Component)]
pub struct CommunityCard {
    pub index: usize,
    pub is_hidden: bool,
}

#[derive(Component)]
pub struct PotDisplay;

#[derive(Component)]
pub struct HandNumberDisplay;

#[derive(Component)]
pub struct PlayerChipsDisplay;

#[derive(Component)]
pub struct OpponentChipsDisplay;

#[derive(Component)]
pub struct RoundDisplay;

#[derive(Component, Default)]
pub struct ActionDisplay;

/// Represents all possible poker actions a player can take during a betting round.
/// The derived `Ord` implementation follows standard poker action ordering.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PokerAction {
    Check,
    Bet,
    Call,
    Raise,
    Fold,
}

/// Evaluates a hand rank and primary card value to produce a normalized score (0.0-1.0).
/// Used by AI to compare hand strengths quantitatively.
///
/// # Score Calculation
/// - Base score from hand rank (0.1-0.9)
/// - Normalized primary value added as decimal (0.0-0.1)
///
/// # Precision Note
/// Card ranks are 2-14, well within f32 precision. The conversion is safe.
fn evaluate_hand_rank_score(hand_rank: HandRank, primary_value: u8) -> f32 {
    let base_score = match hand_rank {
        HandRank::HighCard => 0.1,
        HandRank::Pair => 0.2,
        HandRank::TwoPair => 0.3,
        HandRank::ThreeOfAKind => 0.4,
        HandRank::Straight => 0.5,
        HandRank::Flush => 0.6,
        HandRank::FullHouse => 0.7,
        HandRank::FourOfAKind => 0.8,
        HandRank::StraightFlush => 0.9,
    };
    // Safe: primary_value is a card rank (2-14), well within f32 precision
    let normalized = (f32::from(primary_value) / 13.0) * 0.1;
    base_score + normalized
}

/// Evaluates hand strength as a value between 0.0 and 1.0
/// where 1.0 is the strongest possible hand (royal flush).
pub fn evaluate_current_hand_strength(game_state: &GameStateResource) -> f32 {
    let player_idx = game_state.current_player;
    let hole_cards = if player_idx == 0 {
        game_state.p1_hole
    } else {
        game_state.p2_hole
    };
    let mut cards: Vec<Card> = hole_cards.to_vec();

    // Add community cards
    cards.extend(
        game_state
            .community_cards
            .iter()
            .copied()
            .filter(|card| !card.is_placeholder),
    );

    if cards.len() < 5 {
        // Preflop: simple evaluation based on card ranks
        // Safe conversion: card ranks are 2-14, well within u8 range
        let ranks: Vec<u8> = cards.iter().map(|c| c.rank as u8).collect();
        let mut score = 0.0;
        for &rank in &ranks {
            score += f32::from(rank) / 13.0; // Normalize to 0-1
        }
        score / 2.0 // Average
    } else {
        // Postflop: evaluate hand
        let evaluated = evaluate_hand(&cards);
        let primary_value = evaluated
            .primary_values
            .first()
            .copied()
            .unwrap_or(Rank::Two);
        evaluate_hand_rank_score(evaluated.hand_rank, primary_value as u8)
    }
}

/// Chooses an action based on hand strength, position, and pot odds.
/// Uses a more sophisticated strategy considering multiple factors.
pub fn choose_action_based_on_strength<'a>(
    actions: &'a [PokerAction],
    strength: f32,
    game_state: &GameStateResource,
) -> &'a PokerAction {
    let current_bet = game_state.current_bet;
    let player_bet = game_state.player_bets[game_state.current_player];
    let to_call = current_bet.saturating_sub(player_bet);
    let pot_size = game_state.pot + game_state.pot_remainder;

    // Calculate pot odds: ratio of call amount to total pot after call
    // Used for AI decision making, minor precision loss is acceptable
    let pot_odds = if to_call > 0 {
        f32::from(to_call) / (f32::from(pot_size) + f32::from(to_call))
    } else {
        0.0
    };

    // Position advantage (dealer acts last)
    let is_dealer = game_state.current_player == game_state.dealer_position;
    let position_bonus = if is_dealer { AI_POSITION_BONUS } else { 0.0 };

    // Adjust strength based on position and pot odds
    let adjusted_strength = (strength + position_bonus).min(1.0);

    // Preflop adjustments
    let preflop_adjustment = if game_state.current_round == PokerRound::PreFlop {
        // Be more aggressive preflop with position
        if is_dealer {
            AI_PREFLOP_DEALER_BONUS
        } else {
            AI_PREFLOP_NON_DEALER_PENALTY
        }
    } else {
        0.0
    };
    let final_strength = (adjusted_strength + preflop_adjustment).clamp(0.0, 1.0);

    // Decision thresholds based on strength and pot odds
    if final_strength < AI_STRENGTH_FOLD_THRESHOLD
        || (final_strength < 0.4 && pot_odds > AI_POT_ODDS_BAD_THRESHOLD)
    {
        // Weak hand or bad pot odds: fold if possible
        if actions.contains(&PokerAction::Fold) && to_call > 0 {
            if let Some(fold_action) = actions.iter().find(|a| matches!(a, PokerAction::Fold)) {
                return fold_action;
            }
        }
    }

    if final_strength >= AI_STRENGTH_RAISE_THRESHOLD {
        // Very strong hand: raise or bet
        if actions.contains(&PokerAction::Raise) {
            if let Some(raise_action) = actions.iter().find(|a| matches!(a, PokerAction::Raise)) {
                return raise_action;
            }
        } else if actions.contains(&PokerAction::Bet) {
            if let Some(bet_action) = actions.iter().find(|a| matches!(a, PokerAction::Bet)) {
                return bet_action;
            }
        }
    } else if final_strength >= AI_STRENGTH_CALL_THRESHOLD {
        // Medium-strong hand: call or check
        if actions.contains(&PokerAction::Check) {
            if let Some(check_action) = actions.iter().find(|a| matches!(a, PokerAction::Check)) {
                return check_action;
            }
        } else if actions.contains(&PokerAction::Call) && pot_odds < AI_POT_ODDS_CALL_THRESHOLD {
            if let Some(call_action) = actions.iter().find(|a| matches!(a, PokerAction::Call)) {
                return call_action;
            }
        }
    } else if final_strength >= 0.3 {
        // Medium hand: check or call with good pot odds
        if actions.contains(&PokerAction::Check) {
            if let Some(check_action) = actions.iter().find(|a| matches!(a, PokerAction::Check)) {
                return check_action;
            }
        } else if actions.contains(&PokerAction::Call) && pot_odds < AI_POT_ODDS_GOOD_THRESHOLD {
            if let Some(call_action) = actions.iter().find(|a| matches!(a, PokerAction::Call)) {
                return call_action;
            }
        }
    }

    // Default: check if available, otherwise first available action
    actions
        .iter()
        .find(|a| matches!(a, PokerAction::Check))
        .unwrap_or(&actions[0])
}

/// Returns all valid actions for the current player given the game state.
pub fn get_valid_actions(game_state: &GameStateResource, config: &GameConfig) -> Vec<PokerAction> {
    let mut actions = Vec::new();
    let player_idx = game_state.current_player;
    let player_chips = game_state.player_chips[player_idx];
    let player_bet = game_state.player_bets[player_idx];
    let current_bet = game_state.current_bet;

    // Check is only available when no bet has been made this round
    if current_bet == 0 {
        actions.push(PokerAction::Check);
    }

    if current_bet > 0 {
        let call_amount = current_bet - player_bet;
        if player_chips >= call_amount && call_amount > 0 {
            actions.push(PokerAction::Call);
        }
        let raise_cost = call_amount + config.raise_amount;
        if player_chips >= raise_cost {
            actions.push(PokerAction::Raise);
        }
    } else if player_chips >= config.bet_amount {
        actions.push(PokerAction::Bet);
    }

    actions.push(PokerAction::Fold);
    actions
}

/// Places a bet for the current player, updating chips and pot accordingly.
/// Ensures no negative chip counts and proper bounds checking.
pub fn place_bet(
    game_state: &mut GameStateResource,
    amount: u32,
    is_raise: bool,
    new_current_bet: u32,
) {
    let player_idx = game_state.current_player;
    if player_idx >= PLAYER_COUNT {
        error!("Invalid player index: {}", player_idx);
        return;
    }

    let available_chips = game_state.player_chips[player_idx];
    let actual_amount = amount.min(available_chips);
    game_state.player_chips[player_idx] -= actual_amount;
    game_state.player_bets[player_idx] += actual_amount;
    game_state.pot += actual_amount;

    if is_raise {
        game_state.current_bet = new_current_bet;
    }
}

/// Advances to the next betting round when both players have matched bets.
pub fn advance_street(game_state: &mut GameStateResource, config: &GameConfig) {
    let both_players_matched_bet = game_state.player_bets[0] == game_state.current_bet
        && game_state.player_bets[1] == game_state.current_bet;

    let can_check = game_state.current_bet == 0;

    if both_players_matched_bet || can_check {
        match game_state.current_round {
            PokerRound::PreFlop => game_state.current_round = PokerRound::Flop,
            PokerRound::Flop => {
                game_state.current_round = PokerRound::Turn;
            }
            PokerRound::Turn => {
                game_state.current_round = PokerRound::River;
            }
            PokerRound::River => {
                game_state.current_round = PokerRound::Showdown;
                game_state.showdown_timer = config.showdown_duration;
            }
            PokerRound::Showdown => {}
        }

        if game_state.current_round != PokerRound::Showdown {
            game_state.current_bet = 0;
            game_state.player_bets = [0; PLAYER_COUNT];
            game_state.current_player = game_state.dealer_position;
            game_state.pot_remainder = 0;
        }
    }
}

/// Performs the AI's betting action based on hand strength evaluation.
pub fn perform_validated_action(game_state: &mut GameStateResource, config: &GameConfig) {
    let actions = get_valid_actions(game_state, config);
    if actions.is_empty() {
        return;
    }

    let hand_strength = evaluate_current_hand_strength(game_state);
    let action = choose_action_based_on_strength(&actions, hand_strength, game_state);

    let player_idx = game_state.current_player;
    match action {
        PokerAction::Check => {
            game_state.last_action = format!("P{}: Check", player_idx + 1);
        }
        PokerAction::Bet => {
            let bet_amount = config.bet_amount;
            if game_state.player_chips[player_idx] >= bet_amount {
                place_bet(game_state, bet_amount, true, bet_amount);
                game_state.last_action = format!("P{}: Bet ${}", player_idx + 1, bet_amount);
            } else if game_state.player_chips[player_idx] > 0 {
                let all_in_amount = game_state.player_chips[player_idx];
                place_bet(game_state, all_in_amount, true, all_in_amount);
                game_state.last_action = format!("P{}: All-in", player_idx + 1);
            } else {
                game_state.last_action = format!("P{}: Check (no chips)", player_idx + 1);
            }
        }
        PokerAction::Call => {
            let call_amount = game_state
                .current_bet
                .saturating_sub(game_state.player_bets[game_state.current_player]);
            if call_amount > 0 && game_state.player_chips[player_idx] >= call_amount {
                place_bet(game_state, call_amount, false, 0);
                game_state.last_action = format!("P{}: Call", player_idx + 1);
            } else if call_amount > 0 && game_state.player_chips[player_idx] > 0 {
                let all_in_amount = game_state.player_chips[player_idx];
                place_bet(game_state, all_in_amount, false, 0);
                game_state.last_action = format!("P{}: Call all-in", player_idx + 1);
            }
        }
        PokerAction::Raise => {
            let raise_amount = game_state.current_bet + config.raise_amount;
            let actual_raise = raise_amount - game_state.player_bets[game_state.current_player];
            if game_state.player_chips[player_idx] >= actual_raise {
                place_bet(game_state, actual_raise, true, raise_amount);
                game_state.last_action =
                    format!("P{}: Raise ${}", player_idx + 1, config.raise_amount);
            } else if game_state.player_chips[player_idx] > 0 {
                let all_in_amount = game_state.player_chips[player_idx];
                let player_bet = game_state.player_bets[game_state.current_player];
                place_bet(game_state, all_in_amount, true, player_bet + all_in_amount);
                game_state.last_action = format!("P{}: All-in", player_idx + 1);
            } else {
                game_state.last_action = format!("P{}: Check (no chips)", player_idx + 1);
            }
        }
        PokerAction::Fold => {
            let winner = (game_state.current_player + 1) % 2;
            game_state.winner = Some(winner);
            let total_pot = game_state.pot + game_state.pot_remainder;
            game_state.player_chips[winner] += total_pot;
            game_state.last_winner_message = format!(
                "P{} folds - P{} wins",
                game_state.current_player + 1,
                winner + 1
            );
            game_state.pot = 0;
            game_state.pot_remainder = 0;
            game_state.current_round = PokerRound::Showdown;
            game_state.showdown_timer = config.fold_showdown_duration;
            return;
        }
    }

    game_state.current_player = (game_state.current_player + 1) % PLAYER_COUNT;
    advance_street(game_state, config);
}

/// Draws a card from the deck, reshuffling if necessary.
/// Returns an error if no cards are available.
pub fn draw_card(game_state: &mut GameStateResource) -> Result<Card, &'static str> {
    if let Some(c) = game_state.deck.draw() {
        Ok(c)
    } else {
        warn!("Deck empty - creating new deck");
        game_state.deck = Deck::new();
        game_state.deck.draw().ok_or("Failed to draw from new deck")
    }
}

/// Distributes the pot to the winning player and clears the pot.
pub fn distribute_pot(game_state: &mut GameStateResource, winner: usize) {
    let total_pot = game_state.pot + game_state.pot_remainder;
    game_state.player_chips[winner] += total_pot;
    game_state.last_winner_message = if winner == 0 { "P1 wins" } else { "P2 wins" }.to_string();
    game_state.pot = 0;
    game_state.pot_remainder = 0;
}

/// Splits the pot between both players in case of a tie and clears the pot.
/// In case of an odd pot, the extra chip goes to the dealer (following standard poker rules).
pub fn split_pot(game_state: &mut GameStateResource) {
    let total_pot = game_state.pot + game_state.pot_remainder;
    let split_amount = total_pot / 2;
    let remainder = total_pot % 2;
    let dealer = game_state.dealer_position;
    let other_player = (dealer + 1) % PLAYER_COUNT;

    // Dealer gets the remainder chip (if any) per standard poker rules
    game_state.player_chips[dealer] += split_amount + remainder;
    game_state.player_chips[other_player] += split_amount;
    game_state.pot = 0;
    game_state.pot_remainder = 0;
    game_state.last_winner_message = "Split pot".to_string();
}

/// Processes the showdown result and awards the pot to the winner(s).
pub fn process_showdown_result(game_state: &mut GameStateResource) {
    let result = determine_winner(
        &game_state.p1_hole,
        &game_state.p2_hole,
        &game_state.community_cards,
    );

    match result {
        0 => {
            game_state.winner = Some(0);
            distribute_pot(game_state, 0);
        }
        1 => {
            game_state.winner = Some(1);
            distribute_pot(game_state, 1);
        }
        _ => {
            split_pot(game_state);
        }
    }
}
