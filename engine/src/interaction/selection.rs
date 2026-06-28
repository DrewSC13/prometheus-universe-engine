use bevy::prelude::*;

use crate::simulation::bodies::{BodyClass, BodyId, CelestialBodyDefinition, SOLAR_SYSTEM_BODIES};
use crate::simulation::catalog::body_definition;

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
            "Selected body: {} | class: {} | radius: {:.0} km | mass: {:.3e} kg",
            body.name,
            body_class_name(body.class),
            body.physical_radius_meters / 1_000.0,
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
        BodyClass::Star => "star",
        BodyClass::TerrestrialPlanet => "terrestrial planet",
        BodyClass::GasGiant => "gas giant",
        BodyClass::IceGiant => "ice giant",
        BodyClass::NaturalSatellite => "natural satellite",
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
        assert_eq!(body_class_name(BodyClass::Star), "star");
        assert_eq!(
            body_class_name(BodyClass::TerrestrialPlanet),
            "terrestrial planet"
        );
        assert_eq!(body_class_name(BodyClass::GasGiant), "gas giant");
        assert_eq!(body_class_name(BodyClass::IceGiant), "ice giant");
        assert_eq!(
            body_class_name(BodyClass::NaturalSatellite),
            "natural satellite"
        );
    }
}
