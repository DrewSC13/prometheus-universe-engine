use bevy::prelude::*;

use crate::simulation::bodies::{
    body_axial_tilt_degrees, body_rotation_period_hours, BodyClass, BodyId,
    CelestialBodyDefinition, SOLAR_SYSTEM_BODIES,
};
use crate::simulation::catalog::{body_definition, body_position_meters};

const KILOMETERS_PER_METER: f64 = 1.0 / 1_000.0;
const MILLION_KILOMETERS: f64 = 1_000_000.0;
const DAYS_PER_YEAR: f64 = 365.25;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SelectedBody {
    pub id: Option<BodyId>,
}

pub struct BodySelectionPlugin;

impl Plugin for BodySelectionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedBody::default())
            .add_systems(Update, keyboard_body_selection);
    }
}

fn keyboard_body_selection(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut selected_body: ResMut<SelectedBody>,
) {
    let next_selection = if keyboard.just_pressed(KeyCode::KeyN) {
        next_body_id(selected_body.id)
    } else if keyboard.just_pressed(KeyCode::KeyP) {
        previous_body_id(selected_body.id)
    } else if keyboard.just_pressed(KeyCode::Escape) {
        None
    } else {
        return;
    };

    selected_body.id = next_selection;

    match selected_body_definition(*selected_body) {
        Some(body) => info!(
            "Selected body: {} | class: {} | radius: {} | mass: {:.3e} kg",
            body.name,
            body_class_name(body.class),
            format_radius_meters(body.physical_radius_meters),
            body.mass_kg
        ),
        None => info!("Selected body cleared."),
    }
}

pub fn selected_body_definition(
    selected_body: SelectedBody,
) -> Option<&'static CelestialBodyDefinition> {
    selected_body.id.and_then(body_definition)
}

pub fn next_body_id(current: Option<BodyId>) -> Option<BodyId> {
    cycle_body_id(current, 1)
}

pub fn previous_body_id(current: Option<BodyId>) -> Option<BodyId> {
    cycle_body_id(current, -1)
}

pub fn body_class_name(class: BodyClass) -> &'static str {
    match class {
        BodyClass::Star => "estrella",
        BodyClass::TerrestrialPlanet => "planeta terrestre",
        BodyClass::GasGiant => "gigante gaseoso",
        BodyClass::IceGiant => "gigante helado",
        BodyClass::NaturalSatellite => "satelite natural",
    }
}

pub fn selected_body_compact_label(selected_body: SelectedBody) -> &'static str {
    selected_body_definition(selected_body)
        .map(|body| body.name)
        .unwrap_or("none")
}

pub fn selected_body_hud_summary(selected_body: SelectedBody, days_since_j2000: f64) -> String {
    match selected_body_definition(selected_body) {
        Some(body) => format!(
            "Nombre: {}\nClase: {}\nRadio: {}\nMasa: {:.3e} kg\nOrbita: {}\nDist. al Sol: {}\nDist. al padre: {}\nPeriodo orbital: {}\nRotacion: {}\nInclinacion axial: {:.2} deg\nEscala visual: educativa",
            body.name,
            body_class_name(body.class),
            format_radius_meters(body.physical_radius_meters),
            body.mass_kg,
            body_orbit_parent_name(body),
            format_optional_distance(body_distance_to_sun_meters(body, days_since_j2000)),
            format_optional_distance(body_distance_to_parent_meters(body, days_since_j2000)),
            format_orbital_period(body),
            format_rotation_period_hours(body_rotation_period_hours(body.id)),
            body_axial_tilt_degrees(body.id)
        ),
        None => "Nombre: none\nClase: -\nRadio: -\nMasa: -\nOrbita: -\nDist. al Sol: -\nDist. al padre: -\nPeriodo orbital: -\nRotacion: -\nInclinacion axial: -\nEscala visual: educativa".to_string(),
    }
}

pub fn body_orbit_parent_name(body: &CelestialBodyDefinition) -> &'static str {
    body.orbit
        .and_then(|orbit| body_definition(orbit.parent).map(|parent| parent.name))
        .unwrap_or("none")
}

pub fn body_distance_to_sun_meters(
    body: &CelestialBodyDefinition,
    days_since_j2000: f64,
) -> Option<f64> {
    let body_position = body_position_meters(body.id, days_since_j2000)?;
    let sun_position = body_position_meters(BodyId::Sun, days_since_j2000)?;

    Some(body_position.distance(sun_position))
}

pub fn body_distance_to_parent_meters(
    body: &CelestialBodyDefinition,
    days_since_j2000: f64,
) -> Option<f64> {
    let orbit = body.orbit?;
    let body_position = body_position_meters(body.id, days_since_j2000)?;
    let parent_position = body_position_meters(orbit.parent, days_since_j2000)?;

    Some(body_position.distance(parent_position))
}

pub fn format_radius_meters(radius_meters: f64) -> String {
    format!("{:.0} km", radius_meters * KILOMETERS_PER_METER)
}

pub fn format_distance_meters(distance_meters: f64) -> String {
    let kilometers = distance_meters * KILOMETERS_PER_METER;

    if kilometers >= MILLION_KILOMETERS {
        format!("{:.3} millones km", kilometers / MILLION_KILOMETERS)
    } else {
        format!("{kilometers:.0} km")
    }
}

pub fn format_optional_distance(distance_meters: Option<f64>) -> String {
    distance_meters
        .map(format_distance_meters)
        .unwrap_or_else(|| "none".to_string())
}

pub fn format_orbital_period(body: &CelestialBodyDefinition) -> String {
    body.orbit
        .map(|orbit| format_period_days(orbit.period_days))
        .unwrap_or_else(|| "none".to_string())
}

pub fn format_period_days(period_days: f64) -> String {
    if period_days >= DAYS_PER_YEAR {
        format!("{:.2} anios", period_days / DAYS_PER_YEAR)
    } else if period_days >= 1.0 {
        format!("{period_days:.3} dias")
    } else {
        format!("{:.2} h", period_days * 24.0)
    }
}

pub fn format_rotation_period_hours(period_hours: f64) -> String {
    let suffix = if period_hours < 0.0 { " retr." } else { "" };
    let absolute_hours = period_hours.abs();

    if absolute_hours >= 48.0 {
        format!("{:.3} dias{}", absolute_hours / 24.0, suffix)
    } else {
        format!("{absolute_hours:.2} h{suffix}")
    }
}

fn cycle_body_id(current: Option<BodyId>, step: isize) -> Option<BodyId> {
    if SOLAR_SYSTEM_BODIES.is_empty() {
        return None;
    }

    let current_index = current.and_then(body_index);

    let next_index = match current_index {
        Some(index) => {
            let len = SOLAR_SYSTEM_BODIES.len() as isize;
            (index as isize + step).rem_euclid(len) as usize
        }
        None if step >= 0 => 0,
        None => SOLAR_SYSTEM_BODIES.len() - 1,
    };

    Some(SOLAR_SYSTEM_BODIES[next_index].id)
}

fn body_index(id: BodyId) -> Option<usize> {
    SOLAR_SYSTEM_BODIES.iter().position(|body| body.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selected_body_starts_empty() {
        assert_eq!(SelectedBody::default().id, None);
    }

    #[test]
    fn next_body_starts_at_sun_when_selection_is_empty() {
        assert_eq!(next_body_id(None), Some(BodyId::Sun));
    }

    #[test]
    fn previous_body_starts_at_last_catalog_body_when_selection_is_empty() {
        assert_eq!(previous_body_id(None), Some(BodyId::Triton));
    }

    #[test]
    fn next_body_advances_in_catalog_order() {
        assert_eq!(next_body_id(Some(BodyId::Sun)), Some(BodyId::Mercury));
        assert_eq!(next_body_id(Some(BodyId::Earth)), Some(BodyId::Moon));
    }

    #[test]
    fn previous_body_wraps_from_sun_to_last_body() {
        assert_eq!(previous_body_id(Some(BodyId::Sun)), Some(BodyId::Triton));
    }

    #[test]
    fn selected_body_definition_reads_catalog_data() {
        let selected = SelectedBody {
            id: Some(BodyId::Earth),
        };

        let body = selected_body_definition(selected).unwrap();

        assert_eq!(body.name, "Tierra");
        assert_eq!(body.class, BodyClass::TerrestrialPlanet);
    }

    #[test]
    fn body_class_names_are_stable() {
        assert_eq!(body_class_name(BodyClass::Star), "estrella");
        assert_eq!(
            body_class_name(BodyClass::TerrestrialPlanet),
            "planeta terrestre"
        );
        assert_eq!(body_class_name(BodyClass::GasGiant), "gigante gaseoso");
        assert_eq!(body_class_name(BodyClass::IceGiant), "gigante helado");
        assert_eq!(
            body_class_name(BodyClass::NaturalSatellite),
            "satelite natural"
        );
    }

    #[test]
    fn selected_body_compact_label_reports_none_when_empty() {
        assert_eq!(selected_body_compact_label(SelectedBody::default()), "none");
    }

    #[test]
    fn selected_body_compact_label_reports_body_name() {
        let selected = SelectedBody {
            id: Some(BodyId::Earth),
        };

        assert_eq!(selected_body_compact_label(selected), "Tierra");
    }

    #[test]
    fn body_orbit_parent_name_reports_catalog_parent() {
        let earth = body_definition(BodyId::Earth).unwrap();
        let sun = body_definition(BodyId::Sun).unwrap();

        assert_eq!(body_orbit_parent_name(earth), "Sol");
        assert_eq!(body_orbit_parent_name(sun), "none");
    }

    #[test]
    fn selected_body_hud_summary_contains_core_fields() {
        let selected = SelectedBody {
            id: Some(BodyId::Earth),
        };

        let summary = selected_body_hud_summary(selected, 0.0);

        assert!(summary.contains("Nombre: Tierra"));
        assert!(summary.contains("Clase: planeta terrestre"));
        assert!(summary.contains("Radio: 6371 km"));
        assert!(summary.contains("Masa: 5.972e24 kg"));
        assert!(summary.contains("Orbita: Sol"));
        assert!(summary.contains("Dist. al Sol:"));
        assert!(summary.contains("Dist. al padre:"));
        assert!(summary.contains("Periodo orbital:"));
        assert!(summary.contains("Rotacion:"));
        assert!(summary.contains("Inclinacion axial: 23.44 deg"));
        assert!(summary.contains("Escala visual: educativa"));
    }

    #[test]
    fn moon_parent_distance_uses_catalog_orbit_distance() {
        let moon = body_definition(BodyId::Moon).unwrap();
        let distance = body_distance_to_parent_meters(moon, 0.0).unwrap();

        assert!((distance - 384_400_000.0).abs() < 1.0);
    }

    #[test]
    fn sun_has_no_parent_distance_or_orbital_period() {
        let sun = body_definition(BodyId::Sun).unwrap();

        assert_eq!(body_distance_to_parent_meters(sun, 0.0), None);
        assert_eq!(format_orbital_period(sun), "none");
    }

    #[test]
    fn distance_formatter_uses_millions_for_large_orbits() {
        assert_eq!(
            format_distance_meters(149_597_870_700.0),
            "149.598 millones km"
        );
        assert_eq!(format_distance_meters(384_400_000.0), "384400 km");
    }

    #[test]
    fn rotation_formatter_marks_retrograde_rotation() {
        assert_eq!(format_rotation_period_hours(23.934_469_6), "23.93 h");
        assert!(format_rotation_period_hours(-5_832.5).contains("retr."));
    }
}
