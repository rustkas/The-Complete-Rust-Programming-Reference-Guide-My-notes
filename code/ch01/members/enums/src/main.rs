use enums::{Direction, PlayerAction};

fn main() {
    use PlayerAction::*;

    let simulated_player_action = Move {
        direction: Direction::S,
        speed: 20,
    };
    let array = [Wait, Attack(Direction::E), simulated_player_action];
    for simulated_player_action in array {
        match simulated_player_action {
            Wait => println!("Player wants to wait"),
            Move { direction, speed } => {
                println!("Player wants to move in direction {direction:?} with speed {speed}")
            }
            Attack(direction) => {
                println!("Player wants to attack direction {direction:?}")
            }
        };
    }
}
