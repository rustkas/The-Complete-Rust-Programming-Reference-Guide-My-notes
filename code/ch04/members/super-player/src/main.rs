
use super_player::media::{Playable, Audio, Video};

fn main() {
    println!("Super player!");
    let audio = Audio("ambient_music.mp3".to_string());
    let video = Video("big_buck_bunny.mkv".to_string());
    audio.play();
    video.play();
}
