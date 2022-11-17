//! <https://pngquant.org/lib/>

mod common;
mod crawler;
mod image_renderer;
use tokio::runtime::Runtime;


 fn main() {
    dotenv::dotenv().ok();
    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
        crawler::run().await;
    });

    /*  let ir_handle = thread::spawn(move || image_renderer::run());
    ir_handle.join().unwrap(); */
    // TODO launch an api server here.
}
