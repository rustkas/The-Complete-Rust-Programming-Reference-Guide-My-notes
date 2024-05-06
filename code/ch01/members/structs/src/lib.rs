pub struct Player {
  pub name: String,
  pub iq: u8,
  pub friends: u8,
  pub score: u16
}

pub fn bump_player_score(mut player: Player, score: u16) {
  player.score += score;
  println!("Updated player stats:");
  println!("Name: {}", player.name);
  println!("IQ: {}", player.iq);
  println!("Friends: {}", player.friends);
  println!("Score: {}", player.score);
}

