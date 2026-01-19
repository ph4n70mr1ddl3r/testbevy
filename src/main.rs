//! # Poker Simulation Game
//!
//! A heads-up Texas Hold'em poker simulation built with Bevy.
//! Features AI opponents, smooth animations, and a complete poker rule implementation.

use bevy::prelude::*;

mod ai;
mod animation;
mod constants;
mod game;
mod poker_logic;
mod ui;

use crate::game::{AnimationConfig, UIPositioning};
use crate::poker_logic::{Card, Deck, PokerRound};

#[derive(Clone, Copy, Resource)]
struct GameConfig {
    card_width: f32,
    card_height: f32,
    card_offset_spacing: f32,
    community_card_scale: f32,
    card_target_y_offset: f32,
    animation_start_y: f32,
    community_card_start_y: f32,
    action_delay: f32,
    showdown_duration: f32,
    fold_showdown_duration: f32,
    starting_chips: u32,
    bet_amount: u32,
    raise_amount: u32,
    screen_width: f32,
    screen_height: f32,
    ui_positions: UIPositioning,
    animations: AnimationConfig,
    min_cards_for_reshuffle: usize,
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
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
            ui_positions: UIPositioning {
                pot_display_y: 130.0,
                hand_number_x: -160.0,
                hand_number_y: 360.0,
                round_display_x: 140.0,
                round_display_y: 360.0,
                action_display_y: -180.0,
                player_label_offset: 20.0,
                chip_label_offset: -5.0,
            },
            animations: AnimationConfig {
                card_deal_delay: 0.2,
                deal_duration: 0.5,
                community_delay_start: 0.9,
                community_delay_increment: 0.15,
                community_duration: 0.4,
                easing_power: 3,
            },
            min_cards_for_reshuffle: 9,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Resource)]
/// Color palette resource for consistent styling across the game UI.
/// Contains all color values used for cards, table, text, and chips.
struct ColorPalette {
    card_text_red: Color,
    card_text_black: Color,
    table_green_dark: Color,
    table_green_light: Color,
    face_up_white: Color,
    face_down_dark: Color,
    text_gray_dim: Color,
    text_gray_light: Color,
    text_gray_med: Color,
    text_white: Color,
    chip_gold: Color,
}

impl Default for ColorPalette {
    fn default() -> Self {
        ColorPalette {
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

/// Returns the display name for a poker round.
const fn get_round_name(round: PokerRound) -> &'static str {
    match round {
        PokerRound::PreFlop => "Pre-Flop",
        PokerRound::Flop => "Flop",
        PokerRound::Turn => "Turn",
        PokerRound::River => "River",
        PokerRound::Showdown => "Showdown",
    }
}

#[derive(Component)]
struct CardEntity;

#[derive(Component)]
struct HandMarker;

#[derive(Component, Default)]
struct DealAnimation {
    start_pos: Vec3,
    target_pos: Vec3,
    start_time: f32,
    duration: f32,
    delay: f32,
}

#[derive(Resource, Default)]
/// Main game state resource tracking all game data including deck, pot,
/// current round, player states, chips, bets, and community cards.
struct GameStateResource {
    deck: Deck,
    pot: u32,
    pot_remainder: u32,
    current_round: PokerRound,
    dealer_position: usize,
    current_player: usize,
    last_action: String,
    showdown_timer: f32,
    action_tick: u32,
    hand_number: i32,
    animation_start_time: f32,
    player_chips: [u32; 2],
    player_bets: [u32; 2],
    current_bet: u32,
    needs_cleanup: bool,
    winner: Option<usize>,
    last_winner_message: String,
    p1_hole: [Card; 2],
    p2_hole: [Card; 2],
    community_cards: [Card; 5],
}

#[derive(Component)]
struct CommunityCard {
    index: usize,
    is_hidden: bool,
}

#[derive(Component)]
struct PotDisplay;

#[derive(Component)]
struct HandNumberDisplay;

#[derive(Component)]
struct PlayerChipsDisplay;

#[derive(Component)]
struct OpponentChipsDisplay;

#[derive(Component)]
struct RoundDisplay;

#[derive(Component, Default)]
struct ActionDisplay;

#[derive(Clone, Copy, PartialEq, Eq)]
/// Represents all possible poker actions a player can take during a betting round.
enum PokerAction {
    Check,
    Bet,
    Call,
    Raise,
    Fold,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Heads-Up Poker".into(),
                resolution: (375.0, 812.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameConfig>()
        .init_resource::<ColorPalette>()
        .init_resource::<GameStateResource>()
        .add_systems(Startup, ai::setup_game)
        .add_systems(
            Update,
            (
                ai::cleanup_old_hand,
                ai::start_hand_system,
                ai::handle_betting,
                animation::update_animations,
                ai::check_game_flow,
                ai::handle_showdown,
                ui::update_card_visuals,
                ui::update_ui,
            )
                .chain(),
        )
        .run();
}
