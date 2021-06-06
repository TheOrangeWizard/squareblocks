use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    pbr::AmbientLight,
};
use bevy_flycam::{PlayerPlugin, MovementSettings, FlyCam};

use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use std::any::Any;

use rand::{rngs::StdRng, Rng, SeedableRng};
use noise::{Fbm};

mod chunk;

const CHUNKSIZE: u8 = 16;

struct WorldMeta {
    chunks: Vec<[i16; 3]>,

    //loaded_chunks: Vec<[i16; 3]>,
    //rendered_chunks: Vec<[i16; 3]>
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "squareblocks".to_string(),
            vsync: false,
            ..Default::default()
        })
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 25.0, // default: 12.0
        })
        .insert_resource(WorldMeta {chunks: vec![]})
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup.system())
        .add_system(load_chunks.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    println!("{}", f32::MAX);
    // light
    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 0.4;
    // camera
    // commands.spawn_bundle(PerspectiveCameraBundle {
    //     transform: Transform::from_xyz(0.0, 50.0, 150.0)
    //         .looking_at(Vec3::ZERO, Vec3::Y),
    //     ..Default::default()
    // });
}

fn load_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    /*
    mut q: QuerySet<(
        Query<&mut Transform, With<&'static FlyCam>>,
        Query<&mut Transform, With<&'static PbrBundle>>
    )>,
    */
    mut cam: Query<(&FlyCam, &mut Transform)>,
    mut world_meta: ResMut<WorldMeta>

) {
    let mut view_range = 4;
    let mut rng = StdRng::from_entropy();

    /*
    for transform in q.q1_mut().iter_mut() {
        loaded_chunks.push([
            transform.translation.x,
            transform.translation.y,
            transform.translation.z
        ])
    } */

    for (camera, mut transform) in cam.iter_mut() { //q.q0_mut().iter_mut() {
        let (x, y, z) = (chunk::chunk_coords(transform.translation.x as i16,
                                             transform.translation.y as i16,
                                             transform.translation.z as i16));
        //println!("CAMERA {}, {}, {}", x, y, z);
        //println!("{}", world_meta.chunks.len());
        for cy in 0..16 {
            for cx in (&x- &view_range)..(&x + &view_range + 1) {
                for cz in (&z - &view_range)..(&z + &view_range + 1) {
                    let (wx, wy, wz) = ((cx * CHUNKSIZE as i16) as f32,
                                        (cy * CHUNKSIZE as i16) as f32,
                                        (cz * CHUNKSIZE as i16) as f32);
                    let mut chunk_transform = Transform::from_xyz(wx, wy, wz);
                    if !world_meta.chunks.contains(&[
                        cx,
                        cy,
                        cz
                    ]) {
                        let chunk_data = chunk::Chunk::generate(
                            Fbm::new(), &50.0, cx as f64, cy as f64, cz as f64);
                        if !chunk_data.is_empty {
                            let chunk_mesh = meshes.add(Mesh::from(chunk_data.make_mesh()));

                            commands.spawn_bundle(PbrBundle {
                                mesh: chunk_mesh,
                                material: materials.add(StandardMaterial {
                                    base_color: Color::rgb(
                                        0.10, //rng.gen_range(0.0..1.0),
                                        0.50, //rng.gen_range(0.0..1.0),
                                        0.10, //rng.gen_range(0.0..1.0),
                                    ),
                                    roughness: 0.5,
                                    ..Default::default()
                                }),
                                transform: chunk_transform,
                                ..Default::default()
                            });
                        }

                        world_meta.chunks.push([
                            cx,
                            cy,
                            cz
                        ]);

                        return
                    }
                }
            }
        }
    }
}