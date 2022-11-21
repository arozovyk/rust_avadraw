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

    let _crawler_handle = tokio::spawn(async move {
        crawler::run(&tx).await;
    });

    let _ir_handler = tokio::spawn(async move {
        image_renderer::run(&mut rx).await;
    });

    server::run().await
}
