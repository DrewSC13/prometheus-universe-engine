use bevy::prelude::*;

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
    mut query: Query<&mut Text, With<SimulationHudText>>,
) {
    let simulation_time = simulation_clock.0;

    let direction = match simulation_time.direction {
        TimeDirection::Forward => "Forward",
        TimeDirection::Backward => "Backward",
    };

    let paused = if simulation_time.paused { "yes" } else { "no" };

    for mut text in query.iter_mut() {
        text.0 = format!(
            "Prometheus Universe Engine\n\
             Fase 1: Sistema Solar catalogado\n\
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
             R = reset J2000",
            simulation_time.jd_tdb,
            simulation_time.days_since_j2000(),
            simulation_time.time_scale,
            direction,
            paused,
        );
    }
}
