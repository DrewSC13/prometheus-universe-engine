use bevy::prelude::*;

use crate::render::solar_system::{LabelVisibilityMode, OrbitVisibilityMode};
use crate::simulation::bodies::{orbiting_bodies, root_bodies, SOLAR_SYSTEM_BODIES};
use crate::time::{SimulationClock, TimeDirection};

#[derive(Component, Debug)]
pub struct SimulationHudText;

pub struct SimulationHudPlugin;

impl Plugin for SimulationHudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_simulation_hud)
            .add_systems(Update, update_simulation_hud);
    }
}

fn spawn_simulation_hud(mut commands: Commands) {
    commands.spawn((
        Text::new("Prometheus Universe Engine"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(12.0),
            top: Val::Px(12.0),
            ..default()
        },
        SimulationHudText,
    ));
}

fn update_simulation_hud(
    simulation_clock: Res<SimulationClock>,
    label_visibility_mode: Option<Res<LabelVisibilityMode>>,
    orbit_visibility_mode: Option<Res<OrbitVisibilityMode>>,
    mut query: Query<&mut Text, With<SimulationHudText>>,
) {
    let simulation_time = simulation_clock.0;

    let direction = match simulation_time.direction {
        TimeDirection::Forward => "Forward",
        TimeDirection::Backward => "Backward",
    };

    let paused = if simulation_time.paused { "yes" } else { "no" };

    let total_bodies = SOLAR_SYSTEM_BODIES.len();
    let root_body_count = root_bodies().count();
    let orbiting_body_count = orbiting_bodies().count();

    let label_mode = label_visibility_mode
        .as_deref()
        .map(LabelVisibilityMode::as_str)
        .unwrap_or("unknown");

    let orbit_mode = orbit_visibility_mode
        .as_deref()
        .map(OrbitVisibilityMode::as_str)
        .unwrap_or("unknown");

    for mut text in query.iter_mut() {
        text.0 = format!(
            "Prometheus Universe Engine\n\
             Fase 1: Sistema Solar catalogado\n\
             \n\
             Catalogo:\n\
             Cuerpos totales: {}\n\
             Cuerpos raiz: {}\n\
             Cuerpos orbitando: {}\n\
             Etiquetas: {}\n\
             Orbitas: {}\n\
             \n\
             Tiempo:\n\
             JD TDB: {:.5}\n\
             Dias desde J2000: {:.2}\n\
             Escala temporal: x{:.0}\n\
             Direccion: {}\n\
             Pausado: {}\n\
             \n\
             Controles:\n\
             Space = pausa/reanuda\n\
             1-6 = velocidad\n\
             B = invertir tiempo\n\
             R = reset J2000\n\
             L = etiquetas\n\
             O = orbitas",
            total_bodies,
            root_body_count,
            orbiting_body_count,
            label_mode,
            orbit_mode,
            simulation_time.jd_tdb,
            simulation_time.days_since_j2000(),
            simulation_time.time_scale,
            direction,
            paused,
        );
    }
}
