use crate::engine::state::{state_to_matrix, from_matrix, GameState};
use crate::engine::utils::direction::Rotation;

use super::super::state::{Battle, GameCell};

use super::ExecutableAction;

pub struct RotateShield(pub Rotation);

impl ExecutableAction for RotateShield {
    fn execute(&self, bot_pos_x: usize, bot_pos_y: usize, state: GameState) -> GameState {
        let mut map = state_to_matrix(state);

        if let GameCell::Bot(mut bot) = map[bot_pos_x][bot_pos_y] {
            bot.shield_direction = bot.shield_direction.rotate(self.0);

            map[bot_pos_x][bot_pos_y] = GameCell::Bot(bot);
        }
        from_matrix(map)
    }
}
