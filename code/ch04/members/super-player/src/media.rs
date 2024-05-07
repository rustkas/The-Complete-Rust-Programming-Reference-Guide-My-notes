pub trait Playable {
    fn play(&self);
    fn pause() {
        println!("Paused");
    }
}

pub struct Audio(pub String);
pub struct Video(pub String);

impl Playable for Audio {
    fn play(&self) {
        println!("🎵 Now playing: {}", self.0);
    }
}

impl Playable for Video {
    fn play(&self) {
        println!("🎵 Now playing: {}", self.0);
    }
}
