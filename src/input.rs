use crate::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct GameInputPlugin;
impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerActions>::default());
    }
}

#[derive(Actionlike, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum PlayerActions {
    Left,
    Right,
}
impl PlayerActions {
    fn input_map() -> InputMap<Self> {
        InputMap::new([(KeyCode::A, Self::Left), (KeyCode::D, Self::Right)])
    }
}
