use bevy::prelude::*;

use assets::{GameAssetsPlugin, ImageAssets};
use camera::spawn_camera;

mod assets;
mod camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameAssetsPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_ball)
        .run();
}

fn spawn_ball(mut cmd: Commands, assets: Res<ImageAssets>) {
    cmd.spawn((
        SpriteBundle {
            texture: assets.ball.clone(),
            ..default()
        },
        Name::new("Player"),
    ));
}
