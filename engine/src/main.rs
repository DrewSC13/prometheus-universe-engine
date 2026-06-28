use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use prometheus_engine::camera::presets::CameraViewPresetsPlugin;
use prometheus_engine::camera::{FreeCamera, FreeCameraPlugin};
use prometheus_engine::coordinates::GlobalPositionComponent;
use prometheus_engine::floating_origin::FloatingOriginRuntimePlugin;
use prometheus_engine::interaction::focus::BodyFocusPlugin;
use prometheus_engine::interaction::picking::BodyPickingPlugin;
use prometheus_engine::interaction::selection::BodySelectionPlugin;
use prometheus_engine::render::solar_system::SolarSystemRenderPlugin;
use prometheus_engine::time::{
    SimulationClock, SimulationTime, SimulationTimeControlsPlugin, DEFAULT_TIME_SCALE,
};
use prometheus_engine::ui::hud::SimulationHudPlugin;

fn main() {
    let mut simulation_time = SimulationTime::j2000();
    simulation_time.set_time_scale(DEFAULT_TIME_SCALE);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            FreeCameraPlugin,
            CameraViewPresetsPlugin,
            FloatingOriginRuntimePlugin,
            SimulationTimeControlsPlugin,
            BodySelectionPlugin,
            BodyPickingPlugin,
            BodyFocusPlugin,
            SolarSystemRenderPlugin,
            SimulationHudPlugin,
        ))
        .insert_resource(SimulationClock(simulation_time))
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup)
        .add_systems(Update, advance_simulation_time)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Tonemapping::TonyMcMapface,
        Bloom::NATURAL,
        FreeCamera,
        GlobalPositionComponent::default(),
        Transform::from_xyz(0.0, 30.0, 115.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 1_200.0,
            ..default()
        },
        Transform::from_xyz(5.0, 8.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    info!("Prometheus Universe Engine iniciado.");
    info!("Fase 1+ Visual Polish: Sistema Solar catalogado procedural.");
    info!("Controles: Space pausa, 1-9/0 velocidad, B invierte tiempo, R reset.");
    info!("Interacción: click izquierdo selecciona, N/P cambia, G enfoca, Escape limpia.");
    info!("Render: fondo espacial negro, starfield procedural y polish visual espacial.");
}

fn advance_simulation_time(time: Res<Time>, mut simulation_clock: ResMut<SimulationClock>) {
    simulation_clock
        .0
        .tick_seconds(f64::from(time.delta_secs()));
}
