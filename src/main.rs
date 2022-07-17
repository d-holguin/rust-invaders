#![allow(unused)] //comment out later
use bevy::prelude::*;
use components::{Movable, Velocity};
use player::PlayerPlugin;
use enemy::EnemyPlugin;

mod components;
mod enemy;
mod player;
// region: Asset Constants
const PLAYER_SPRITE: &str = "player_b_01.png";
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9.0, 54.0);
const PLAYER_SIZE: (f32, f32) = (144.0, 75.0);
//enemy
const ENEMY_SPRITE: &str = "enemy_a_01.png";
const ENEMY_SIZE: (f32, f32) = (144.0, 75.0);
const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
const ENEMY_LASER_SIZE: (f32, f32) = (17.0, 55.0);

const SPRITE_SCALE: f32 = 0.5;
// region: Game constant
const TIME_STEP: f32 = 1.0 / 60.0;
const BASE_SPEED: f32 = 500.0;
// region: Resources
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
    enemy: Handle<Image>,
    enemy_laser: Handle<Image>,
}
// endregion: resources

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Rust Invaders!".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_startup_system(setup_system)
        .add_system(movable_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    //capture window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());
    // pos for window
    //window.set_position(IVec2::new(2780, 4900));

    // adding WinSize Resource
    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);
    // adding GameTextures Resource
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
    };
    commands.insert_resource(game_textures);
}
fn movable_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        if movable.auto_despawn {
            const MARGIN: f32 = 200.0;
            if translation.y > win_size.h / 2.0 + MARGIN
                || translation.y < -win_size.h / 2.0 - MARGIN
                || translation.x > win_size.w / 2.0 + MARGIN
                || translation.x < -win_size.w / 2.0 - MARGIN
            {
                commands.entity(entity).despawn();
            }
        }
    }
}
