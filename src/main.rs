use bevy::{
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{mesh::PrimitiveTopology, render_resource::WgpuFeatures, settings::WgpuSettings},
};
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .insert_resource(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin::default())
        .add_plugin(WireframePlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_lighting)
        .add_startup_system(setup_plane)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands
    .spawn(Camera3dBundle::default())
    .insert(OrbitCameraBundle::new(
        OrbitCameraController::default(),
        Vec3::new(-10.0, 10.0, 0.0),
        Vec3::new(2.5, 0.0, -2.5),
    ));
}

fn setup_lighting(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(2.5, 10.0, -2.5),
        point_light: PointLight {
            color: Color::rgb(1.0, 0.2, 1.0),
            intensity: 2000.0,
            range: 11.0,
            radius: 10.0,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn setup_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut _wireframe_config: ResMut<WireframeConfig>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let map_side_len = 10.0;
    let min_x = -5.0;
    let tile_side_step = map_side_len / 2.0 / 2.0;

    let mut complete_positions: Vec<[f32; 3]> = vec![];
    let mut complete_uvs: Vec<[f32; 2]> = vec![];

    //  (+,-)      (+.+)   (+,-)      (+.+)
    //    a -------- d       a -------- d
    //    | \        |       |        / |
    //    |  \       |       |       /  |
    //    |   \      |       |      /   |
    //    |    \     |       |     /    |
    //    |     \    |       |    /     |
    //    |      \   |       |   /      |
    //    |       \  |       |  /       |
    //    |        \ |       | /        |
    //    b -------- c       b -------- c
    //  (-.-)      (-.+)   (-.-)      (-.+)
    //
    //  (+,-)      (+.+)   (+,-)      (+.+)
    //    a -------- d       a -------- d
    //    |        / |       | \        |
    //    |       /  |       |  \       |
    //    |      /   |       |   \      |
    //    |     /    |       |    \     |
    //    |    /     |       |     \    |
    //    |   /      |       |      \   |
    //    |  /       |       |       \  |
    //    | /        |       |        \ |
    //    b -------- c       b -------- c
    //  (-.-)      (-.+)   (-.-)      (-.+)

    let a: Vec3 = Vec3::new(tile_side_step, tile_side_step, -tile_side_step);
    let b: Vec3 = Vec3::new(-tile_side_step, 0.0, -tile_side_step);
    let c: Vec3 = Vec3::new(-tile_side_step, 0.0, tile_side_step);
    let d: Vec3 = Vec3::new(tile_side_step, 0.0, tile_side_step);

    let (positions, uvs) = plane_positions_and_uvs(a, b, c, d, map_side_len, min_x);

    complete_positions = [complete_positions, positions].concat();
    complete_uvs = [complete_uvs, uvs].concat();

    let a: Vec3 = Vec3::new(a.x + tile_side_step * 2.0, a.y - tile_side_step, a.z);
    let b: Vec3 = Vec3::new(b.x + tile_side_step * 2.0, b.y + tile_side_step, b.z);
    let c: Vec3 = Vec3::new(c.x + tile_side_step * 2.0, c.y, c.z);
    let d: Vec3 = Vec3::new(d.x + tile_side_step * 2.0, d.y, d.z);

    let (positions, uvs) = plane_positions_and_uvs(b, c, d, a, map_side_len, min_x);

    complete_positions = [complete_positions, positions].concat();
    complete_uvs = [complete_uvs, uvs].concat();

    let a: Vec3 = Vec3::new(a.x, a.y, a.z - tile_side_step * 2.0);
    let b: Vec3 = Vec3::new(b.x, b.y - tile_side_step, b.z - tile_side_step * 2.0);
    let c: Vec3 = Vec3::new(c.x, c.y + tile_side_step, c.z - tile_side_step * 2.0);
    let d: Vec3 = Vec3::new(d.x, d.y, d.z - tile_side_step * 2.0);

    let (positions, uvs) = plane_positions_and_uvs(a, b, c, d, map_side_len, min_x);

    complete_positions = [complete_positions, positions].concat();
    complete_uvs = [complete_uvs, uvs].concat();

    let a: Vec3 = Vec3::new(a.x - tile_side_step * 2.0, a.y, a.z);
    let b: Vec3 = Vec3::new(b.x - tile_side_step * 2.0, b.y, b.z);
    let c: Vec3 = Vec3::new(c.x - tile_side_step * 2.0, c.y - tile_side_step, c.z);
    let d: Vec3 = Vec3::new(d.x - tile_side_step * 2.0, d.y + tile_side_step, d.z);

    let (positions, uvs) = plane_positions_and_uvs(b, c, d, a, map_side_len, min_x);

    complete_positions = [complete_positions, positions].concat();
    complete_uvs = [complete_uvs, uvs].concat();

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, complete_positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, complete_uvs);
    mesh.compute_flat_normals();
    let pbr_bundle = PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        ..Default::default()
    };

    commands.spawn(pbr_bundle).insert(Wireframe);
}

fn plane_positions_and_uvs(
    a: Vec3,
    b: Vec3,
    c: Vec3,
    d: Vec3,
    map_side_len: f32,
    min_x: f32,
) -> (Vec<[f32; 3]>, Vec<[f32; 2]>) {
    let uv_calculate = |c: f32| -> f32 { (c + min_x.abs()) / map_side_len };
    let get_uv = |x: f32, z: f32| -> [f32; 2] { [uv_calculate(z), uv_calculate(x)] };

    let mut positions: Vec<[f32; 3]> = vec![];
    let mut uvs: Vec<[f32; 2]> = vec![];

    //  (+,-)
    //    a
    //    | \
    //    |  \
    //    |   \
    //    |    \
    //    |     \
    //    |      \
    //    |       \
    //    |        \
    //    b -------- c
    //  (-.-)      (-.+)

    positions.push(a.into());
    uvs.push(get_uv(a.x, a.z));

    positions.push(b.into());
    uvs.push(get_uv(b.x, b.z));

    positions.push(c.into());
    uvs.push(get_uv(c.x, c.z));

    //  (+,-)      (+.+)
    //    a -------- d
    //      \        |
    //       \       |
    //        \      |
    //         \     |
    //          \    |
    //           \   |
    //            \  |
    //             \ |
    //               c
    //             (-.+)

    positions.push(c.into());
    uvs.push(get_uv(c.x, c.z));

    positions.push(d.into());
    uvs.push(get_uv(d.x, d.z));

    positions.push(a.into());
    uvs.push(get_uv(a.x, a.z));

    (positions, uvs)
}
