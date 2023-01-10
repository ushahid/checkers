use bevy::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    for i in -5..5 {
        // box
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box{min_x: i as f32 * 1.2 + 0.2, max_x: i as f32 * 1.2 + 1.0 + 0.2, min_y: -0.2, max_y: 0.2, min_z: -1.0, max_z: 1.0})),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_scale(Vec3::ONE * 1.0),
            ..default()
        });
    }

    // commands.insert_resource(AmbientLight{
    //     color: Color::rgb(1.0, 1.0, 1.0).into(),
    //     brightness: 1.0,
    // });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 20000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}