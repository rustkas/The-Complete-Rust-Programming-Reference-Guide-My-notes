#[derive(Debug)]
pub enum Direction { 
    N, 
    E, 
    S, 
    W
}

pub enum PlayerAction {
    Move {
        direction: Direction,
        speed: u8
    },
    Wait, 
    Attack(Direction)   
}

