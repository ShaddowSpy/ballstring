use crate::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball);
    }
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
        PlayerActions::input_manager(),
    ));
}
