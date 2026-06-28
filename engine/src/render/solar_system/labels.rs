use super::body_visual_position;

use crate::simulation::bodies::{BodyClass, BodyId, CelestialBodyDefinition, SOLAR_SYSTEM_BODIES};
use crate::time::SimulationClock;

use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelVisibilityMode {
    MajorOnly,
    All,
    None,
}

impl Default for LabelVisibilityMode {
    fn default() -> Self {
        Self::MajorOnly
    }
}

impl LabelVisibilityMode {
    pub fn next(self) -> Self {
        match self {
            Self::MajorOnly => Self::All,
            Self::All => Self::None,
            Self::None => Self::MajorOnly,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MajorOnly => "major",
            Self::All => "all",
            Self::None => "none",
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct SolarBodyLabel {
    pub id: BodyId,
}

pub(super) fn keyboard_label_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut label_visibility_mode: ResMut<LabelVisibilityMode>,
) {
    if keyboard.just_pressed(KeyCode::KeyL) {
        *label_visibility_mode = label_visibility_mode.next();
        info!("Label visibility mode: {}", label_visibility_mode.as_str());
    }
}

pub(super) fn label_font_size(body: &CelestialBodyDefinition) -> f32 {
    match body.class {
        BodyClass::Star => 42.0,
        BodyClass::GasGiant | BodyClass::IceGiant => 30.0,
        BodyClass::TerrestrialPlanet => 26.0,
        BodyClass::NaturalSatellite => 22.0,
    }
}

pub(super) fn label_vertical_offset(body: &CelestialBodyDefinition) -> f32 {
    match body.class {
        BodyClass::Star => body.visual_radius + 2.2,
        BodyClass::GasGiant | BodyClass::IceGiant => body.visual_radius + 1.5,
        BodyClass::TerrestrialPlanet => body.visual_radius + 1.1,
        BodyClass::NaturalSatellite => body.visual_radius + 0.9,
    }
}

pub(super) fn label_color(body: &CelestialBodyDefinition) -> Color {
    match body.class {
        BodyClass::Star => Color::srgb(1.0, 0.92, 0.35),
        BodyClass::TerrestrialPlanet => Color::srgb(0.88, 0.95, 1.0),
        BodyClass::GasGiant => Color::srgb(1.0, 0.78, 0.52),
        BodyClass::IceGiant => Color::srgb(0.55, 0.95, 1.0),
        BodyClass::NaturalSatellite => Color::srgb(0.82, 0.82, 0.82),
    }
}

pub(super) fn spawn_label(
    commands: &mut Commands,
    text: &'static str,
    id: BodyId,
    font_size: f32,
    color: Color,
) {
    commands.spawn((
        Text2d::new(text),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(color),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Visibility::Visible,
        SolarBodyLabel { id },
    ));
}

pub(super) fn update_solar_body_labels(
    simulation_clock: Res<SimulationClock>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<SolarBodyLabel>)>,
    mut query: Query<(&SolarBodyLabel, &mut Transform)>,
) {
    let days_since_j2000 = simulation_clock.0.days_since_j2000();
    let camera_rotation = camera_query
        .iter()
        .next()
        .map(|transform| transform.rotation)
        .unwrap_or(Quat::IDENTITY);

    for (label, mut transform) in query.iter_mut() {
        let Some(body) = SOLAR_SYSTEM_BODIES.iter().find(|body| body.id == label.id) else {
            continue;
        };

        let Some(visual_position) = body_visual_position(label.id, days_since_j2000) else {
            continue;
        };

        transform.translation = visual_position + Vec3::new(0.0, label_vertical_offset(body), 0.0);
        transform.rotation = camera_rotation;
    }
}

pub(super) fn apply_label_visibility(
    label_visibility_mode: Res<LabelVisibilityMode>,
    mut query: Query<(&SolarBodyLabel, &mut Visibility)>,
) {
    for (label, mut visibility) in query.iter_mut() {
        let visible = match *label_visibility_mode {
            LabelVisibilityMode::MajorOnly => is_major_body_label(label.id),
            LabelVisibilityMode::All => true,
            LabelVisibilityMode::None => false,
        };

        *visibility = if visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

pub(super) fn is_major_body_label(id: BodyId) -> bool {
    matches!(
        id,
        BodyId::Sun
            | BodyId::Mercury
            | BodyId::Venus
            | BodyId::Earth
            | BodyId::Mars
            | BodyId::Jupiter
            | BodyId::Saturn
            | BodyId::Uranus
            | BodyId::Neptune
    )
}
