use bevy::math::primitives::Sphere;
use bevy::prelude::*;

use crate::coordinates::{GlobalPosition, GlobalPositionComponent};
use crate::floating_origin::FloatingOriginRuntime;
use crate::simulation::solar_system::{solar_earth_moon_state, CelestialBodyKind};
use crate::time::SimulationClock;

const DISTANCE_VISUAL_SCALE: f32 = 1.0 / 2_000_000_000.0;
const SUN_VISUAL_RADIUS: f32 = 3.0;
const EARTH_VISUAL_RADIUS: f32 = 0.45;
const MOON_VISUAL_RADIUS: f32 = 0.16;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolarBodyVisual {
    pub kind: CelestialBodyKind,
}

pub struct SolarSystemRenderPlugin;

impl Plugin for SolarSystemRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_solar_earth_moon_visuals)
            .add_systems(Update, update_solar_earth_moon_visuals);
    }
}

fn spawn_solar_earth_moon_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.08,
        ..default()
    });

    let sphere = meshes.add(Sphere::new(1.0).mesh().uv(32, 18));

    let sun_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.85, 0.25),
        ..default()
    });

    let earth_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.35, 1.0),
        ..default()
    });

    let moon_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.65, 0.65, 0.65),
        ..default()
    });

    commands.spawn((
        Mesh3d(sphere.clone()),
        MeshMaterial3d(sun_material),
        Transform::from_scale(Vec3::splat(SUN_VISUAL_RADIUS)),
        SolarBodyVisual {
            kind: CelestialBodyKind::Sun,
        },
        GlobalPositionComponent {
            position: GlobalPosition::ZERO,
        },
    ));

    commands.spawn((
        Mesh3d(sphere.clone()),
        MeshMaterial3d(earth_material),
        Transform::from_scale(Vec3::splat(EARTH_VISUAL_RADIUS)),
        SolarBodyVisual {
            kind: CelestialBodyKind::Earth,
        },
        GlobalPositionComponent {
            position: GlobalPosition::ZERO,
        },
    ));

    commands.spawn((
        Mesh3d(sphere),
        MeshMaterial3d(moon_material),
        Transform::from_scale(Vec3::splat(MOON_VISUAL_RADIUS)),
        SolarBodyVisual {
            kind: CelestialBodyKind::Moon,
        },
        GlobalPositionComponent {
            position: GlobalPosition::ZERO,
        },
    ));

    commands.spawn((
        PointLight {
            intensity: 8_000_000.0,
            range: 500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn update_solar_earth_moon_visuals(
    simulation_clock: Res<SimulationClock>,
    floating_origin: Res<FloatingOriginRuntime>,
    mut query: Query<(
        &SolarBodyVisual,
        &mut GlobalPositionComponent,
        &mut Transform,
    )>,
) {
    let days_since_j2000 = simulation_clock.0.days_since_j2000();
    let state = solar_earth_moon_state(days_since_j2000);

    for (body, mut global_position, mut transform) in query.iter_mut() {
        let physical_position = match body.kind {
            CelestialBodyKind::Sun => state.sun_position_meters,
            CelestialBodyKind::Earth => state.earth_position_meters,
            CelestialBodyKind::Moon => state.moon_position_meters,
        };

        global_position.position = GlobalPosition {
            meters_from_origin: physical_position,
        };

        let local_meters =
            physical_position - floating_origin.origin.origin_global.meters_from_origin;

        transform.translation = (local_meters * f64::from(DISTANCE_VISUAL_SCALE)).as_vec3();
    }
}
