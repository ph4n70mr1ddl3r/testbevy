//! Game constants for poker simulation
//!
//! This module contains all constant values used throughout the game,
//! organized by category for easy maintenance.

/// Number of players in the game (heads-up = 2)
pub const PLAYER_COUNT: usize = 2;

// Font sizes for UI text elements
/// Font size for the pot display
pub const POT_FONT_SIZE: f32 = 22.0;
/// Font size for hand number display
pub const HAND_NUMBER_FONT_SIZE: f32 = 14.0;
/// Font size for player chip count
pub const PLAYER_CHIPS_FONT_SIZE: f32 = 16.0;
/// Font size for opponent chip count
pub const OPPONENT_CHIPS_FONT_SIZE: f32 = 14.0;
/// Font size for round indicator
pub const ROUND_FONT_SIZE: f32 = 18.0;
/// Font size for action text
pub const ACTION_FONT_SIZE: f32 = 16.0;
/// Font size for community card text
pub const COMMUNITY_CARD_FONT_SIZE: f32 = 12.0;
/// Font size for player labels (YOU/OPP)
pub const PLAYER_LABEL_FONT_SIZE: f32 = 20.0;
/// Font size for chip labels
pub const CHIP_LABEL_FONT_SIZE: f32 = 18.0;

// Animation and timing constants
/// Initial delay before betting actions begin (seconds)
pub const BETTING_INITIAL_DELAY: f32 = 1.0;

// UI positioning ratios (relative to screen dimensions)
/// Y position ratio for top player (as fraction of screen height)
pub const PLAYER_Y_TOP_RATIO: f32 = 0.25;
/// Y position ratio for bottom player (as fraction of screen height)
pub const PLAYER_Y_BOTTOM_RATIO: f32 = -0.32;

// Table rendering positions
/// Z-index for dark table layer
pub const TABLE_DARK_Z: f32 = 0.0;
/// Y offset for dark table layer
pub const TABLE_DARK_Y: f32 = -20.0;
/// Z-index for light table layer (above dark)
pub const TABLE_LIGHT_Z: f32 = 0.1;
/// Y offset for light table layer
pub const TABLE_LIGHT_Y: f32 = -30.0;

// Card text positioning offsets
/// X offset for top-left card text
pub const CARD_TEXT_TOP_OFFSET_X: f32 = 8.0;
/// Y offset for top-left card text
pub const CARD_TEXT_TOP_OFFSET_Y: f32 = -12.0;
/// X offset for bottom-right card text
pub const CARD_TEXT_BOTTOM_OFFSET_X: f32 = -8.0;
/// Y offset for bottom-right card text
pub const CARD_TEXT_BOTTOM_OFFSET_Y: f32 = 12.0;

// Chip display positions
/// Y position for player chips display
pub const PLAYER_CHIPS_Y: f32 = -260.0;
/// Y position for opponent chips display
pub const OPPONENT_CHIPS_Y: f32 = 60.0;

// Table dimension ratios (relative to screen size)
/// Height ratio for dark table background
pub const TABLE_DARK_HEIGHT_RATIO: f32 = 0.55;
/// Width ratio for dark table background
pub const TABLE_DARK_WIDTH_RATIO: f32 = 1.0;
/// Height ratio for light table background
pub const TABLE_LIGHT_HEIGHT_RATIO: f32 = 0.48;
/// Width ratio for light table background
pub const TABLE_LIGHT_WIDTH_RATIO: f32 = 0.94;

// Z positions for rendering layers (higher = closer to camera)
/// Z position for player cards
pub const CARD_Z_POSITION: f32 = 1.0;
/// Z position for card text (slightly above cards)
pub const CARD_TEXT_Z_POSITION: f32 = 1.1;
/// Z position for community cards
pub const COMMUNITY_CARD_Z_POSITION: f32 = 0.5;
/// Target Z position for cards after animation
pub const CARD_TARGET_Z: f32 = 1.0;
/// Z position for UI text elements
pub const UI_TEXT_Z_POSITION: f32 = 1.0;

// Card counts revealed per betting round
/// Number of cards revealed on the flop
pub const FLOP_CARD_COUNT: usize = 3;
/// Number of cards revealed on the turn
pub const TURN_CARD_COUNT: usize = 4;
/// Number of cards revealed on the river
pub const RIVER_CARD_COUNT: usize = 5;

// Card positioning offsets
/// Offset for centering player cards (0.5 = center between 2 cards)
pub const PLAYER_CARD_CENTER_OFFSET: f32 = 0.5;
/// Center index for community cards (0-4, centered on 2)
pub const COMMUNITY_CARD_CENTER_INDEX: f32 = 2.0;

// Card evaluation constants
/// Minimum cards required for hand evaluation (5-card poker hand)
pub const MIN_CARDS_FOR_HAND_EVALUATION: usize = 5;

// AI decision thresholds
/// Hand strength below which AI will fold (unless pot odds are good)
/// Range: 0.0-1.0, where higher values mean stronger hands
pub const AI_STRENGTH_FOLD_THRESHOLD: f32 = 0.25;
/// Minimum hand strength for AI to call
/// Range: 0.0-1.0, should be higher than FOLD_THRESHOLD
pub const AI_STRENGTH_CALL_THRESHOLD: f32 = 0.5;
/// Minimum hand strength for AI to raise/bet
/// Range: 0.0-1.0, should be higher than CALL_THRESHOLD
pub const AI_STRENGTH_RAISE_THRESHOLD: f32 = 0.7;
/// Pot odds threshold considered unfavorable (fold if strength < 0.4)
/// Pot odds = call_amount / (pot + call_amount)
pub const AI_POT_ODDS_BAD_THRESHOLD: f32 = 0.3;
/// Maximum pot odds for calling with medium strength (strength >= 0.5)
pub const AI_POT_ODDS_CALL_THRESHOLD: f32 = 0.25;
/// Pot odds threshold considered favorable for calling with marginal hands
/// Used when hand strength is between 0.3 and CALL_THRESHOLD
pub const AI_POT_ODDS_GOOD_THRESHOLD: f32 = 0.2;
/// Bonus to hand strength when in dealer position (acts last)
/// Applied to all betting rounds
pub const AI_POSITION_BONUS: f32 = 0.1;
/// Bonus to hand strength preflop when in dealer position
/// Additional to POSITION_BONUS during preflop only
pub const AI_PREFLOP_DEALER_BONUS: f32 = 0.05;
/// Penalty to hand strength preflop when not in dealer position
/// Applied during preflop only to discourage early position play
pub const AI_PREFLOP_NON_DEALER_PENALTY: f32 = -0.05;
