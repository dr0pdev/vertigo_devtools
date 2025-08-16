use bevy::prelude::*;
use bevy_flycam::prelude::*;
use bevy_voxel_world::prelude::*;
use std::sync::Arc;

#[derive(Resource, Clone, Default)]
struct MainWorld;

const BASE: u8 = 0;

impl VoxelWorldConfig for MainWorld {
    type MaterialIndex = u8;
    type ChunkUserBundle = ();

    fn texture_index_mapper(&self) -> Arc<dyn Fn(Self::MaterialIndex) -> [u32; 3] + Send + Sync> {
        Arc::new(|mat| match mat {
            BASE => [34, 18, 17], // [top ,sides, bottom]
            _ => [0, 0, 0],
        })
    }

    fn voxel_texture(&self) -> Option<(String, u32)> {
        Some(("spritesheet_tiles.png".into(), 99))
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                position: WindowPosition::Centered(MonitorSelection::Primary),
                resolution: Vec2::new(800.0, 800.0).into(),
                title: String::from("Vertigo Dev Build"),
                ..Default::default()
            }),
            exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
            close_when_requested: true,
        }))
        .add_plugins(NoCameraPlayerPlugin)
        .add_plugins(VoxelWorldPlugin::with_config(MainWorld))
        .add_systems(Startup, (setup, create_voxel_scene).chain())
        .run();
}

fn setup(mut commands: Commands) {
    // Camera with VoxelWorldCamera component
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        VoxelWorldCamera::<MainWorld>::default(),
        FlyCam,
    ));

    // Ambient light, same color as sun
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.98, 0.95, 0.82),
        brightness: 1000.0,
        affects_lightmapped_meshes: true,
    });
}

fn create_voxel_scene(mut voxel_world: VoxelWorld<MainWorld>) {
    // 20 by 20 floor
    for x in -10..10 {
        for z in -10..10 {
            voxel_world.set_voxel(IVec3::new(x, 0, z), WorldVoxel::Solid(BASE));
            // Grassy floor
        }
    }
}
