use bevy::prelude::*;

use crate::coordinates::{GlobalPosition, GlobalPositionComponent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraViewPreset {
    Overview,
    Wide,
    InnerSystem,
}

pub struct CameraViewPresetsPlugin;

impl Plugin for CameraViewPresetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_camera_view_presets);
    }
}

fn keyboard_camera_view_presets(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, Option<&mut GlobalPositionComponent>), With<Camera3d>>,
) {
    let preset = if keyboard.just_pressed(KeyCode::KeyC) {
        Some(CameraViewPreset::Overview)
    } else if keyboard.just_pressed(KeyCode::KeyV) {
        Some(CameraViewPreset::Wide)
    } else if keyboard.just_pressed(KeyCode::KeyF) {
        Some(CameraViewPreset::InnerSystem)
    } else {
        None
    };

    let Some(preset) = preset else {
        return;
    };

    for (mut transform, global_position) in query.iter_mut() {
        *transform = camera_preset_transform(preset);

        if let Some(mut global_position) = global_position {
            global_position.position = GlobalPosition::ZERO;
        }
    }

    info!("Camera view preset: {}", camera_preset_name(preset));
}

pub fn camera_preset_name(preset: CameraViewPreset) -> &'static str {
    match preset {
        CameraViewPreset::Overview => "overview",
        CameraViewPreset::Wide => "wide",
        CameraViewPreset::InnerSystem => "inner-system",
    }
}

pub fn initial_camera_transform() -> Transform {
    camera_preset_transform(CameraViewPreset::Overview)
}

pub fn camera_preset_transform(preset: CameraViewPreset) -> Transform {
    match preset {
        CameraViewPreset::Overview => {
            Transform::from_xyz(0.0, 90.0, 240.0).looking_at(Vec3::ZERO, Vec3::Y)
        }
        CameraViewPreset::Wide => {
            Transform::from_xyz(0.0, 160.0, 420.0).looking_at(Vec3::ZERO, Vec3::Y)
        }
        CameraViewPreset::InnerSystem => {
            Transform::from_xyz(0.0, 28.0, 85.0).looking_at(Vec3::ZERO, Vec3::Y)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_camera_matches_realistic_overview() {
        assert_eq!(
            initial_camera_transform().translation,
            camera_preset_transform(CameraViewPreset::Overview).translation
        );
    }

    #[test]
    fn camera_preset_names_are_stable() {
        assert_eq!(camera_preset_name(CameraViewPreset::Overview), "overview");
        assert_eq!(camera_preset_name(CameraViewPreset::Wide), "wide");
        assert_eq!(
            camera_preset_name(CameraViewPreset::InnerSystem),
            "inner-system"
        );
    }

    #[test]
    fn wide_camera_preset_is_farther_than_inner_system() {
        let wide = camera_preset_transform(CameraViewPreset::Wide);
        let inner = camera_preset_transform(CameraViewPreset::InnerSystem);

        assert!(wide.translation.length() > inner.translation.length());
    }

    #[test]
    fn wide_camera_preset_covers_realistic_outer_system() {
        let wide = camera_preset_transform(CameraViewPreset::Wide);

        assert!(wide.translation.length() > 400.0);
    }

    #[test]
    fn overview_camera_preset_is_between_inner_and_wide() {
        let overview = camera_preset_transform(CameraViewPreset::Overview);
        let wide = camera_preset_transform(CameraViewPreset::Wide);
        let inner = camera_preset_transform(CameraViewPreset::InnerSystem);

        assert!(overview.translation.length() > inner.translation.length());
        assert!(overview.translation.length() < wide.translation.length());
    }
}
