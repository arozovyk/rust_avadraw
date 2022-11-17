use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tile {
    pub tile_x: i32,
    pub tile_y: i32,
    pub price: String,
    pub owner: String,
    pub url: String,
}

impl From<&Row> for Tile {
    fn from(row: &Row) -> Self {
        Self {
            tile_x: row.get(0),
            tile_y: row.get(1),
            price: row.get(2),
            owner: row.get(3),
            url: row.get(4),
        }
    }
}


/* impl Tile {
    pub fn new(tile_x: i32, tile_y: i32, price: String, owner: String, url: String) -> Tile {
        Tile {
            tile_x,
            tile_y,
            price,
            owner,
            url,
        }
    }
}
 */
