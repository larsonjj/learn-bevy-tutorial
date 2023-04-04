use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

const PLAYER_SPEED: f32 = 300.;
const PLAYER_SIZE: f32 = 64.;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
            .add_system(move_player_controller.in_set(OnUpdate(GameState::Playing)));
    }
}

fn spawn_player(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands
        .spawn(SpriteBundle {
            texture: textures.texture_ball.clone(),
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        })
        .insert(Player)
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(PLAYER_SIZE / 2.0))
        .insert(Velocity::linear(Vec2::ZERO))
        .insert(GravityScale(0.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ActiveEvents::COLLISION_EVENTS);
}

fn move_player_controller(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Velocity, With<Player>>,
) {
    for mut player_controller_velocity in &mut player_query {
        if actions.player_movement.is_none() {
            return player_controller_velocity.linvel = Vec2::ZERO;
        }
        player_controller_velocity.linvel = Vec2::new(
            actions.player_movement.unwrap().x * PLAYER_SPEED * time.delta_seconds(),
            actions.player_movement.unwrap().y * PLAYER_SPEED * time.delta_seconds(),
        ) * 100.;
    }
}
