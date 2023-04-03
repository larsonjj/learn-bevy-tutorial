use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::physics::PLAY_AREA_BORDER_MARGIN;
use crate::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

const PLAYER_SPEED: f32 = 150.;
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
        .insert(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController::default())
        .insert(Collider::ball(PLAYER_SIZE / 2.0))
        .insert(GravityScale(0.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ActiveEvents::COLLISION_EVENTS);
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }

    let speed = PLAYER_SPEED;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
}

fn move_player_controller(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut KinematicCharacterController, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }

    let speed = PLAYER_SPEED;

    for mut player_controller in &mut player_query {
        // Print out player controller translation
        println!(
            "Player controller translation: {:?}",
            player_controller.translation
        );
        player_controller.translation = match player_controller.translation {
            Some(mut vector) => {
                vector.x = actions.player_movement.unwrap().x * speed * time.delta_seconds();
                vector.y = actions.player_movement.unwrap().y * speed * time.delta_seconds();
                Some(vector)
            }
            None => Some(Vec2::new(
                actions.player_movement.unwrap().x * speed * time.delta_seconds(),
                actions.player_movement.unwrap().y * speed * time.delta_seconds(),
            )),
        }
    }
}
