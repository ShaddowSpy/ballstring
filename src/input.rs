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
    pub fn input_manager() -> InputManagerBundle<Self> {
        InputManagerBundle::<Self> {
            action_state: ActionState::default(),
            input_map: InputMap::new([(KeyCode::A, Self::Left), (KeyCode::D, Self::Right)]),
        }
    }
}
