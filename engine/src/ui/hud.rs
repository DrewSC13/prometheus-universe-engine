use bevy::prelude::*;

use crate::interaction::selection::{
    selected_body_compact_label, selected_body_hud_summary, SelectedBody,
};
use crate::render::solar_system::{LabelVisibilityMode, OrbitVisibilityMode};
use crate::simulation::bodies::{orbiting_bodies, root_bodies, SOLAR_SYSTEM_BODIES};
use crate::time::{format_time_scale, SimulationClock, TimeDirection};

#[derive(Component, Debug)]
pub struct SimulationHudText;

const HUD_PANEL_ALPHA_FULL: f32 = 0.72;
const HUD_PANEL_ALPHA_COMPACT: f32 = 0.56;
const HUD_PANEL_ALPHA_HIDDEN: f32 = 0.0;

#[derive(Resource, Debug, Clone, Copy)]
pub struct HudVisibility {
    pub visible: bool,
    pub compact: bool,
}

impl Default for HudVisibility {
    fn default() -> Self {
        Self {
            visible: true,
            compact: false,
        }
    }
}

pub struct SimulationHudPlugin;

impl Plugin for SimulationHudPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HudVisibility::default())
            .add_systems(Startup, spawn_simulation_hud)
            .add_systems(Update, (update_simulation_hud, toggle_hud_visibility));
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
        BackgroundColor(hud_panel_background_color(true, false).0),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(12.0),
            top: Val::Px(12.0),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        SimulationHudText,
    ));
}

fn hud_panel_background_color(visible: bool, compact: bool) -> BackgroundColor {
    BackgroundColor(Color::srgba(
        0.0,
        0.0,
        0.0,
        hud_panel_alpha(visible, compact),
    ))
}

fn hud_panel_alpha(visible: bool, compact: bool) -> f32 {
    if !visible {
        HUD_PANEL_ALPHA_HIDDEN
    } else if compact {
        HUD_PANEL_ALPHA_COMPACT
    } else {
        HUD_PANEL_ALPHA_FULL
    }
}

fn toggle_hud_visibility(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut hud_visibility: ResMut<HudVisibility>,
) {
    if keyboard.just_pressed(KeyCode::KeyH) {
        hud_visibility.visible = !hud_visibility.visible;
        info!("HUD visible: {}", hud_visibility.visible);
    }

    if keyboard.just_pressed(KeyCode::KeyM) {
        hud_visibility.compact = !hud_visibility.compact;
        info!("HUD compact mode: {}", hud_visibility.compact);
    }
}

fn update_simulation_hud(
    simulation_clock: Res<SimulationClock>,
    label_visibility_mode: Option<Res<LabelVisibilityMode>>,
    orbit_visibility_mode: Option<Res<OrbitVisibilityMode>>,
    selected_body: Option<Res<SelectedBody>>,
    hud_visibility: Res<HudVisibility>,
    mut query: Query<(&mut Text, &mut BackgroundColor), With<SimulationHudText>>,
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

    let selected_body = selected_body.as_deref().copied().unwrap_or_default();
    let selected_body_label = selected_body_compact_label(selected_body);
    let selected_body_summary = selected_body_hud_summary(selected_body);

    for (mut text, mut background) in query.iter_mut() {
        if !hud_visibility.visible {
            text.0.clear();
            *background = hud_panel_background_color(false, hud_visibility.compact);
            continue;
        }

        if hud_visibility.compact {
            *background = hud_panel_background_color(true, true);
            text.0 = format!(
                "Prometheus | Fase 2 | JD {:.2} | vel {} | {} | pausa: {} | seleccion: {} | click seleccionar | G enfocar | H/M HUD",
                simulation_time.jd_tdb,
                format_time_scale(simulation_time.time_scale),
                direction,
                paused,
                selected_body_label,
            );
            continue;
        }

        *background = hud_panel_background_color(true, false);

        let time_scale_label = format_time_scale(simulation_time.time_scale);

        text.0 = format!(
            "PROMETHEUS UNIVERSE ENGINE\n\
             Fase 2 - Body Inspector\n\
             -----------------------------\n\
             \n\
             [CATALOGO]\n\
             Cuerpos totales: {}\n\
             Cuerpos raiz: {}\n\
             Cuerpos orbitando: {}\n\
             Etiquetas: {}\n\
             Orbitas: {}\n\
             \n\
             [SELECCION]\n\
             {}\n\
             \n\
             [TIEMPO]\n\
             JD TDB: {:.5}\n\
             Dias desde J2000: {:.2}\n\
             Velocidad: {}\n\
             Direccion: {}\n\
             Pausado: {}\n\
             \n\
             [CONTROLES]\n\
             Mouse: click izquierdo selecciona\n\
             Seleccion: N/P cambia | Escape limpia\n\
             Camara: G enfoca | C/V/F vistas\n\
             Tiempo: Space pausa | 1-9/0 velocidad | B invierte | R reset\n\
             Visual: L etiquetas | O orbitas | H HUD | M compacto",
            total_bodies,
            root_body_count,
            orbiting_body_count,
            label_mode,
            orbit_mode,
            selected_body_summary,
            simulation_time.jd_tdb,
            simulation_time.days_since_j2000(),
            time_scale_label,
            direction,
            paused,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hud_panel_alpha_uses_full_opacity_for_expanded_mode() {
        assert_eq!(hud_panel_alpha(true, false), HUD_PANEL_ALPHA_FULL);
    }

    #[test]
    fn hud_panel_alpha_uses_lighter_opacity_for_compact_mode() {
        assert_eq!(hud_panel_alpha(true, true), HUD_PANEL_ALPHA_COMPACT);
    }

    #[test]
    fn hud_panel_alpha_is_transparent_when_hidden() {
        assert_eq!(hud_panel_alpha(false, false), HUD_PANEL_ALPHA_HIDDEN);
    }
}
