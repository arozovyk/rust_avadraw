#[derive(Debug)]
pub enum Command {
    Buy { from: String, price: u32 },
    Draw { x: u32, y: u32 },
}


