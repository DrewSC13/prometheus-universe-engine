use bevy::prelude::*;
use prometheus_engine::camera::{FreeCamera, FreeCameraPlugin};
use prometheus_engine::coordinates::GlobalPositionComponent;
use prometheus_engine::floating_origin::FloatingOriginRuntimePlugin;
use prometheus_engine::render::solar_system::SolarSystemRenderPlugin;
use prometheus_engine::time::{SimulationClock, SimulationTime};

fn main() {
    let mut simulation_time = SimulationTime::j2000();
    simulation_time.set_time_scale(50_000.0);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            FreeCameraPlugin,
            FloatingOriginRuntimePlugin,
            SolarSystemRenderPlugin,
        ))
        .insert_resource(SimulationClock(simulation_time))
        .add_systems(Startup, setup)
        .add_systems(Update, advance_simulation_time)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        FreeCamera,
        GlobalPositionComponent::default(),
        Transform::from_xyz(0.0, 12.0, 42.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 5_000.0,
            ..default()
        },
        Transform::from_xyz(5.0, 8.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    info!("Prometheus Universe Engine iniciado.");
    info!("Fase 0: escala educativa Sol-Tierra-Luna con cámara libre y Floating Origin.");
}

fn advance_simulation_time(time: Res<Time>, mut simulation_clock: ResMut<SimulationClock>) {
    simulation_clock
        .0
        .tick_seconds(f64::from(time.delta_secs()));
}
