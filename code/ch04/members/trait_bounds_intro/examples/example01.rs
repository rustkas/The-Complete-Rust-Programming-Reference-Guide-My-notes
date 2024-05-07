use trait_bounds_intro::{Enemy, Game, Hero};

fn main() {
  let game = Game;
  game.load(Enemy);
  game.load(Hero);

}
