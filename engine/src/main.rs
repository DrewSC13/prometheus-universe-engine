use bevy::prelude::*;
use prometheus_engine::time::SimulationTime;

#[derive(Resource, Debug)]
struct EngineSimulationTime(SimulationTime);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(EngineSimulationTime(SimulationTime::j2000()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    info!("Prometheus Universe Engine iniciado.");
    info!("Fase 0: escena Bevy base, tiempo, coordenadas y Floating Origin.");
}
