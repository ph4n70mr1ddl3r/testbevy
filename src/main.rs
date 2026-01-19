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

use crate::game::*;

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
