use tokio_postgres::{Client, NoTls};

use crate::types::Tile;
#[allow(dead_code)]

async fn get_client() -> Client {
    let (client, connection) =
        tokio_postgres::connect("postgres://artemiyrozovyk@localhost/avadrawit", NoTls)
            .await
            .unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    client
}
#[allow(dead_code)]

pub async fn select_board() -> Vec<Tile> {
    let client = get_client().await;
    let rows = client.query("SELECT * FROM BOARD", &[]).await.unwrap();
    let tiles = rows.iter().map(|row| Tile::from(row));
    tiles.collect()
}
#[allow(dead_code)]

pub async fn update_board() {
    let client = get_client().await;
    client
        .query(
            "UPDATE BOARD SET price=$1 WHERE tile_x=99 and tile_y=99",
            &[&"101"],
        )
        .await
        .unwrap();
}
