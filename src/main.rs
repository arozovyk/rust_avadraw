//! <https://pngquant.org/lib/>

mod common;
mod crawler;
mod image_renderer;

use tokio::runtime::Runtime;

fn main() {
    dotenv::dotenv().ok();
    let rt = Runtime::new().unwrap();
    let handle = rt.spawn(async move {
        tokio::spawn(async {
            crawler::run().await;
        });
        // It working only because image renderer loops
        // Ending this task will also drop the crawler task
        tokio::spawn(async {
            image_renderer::run().await;
        })
        .await
        .unwrap();
    });
    println!("Crawler and IR have been launched in a tokio runtime");
    rt.block_on(async move {
        handle.await.unwrap();
    })
    /*  let ir_handle = thread::spawn(move || image_renderer::run());
    ir_handle.join().unwrap(); */
    // TODO launch an api server here.
}
