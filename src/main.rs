use bevy::prelude::*;
use rand::seq::SliceRandom;

mod poker_logic;
use poker_logic::{Card, Deck, PokerRound};

const CARD_WIDTH: f32 = 55.0;
const CARD_HEIGHT: f32 = 77.0;
const ACTION_DELAY: f32 = 2.5;
const SHOWDOWN_DURATION: f32 = 5.0;
const PREFLOP_POT_INCREMENT: i32 = 20;
const FLOP_POT_INCREMENT: i32 = 30;
const TURN_POT_INCREMENT: i32 = 40;
const RIVER_POT_INCREMENT: i32 = 50;
const STARTING_CHIPS: i32 = 1000;

#[derive(Component)]
struct CardEntity {
    card: Card,
}

#[derive(Component)]
struct HandMarker;

#[derive(Component)]
struct DealAnimation {
    start_pos: Vec3,
    target_pos: Vec3,
    start_time: f32,
    duration: f32,
    delay: f32,
}

#[derive(Resource, Default)]
struct GameStateResource {
    deck: Deck,
    pot: i32,
    current_round: PokerRound,
    dealer_position: usize,
    current_player: usize,
    street_cards: Vec<Card>,
    last_action: String,
    showdown_timer: f32,
    hand_number: i32,
    animation_start_time: f32,
    player_chips: [i32; 2],
    needs_cleanup: bool,
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
        .init_resource::<GameStateResource>()
        .add_systems(Startup, setup_game)
        .add_systems(
            Update,
            (
                cleanup_old_hand,
                start_hand_system,
                handle_betting,
                update_animations,
                check_game_flow,
                handle_showdown,
                update_card_visuals,
                update_ui,
            ),
        )
        .run();
}

fn setup_game(mut commands: Commands, mut game_state: ResMut<GameStateResource>) {
    commands.spawn(Camera2dBundle::default());
    game_state.hand_number = 1;
    game_state.player_chips = [STARTING_CHIPS, STARTING_CHIPS];
    game_state.deck = Deck::new();
}

fn start_hand_system(
    mut commands: Commands,
    mut game_state: ResMut<GameStateResource>,
    time: Res<Time>,
) {
    if game_state.hand_number == 1 || game_state.showdown_timer < -0.5 {
        game_state.needs_cleanup = true;
        game_state.animation_start_time = time.elapsed_seconds();
        game_state.showdown_timer = 0.0;
        game_state.deck = Deck::new();
        start_hand(&mut commands, &mut game_state);
    }
}

fn start_hand(commands: &mut Commands, game_state: &mut GameStateResource) {
    game_state.pot = 0;
    game_state.current_round = PokerRound::PreFlop;
    game_state.street_cards.clear();
    game_state.last_action = format!("Hand #{}", game_state.hand_number);
    game_state.hand_number += 1;
    game_state.showdown_timer = 0.0;
    game_state.dealer_position = (game_state.dealer_position + 1) % 2;
    game_state.current_player = (game_state.dealer_position + 1) % 2;

    let screen_width = 375.0;
    let screen_height = 812.0;

    spawn_table(commands, screen_width, screen_height);

    let player_y_top = screen_height * 0.25;
    let player_y_bottom = -screen_height * 0.32;

    for id in 0..2 {
        spawn_player(
            commands,
            game_state,
            id,
            0.0,
            if id == 0 {
                player_y_top
            } else {
                player_y_bottom
            },
        );
    }

    for i in 0..5 {
        spawn_community_card(commands, game_state, i);
    }

    spawn_ui(commands, game_state);
}

fn spawn_table(commands: &mut Commands, screen_width: f32, screen_height: f32) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.12, 0.45, 0.18),
                custom_size: Some(Vec2::new(screen_width, screen_height * 0.55)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -20.0, 0.0),
            ..default()
        },
        HandMarker,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.18, 0.55, 0.22),
                custom_size: Some(Vec2::new(screen_width * 0.94, screen_height * 0.48)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -30.0, 0.1),
            ..default()
        },
        HandMarker,
    ));
}

fn spawn_player(
    commands: &mut Commands,
    game_state: &mut GameStateResource,
    id: usize,
    x_pos: f32,
    y_pos: f32,
) {
    let card_target_y = y_pos + 100.0;

    for j in 0..2 {
        let card_offset = (j as f32 - 0.5) * 65.0;
        let target_pos = Vec3::new(x_pos + card_offset, card_target_y, 1.0);
        let card = game_state.deck.draw().unwrap_or(Card::placeholder());

        let card_color = if card.is_red() {
            Color::srgb(0.98, 0.95, 0.95)
        } else {
            Color::srgb(0.95, 0.98, 0.98)
        };
        let text_color = if card.is_red() {
            Color::srgb(0.85, 0.1, 0.1)
        } else {
            Color::srgb(0.1, 0.1, 0.1)
        };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: card_color,
                    custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 350.0, 1.0),
                ..default()
            },
            CardEntity { card },
            DealAnimation {
                start_pos: Vec3::new(0.0, 350.0, 1.0),
                target_pos,
                start_time: 0.0,
                duration: 0.5,
                delay: (id * 2 + j) as f32 * 0.2,
            },
            HandMarker,
        ));

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    format!("{}\n{}", card.rank_str(), card.suit_str()),
                    TextStyle {
                        font_size: 14.0,
                        color: text_color,
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(
                    target_pos.x - CARD_WIDTH / 2.0 + 8.0,
                    target_pos.y + CARD_HEIGHT / 2.0 - 12.0,
                    1.1,
                ),
                ..default()
            },
            HandMarker,
        ));

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    format!("{}\n{}", card.rank_str(), card.suit_str()),
                    TextStyle {
                        font_size: 14.0,
                        color: text_color,
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(
                    target_pos.x + CARD_WIDTH / 2.0 - 8.0,
                    target_pos.y - CARD_HEIGHT / 2.0 + 12.0,
                    1.1,
                )
                .with_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
                ..default()
            },
            HandMarker,
        ));
    }

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                if id == 0 { "YOU" } else { "OPP" },
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            transform: Transform::from_xyz(x_pos, y_pos + 20.0, 1.0),
            ..default()
        },
        HandMarker,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("${}", STARTING_CHIPS),
                TextStyle {
                    font_size: 18.0,
                    color: Color::srgb(1.0, 0.85, 0.0),
                    ..default()
                },
            ),
            transform: Transform::from_xyz(x_pos, y_pos - 5.0, 1.0),
            ..default()
        },
        HandMarker,
    ));
}

fn spawn_community_card(commands: &mut Commands, game_state: &mut GameStateResource, i: usize) {
    let x_offset = (i as f32 - 2.0) * 65.0;
    let community_card = if i < 3 {
        game_state.deck.draw().unwrap_or(Card::placeholder())
    } else {
        Card::placeholder()
    };

    let card_color = if community_card.is_red() {
        Color::srgb(0.98, 0.95, 0.95)
    } else {
        Color::srgb(0.95, 0.98, 0.98)
    };
    let text_color = if community_card.is_red() {
        Color::srgb(0.85, 0.1, 0.1)
    } else {
        Color::srgb(0.1, 0.1, 0.1)
    };

    let target_pos = Vec3::new(x_offset, 0.0, 0.5);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: card_color,
                custom_size: Some(Vec2::new(CARD_WIDTH * 0.85, CARD_HEIGHT * 0.85)),
                ..default()
            },
            transform: Transform::from_xyz(x_offset, 280.0, 0.5),
            ..default()
        },
        CardEntity {
            card: community_card,
        },
        DealAnimation {
            start_pos: Vec3::new(x_offset, 280.0, 0.5),
            target_pos,
            start_time: 0.0,
            duration: 0.4,
            delay: 0.9 + i as f32 * 0.15,
        },
        HandMarker,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!(
                    "{}\n{}",
                    community_card.rank_str(),
                    community_card.suit_str()
                ),
                TextStyle {
                    font_size: 12.0,
                    color: text_color,
                    ..default()
                },
            ),
            transform: Transform::from_xyz(
                x_offset - CARD_WIDTH * 0.85 / 2.0 + 6.0,
                CARD_HEIGHT * 0.85 / 2.0 - 8.0,
                0.6,
            ),
            ..default()
        },
        HandMarker,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!(
                    "{}\n{}",
                    community_card.rank_str(),
                    community_card.suit_str()
                ),
                TextStyle {
                    font_size: 12.0,
                    color: text_color,
                    ..default()
                },
            ),
            transform: Transform::from_xyz(
                x_offset + CARD_WIDTH * 0.85 / 2.0 - 6.0,
                -CARD_HEIGHT * 0.85 / 2.0 + 8.0,
                0.6,
            )
            .with_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
            ..default()
        },
        HandMarker,
    ));
}

fn spawn_ui(commands: &mut Commands, game_state: &mut GameStateResource) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("Pot: ${}", game_state.pot),
                TextStyle {
                    font_size: 22.0,
                    color: Color::srgb(1.0, 0.85, 0.0),
                    ..default()
                },
            ),
            transform: Transform::from_xyz(0.0, 130.0, 1.0),
            ..default()
        },
        PotDisplay,
        HandMarker,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("Hand: #{}", game_state.hand_number),
                TextStyle {
                    font_size: 14.0,
                    color: Color::srgb(0.6, 0.6, 0.6),
                    ..default()
                },
            ),
            transform: Transform::from_xyz(-160.0, 360.0, 1.0),
            ..default()
        },
        HandNumberDisplay,
        HandMarker,
    ));
}

fn cleanup_old_hand(
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

fn handle_betting(mut game_state: ResMut<GameStateResource>, time: Res<Time>) {
    let action_delay = ACTION_DELAY;
    let elapsed = time.elapsed_seconds() - game_state.animation_start_time;

    if elapsed > 1.0 && (elapsed % action_delay) < time.delta_seconds() {
        perform_random_action(&mut game_state);
    }
}

fn perform_random_action(game_state: &mut GameStateResource) {
    let actions = ["Check", "Bet $50", "Call", "Raise $100", "Fold"];
    let action = actions.choose(&mut rand::thread_rng()).unwrap();
    game_state.last_action = format!("{}", action);

    match game_state.current_round {
        PokerRound::PreFlop => game_state.pot += PREFLOP_POT_INCREMENT,
        PokerRound::Flop => game_state.pot += FLOP_POT_INCREMENT,
        PokerRound::Turn => game_state.pot += TURN_POT_INCREMENT,
        PokerRound::River => game_state.pot += RIVER_POT_INCREMENT,
        PokerRound::Showdown => {}
    }

    advance_street(game_state);
}

fn advance_street(game_state: &mut GameStateResource) {
    match game_state.current_round {
        PokerRound::PreFlop => game_state.current_round = PokerRound::Flop,
        PokerRound::Flop => game_state.current_round = PokerRound::Turn,
        PokerRound::Turn => game_state.current_round = PokerRound::River,
        PokerRound::River => {
            game_state.current_round = PokerRound::Showdown;
            game_state.showdown_timer = SHOWDOWN_DURATION;
        }
        PokerRound::Showdown => {}
    }
}

fn update_animations(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut DealAnimation)>,
) {
    let elapsed = time.elapsed_seconds();

    for (entity, mut transform, anim) in query.iter_mut() {
        let anim_elapsed = elapsed - anim.start_time - anim.delay;

        if anim_elapsed > 0.0 {
            let t = (anim_elapsed / anim.duration).min(1.0);
            let eased = 1.0 - (1.0 - t).powi(3);
            transform.translation = anim.start_pos.lerp(anim.target_pos, eased);

            if t >= 1.0 {
                commands.entity(entity).remove::<DealAnimation>();
            }
        }
    }
}

fn check_game_flow(mut game_state: ResMut<GameStateResource>, time: Res<Time>) {
    if game_state.current_round == PokerRound::Showdown {
        game_state.showdown_timer -= time.delta_seconds();
    }
}

fn handle_showdown(mut commands: Commands, mut game_state: ResMut<GameStateResource>) {
    if game_state.current_round == PokerRound::Showdown && game_state.showdown_timer <= 0.0 {
        game_state.current_round = PokerRound::PreFlop;
        game_state.pot = 0;
        game_state.street_cards.clear();
        game_state.last_action = format!("Hand #{}", game_state.hand_number);
        game_state.showdown_timer = -1.0;
        start_hand(&mut commands, &mut game_state);
    }
}

fn update_card_visuals(mut query: Query<(&mut Sprite, &CardEntity)>) {
    for (mut sprite, card_entity) in query.iter_mut() {
        let card = card_entity.card;
        let color = if card.is_red() {
            Color::srgb(0.98, 0.95, 0.95)
        } else {
            Color::srgb(0.95, 0.98, 0.98)
        };
        sprite.color = color;
    }
}

fn update_ui(game_state: Res<GameStateResource>, mut text_query: Query<&mut Text>) {
    let pot_text = format!("Pot: ${}", game_state.pot);
    let hand_text = format!("Hand: #{}", game_state.hand_number);
    let chip_text = format!("${}", STARTING_CHIPS);

    for mut text in text_query.iter_mut() {
        if text.sections[0].value.starts_with("Pot:") {
            text.sections[0].value = pot_text.clone();
        } else if text.sections[0].value.starts_with("Hand:") {
            text.sections[0].value = hand_text.clone();
        } else if text.sections[0].value.starts_with("$") {
            text.sections[0].value = chip_text.clone();
        }
    }
}

#[derive(Component)]
struct PotDisplay;

#[derive(Component)]
struct HandNumberDisplay;
