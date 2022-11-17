//! <https://pngquant.org/lib/>

mod common;
mod crawler;
mod image_renderer;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let crawler_handle = tokio::spawn(async {
        crawler::run().await;
    });
    let ir_handler = tokio::spawn(async {
        image_renderer::run().await;
    });
    crawler_handle.await.unwrap();
    ir_handler.await.unwrap();

    // TODO launch an api server here.
}
