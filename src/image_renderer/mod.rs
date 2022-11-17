//! <https://pngquant.org/lib/>
use tokio::sync::mpsc::Receiver;

use std::{fs::File, path::Path};

use crate::comms::Command;

extern crate oxipng;
// Monitors the database for changes
// Update image on image event,
// Update board_db on buy event
#[allow(dead_code)]
pub async fn run(rx: &mut Receiver<Command>) {
    // Listen on commands from crawler
    while let Some(cmd) = rx.recv().await {
        use Command::*;
        match cmd {
            Buy { from, price } => {
                println!("Got a buy event from : {}, price :{}", from, price)
            }
            Draw { x, y } => {
                println!("Got a draw event x: {}, y: {} ", x, y)
            }
        }
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
