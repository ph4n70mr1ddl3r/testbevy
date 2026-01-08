use bevy::prelude::*;
use rand::seq::SliceRandom;

mod poker_logic;
use poker_logic::*;

const CARD_WIDTH: f32 = 55.0;
const CARD_HEIGHT: f32 = 77.0;

#[derive(Component)]
struct CardEntity {
    card: Card,
}

#[derive(Component)]
struct ChipStack {
    player_id: usize,
}

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
    game_paused: bool,
    hand_number: i32,
    animation_start_time: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Heads-Up Poker".into(),
                resolution: (375., 812.).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameStateResource>()
        .add_systems(Startup, setup_game)
        .add_systems(Update, (
            start_hand_system,
            handle_betting,
            update_animations,
            check_game_flow,
            handle_showdown,
            update_card_visuals,
            update_ui,
        ))
        .run();
}

fn setup_game(
    mut commands: Commands,
    mut game_state: ResMut<GameStateResource>,
) {
    commands.spawn(Camera2dBundle::default());
    game_state.animation_start_time = 0.0;
    game_state.hand_number = 1;
    game_state.deck = Deck::new();
}

fn start_hand_system(
    mut commands: Commands,
    mut game_state: ResMut<GameStateResource>,
    time: Res<Time>,
) {
    if game_state.hand_number == 1 || game_state.showdown_timer < -0.5 {
        game_state.animation_start_time = time.elapsed_seconds();
        game_state.showdown_timer = 0.0;
        start_hand(&mut commands, &mut game_state);
    }
}

fn start_hand(
    commands: &mut Commands,
    game_state: &mut GameStateResource,
) {
    game_state.deck = Deck::new();
    game_state.pot = 0;
    game_state.current_round = PokerRound::PreFlop;
    game_state.street_cards.clear();
    game_state.last_action = format!("Hand #{}", game_state.hand_number);
    game_state.hand_number += 1;
    game_state.showdown_timer = 0.0;
    game_state.game_paused = false;
    game_state.dealer_position = (game_state.dealer_position + 1) % 2;
    game_state.current_player = (game_state.dealer_position + 1) % 2;

    let screen_width = 375.0;
    let screen_height = 812.0;

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.12, 0.45, 0.18),
            custom_size: Some(Vec2::new(screen_width, screen_height * 0.55)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, -20.0, 0.0),
        ..default()
    });

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.18, 0.55, 0.22),
            custom_size: Some(Vec2::new(screen_width * 0.94, screen_height * 0.48)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, -30.0, 0.1),
        ..default()
    });

    let player_y_top = screen_height * 0.25;
    let player_y_bottom = -screen_height * 0.32;
    
    for id in 0..2 {
        let x_pos = if id == 0 { -95.0 } else { 95.0 };
        let y_pos = if id == 0 { player_y_top } else { player_y_bottom };
        let card_target_y = y_pos + 100.0;
        
        commands.spawn(ChipStack { player_id: id });

        for j in 0..2 {
            let card_offset = (j as f32 - 0.5) * 45.0;
            let target_pos = Vec3::new(x_pos + card_offset, card_target_y, 1.0);
            let card = game_state.deck.draw().unwrap_or(Card::placeholder());

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
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
        ));

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    "$1000",
                    TextStyle {
                        font_size: 18.0,
                        color: Color::srgb(1.0, 0.85, 0.0),
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(x_pos, y_pos - 5.0, 1.0),
                ..default()
            },
            ChipStack { player_id: id },
        ));
    }

    for i in 0..5 {
        let x_offset = (i as f32 - 2.0) * 52.0;
        let community_card = if i < 3 {
            game_state.deck.draw().unwrap_or(Card::placeholder())
        } else {
            Card::placeholder()
        };
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(CARD_WIDTH * 0.85, CARD_HEIGHT * 0.85)),
                    ..default()
                },
                transform: Transform::from_xyz(x_offset, 0.0, 0.5),
                ..default()
            },
            CardEntity { card: community_card },
            DealAnimation {
                start_pos: Vec3::new(x_offset, 280.0, 0.5),
                target_pos: Vec3::new(x_offset, 0.0, 0.5),
                start_time: 0.0,
                duration: 0.4,
                delay: 0.9 + i as f32 * 0.15,
            },
        ));
    }

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Pot: $0",
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
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Hand: #1",
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
    ));
}

fn handle_betting(
    mut game_state: ResMut<GameStateResource>,
    time: Res<Time>,
) {
    if game_state.game_paused {
        return;
    }

    let action_delay = 2.5;
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
        PokerRound::PreFlop => game_state.pot += 20,
        PokerRound::Flop => game_state.pot += 30,
        PokerRound::Turn => game_state.pot += 40,
        PokerRound::River => game_state.pot += 50,
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
            game_state.showdown_timer = 5.0;
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

fn check_game_flow(
    mut game_state: ResMut<GameStateResource>,
    time: Res<Time>,
) {
    if game_state.current_round == PokerRound::Showdown {
        game_state.showdown_timer -= time.delta_seconds();
    }
}

fn handle_showdown(
    mut commands: Commands,
    mut game_state: ResMut<GameStateResource>,
) {
    if game_state.current_round == PokerRound::Showdown && game_state.showdown_timer <= 0.0 {
        game_state.current_round = PokerRound::PreFlop;
        game_state.pot = 0;
        game_state.street_cards.clear();
        game_state.game_paused = false;
        game_state.last_action = format!("Hand #{}", game_state.hand_number);
        game_state.showdown_timer = -1.0;
        start_hand(&mut commands, &mut game_state);
    }
}

fn update_card_visuals(
    mut query: Query<(&mut Sprite, &CardEntity)>,
) {
    for (mut sprite, card_entity) in query.iter_mut() {
        let card = card_entity.card;
        let color = match card.suit {
            Suit::Hearts | Suit::Diamonds => Color::srgb(0.95, 0.25, 0.25),
            Suit::Clubs | Suit::Spades => Color::srgb(0.15, 0.15, 0.15),
        };
        sprite.color = color;
    }
}

fn update_ui(
    game_state: Res<GameStateResource>,
    mut text_query: Query<&mut Text>,
) {
    let pot_text = format!("Pot: ${}", game_state.pot);
    let hand_text = format!("Hand: #{}", game_state.hand_number);
    let chip_text = format!("${}", 1000 - game_state.pot / 2);

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
