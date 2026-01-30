use crate::game::DealAnimation;
use bevy::prelude::*;

/// Updates all card deal animations with easing.
pub fn update_animations(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<GameConfig>,
    mut query: Query<(Entity, &mut Transform, &DealAnimation)>,
) {
    let elapsed = time.elapsed_seconds();

    for (entity, mut transform, anim) in query.iter_mut() {
        let anim_elapsed = elapsed - anim.start_time - anim.delay;

        if anim_elapsed > 0.0 && anim.duration > 0.0 {
            let t = (anim_elapsed / anim.duration).min(1.0);
            let eased = 1.0 - (1.0 - t).powi(config.animations.easing_power);
            transform.translation = anim.start_pos.lerp(anim.target_pos, eased);

            if t >= 1.0 {
                commands.entity(entity).remove::<DealAnimation>();
            }
        }
    }
}
