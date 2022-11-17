//! <https://pngquant.org/lib/>

mod common;
mod crawler;
mod image_renderer;
use std::thread;
#[tokio::main]

async fn main() {
    let ir_handle = thread::spawn(move || image_renderer::run() );
    let crawler_handle = thread::spawn(move || crawler::run());
    let crawler_handle =  crawler_handle.join().unwrap() ;
    let ir_handle = ir_handle.join().unwrap() ;
    
}
