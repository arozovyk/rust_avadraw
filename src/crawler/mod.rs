use std::env;
use std::str::FromStr;
use tokio::sync::mpsc::Sender;
use tokio::time::*;

use web3::contract::{Contract, Error, Options};
use web3::transports::WebSocket;
use web3::types::{Address, H160, U256};

use crate::comms::Command;

fn wei_to_eth(wei_val: U256) -> f64 {
    let res = wei_val.as_u128() as f64;
    res / 1_000_000_000_000_000_000.0
}

async fn get_web3_instance() -> web3::Web3<web3::transports::WebSocket> {
    let websocket = web3::transports::WebSocket::new(&env::var("WS_ENDPOINT").unwrap())
        .await
        .unwrap();
    web3::Web3::new(websocket)
}
#[allow(dead_code)]
async fn get_account_balance() -> Result<(), Error> {
    let web3s = get_web3_instance().await;

    let mut accounts = web3s.eth().accounts().await?;

    accounts.push(H160::from_str("0x8db97C7cEcE249c2b98bDC0226Cc4C2A57BF52FC").unwrap());
    println!("Accounts: {:?}", accounts);

    for account in accounts {
        let balance = web3s.eth().balance(account, None).await?;
        println!("Eth balance of {:?}: {}", account, wei_to_eth(balance));
    }
    Ok(())
}

// return a tuple (price, owner_addr) given the coordinates
async fn get_tile(board_contract: Contract<WebSocket>, x: U256, y: U256) -> (U256, H160) {
    board_contract
        .query("tiles", (x, y), None, Options::default(), None)
        .await
        .unwrap()
}

#[allow(dead_code)]
pub async fn call_board() -> Result<(), Error> {
    let web3s = get_web3_instance().await;

    let board_addr = Address::from_str(&env::var("BOARD_ADDRESS").unwrap()).unwrap();
    let board_contract =
        Contract::from_json(web3s.eth(), board_addr, include_bytes!("Board.json")).unwrap();

    let tile = get_tile(
        board_contract,
        U256::from_str("0").unwrap(),
        U256::from_str("0").unwrap(),
    )
    .await;

    println!("{:?}", tile);
    Ok(())
}

// monitors the contract for changes
// puts draw events into the db

pub async fn run(tx: &Sender<Command>) {
    let mut i = 0;

    loop {
        tokio::time::sleep(Duration::from_secs(2)).await;
        println!("Crawler step {} ", i);
        if i % 5 == 0 {
            println!("Crawler sends buy command ");
            let cmd = Command::Buy {
                from: "Dog".into(),
                price: 420,
            };
            tx.send(cmd).await.unwrap();
        }
        call_board().await.unwrap();
        i += 1;
    }
}
