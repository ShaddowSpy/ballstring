pub mod prelude {
    pub use crate::assets::ImageAssets;
    pub use bevy::prelude::*;
}
use crate::prelude::*;

use bevy_rapier2d::prelude::*;

use assets::GameAssetsPlugin;
use camera::spawn_camera;

mod assets;
mod camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(GameAssetsPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_ball)
        .add_systems(Startup, spawn_floor)
        .run();
}

fn spawn_ball(mut cmd: Commands, assets: Res<ImageAssets>) {
    cmd.spawn((
        SpriteBundle {
            texture: assets.ball.clone(),
            transform: Transform::from_scale(Vec3::new(4.0, 4.0, 1.0)),
            ..default()
        },
        Name::new("Player"),
        RigidBody::Dynamic,
        Collider::ball(8.0),
    ));
}

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
    ));
}
