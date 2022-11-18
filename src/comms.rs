#[derive(Debug)]
pub enum Command {
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
