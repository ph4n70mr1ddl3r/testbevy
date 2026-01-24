use crate::constants::*;
use crate::game::*;
use crate::poker_logic::{Card, PokerRound};
use bevy::prelude::*;

/// Spawns the table background with two layers of green felt.
pub fn spawn_table(
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

/// Spawns a player's hole cards and labels.
#[allow(clippy::cast_precision_loss)]
pub fn spawn_player(
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
        let card_offset = (j as f32 - PLAYER_CARD_CENTER_OFFSET) * config.card_offset_spacing;
        let target_pos = Vec3::new(x_pos + card_offset, card_target_y, 1.0);
        let card = draw_card(game_state).expect("Failed to draw card from deck");

        if id == 0 {
            game_state.p1_hole[j] = card;
        } else {
            game_state.p2_hole[j] = card;
        }

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
                duration: config.animations.deal_duration,
                delay: (id * 2 + j) as f32 * config.animations.card_deal_delay,
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
        config.ui_positions.player_label_offset
    } else {
        config.ui_positions.chip_label_offset
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
            transform: Transform::from_xyz(
                x_pos,
                y_pos + config.ui_positions.player_label_offset,
                UI_TEXT_Z_POSITION,
            ),
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
            transform: Transform::from_xyz(x_pos, y_pos + chip_y_offset, UI_TEXT_Z_POSITION),
            ..default()
        },
        HandMarker,
    ));
}

/// Spawns a community card with face-down animation.
#[allow(clippy::cast_precision_loss)]
pub fn spawn_community_card(
    commands: &mut Commands,
    game_state: &mut GameStateResource,
    config: &GameConfig,
    colors: &ColorPalette,
    i: usize,
    animation_start_time: f32,
) {
    let x_offset = (i as f32 - COMMUNITY_CARD_CENTER_INDEX) * config.card_offset_spacing;
    let community_card = draw_card(game_state).expect("Failed to draw community card from deck");

    game_state.community_cards[i] = community_card;

    let is_hidden = matches!(i, 3 | 4);

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
            duration: config.animations.community_duration,
            delay: config.animations.community_delay_start
                + i as f32 * config.animations.community_delay_increment,
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

/// Spawns all UI text elements for displaying game state.
pub fn spawn_ui(
    commands: &mut Commands,
    game_state: &GameStateResource,
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
            transform: Transform::from_xyz(
                0.0,
                config.ui_positions.pot_display_y,
                UI_TEXT_Z_POSITION,
            ),
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
            transform: Transform::from_xyz(
                config.ui_positions.hand_number_x,
                config.ui_positions.hand_number_y,
                UI_TEXT_Z_POSITION,
            ),
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
            transform: Transform::from_xyz(0.0, PLAYER_CHIPS_Y, UI_TEXT_Z_POSITION),
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
            transform: Transform::from_xyz(0.0, OPPONENT_CHIPS_Y, UI_TEXT_Z_POSITION),
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
            transform: Transform::from_xyz(
                config.ui_positions.round_display_x,
                config.ui_positions.round_display_y,
                UI_TEXT_Z_POSITION,
            ),
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
            transform: Transform::from_xyz(
                0.0,
                config.ui_positions.action_display_y,
                UI_TEXT_Z_POSITION,
            ),
            ..default()
        },
        ActionDisplay,
        HandMarker,
    ));
}

/// Spawns both players with their hole cards and labels.
pub fn spawn_all_players(
    commands: &mut Commands,
    game_state: &mut GameStateResource,
    config: &GameConfig,
    colors: ColorPalette,
    animation_start_time: f32,
) {
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
}

/// Spawns all 5 community cards in their initial face-down positions.
pub fn spawn_all_community_cards(
    commands: &mut Commands,
    game_state: &mut GameStateResource,
    config: &GameConfig,
    colors: &ColorPalette,
    animation_start_time: f32,
) {
    for i in 0..5 {
        spawn_community_card(
            commands,
            game_state,
            config,
            colors,
            i,
            animation_start_time,
        );
    }
}

/// Struct for organizing card text rendering parameters.
pub struct CardTextParams {
    pub card: Card,
    pub target_pos: Vec3,
    pub offset_x: f32,
    pub offset_y: f32,
    pub rotation: f32,
    pub text_color: Color,
    pub font_size: f32,
}

impl CardTextParams {
    pub const fn new(
        card: Card,
        target_pos: Vec3,
        offset_x: f32,
        offset_y: f32,
        rotation: f32,
        text_color: Color,
        font_size: f32,
    ) -> Self {
        Self {
            card,
            target_pos,
            offset_x,
            offset_y,
            rotation,
            text_color,
            font_size,
        }
    }

    pub fn spawn(&self, commands: &mut Commands) {
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

/// Spawns the text elements for a card (rank and suit).
pub fn spawn_card_text(
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
        core::f32::consts::PI,
        text_color,
        font_size,
    )
    .spawn(commands);
}

/// Updates UI text elements to reflect current game state.
pub fn update_ui(
    game_state: Res<GameStateResource>,
    mut pot_query: Query<&mut Text, With<PotDisplay>>,
    mut hand_number_query: Query<&mut Text, With<HandNumberDisplay>>,
    mut player_chips_query: Query<&mut Text, With<PlayerChipsDisplay>>,
    mut opponent_chips_query: Query<&mut Text, With<OpponentChipsDisplay>>,
    mut round_query: Query<&mut Text, With<RoundDisplay>>,
    mut action_query: Query<&mut Text, With<ActionDisplay>>,
) {
    for mut text in pot_query.iter_mut() {
        text.sections[0].value = format!("Pot: ${}", game_state.pot);
    }

    for mut text in hand_number_query.iter_mut() {
        text.sections[0].value = format!("Hand: #{}", game_state.hand_number);
    }

    for mut text in player_chips_query.iter_mut() {
        text.sections[0].value = format!("Chips: ${}", game_state.player_chips[0]);
    }

    for mut text in opponent_chips_query.iter_mut() {
        text.sections[0].value = format!("P2: ${}", game_state.player_chips[1]);
    }

    for mut text in round_query.iter_mut() {
        text.sections[0].value = get_round_name(game_state.current_round).to_string();
    }

    let action_text = if game_state.winner.is_some() {
        game_state.last_winner_message.clone()
    } else {
        game_state.last_action.clone()
    };

    if let Some(mut text) = action_query.iter_mut().next() {
        text.sections[0].value = action_text;
    }
}

/// Updates community card visuals based on the current round.
pub fn update_card_visuals(
    mut query: Query<(&mut Sprite, Option<&CommunityCard>)>,
    game_state: Res<GameStateResource>,
    colors: Res<ColorPalette>,
) {
    let face_up_color = colors.face_up_white;
    let face_down_color = colors.face_down_dark;

    for (mut sprite, community_card) in query.iter_mut() {
        if let Some(cc) = community_card {
            let should_reveal = match game_state.current_round {
                PokerRound::Flop => cc.index < FLOP_CARD_COUNT,
                PokerRound::Turn => cc.index < TURN_CARD_COUNT,
                PokerRound::River | PokerRound::Showdown => cc.index < RIVER_CARD_COUNT,
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
