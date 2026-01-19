// Player and game setup
pub const PLAYER_COUNT: usize = 2;

// Animation constants
pub const ANIMATION_CARD_DEAL_DELAY: f32 = 0.2;
pub const ANIMATION_DEAL_DURATION: f32 = 0.5;
pub const ANIMATION_COMMUNITY_DELAY_START: f32 = 0.9;
pub const ANIMATION_COMMUNITY_DELAY_INCREMENT: f32 = 0.15;
pub const ANIMATION_COMMUNITY_DURATION: f32 = 0.4;
pub const ANIMATION_EASING_POWER: i32 = 3;

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
pub const SHOWDOWN_TIMER_RESET_THRESHOLD: f32 = -0.5;

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

// Z positions
pub const CARD_Z_POSITION: f32 = 1.0;
pub const CARD_TEXT_Z_POSITION: f32 = 1.1;
pub const COMMUNITY_CARD_Z_POSITION: f32 = 0.5;
pub const CARD_TARGET_Z: f32 = 1.0;
pub const UI_TEXT_Z_POSITION: f32 = 1.0;

// Card counts
pub const FLOP_CARD_COUNT: usize = 3;
pub const TURN_CARD_COUNT: usize = 4;
pub const RIVER_CARD_COUNT: usize = 5;

// Card positioning
pub const PLAYER_CARD_CENTER_OFFSET: f32 = 0.5;
pub const COMMUNITY_CARD_CENTER_INDEX: f32 = 2.0;

// Card evaluation
pub const MIN_CARDS_FOR_HAND_EVALUATION: usize = 5;
