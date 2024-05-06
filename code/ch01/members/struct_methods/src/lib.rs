pub struct Player {
    pub name: String,
    iq: u8,
    friends: u8,
}

impl Player {
    pub fn with_name(name: &str) -> Player {
        Player {
            name: name.to_string(),
            iq: 100,
            friends: 100,
        }
    }

    pub fn get_friends(&self) -> u8 {
        self.friends
    }

    pub fn set_friends(&mut self, count: u8) {
        self.friends = count;
    }

    pub fn get_iq(&self) -> u8 {
        self.iq
    }
}

#[test]
fn test() {
    let mut player = Player::with_name("Dave");
    let frieds_count = 23;
    player.set_friends(23);
    println!("{}'s friends count: {}", player.name, player.get_friends());
    // another way to call instance methods.
    let current_frieds_count = Player::get_friends(&player);

    assert_eq!(frieds_count, current_frieds_count);
}
