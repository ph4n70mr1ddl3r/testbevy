use crate::constants::*;
use crate::game::*;
use crate::poker_logic::{Deck, PokerRound};
use bevy::prelude::*;

/// System that handles betting actions with AI decision making.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn handle_betting(
    config: Res<GameConfig>,
    mut game_state: ResMut<GameStateResource>,
    time: Res<Time>,
) {
    let action_delay = config.action_delay;
    let elapsed = time.elapsed_seconds() - game_state.animation_start_time;

    let current_tick = ((elapsed - BETTING_INITIAL_DELAY).max(0.0) / action_delay) as u32;
    if elapsed > BETTING_INITIAL_DELAY && current_tick > game_state.action_tick {
        perform_validated_action(&mut game_state, &config);
        game_state.action_tick = current_tick;
    }
}

/// System that checks game flow and manages round transitions.
pub fn check_game_flow(mut game_state: ResMut<GameStateResource>, time: Res<Time>) {
    if game_state.current_round == PokerRound::Showdown {
        game_state.showdown_timer -= time.delta_seconds();
    }
}

/// System that handles showdown resolution and hand cleanup.
pub fn handle_showdown(mut game_state: ResMut<GameStateResource>) {
    if game_state.current_round == PokerRound::Showdown && game_state.showdown_timer <= 0.0 {
        if game_state.winner.is_none() {
            process_showdown_result(&mut game_state);
        }

        game_state.current_round = PokerRound::PreFlop;
        game_state.showdown_timer = -1.0;
        game_state.needs_hand_restart = true;
    }
}

/// Starts a new hand, resetting all game state and spawning entities.
pub fn start_hand(
    commands: &mut Commands,
    game_state: &mut GameStateResource,
    config: &GameConfig,
    colors: &ColorPalette,
    time: &Time,
) {
    let animation_start_time = time.elapsed_seconds();
    game_state.pot = 0;
    game_state.pot_remainder = 0;
    game_state.current_round = PokerRound::PreFlop;
    game_state.last_action = "New hand".to_string();
    if game_state.hand_number > 0 {
        game_state.hand_number += 1;
    } else {
        game_state.hand_number = 1;
    }
    game_state.showdown_timer = 0.0;
    game_state.dealer_position = (game_state.dealer_position + 1) % PLAYER_COUNT;
    game_state.current_player = (game_state.dealer_position + 1) % PLAYER_COUNT;
    game_state.player_bets = [0; PLAYER_COUNT];
    game_state.current_bet = 0;
    game_state.winner = None;
    game_state.last_winner_message = "".to_string();

    if game_state.deck.cards_remaining() < config.min_cards_for_reshuffle {
        game_state.deck = Deck::new();
    }

    use crate::ui::*;
    spawn_table(commands, config.screen_width, config.screen_height, *colors);
    spawn_all_players(commands, game_state, config, *colors, animation_start_time);
    spawn_all_community_cards(commands, game_state, config, colors, animation_start_time);
    spawn_ui(commands, game_state, config, colors);
}

/// System that manages hand cleanup between rounds.
pub fn cleanup_old_hand(
    mut commands: Commands,
    hand_query: Query<Entity, With<HandMarker>>,
    mut game_state: ResMut<GameStateResource>,
) {
    if game_state.needs_cleanup {
        for entity in hand_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        game_state.needs_cleanup = false;
    }
}

/// System that triggers hand start when appropriate.
pub fn start_hand_system(
    mut commands: Commands,
    mut game_state: ResMut<GameStateResource>,
    config: Res<GameConfig>,
    colors: Res<ColorPalette>,
    time: Res<Time>,
) {
    if game_state.needs_hand_restart {
        game_state.needs_cleanup = true;
        game_state.animation_start_time = time.elapsed_seconds();
        game_state.showdown_timer = 0.0;
        game_state.action_tick = 0;
        game_state.winner = None;
        game_state.last_winner_message = "".to_string();
        game_state.needs_hand_restart = false;
        start_hand(&mut commands, &mut game_state, &config, &colors, time);
    }
}

/// System that sets up the initial game state.
pub fn setup_game(
    mut commands: Commands,
    mut game_state: ResMut<GameStateResource>,
    config: Res<GameConfig>,
) {
    commands.spawn((Camera2d, HandMarker));
    game_state.hand_number = 0;
    game_state.player_chips = [config.starting_chips; PLAYER_COUNT];
    game_state.player_bets = [0; PLAYER_COUNT];
    game_state.current_bet = 0;
    game_state.winner = None;
    game_state.dealer_position = 0;
    game_state.needs_hand_restart = true;
}
