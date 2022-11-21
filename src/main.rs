//! <https://pngquant.org/lib/>

mod common;
mod comms;
mod crawler;
mod image_renderer;
mod server;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (tx, mut rx) = mpsc::channel::<comms::Command>(32);
    dotenv::dotenv().ok();

    tokio::spawn(async move {
        crawler::run(&tx).await;
    });

    tokio::spawn(async move {
        image_renderer::run(&mut rx).await;
    });

    server::run().await
}
