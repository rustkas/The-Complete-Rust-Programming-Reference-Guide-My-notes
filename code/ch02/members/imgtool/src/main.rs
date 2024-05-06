use std::path::Path;
use open::that_in_background;

fn main() {
    let image_path = "members/imgtool/assets/ferris.png";
    let path = Path::new(&image_path);
    let img = image::open(path).unwrap();
    let rotated = img.rotate90();
    rotated.save(path).unwrap();

    println!("Image rotated and saved successfully!");

    let join_handle = that_in_background(path); 
    if let Err(err) = join_handle.join() {
        eprintln!("Failed to open image: {:?}", err);
    }

}
