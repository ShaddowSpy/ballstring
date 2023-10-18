pub mod prelude {
    pub use crate::assets::ImageAssets;
    pub use bevy::prelude::*;
}
use crate::prelude::*;

use assets::GameAssetsPlugin;
use camera::spawn_camera;

mod assets;
mod camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(GameAssetsPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_ball)
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
    ));
}
