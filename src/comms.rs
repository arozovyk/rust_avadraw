#[derive(Debug)]
pub enum Command {
    #[allow(dead_code)]
    Buy {
        from: String,
        price: u32,
    },
    #[allow(dead_code)]
    Draw {
        x: u32,
        y: u32,
    },
}
