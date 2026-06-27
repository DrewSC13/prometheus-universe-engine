use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct FreeCamera;

#[derive(Resource, Debug, Clone, Copy)]
pub struct FreeCameraSettings {
    pub base_speed_mps: f32,
    pub fast_multiplier: f32,
    pub slow_multiplier: f32,
    pub mouse_sensitivity: f32,
}

impl Default for FreeCameraSettings {
    fn default() -> Self {
        Self {
            base_speed_mps: 25.0,
            fast_multiplier: 10.0,
            slow_multiplier: 0.25,
            mouse_sensitivity: 0.002,
        }
    }
}

pub struct FreeCameraPlugin;

impl Plugin for FreeCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FreeCameraSettings::default())
            .add_systems(Update, (free_camera_movement, free_camera_mouse_look));
    }
}

fn free_camera_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<FreeCameraSettings>,
    mut query: Query<&mut Transform, With<FreeCamera>>,
) {
    for mut transform in query.iter_mut() {
        let mut input = Vec3::ZERO;

        if keyboard.pressed(KeyCode::KeyW) {
            input.z -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            input.z += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            input.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            input.x += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyE) {
            input.y += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyQ) {
            input.y -= 1.0;
        }

        if input == Vec3::ZERO {
            continue;
        }

        let mut speed = settings.base_speed_mps;

        if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
            speed *= settings.fast_multiplier;
        }

        if keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight) {
            speed *= settings.slow_multiplier;
        }

        let forward = transform.rotation * Vec3::NEG_Z;
        let right = transform.rotation * Vec3::X;
        let up = Vec3::Y;

        let movement = (right * input.x + up * input.y + forward * -input.z).normalize_or_zero();

        transform.translation += movement * speed * time.delta_secs();
    }
}

fn free_camera_mouse_look(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    settings: Res<FreeCameraSettings>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<FreeCamera>>,
) {
    if !mouse_buttons.pressed(MouseButton::Right) {
        mouse_motion_events.clear();
        return;
    }

    let mut delta = Vec2::ZERO;

    for event in mouse_motion_events.read() {
        delta += event.delta;
    }

    if delta == Vec2::ZERO {
        return;
    }

    for mut transform in query.iter_mut() {
        let (mut yaw, mut pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);

        yaw -= delta.x * settings.mouse_sensitivity;
        pitch -= delta.y * settings.mouse_sensitivity;

        let pitch_limit = std::f32::consts::FRAC_PI_2 - 0.01;
        pitch = pitch.clamp(-pitch_limit, pitch_limit);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
    }
}
