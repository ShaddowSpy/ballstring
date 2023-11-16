pub mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_rapier2d::prelude::{
        Collider, NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin, RigidBody,
    };

    pub use crate::assets::ImageAssets;
    pub use crate::input::PlayerActions;
}
use crate::prelude::*;

use camera::spawn_camera;

use assets::GameAssetsPlugin;
use player::PlayerPlugin;
use terrain::TerrainPlugin;

mod assets;
mod camera;
mod input;
mod player;
mod terrain;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(GameAssetsPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(TerrainPlugin)
        .add_systems(Startup, spawn_camera)
        .run();
}
