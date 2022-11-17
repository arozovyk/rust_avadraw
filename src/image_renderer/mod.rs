//! <https://pngquant.org/lib/>
use std::thread::sleep;
use std::time::Duration;
use std::{fs::File, path::Path};

extern crate oxipng;
// Monitors the database for changes
// Update image on image event,
// Update board_db on buy event
#[allow(dead_code)]
pub fn run() {
    let mut i = 0;
    loop {
        sleep(Duration::from_secs(2));
        println!("Image renderer step {} ", i);
        i += 1;
    }
}
#[allow(dead_code)]

fn main() {
    let mut img = image::open(&Path::new(&"images/img1.png".to_string()))
        .ok()
        .expect("Opening image failed");
    let mut file = File::create("res.png").unwrap();

    let img2 = image::open(&Path::new(&"images/img2.png".to_string()))
        .ok()
        .expect("Opening image failed");
    image::imageops::overlay(&mut img, &img2, 100, 100);
    img.write_to(&mut file, image::ImageOutputFormat::Png)
        .unwrap();

    oxipng::optimize(
        &oxipng::InFile::Path(Path::new(&"res.png".to_string()).to_path_buf()),
        &oxipng::OutFile::Path(Some(Path::new(&"compressed.png".to_string()).to_path_buf())),
        &oxipng::Options::max_compression(),
    )
    .unwrap();
}
