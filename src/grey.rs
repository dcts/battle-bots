use battle_bots_engine::*;


/**
 * This is a secret message
 * did you already write a note for PJ?
 */
/**
 * The grey bot is broken! It's using all the functions below, but they seem not to be implemented correctly
 *
 * Please help us fix the bot!
 *
 * Instructions
 * =============
 *
 * Implement all the functions below
 * Run a battle (`cargo run`) after they have been implemented to test that the grey bot works again
 */

/*
// GRID POSITION (x,y) DERIVED FROM
// https://github.com/holochain-immersive/battle-bots-engine/blob/1cf09bf8fd34958e31a084d08e902b7ed9665684/src/direction.rs#L20
// Direction::Up if y + 1 < MAP_HEIGHT => (x, y + 1),
// Direction::Down if (y as isize) - 1 >= 0 => (x, y - 1),
// Direction::Right if x + 1 < MAP_WIDTH => (x + 1, y),
// Direction::Left if (x as isize) - 1 >= 0 => (x - 1, y),

// CONCLUSION
// UP    => y + 1
// DOWN  => y - 1
// RIGHT => x + 1
// LEFT  => x - 1
*/

// Returns the position that's adjacent to the given one in the given direction, in the form (x, y)
// eg. adjacent_position_in_direction(4, 5, Direction::Down) == (4, 6)
pub fn adjacent_position_in_direction(x: usize, y: usize, direction: Direction) -> (usize, usize) {
    match direction {
        Direction::Up => (x, y + 1),
        Direction::Down => (x, y - 1),
        Direction::Right => (x + 1, y),
        Direction::Left => (x - 1, y),
    }
}

// Returns whether there is a bot in the given position
pub fn is_bot(game_state: &GameState, position: &Position) -> bool {
    game_state
        .bots
        .iter()
        .any(|(pos, _)| pos.x == position.x && pos.y == position.y )
}

// Returns the shortest way to rotate the "from" direction to get the "to" direction
// Assumes that from and to are not equal
// eg. shortest_rotation(Direction::Up, Direction::Right) == Rotation::Clockwise
pub fn shortest_rotation(from: &Direction, to: &Direction) -> Rotation {
    match (from, to) {
        (Direction::Up, Direction::Right) => Rotation::Clockwise,
        (Direction::Right, Direction::Down) => Rotation::Clockwise,
        (Direction::Down, Direction::Left) => Rotation::Clockwise,
        (Direction::Left, Direction::Up) => Rotation::Clockwise,
        _ => Rotation::Counterclockwise
    }
}

// Rotate the given direction with the given rotation
// eg. rotate_direction(Direction::Up, Rotation::Clockwise) == Direction::Right
pub fn rotate_direction(direction: &Direction, rotation: &Rotation) -> Direction {
    match (direction, rotation) {
        (Direction::Up, Rotation::Clockwise) => Direction::Right,
        (Direction::Up, Rotation::Counterclockwise) => Direction::Left,
        (Direction::Down, Rotation::Clockwise) => Direction::Left,
        (Direction::Down, Rotation::Counterclockwise) => Direction::Right,
        (Direction::Left, Rotation::Clockwise) => Direction::Up,
        (Direction::Left, Rotation::Counterclockwise) => Direction::Down,
        (Direction::Right, Rotation::Clockwise) => Direction::Down,
        (Direction::Right, Rotation::Counterclockwise) => Direction::Up,
    }
}

// ADD UNIT TESTS
#[cfg(test)]
mod tests {
    use crate::grey::*;

    #[test]
    fn adjacent_position_in_direction_test() {
        // UP    => y + 1
        assert_eq!(adjacent_position_in_direction(4, 5, Direction::Up), (4, 6));
        // DOWN  => y - 1
        assert_eq!(
            adjacent_position_in_direction(4, 5, Direction::Down),
            (4, 4)
        );
        // RIGHT => x + 1
        assert_eq!(
            adjacent_position_in_direction(4, 5, Direction::Right),
            (5, 5)
        );
        // LEFT  => x - 1
        assert_eq!(
            adjacent_position_in_direction(4, 5, Direction::Left),
            (3, 5)
        );
    }

    // #[test]
    // fn is_bot_test() {
    //     // TODO
    // }


    #[test]
    fn shortest_rotation_test() {
        fn test(from: &Direction, to: &Direction, rotation: Rotation) {
            let actual: Rotation = shortest_rotation(&from, &to);
            let success: bool = match (actual, rotation) {
                (Rotation::Clockwise, Rotation::Clockwise) => true,
                (Rotation::Counterclockwise, Rotation::Counterclockwise) => true,
                _ => false,
            };
            assert_eq!(success, true);
        }
        // clockwise cases
        test(&Direction::Right, &Direction::Up, Rotation::Counterclockwise);
        test(&Direction::Up, &Direction::Left, Rotation::Counterclockwise);
        test(&Direction::Left, &Direction::Down, Rotation::Counterclockwise);
        test(&Direction::Down, &Direction::Right, Rotation::Counterclockwise);
        // counter clockwise cases
        test(&Direction::Up, &Direction::Right, Rotation::Clockwise);
        test(&Direction::Right, &Direction::Down, Rotation::Clockwise);
        test(&Direction::Down, &Direction::Left, Rotation::Clockwise);
        test(&Direction::Left, &Direction::Up, Rotation::Clockwise);
    }

    #[test]
    fn rotate_direction_test() {
        // TODO => make this terrible code DRY!
        let actual: Direction = rotate_direction(&Direction::Up, &Rotation::Clockwise);
        let mut success: bool = false;
        println!("success = {}", success); // remove to see warning. Keep to avoid warning...
        match actual {
            Direction::Right => {
                success = true;
            }
            _ => {
                success = false;
            }
        }
        assert_eq!(success, true);

        let actual: Direction = rotate_direction(&Direction::Up, &Rotation::Counterclockwise);
        match actual {
            Direction::Left => {
                success = true;
            }
            _ => {
                success = false;
            }
        }
        assert_eq!(success, true);
    }
}
