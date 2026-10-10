use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

const CAMERA_MOVE_SPEED: f32 = 20.0;
const CAMERA_TURN_SPEED: f32 = 0.5;

pub struct FreeCamPlugin;
impl Plugin for FreeCamPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (
                move_camera,
                rotate_camera,
            ));
    }
} 

fn spawn_camera(
    mut commands: Commands,
){
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 45.0)
    ));
}
fn move_camera(
    mut camera: Single<&mut Transform, With<Camera3d>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::ArrowUp)
        || keyboard_input.pressed(KeyCode::KeyW)
    {
        direction += *camera.forward();
    }

    if keyboard_input.pressed(KeyCode::ArrowDown)
        || keyboard_input.pressed(KeyCode::KeyS)
    {
        direction -= *camera.forward();
    }

    if keyboard_input.pressed(KeyCode::ArrowLeft)
        || keyboard_input.pressed(KeyCode::KeyA)
    {
        direction -= *camera.right();
    }

    if keyboard_input.pressed(KeyCode::ArrowRight)
        || keyboard_input.pressed(KeyCode::KeyD)
    {
        direction += *camera.right();
    }

    if keyboard_input.pressed(KeyCode::KeyQ) {
        direction -= Vec3::Y;
    }

    if keyboard_input.pressed(KeyCode::KeyE) {
        direction += Vec3::Y;
    }

    if direction != Vec3::ZERO {
        direction = direction.normalize() * CAMERA_MOVE_SPEED * time.delta_secs();
        camera.translation += direction;
    }
}

fn rotate_camera(
    camera: Single<&mut Transform, With<Camera3d>>,
    mouse_input: Res<ButtonInput<MouseButton>>,    
    mut mouse_motion: MessageReader<MouseMotion>,
    time: Res<Time>,
){
    if mouse_input.pressed(MouseButton::Right)  {
        let mut transform = camera.into_inner();
        let mut delta = Vec2::ZERO;

        for event in mouse_motion.read() {
            delta += event.delta;
        }

        let turn_speed = CAMERA_TURN_SPEED * time.delta_secs();
        transform.rotate_y(-delta.x * turn_speed);
        transform.rotate_local_x(-delta.y * turn_speed);
    }
}