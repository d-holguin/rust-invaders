use crate::{
    components::{Enemy, FromEnemy, SpriteSize},
    EnemyCount, GameTextures, WinSize, ENEMY_MAX, ENEMY_SIZE, SPRITE_SCALE,
};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enemy_spawn_system);
    }
}
fn enemy_spawn_system(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    if enemy_count.0 < ENEMY_MAX {
        //compute the x/y
        let mut rng = thread_rng();
        let w_span = win_size.w / 2.0 - 100.0;
        let h_span = win_size.h / 2.0 - 100.0;
        let x = rng.gen_range(-w_span..w_span);
        let y = rng.gen_range(-h_span..h_span);
        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.enemy.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 10.0),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy)
            .insert(SpriteSize::from(ENEMY_SIZE));

        enemy_count.0 += 1;
    }
}
