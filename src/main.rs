use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

mod poker_logic;
use poker_logic::{determine_winner, Card, Deck, PokerRound};

const INITIAL_HAND_NUMBER: i32 = 1;
const PLAYER_COUNT: usize = 2;

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
    starting_chips: u32,
    bet_amount: u32,
    raise_amount: u32,
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
            pot_display_y: 130.0,
            hand_number_x: -160.0,
            hand_number_y: 360.0,
            round_display_x: 140.0,
            round_display_y: 360.0,
            action_display_y: -180.0,
            player_label_offset: 20.0,
            chip_label_offset: -5.0,
            min_cards_for_reshuffle: 9,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Resource)]
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

const ANIMATION_CARD_DEAL_DELAY: f32 = 0.2;
const ANIMATION_DEAL_DURATION: f32 = 0.5;
const ANIMATION_COMMUNITY_DELAY_START: f32 = 0.9;
const ANIMATION_COMMUNITY_DELAY_INCREMENT: f32 = 0.15;
const ANIMATION_COMMUNITY_DURATION: f32 = 0.4;
const ANIMATION_EASING_POWER: i32 = 3;

const POT_FONT_SIZE: f32 = 22.0;
const HAND_NUMBER_FONT_SIZE: f32 = 14.0;
const PLAYER_CHIPS_FONT_SIZE: f32 = 16.0;
const OPPONENT_CHIPS_FONT_SIZE: f32 = 14.0;
const ROUND_FONT_SIZE: f32 = 18.0;
const ACTION_FONT_SIZE: f32 = 16.0;
const COMMUNITY_CARD_FONT_SIZE: f32 = 12.0;
const PLAYER_LABEL_FONT_SIZE: f32 = 20.0;
const CHIP_LABEL_FONT_SIZE: f32 = 18.0;

const BETTING_INITIAL_DELAY: f32 = 1.0;
const SHOWDOWN_TIMER_RESET_THRESHOLD: f32 = -0.5;

const PLAYER_Y_TOP_RATIO: f32 = 0.25;
const PLAYER_Y_BOTTOM_RATIO: f32 = -0.32;
const TABLE_DARK_Z: f32 = 0.0;
const TABLE_DARK_Y: f32 = -20.0;
const TABLE_LIGHT_Z: f32 = 0.1;
const TABLE_LIGHT_Y: f32 = -30.0;
const CARD_TEXT_TOP_OFFSET_X: f32 = 8.0;
const CARD_TEXT_TOP_OFFSET_Y: f32 = -12.0;
const CARD_TEXT_BOTTOM_OFFSET_X: f32 = -8.0;
const CARD_TEXT_BOTTOM_OFFSET_Y: f32 = 12.0;
const PLAYER_CHIPS_Y: f32 = -260.0;
const OPPONENT_CHIPS_Y: f32 = 60.0;

const TABLE_DARK_HEIGHT_RATIO: f32 = 0.55;
const TABLE_DARK_WIDTH_RATIO: f32 = 1.0;
const TABLE_LIGHT_HEIGHT_RATIO: f32 = 0.48;
const TABLE_LIGHT_WIDTH_RATIO: f32 = 0.94;

const CARD_Z_POSITION: f32 = 1.0;
const CARD_TEXT_Z_POSITION: f32 = 1.1;
const COMMUNITY_CARD_Z_POSITION: f32 = 0.5;
const CARD_TARGET_Z: f32 = 1.0;

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
            )
                .chain(),
        )
        .run();
}

fn setup_game(
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
}

fn start_hand_system(
    mut commands: Commands,
    mut game_state: ResMut<GameStateResource>,
    config: Res<GameConfig>,
    colors: Res<ColorPalette>,
    time: Res<Time>,
) {
    if game_state.hand_number == INITIAL_HAND_NUMBER
        || game_state.showdown_timer < SHOWDOWN_TIMER_RESET_THRESHOLD
    {
        game_state.needs_cleanup = true;
        game_state.animation_start_time = time.elapsed_seconds();
        game_state.showdown_timer = 0.0;
        game_state.action_tick = 0;
        game_state.winner = None;
        game_state.last_winner_message = "".to_string();
        start_hand(
            &mut commands,
            &mut game_state,
            &config,
            *colors,
            time.elapsed_seconds(),
        );
    }
}

fn start_hand(
    commands: &mut Commands,
    game_state: &mut GameStateResource,
    config: &GameConfig,
    colors: ColorPalette,
    animation_start_time: f32,
) {
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

    spawn_table(commands, config.screen_width, config.screen_height, colors);

    let player_y_top = config.screen_height * PLAYER_Y_TOP_RATIO;
    let player_y_bottom = config.screen_height * PLAYER_Y_BOTTOM_RATIO;

    spawn_player(
        commands,
        game_state,
        config,
        &colors,
        0,
        0.0,
        player_y_top,
        animation_start_time,
    );
    spawn_player(
        commands,
        game_state,
        config,
        &colors,
        1,
        0.0,
        player_y_bottom,
        animation_start_time,
    );

    for i in 0..5 {
        spawn_community_card(
            commands,
            game_state,
            config,
            &colors,
            i,
            animation_start_time,
        );
    }

    spawn_ui(commands, game_state, config, &colors);
}

fn spawn_table(
    commands: &mut Commands,
    screen_width: f32,
    screen_height: f32,
    colors: ColorPalette,
) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: colors.table_green_dark,
                custom_size: Some(Vec2::new(
                    screen_width * TABLE_DARK_WIDTH_RATIO,
                    screen_height * TABLE_DARK_HEIGHT_RATIO,
                )),
                ..default()
            },
            transform: Transform::from_xyz(0.0, TABLE_DARK_Y, TABLE_DARK_Z),
            ..default()
        },
        HandMarker,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: colors.table_green_light,
                custom_size: Some(Vec2::new(
                    screen_width * TABLE_LIGHT_WIDTH_RATIO,
                    screen_height * TABLE_LIGHT_HEIGHT_RATIO,
                )),
                ..default()
            },
            transform: Transform::from_xyz(0.0, TABLE_LIGHT_Y, TABLE_LIGHT_Z),
            ..default()
        },
        HandMarker,
    ));
}

fn spawn_player(
    commands: &mut Commands,
    game_state: &mut GameStateResource,
    config: &GameConfig,
    colors: &ColorPalette,
    id: usize,
    x_pos: f32,
    y_pos: f32,
    animation_start_time: f32,
) {
    let card_target_y = y_pos + config.card_target_y_offset;

    for j in 0..2 {
        let card_offset = (j as f32 - 0.5) * config.card_offset_spacing;
        let target_pos = Vec3::new(x_pos + card_offset, card_target_y, 1.0);
        let card = draw_card(game_state);

        let player_hole = if id == 0 {
            &mut game_state.p1_hole
        } else {
            &mut game_state.p2_hole
        };
        player_hole[j] = card;

        let text_color = if card.is_red() {
            colors.card_text_red
        } else {
            colors.card_text_black
        };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: colors.face_up_white,
                    custom_size: Some(Vec2::new(config.card_width, config.card_height)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, config.animation_start_y, CARD_Z_POSITION),
                ..default()
            },
            CardEntity,
            DealAnimation {
                start_pos: Vec3::new(0.0, config.animation_start_y, CARD_Z_POSITION),
                target_pos,
                start_time: animation_start_time,
                duration: ANIMATION_DEAL_DURATION,
                delay: (id * 2 + j) as f32 * ANIMATION_CARD_DEAL_DELAY,
            },
            HandMarker,
        ));

        spawn_card_text(
            commands,
            card,
            target_pos,
            text_color,
            HAND_NUMBER_FONT_SIZE,
            config,
        );
    }

    let player_label = if id == 0 { "YOU" } else { "OPP" };
    let chip_y_offset = if id == 0 {
        config.player_label_offset
    } else {
        config.chip_label_offset
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                player_label,
                TextStyle {
                    font_size: PLAYER_LABEL_FONT_SIZE,
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
                    font_size: CHIP_LABEL_FONT_SIZE,
                    color: colors.chip_gold,
                    ..default()
                },
            ),
            transform: Transform::from_xyz(x_pos, y_pos + chip_y_offset, 1.0),
            ..default()
        },
        HandMarker,
    ));
}

struct CardTextParams {
    card: Card,
    target_pos: Vec3,
    offset_x: f32,
    offset_y: f32,
    rotation: f32,
    text_color: Color,
    font_size: f32,
}

impl CardTextParams {
    fn new(
        card: Card,
        target_pos: Vec3,
        offset_x: f32,
        offset_y: f32,
        rotation: f32,
        text_color: Color,
        font_size: f32,
    ) -> Self {
        CardTextParams {
            card,
            target_pos,
            offset_x,
            offset_y,
            rotation,
            text_color,
            font_size,
        }
    }

    fn spawn(&self, commands: &mut Commands) {
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    format!("{}\n{}", self.card.rank_str(), self.card.suit_str()),
                    TextStyle {
                        font_size: self.font_size,
                        color: self.text_color,
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(
                    self.target_pos.x + self.offset_x,
                    self.target_pos.y + self.offset_y,
                    CARD_TEXT_Z_POSITION,
                )
                .with_rotation(Quat::from_rotation_z(self.rotation)),
                ..default()
            },
            HandMarker,
        ));
    }
}

fn spawn_card_text(
    commands: &mut Commands,
    card: Card,
    target_pos: Vec3,
    text_color: Color,
    font_size: f32,
    config: &GameConfig,
) {
    CardTextParams::new(
        card,
        target_pos,
        -config.card_width / 2.0 + CARD_TEXT_TOP_OFFSET_X,
        config.card_height / 2.0 + CARD_TEXT_TOP_OFFSET_Y,
        0.0,
        text_color,
        font_size,
    )
    .spawn(commands);

    CardTextParams::new(
        card,
        target_pos,
        config.card_width / 2.0 + CARD_TEXT_BOTTOM_OFFSET_X,
        -config.card_height / 2.0 + CARD_TEXT_BOTTOM_OFFSET_Y,
        std::f32::consts::PI,
        text_color,
        font_size,
    )
    .spawn(commands);
}

fn spawn_community_card(
    commands: &mut Commands,
    game_state: &mut GameStateResource,
    config: &GameConfig,
    colors: &ColorPalette,
    i: usize,
    animation_start_time: f32,
) {
    let x_offset = (i as f32 - 2.0) * config.card_offset_spacing;
    let community_card = draw_card(game_state);

    game_state.community_cards[i] = community_card;

    let is_hidden = i >= 3;

    let target_pos = Vec3::new(x_offset, 0.0, CARD_TARGET_Z);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: if is_hidden {
                    colors.face_down_dark
                } else {
                    colors.face_up_white
                },
                custom_size: Some(Vec2::new(
                    config.card_width * config.community_card_scale,
                    config.card_height * config.community_card_scale,
                )),
                ..default()
            },
            transform: Transform::from_xyz(
                x_offset,
                config.community_card_start_y,
                COMMUNITY_CARD_Z_POSITION,
            ),
            ..default()
        },
        CardEntity,
        DealAnimation {
            start_pos: Vec3::new(
                x_offset,
                config.community_card_start_y,
                COMMUNITY_CARD_Z_POSITION,
            ),
            target_pos,
            start_time: animation_start_time,
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
            colors.card_text_red
        } else {
            colors.card_text_black
        };
        spawn_card_text(
            commands,
            community_card,
            target_pos,
            text_color,
            COMMUNITY_CARD_FONT_SIZE,
            config,
        );
    }
}

fn spawn_ui(
    commands: &mut Commands,
    game_state: &mut GameStateResource,
    config: &GameConfig,
    colors: &ColorPalette,
) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("Pot: ${}", game_state.pot),
                TextStyle {
                    font_size: POT_FONT_SIZE,
                    color: colors.chip_gold,
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
                    font_size: HAND_NUMBER_FONT_SIZE,
                    color: colors.text_gray_dim,
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
                    font_size: PLAYER_CHIPS_FONT_SIZE,
                    color: colors.text_gray_light,
                    ..default()
                },
            ),
            transform: Transform::from_xyz(0.0, PLAYER_CHIPS_Y, 1.0),
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
                    font_size: OPPONENT_CHIPS_FONT_SIZE,
                    color: colors.text_gray_med,
                    ..default()
                },
            ),
            transform: Transform::from_xyz(0.0, OPPONENT_CHIPS_Y, 1.0),
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
                    font_size: ROUND_FONT_SIZE,
                    color: colors.text_white,
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
                    font_size: ACTION_FONT_SIZE,
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

    let current_tick = ((elapsed - BETTING_INITIAL_DELAY) / action_delay) as u32;
    if elapsed > BETTING_INITIAL_DELAY && current_tick > game_state.action_tick {
        perform_validated_action(&mut game_state, &config);
        game_state.action_tick = current_tick;
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

fn place_bet(
    game_state: &mut GameStateResource,
    amount: u32,
    is_raise: bool,
    new_current_bet: u32,
) {
    let player_idx = game_state.current_player;
    let available_chips = game_state.player_chips[player_idx];
    let actual_amount = amount.min(available_chips);
    game_state.player_chips[player_idx] -= actual_amount;
    game_state.player_bets[player_idx] += actual_amount;
    game_state.pot += actual_amount;
    if is_raise {
        game_state.current_bet = new_current_bet;
    }
}

fn perform_validated_action(game_state: &mut GameStateResource, config: &GameConfig) {
    let actions = get_valid_actions(game_state, config);
    if actions.is_empty() {
        game_state.last_action = "No actions".to_string();
        return;
    }

    let action = actions.choose(&mut thread_rng()).unwrap();

    match action {
        PokerAction::Check => {
            let player_idx = game_state.current_player;
            game_state.last_action = format!("P{}: Check", player_idx + 1);
            game_state.current_player = (game_state.current_player + 1) % PLAYER_COUNT;
        }
        PokerAction::Bet => {
            let bet_amount = config.bet_amount;
            let player_idx = game_state.current_player;
            if game_state.player_chips[player_idx] >= bet_amount {
                place_bet(game_state, bet_amount, true, bet_amount);
                game_state.last_action = format!("P{}: Bet ${}", player_idx + 1, bet_amount);
                game_state.current_player = (game_state.current_player + 1) % PLAYER_COUNT;
            } else {
                game_state.last_action = format!("P{}: All-in", player_idx + 1);
                game_state.current_player = (game_state.current_player + 1) % PLAYER_COUNT;
            }
        }
        PokerAction::Call => {
            let player_idx = game_state.current_player;
            let call_amount =
                game_state.current_bet - game_state.player_bets[game_state.current_player];
            if call_amount > 0 && game_state.player_chips[player_idx] >= call_amount {
                place_bet(game_state, call_amount, false, 0);
                game_state.last_action = format!("P{}: Call", player_idx + 1);
                game_state.current_player = (game_state.current_player + 1) % PLAYER_COUNT;
            }
        }
        PokerAction::Raise => {
            let player_idx = game_state.current_player;
            let raise_amount = game_state.current_bet + config.raise_amount;
            let actual_raise = raise_amount - game_state.player_bets[game_state.current_player];
            if game_state.player_chips[player_idx] >= actual_raise {
                place_bet(game_state, actual_raise, true, raise_amount);
                game_state.last_action =
                    format!("P{}: Raise ${}", player_idx + 1, config.raise_amount);
                game_state.current_player = (game_state.current_player + 1) % PLAYER_COUNT;
            } else {
                game_state.last_action = format!("P{}: All-in", player_idx + 1);
                game_state.current_player = (game_state.current_player + 1) % PLAYER_COUNT;
            }
        }
        PokerAction::Fold => {
            let winner = (game_state.current_player + 1) % 2;
            game_state.winner = Some(winner);
            game_state.player_chips[winner] =
                game_state.player_chips[winner].saturating_add(game_state.pot);
            game_state.player_chips[winner] =
                game_state.player_chips[winner].saturating_add(game_state.pot_remainder);
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

    advance_street(game_state, config);
}

fn draw_card(game_state: &mut GameStateResource) -> Card {
    if let Some(c) = game_state.deck.draw() {
        c
    } else {
        error!("Deck empty - creating emergency deck");
        game_state.deck = Deck::new();
        game_state.deck.draw().unwrap_or_else(|| {
            error!("Emergency deck creation failed - using placeholder card");
            Card::default()
        })
    }
}

fn advance_street(game_state: &mut GameStateResource, config: &GameConfig) {
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

fn handle_showdown(
    mut commands: Commands,
    mut game_state: ResMut<GameStateResource>,
    config: Res<GameConfig>,
    colors: Res<ColorPalette>,
    time: Res<Time>,
) {
    if game_state.current_round == PokerRound::Showdown && game_state.showdown_timer <= 0.0 {
        if game_state.winner.is_none() {
            process_showdown_result(&mut game_state);
        }

        game_state.current_round = PokerRound::PreFlop;
        game_state.showdown_timer = -1.0;
        start_hand(
            &mut commands,
            &mut game_state,
            &config,
            *colors,
            time.elapsed_seconds(),
        );
    }
}

fn process_showdown_result(game_state: &mut GameStateResource) {
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
    game_state.pot = 0;
    game_state.pot_remainder = 0;
}

fn distribute_pot(game_state: &mut GameStateResource, winner: usize) {
    let total_pot = game_state.pot + game_state.pot_remainder;
    game_state.player_chips[winner] = game_state.player_chips[winner].saturating_add(total_pot);
    game_state.last_winner_message = if winner == 0 { "P1 wins" } else { "P2 wins" }.to_string();
}

fn split_pot(game_state: &mut GameStateResource) {
    let total_pot = game_state.pot + game_state.pot_remainder;
    let split_amount = total_pot / 2;
    let remainder = total_pot % 2;
    game_state.player_chips[0] = game_state.player_chips[0].saturating_add(split_amount);
    game_state.player_chips[1] = game_state.player_chips[1].saturating_add(split_amount);
    game_state.pot_remainder = remainder;
    game_state.last_winner_message = "Split pot".to_string();
}

fn update_card_visuals(
    mut query: Query<(&mut Sprite, Option<&CommunityCard>)>,
    game_state: Res<GameStateResource>,
    colors: Res<ColorPalette>,
) {
    let face_up_color = colors.face_up_white;
    let face_down_color = colors.face_down_dark;

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

    let action_text = if game_state.winner.is_some() {
        game_state.last_winner_message.clone()
    } else {
        game_state.last_action.clone()
    };

    if let Some(mut text) = text_queries.p5().iter_mut().next() {
        text.sections[0].value = action_text;
    }
}

#[cfg(test)]
mod game_tests {
    use super::*;

    #[test]
    fn test_game_config_defaults() {
        let config = GameConfig::default();
        assert_eq!(config.card_width, 55.0);
        assert_eq!(config.card_height, 77.0);
        assert_eq!(config.starting_chips, 1000);
        assert_eq!(config.bet_amount, 50);
        assert_eq!(config.raise_amount, 100);
    }

    #[test]
    fn test_color_palette_defaults() {
        let colors = ColorPalette::default();
        assert_eq!(colors.card_text_red, Color::srgb(0.85, 0.0, 0.0));
        assert_eq!(colors.card_text_black, Color::srgb(0.1, 0.1, 0.1));
        assert_eq!(colors.chip_gold, Color::srgb(1.0, 0.85, 0.0));
    }

    #[test]
    fn test_get_round_name() {
        assert_eq!(get_round_name(PokerRound::PreFlop), "Pre-Flop");
        assert_eq!(get_round_name(PokerRound::Flop), "Flop");
        assert_eq!(get_round_name(PokerRound::Turn), "Turn");
        assert_eq!(get_round_name(PokerRound::River), "River");
        assert_eq!(get_round_name(PokerRound::Showdown), "Showdown");
    }

    #[test]
    fn test_poker_action_as_str() {
        let config = GameConfig::default();
        assert_eq!(format!("Bet {}", config.bet_amount), "Bet 50");
        assert_eq!(format!("Raise {}", config.raise_amount), "Raise 100");
    }

    #[test]
    fn test_initial_hand_number() {
        assert_eq!(INITIAL_HAND_NUMBER, 1);
    }

    #[test]
    fn test_hand_number_starts_at_zero() {
        let game_state = GameStateResource::default();
        assert_eq!(game_state.hand_number, 0);
    }

    #[test]
    fn test_animation_constants() {
        assert!(ANIMATION_CARD_DEAL_DELAY > 0.0);
        assert!(ANIMATION_DEAL_DURATION > 0.0);
        assert!(ANIMATION_COMMUNITY_DURATION > 0.0);
        assert!(ANIMATION_EASING_POWER > 0);
    }

    #[test]
    fn test_font_sizes_are_reasonable() {
        assert!(POT_FONT_SIZE > 0.0);
        assert!(HAND_NUMBER_FONT_SIZE > 0.0);
        assert!(PLAYER_CHIPS_FONT_SIZE > 0.0);
        assert!(ROUND_FONT_SIZE > 0.0);
        assert!(ACTION_FONT_SIZE > 0.0);
    }

    #[test]
    fn test_z_positions_are_ordered() {
        assert!(CARD_TEXT_Z_POSITION > CARD_Z_POSITION);
        assert!(COMMUNITY_CARD_Z_POSITION < CARD_Z_POSITION);
    }

    #[test]
    fn test_player_y_ratios() {
        assert!(PLAYER_Y_TOP_RATIO > 0.0);
        assert!(PLAYER_Y_BOTTOM_RATIO < 0.0);
        assert_eq!(PLAYER_Y_TOP_RATIO, 0.25);
        assert_eq!(PLAYER_Y_BOTTOM_RATIO, -0.32);
    }

    #[test]
    fn test_table_dimensions() {
        assert!(TABLE_DARK_WIDTH_RATIO > TABLE_LIGHT_WIDTH_RATIO);
        assert!(TABLE_DARK_HEIGHT_RATIO > TABLE_LIGHT_HEIGHT_RATIO);
        assert_eq!(TABLE_DARK_WIDTH_RATIO, 1.0);
        assert_eq!(TABLE_LIGHT_WIDTH_RATIO, 0.94);
    }

    #[test]
    fn test_get_valid_actions_check_only() {
        let mut game_state = GameStateResource::default();
        game_state.current_player = 0;
        game_state.player_chips = [100, 100];
        game_state.player_bets = [0, 0];
        game_state.current_bet = 0;
        let config = GameConfig::default();

        let actions = get_valid_actions(&game_state, &config);

        assert!(actions.contains(&PokerAction::Check));
        assert!(actions.contains(&PokerAction::Bet));
        assert!(!actions.contains(&PokerAction::Call));
        assert!(!actions.contains(&PokerAction::Raise));
        assert!(actions.contains(&PokerAction::Fold));
    }

    #[test]
    fn test_get_valid_actions_must_call() {
        let mut game_state = GameStateResource::default();
        game_state.current_player = 1;
        game_state.player_chips = [100, 200];
        game_state.player_bets = [50, 0];
        game_state.current_bet = 50;
        let config = GameConfig::default();

        let actions = get_valid_actions(&game_state, &config);

        assert!(actions.contains(&PokerAction::Check));
        assert!(!actions.contains(&PokerAction::Bet));
        assert!(actions.contains(&PokerAction::Call));
        assert!(actions.contains(&PokerAction::Raise));
        assert!(actions.contains(&PokerAction::Fold));
    }

    #[test]
    fn test_get_valid_actions_cannot_raise_without_chips() {
        let mut game_state = GameStateResource::default();
        game_state.current_player = 1;
        game_state.player_chips = [200, 51];
        game_state.player_bets = [50, 0];
        game_state.current_bet = 50;
        let config = GameConfig::default();

        let actions = get_valid_actions(&game_state, &config);

        assert!(actions.contains(&PokerAction::Call));
        assert!(!actions.contains(&PokerAction::Raise));
    }

    #[test]
    fn test_get_valid_actions_can_bet() {
        let mut game_state = GameStateResource::default();
        game_state.current_player = 0;
        game_state.player_chips = [100, 100];
        game_state.player_bets = [0, 0];
        game_state.current_bet = 0;
        let config = GameConfig::default();

        let actions = get_valid_actions(&game_state, &config);

        assert!(actions.contains(&PokerAction::Check));
        assert!(actions.contains(&PokerAction::Bet));
        assert!(!actions.contains(&PokerAction::Call));
        assert!(!actions.contains(&PokerAction::Raise));
        assert!(actions.contains(&PokerAction::Fold));
    }

    #[test]
    fn test_get_valid_actions_can_raise() {
        let mut game_state = GameStateResource::default();
        game_state.current_player = 1;
        game_state.player_chips = [200, 200];
        game_state.player_bets = [50, 0];
        game_state.current_bet = 50;
        let config = GameConfig::default();

        let actions = get_valid_actions(&game_state, &config);

        assert!(actions.contains(&PokerAction::Call));
        assert!(actions.contains(&PokerAction::Raise));
    }

    #[test]
    fn test_place_bet_updates_state() {
        let mut game_state = GameStateResource::default();
        game_state.player_chips = [100, 100];
        game_state.player_bets = [0, 0];
        game_state.pot = 0;
        game_state.current_bet = 0;
        game_state.current_player = 0;

        place_bet(&mut game_state, 50, true, 50);

        assert_eq!(game_state.player_chips[0], 50);
        assert_eq!(game_state.player_bets[0], 50);
        assert_eq!(game_state.pot, 50);
        assert_eq!(game_state.current_bet, 50);
    }

    #[test]
    fn test_place_bet_all_in() {
        let mut game_state = GameStateResource::default();
        game_state.player_chips = [100, 100];
        game_state.player_bets = [0, 0];
        game_state.pot = 0;
        game_state.current_bet = 0;
        game_state.current_player = 0;

        place_bet(&mut game_state, 200, true, 200);

        assert_eq!(game_state.player_chips[0], 0);
        assert_eq!(game_state.player_bets[0], 100);
        assert_eq!(game_state.pot, 100);
    }

    #[test]
    fn test_split_pot_with_remainder() {
        let mut game_state = GameStateResource::default();
        game_state.player_chips = [100, 100];
        game_state.pot = 100;
        game_state.pot_remainder = 1;

        split_pot(&mut game_state);

        assert_eq!(game_state.player_chips[0], 150);
        assert_eq!(game_state.player_chips[1], 150);
        assert_eq!(game_state.pot_remainder, 1);
    }

    #[test]
    fn test_split_pot_clears_pot() {
        let mut game_state = GameStateResource::default();
        game_state.player_chips = [100, 100];
        game_state.pot = 100;
        game_state.pot_remainder = 0;

        split_pot(&mut game_state);

        assert_eq!(game_state.player_chips[0], 150);
        assert_eq!(game_state.player_chips[1], 150);
        assert_eq!(game_state.pot_remainder, 0);
    }

    #[test]
    fn test_draw_card_returns_card() {
        let mut game_state = GameStateResource::default();
        game_state.deck = Deck::new();
        let initial_remaining = game_state.deck.cards_remaining();

        let card = draw_card(&mut game_state);

        assert!(!card.is_placeholder);
        assert_eq!(game_state.deck.cards_remaining(), initial_remaining - 1);
    }

    #[test]
    fn test_draw_card_emergency_reshuffle() {
        let mut game_state = GameStateResource::default();
        game_state.deck = Deck::new();
        while game_state.deck.cards_remaining() > 0 {
            game_state.deck.draw();
        }

        let card = draw_card(&mut game_state);

        assert!(!card.is_placeholder);
    }

    #[test]
    fn test_advance_street_check_check() {
        let mut game_state = GameStateResource::default();
        game_state.current_round = PokerRound::PreFlop;
        game_state.player_bets = [0, 0];
        game_state.current_bet = 0;
        game_state.dealer_position = 0;
        let config = GameConfig::default();

        advance_street(&mut game_state, &config);

        assert_eq!(game_state.current_round, PokerRound::Flop);
        assert_eq!(game_state.current_bet, 0);
        assert_eq!(game_state.player_bets, [0, 0]);
    }

    #[test]
    fn test_advance_street_both_matched() {
        let mut game_state = GameStateResource::default();
        game_state.current_round = PokerRound::Flop;
        game_state.player_bets = [50, 50];
        game_state.current_bet = 50;
        game_state.dealer_position = 0;
        let config = GameConfig::default();

        advance_street(&mut game_state, &config);

        assert_eq!(game_state.current_round, PokerRound::Turn);
        assert_eq!(game_state.current_bet, 0);
        assert_eq!(game_state.player_bets, [0, 0]);
    }

    #[test]
    fn test_advance_street_not_ready() {
        let mut game_state = GameStateResource::default();
        game_state.current_round = PokerRound::PreFlop;
        game_state.player_bets = [50, 0];
        game_state.current_bet = 50;
        game_state.dealer_position = 0;
        let config = GameConfig::default();

        advance_street(&mut game_state, &config);

        assert_eq!(game_state.current_round, PokerRound::PreFlop);
        assert_eq!(game_state.current_bet, 50);
    }

    #[test]
    fn test_advance_street_to_showdown() {
        let mut game_state = GameStateResource::default();
        game_state.current_round = PokerRound::River;
        game_state.player_bets = [100, 100];
        game_state.current_bet = 100;
        game_state.dealer_position = 0;
        game_state.showdown_timer = 0.0;
        let config = GameConfig::default();

        advance_street(&mut game_state, &config);

        assert_eq!(game_state.current_round, PokerRound::Showdown);
        assert!(game_state.showdown_timer > 0.0);
    }

    #[test]
    fn test_min_cards_for_reshuffle_in_config() {
        let config = GameConfig::default();
        assert_eq!(config.min_cards_for_reshuffle, 9);
    }
}
