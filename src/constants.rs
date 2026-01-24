// Player and game setup
pub const PLAYER_COUNT: usize = 2;

// Animation constants are now in GameConfig

// Font sizes
pub const POT_FONT_SIZE: f32 = 22.0;
pub const HAND_NUMBER_FONT_SIZE: f32 = 14.0;
pub const PLAYER_CHIPS_FONT_SIZE: f32 = 16.0;
pub const OPPONENT_CHIPS_FONT_SIZE: f32 = 14.0;
pub const ROUND_FONT_SIZE: f32 = 18.0;
pub const ACTION_FONT_SIZE: f32 = 16.0;
pub const COMMUNITY_CARD_FONT_SIZE: f32 = 12.0;
pub const PLAYER_LABEL_FONT_SIZE: f32 = 20.0;
pub const CHIP_LABEL_FONT_SIZE: f32 = 18.0;

// Betting delays
pub const BETTING_INITIAL_DELAY: f32 = 1.0;

// UI ratios and positions
pub const PLAYER_Y_TOP_RATIO: f32 = 0.25;
pub const PLAYER_Y_BOTTOM_RATIO: f32 = -0.32;
pub const TABLE_DARK_Z: f32 = 0.0;
pub const TABLE_DARK_Y: f32 = -20.0;
pub const TABLE_LIGHT_Z: f32 = 0.1;
pub const TABLE_LIGHT_Y: f32 = -30.0;
pub const CARD_TEXT_TOP_OFFSET_X: f32 = 8.0;
pub const CARD_TEXT_TOP_OFFSET_Y: f32 = -12.0;
pub const CARD_TEXT_BOTTOM_OFFSET_X: f32 = -8.0;
pub const CARD_TEXT_BOTTOM_OFFSET_Y: f32 = 12.0;
pub const PLAYER_CHIPS_Y: f32 = -260.0;
pub const OPPONENT_CHIPS_Y: f32 = 60.0;

// Table dimensions
pub const TABLE_DARK_HEIGHT_RATIO: f32 = 0.55;
pub const TABLE_DARK_WIDTH_RATIO: f32 = 1.0;
pub const TABLE_LIGHT_HEIGHT_RATIO: f32 = 0.48;
pub const TABLE_LIGHT_WIDTH_RATIO: f32 = 0.94;

// Z positions for rendering layers
pub const CARD_Z_POSITION: f32 = 1.0;
pub const CARD_TEXT_Z_POSITION: f32 = 1.1;
pub const COMMUNITY_CARD_Z_POSITION: f32 = 0.5;
pub const CARD_TARGET_Z: f32 = 1.0;
pub const UI_TEXT_Z_POSITION: f32 = 1.0;

// Card counts revealed per round
pub const FLOP_CARD_COUNT: usize = 3;
pub const TURN_CARD_COUNT: usize = 4;
pub const RIVER_CARD_COUNT: usize = 5;

// Card positioning offsets
pub const PLAYER_CARD_CENTER_OFFSET: f32 = 0.5;
pub const COMMUNITY_CARD_CENTER_INDEX: f32 = 2.0;

// Card evaluation
pub const MIN_CARDS_FOR_HAND_EVALUATION: usize = 5;

// AI decision thresholds
pub const AI_STRENGTH_FOLD_THRESHOLD: f32 = 0.25;
pub const AI_STRENGTH_CALL_THRESHOLD: f32 = 0.5;
pub const AI_STRENGTH_RAISE_THRESHOLD: f32 = 0.7;
pub const AI_POT_ODDS_BAD_THRESHOLD: f32 = 0.3;
pub const AI_POT_ODDS_CALL_THRESHOLD: f32 = 0.25;
pub const AI_POT_ODDS_GOOD_THRESHOLD: f32 = 0.2;
pub const AI_POSITION_BONUS: f32 = 0.1;
pub const AI_PREFLOP_DEALER_BONUS: f32 = 0.05;
pub const AI_PREFLOP_NON_DEALER_PENALTY: f32 = -0.05;
