use bevy::prelude::*;

pub fn spawn_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}
