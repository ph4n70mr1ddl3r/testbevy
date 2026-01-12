use bevy::prelude::*;
use rand::seq::SliceRandom;

mod poker_logic;
use poker_logic::{determine_winner, Card, Deck, PokerRound};

#[derive(Resource)]
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
    starting_chips: i32,
    bet_amount: i32,
    raise_amount: i32,
    screen_width: f32,
    screen_height: f32,
    pot_display_y: f32,
    hand_number_x: f32,
    hand_number_y: f32,
    round_display_x: f32,
    round_display_y: f32,
    action_display_y: f32,
    player_label_offset: f32,
    chip_label_offset: f32,
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

const ANIMATION_CARD_DEAL_DELAY: f32 = 0.2;
const ANIMATION_DEAL_DURATION: f32 = 0.5;
const ANIMATION_COMMUNITY_DELAY_START: f32 = 0.9;
const ANIMATION_COMMUNITY_DELAY_INCREMENT: f32 = 0.15;
const ANIMATION_COMMUNITY_DURATION: f32 = 0.4;
const ANIMATION_EASING_POWER: i32 = 3;

fn get_round_name(round: PokerRound) -> &'static str {
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
    pot_remainder: i32,
    current_round: PokerRound,
    dealer_position: usize,
    current_player: usize,
    last_action: String,
    showdown_timer: f32,
    hand_number: i32,
    animation_start_time: f32,
    player_chips: [i32; 2],
    player_bets: [i32; 2],
    current_bet: i32,
    needs_cleanup: bool,
    winner: Option<i32>,
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
enum PokerAction {
    Check,
    Bet,
    Call,
    Raise,
    Fold,
}

impl PokerAction {
    fn to_str(&self) -> &'static str {
        match self {
            PokerAction::Check => "Check",
            PokerAction::Bet => "Bet 50",
            PokerAction::Call => "Call",
            PokerAction::Raise => "Raise 100",
            PokerAction::Fold => "Fold",
        }
    }
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
    let config = GameConfig::default();
    commands.spawn((Camera2dBundle::default(), HandMarker));
    game_state.hand_number = 1;
    game_state.player_chips = [config.starting_chips, config.starting_chips];
    game_state.player_bets = [0, 0];
    game_state.current_bet = 0;
    game_state.deck = Deck::new();
    game_state.pot_remainder = 0;
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
        game_state.winner = None;
        game_state.last_winner_message.clear();
        start_hand(&mut commands, &mut game_state);
    }
}

fn start_hand(commands: &mut Commands, game_state: &mut GameStateResource) {
    game_state.pot = 0;
    game_state.pot_remainder = 0;
    game_state.current_round = PokerRound::PreFlop;
    game_state.last_action = format!("Hand #{}", game_state.hand_number);
    game_state.hand_number += 1;
    game_state.showdown_timer = 0.0;
    game_state.dealer_position = (game_state.dealer_position + 1) % 2;
    game_state.current_player = (game_state.dealer_position + 1) % 2;
    game_state.player_bets = [0, 0];
    game_state.current_bet = 0;
    game_state.winner = None;

    if game_state.deck.cards_remaining() < 9 {
        game_state.deck = Deck::new();
    }

    let config = GameConfig::default();
    spawn_table(commands, config.screen_width, config.screen_height);

    let player_y_top = config.screen_height * 0.25;
    let player_y_bottom = -config.screen_height * 0.32;

    for id in 0..2 {
        spawn_player(
            commands,
            game_state,
            &config,
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
        spawn_community_card(commands, game_state, &config, i);
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
    config: &GameConfig,
    id: usize,
    x_pos: f32,
    y_pos: f32,
) {
    let card_target_y = y_pos + config.card_target_y_offset;

    for j in 0..2 {
        let card_offset = (j as f32 - 0.5) * config.card_offset_spacing;
        let target_pos = Vec3::new(x_pos + card_offset, card_target_y, 1.0);
        let card = game_state.deck.draw().unwrap_or(Card::placeholder());

        if id == 0 {
            game_state.p1_hole[j] = card;
        } else {
            game_state.p2_hole[j] = card;
        }

        let text_color = if card.is_red() {
            Color::srgb(0.85, 0.1, 0.1)
        } else {
            Color::srgb(0.1, 0.1, 0.1)
        };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.98, 0.95, 0.95),
                    custom_size: Some(Vec2::new(config.card_width, config.card_height)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, config.animation_start_y, 1.0),
                ..default()
            },
            CardEntity,
            DealAnimation {
                start_pos: Vec3::new(0.0, config.animation_start_y, 1.0),
                target_pos,
                start_time: 0.0,
                duration: ANIMATION_DEAL_DURATION,
                delay: (id * 2 + j) as f32 * ANIMATION_CARD_DEAL_DELAY,
            },
            HandMarker,
        ));

        spawn_card_text(commands, card, target_pos, text_color, 14.0, &config);
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
            transform: Transform::from_xyz(x_pos, y_pos + config.player_label_offset, 1.0),
            ..default()
        },
        HandMarker,
    ));

    let chip_text = format!("${}", game_state.player_chips[id]);
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                chip_text,
                TextStyle {
                    font_size: 18.0,
                    color: Color::srgb(1.0, 0.85, 0.0),
                    ..default()
                },
            ),
            transform: Transform::from_xyz(x_pos, y_pos + config.chip_label_offset, 1.0),
            ..default()
        },
        HandMarker,
    ));
}

fn spawn_card_text(
    commands: &mut Commands,
    card: Card,
    target_pos: Vec3,
    text_color: Color,
    font_size: f32,
    config: &GameConfig,
) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("{}\n{}", card.rank_str(), card.suit_str()),
                TextStyle {
                    font_size,
                    color: text_color,
                    ..default()
                },
            ),
            transform: Transform::from_xyz(
                target_pos.x - config.card_width / 2.0 + 8.0,
                target_pos.y + config.card_height / 2.0 - 12.0,
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
                    font_size,
                    color: text_color,
                    ..default()
                },
            ),
            transform: Transform::from_xyz(
                target_pos.x + config.card_width / 2.0 - 8.0,
                target_pos.y - config.card_height / 2.0 + 12.0,
                1.1,
            )
            .with_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
            ..default()
        },
        HandMarker,
    ));
}

fn spawn_community_card(
    commands: &mut Commands,
    game_state: &mut GameStateResource,
    config: &GameConfig,
    i: usize,
) {
    let x_offset = (i as f32 - 2.0) * config.card_offset_spacing;
    let community_card = if i < 3 {
        game_state.deck.draw().unwrap_or(Card::placeholder())
    } else {
        Card::placeholder()
    };

    game_state.community_cards[i] = community_card;

    let is_hidden = i >= 3;

    let target_pos = Vec3::new(x_offset, 0.0, 0.5);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: if is_hidden {
                    Color::srgb(0.2, 0.3, 0.2)
                } else {
                    Color::srgb(0.98, 0.95, 0.95)
                },
                custom_size: Some(Vec2::new(
                    config.card_width * config.community_card_scale,
                    config.card_height * config.community_card_scale,
                )),
                ..default()
            },
            transform: Transform::from_xyz(x_offset, config.community_card_start_y, 0.5),
            ..default()
        },
        CardEntity,
        DealAnimation {
            start_pos: Vec3::new(x_offset, config.community_card_start_y, 0.5),
            target_pos,
            start_time: 0.0,
            duration: ANIMATION_COMMUNITY_DURATION,
            delay: ANIMATION_COMMUNITY_DELAY_START + i as f32 * ANIMATION_COMMUNITY_DELAY_INCREMENT,
        },
        HandMarker,
        CommunityCard {
            index: i,
            is_hidden,
        },
    ));

    if !is_hidden {
        let text_color = if community_card.is_red() {
            Color::srgb(0.85, 0.1, 0.1)
        } else {
            Color::srgb(0.1, 0.1, 0.1)
        };
        spawn_card_text(
            commands,
            community_card,
            target_pos,
            text_color,
            12.0,
            &config,
        );
    }
}

fn spawn_ui(commands: &mut Commands, game_state: &mut GameStateResource) {
    let config = GameConfig::default();
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
            transform: Transform::from_xyz(0.0, config.pot_display_y, 1.0),
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
            transform: Transform::from_xyz(config.hand_number_x, config.hand_number_y, 1.0),
            ..default()
        },
        HandNumberDisplay,
        HandMarker,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("Chips: ${}", game_state.player_chips[0]),
                TextStyle {
                    font_size: 16.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                    ..default()
                },
            ),
            transform: Transform::from_xyz(0.0, -260.0, 1.0),
            ..default()
        },
        PlayerChipsDisplay,
        HandMarker,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("P2: ${}", game_state.player_chips[1]),
                TextStyle {
                    font_size: 14.0,
                    color: Color::srgb(0.7, 0.7, 0.7),
                    ..default()
                },
            ),
            transform: Transform::from_xyz(0.0, 60.0, 1.0),
            ..default()
        },
        OpponentChipsDisplay,
        HandMarker,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                get_round_name(game_state.current_round).to_string(),
                TextStyle {
                    font_size: 18.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ),
            transform: Transform::from_xyz(config.round_display_x, config.round_display_y, 1.0),
            ..default()
        },
        RoundDisplay,
        HandMarker,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                game_state.last_action.clone(),
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            transform: Transform::from_xyz(0.0, config.action_display_y, 1.0),
            ..default()
        },
        ActionDisplay,
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

fn handle_betting(
    config: Res<GameConfig>,
    mut game_state: ResMut<GameStateResource>,
    time: Res<Time>,
) {
    let action_delay = config.action_delay;
    let elapsed = time.elapsed_seconds() - game_state.animation_start_time;

    if elapsed > 1.0 && (elapsed % action_delay) < time.delta_seconds() {
        perform_validated_action(&mut game_state, &config);
    }
}

fn get_valid_actions(game_state: &GameStateResource, config: &GameConfig) -> Vec<PokerAction> {
    let mut actions = Vec::new();
    let player_idx = game_state.current_player;
    let player_chips = game_state.player_chips[player_idx];
    let player_bet = game_state.player_bets[player_idx];
    let current_bet = game_state.current_bet;

    actions.push(PokerAction::Check);

    if current_bet > 0 {
        let call_amount = current_bet - player_bet;
        if player_chips >= call_amount && call_amount > 0 {
            actions.push(PokerAction::Call);
        }
        if player_chips > current_bet {
            actions.push(PokerAction::Raise);
        }
    } else if player_chips >= config.bet_amount {
        actions.push(PokerAction::Bet);
    }

    actions.push(PokerAction::Fold);

    actions
}

fn perform_validated_action(game_state: &mut GameStateResource, config: &GameConfig) {
    let actions = get_valid_actions(game_state, config);
    if actions.is_empty() {
        game_state.last_action = "No actions available".to_string();
        return;
    }

    let action = actions.choose(&mut rand::thread_rng()).unwrap();
    game_state.last_action = format!("P{}: {}", game_state.current_player + 1, action.to_str());

    match action {
        PokerAction::Check => {
            game_state.current_player = (game_state.current_player + 1) % 2;
        }
        PokerAction::Bet => {
            let bet_amount = config.bet_amount;
            if game_state.player_chips[game_state.current_player] >= bet_amount {
                game_state.player_chips[game_state.current_player] -= bet_amount;
                game_state.player_bets[game_state.current_player] += bet_amount;
                game_state.current_bet = bet_amount;
                game_state.pot += bet_amount;
                game_state.current_player = (game_state.current_player + 1) % 2;
            }
        }
        PokerAction::Call => {
            let call_amount =
                game_state.current_bet - game_state.player_bets[game_state.current_player];
            if call_amount > 0 && game_state.player_chips[game_state.current_player] >= call_amount
            {
                game_state.player_chips[game_state.current_player] -= call_amount;
                game_state.player_bets[game_state.current_player] += call_amount;
                game_state.pot += call_amount;
                game_state.current_player = (game_state.current_player + 1) % 2;
            }
        }
        PokerAction::Raise => {
            let raise_amount = game_state.current_bet + config.raise_amount;
            let actual_raise = raise_amount - game_state.player_bets[game_state.current_player];
            if game_state.player_chips[game_state.current_player] >= actual_raise {
                game_state.player_chips[game_state.current_player] -= actual_raise;
                game_state.player_bets[game_state.current_player] = raise_amount;
                game_state.current_bet = raise_amount;
                game_state.pot += actual_raise;
                game_state.current_player = (game_state.current_player + 1) % 2;
            }
        }
        PokerAction::Fold => {
            let winner = (game_state.current_player + 1) % 2;
            game_state.winner = Some(winner as i32);
            game_state.player_chips[winner] += game_state.pot;
            game_state.player_chips[winner] += game_state.pot_remainder;
            game_state.last_winner_message = format!(
                "P{} folded - P{} wins ${}",
                game_state.current_player + 1,
                winner + 1,
                game_state.pot + game_state.pot_remainder
            );
            game_state.pot = 0;
            game_state.pot_remainder = 0;
            game_state.current_round = PokerRound::Showdown;
            game_state.showdown_timer = config.fold_showdown_duration;
            return;
        }
    }

    advance_street(game_state, config);
}

fn advance_street(game_state: &mut GameStateResource, config: &GameConfig) {
    let both_players_acted = game_state.player_bets[0] == game_state.current_bet
        && game_state.player_bets[1] == game_state.current_bet
        && game_state.current_bet > 0;

    let can_check = game_state.current_bet == 0;

    if both_players_acted || can_check {
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
            game_state.player_bets = [0, 0];
            game_state.current_player = game_state.dealer_position;
            game_state.pot_remainder = 0;
        }
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
            let eased = 1.0 - (1.0 - t).powi(ANIMATION_EASING_POWER);
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
        if game_state.winner.is_none() {
            let result = determine_winner(
                &game_state.p1_hole,
                &game_state.p2_hole,
                &game_state.community_cards,
            );
            match result {
                (0, true) => {
                    game_state.winner = Some(0);
                    let total_pot = game_state.pot + game_state.pot_remainder;
                    game_state.player_chips[0] += total_pot;
                    game_state.last_winner_message = format!("P1 wins ${}!", total_pot);
                }
                (1, true) => {
                    game_state.winner = Some(1);
                    let total_pot = game_state.pot + game_state.pot_remainder;
                    game_state.player_chips[1] += total_pot;
                    game_state.last_winner_message = format!("P2 wins ${}!", total_pot);
                }
                _ => {
                    let split_amount = game_state.pot / 2;
                    let remainder = game_state.pot % 2;
                    game_state.player_chips[0] += split_amount;
                    game_state.player_chips[1] += split_amount;
                    game_state.pot_remainder += remainder;
                    game_state.last_winner_message =
                        format!("Split pot - each wins ${}", split_amount);
                    if remainder > 0 {
                        game_state
                            .last_winner_message
                            .push_str(&format!(" ({} remainder)", remainder));
                    }
                }
            }
            game_state.pot = 0;
            game_state.pot_remainder = 0;
        }

        game_state.current_round = PokerRound::PreFlop;
        game_state.showdown_timer = -1.0;
        start_hand(&mut commands, &mut game_state);
    }
}

fn update_card_visuals(
    mut query: Query<(&mut Sprite, Option<&CommunityCard>)>,
    game_state: Res<GameStateResource>,
) {
    let face_up_color = Color::srgb(0.98, 0.95, 0.95);
    let face_down_color = Color::srgb(0.2, 0.3, 0.2);

    for (mut sprite, community_card) in query.iter_mut() {
        if let Some(cc) = community_card {
            let should_reveal = match game_state.current_round {
                PokerRound::Flop => cc.index < 3,
                PokerRound::Turn => cc.index < 4,
                PokerRound::River | PokerRound::Showdown => cc.index < 5,
                _ => false,
            };

            sprite.color = if should_reveal && cc.is_hidden {
                face_up_color
            } else if cc.is_hidden {
                face_down_color
            } else {
                face_up_color
            };
        }
    }
}

fn update_ui(
    game_state: Res<GameStateResource>,
    mut text_queries: ParamSet<(
        Query<&mut Text, With<PotDisplay>>,
        Query<&mut Text, With<HandNumberDisplay>>,
        Query<&mut Text, With<PlayerChipsDisplay>>,
        Query<&mut Text, With<OpponentChipsDisplay>>,
        Query<&mut Text, With<RoundDisplay>>,
        Query<&mut Text, With<ActionDisplay>>,
    )>,
) {
    for mut text in text_queries.p0().iter_mut() {
        text.sections[0].value = format!("Pot: ${}", game_state.pot);
    }

    for mut text in text_queries.p1().iter_mut() {
        text.sections[0].value = format!("Hand: #{}", game_state.hand_number);
    }

    for mut text in text_queries.p2().iter_mut() {
        text.sections[0].value = format!("Chips: ${}", game_state.player_chips[0]);
    }

    for mut text in text_queries.p3().iter_mut() {
        text.sections[0].value = format!("P2: ${}", game_state.player_chips[1]);
    }

    for mut text in text_queries.p4().iter_mut() {
        text.sections[0].value = get_round_name(game_state.current_round).to_string();
    }

    let action_text = if let Some(winner) = game_state.winner {
        if winner >= 0 {
            format!(
                "Winner: P{} - {}",
                winner + 1,
                game_state.last_winner_message
            )
        } else {
            game_state.last_winner_message.clone()
        }
    } else {
        game_state.last_action.clone()
    };

    for mut text in text_queries.p5().iter_mut() {
        text.sections[0].value = action_text;
        break;
    }
}
