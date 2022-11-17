use crate::types::Tile;

mod select;
#[allow(dead_code)]

pub async fn board() -> Vec<Tile> {
    select::select_board().await
}#[allow(dead_code)]

pub async fn update_board() {
    select::update_board().await
}
