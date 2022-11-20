//! <https://pngquant.org/lib/>

mod common;
mod comms;
mod crawler;
mod image_renderer;
use tokio::sync::mpsc;
 

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<comms::Command>(32);
    dotenv::dotenv().ok();
    let crawler_handle = tokio::spawn(async move {
        crawler::run(&tx).await;
    });

    let ir_handler = tokio::spawn(async move {
        image_renderer::run(&mut rx).await;
    });

    crawler_handle.await.unwrap();
    ir_handler.await.unwrap();

    println!("hey")
    // TODO launch an api server here.
}
