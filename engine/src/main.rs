use bevy::prelude::*;
use prometheus_engine::camera::{FreeCamera, FreeCameraPlugin};
use prometheus_engine::coordinates::GlobalPositionComponent;
use prometheus_engine::floating_origin::FloatingOriginRuntimePlugin;
use prometheus_engine::time::SimulationTime;

#[derive(Resource, Debug)]
struct EngineSimulationTime(SimulationTime);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((FreeCameraPlugin, FloatingOriginRuntimePlugin))
        .insert_resource(EngineSimulationTime(SimulationTime::j2000()))
        .add_systems(Startup, setup)
        .add_systems(Update, advance_simulation_time)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        FreeCamera,
        GlobalPositionComponent::default(),
        Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            ..default()
        },
        Transform::from_xyz(5.0, 8.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    info!("Prometheus Universe Engine iniciado.");
    info!("Fase 0: cámara libre, tiempo, coordenadas y Floating Origin runtime.");
}

fn advance_simulation_time(time: Res<Time>, mut simulation_time: ResMut<EngineSimulationTime>) {
    simulation_time.0.tick_seconds(f64::from(time.delta_secs()));
}
