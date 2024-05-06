use structs::{bump_player_score, Player};

fn main() {
    let name = "Alice".to_string();
    let player = Player {
        name,
        iq: 171,
        friends: 134,
        score: 1129,
    };

    bump_player_score(player, 120);
}
