use crate::prelude::*;

pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_floor);
    }
}

#[derive(Component, Debug)]
pub struct Floor;

fn spawn_floor(mut cmd: Commands, assets: Res<ImageAssets>) {
    cmd.spawn((
        SpriteBundle {
            texture: assets.placeholder.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, -300.0, 20.0),
                scale: Vec3::new(40.0, 2.0, 1.0),
                ..default()
            },
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(8.0, 8.0),
        Floor,
    ));
}
