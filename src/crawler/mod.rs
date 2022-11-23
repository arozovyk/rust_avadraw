use crate::comms::Command;
mod types;
use self::types::Event;
use ethereum_abi::Abi;
use futures_util::StreamExt;
use primitive_types::H256;
use serde::Serialize;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use tokio::sync::mpsc::Sender;
use web3::contract::{Contract, Error, Options};
use web3::transports::WebSocket;
use web3::types::{Address, BlockNumber, FilterBuilder, H160, U256, U64};

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

async fn get_board_contract() -> Contract<WebSocket> {
    let web3s = get_web3_instance().await;
    let board_addr = Address::from_str(&env::var("BOARD_ADDRESS").unwrap()).unwrap();
    Contract::from_json(web3s.eth(), board_addr, include_bytes!("Board.json")).unwrap()
}

#[allow(dead_code)]
pub async fn call_board() -> Result<(), Error> {
    let board_contract = get_board_contract().await;
    let tile = get_tile(
        board_contract,
        U256::from_str("0").unwrap(),
        U256::from_str("0").unwrap(),
    )
    .await;

    println!("{:?}", tile);
    Ok(())
}
fn get_board_filter() -> web3::types::Filter {
    FilterBuilder::default()
        .from_block(BlockNumber::Number(U64::from(0)))
        .to_block(BlockNumber::Latest)
        .address(vec![
            Address::from_str(&env::var("BOARD_ADDRESS").unwrap()).unwrap()
        ])
        .build()
}
fn decode_log(log: &web3::types::Log) -> Event {
    let tx_hash = log.transaction_hash.unwrap();
    let value = tx_hash
        .serialize(serde_json::value::Serializer)
        .ok()
        .unwrap();
    if let serde_json::Value::String(tx_hash_string) = value {
        let mut file = File::create("foo.txt").unwrap();
        file.write_all(tx_hash.as_bytes()).unwrap();
        let ll = log.data.serialize(serde_json::value::Serializer).unwrap();
        let s = ll.as_str().unwrap();
        let data = hex::decode(&s[2..]).unwrap();
        let abi: Abi = {
            let file = File::open("src/crawler/Board.json").expect("failed to open ABI file");
            serde_json::from_reader(file).expect("failed to parse ABI")
        };
        let topic = log.topics[0];

        let decode_log = |log| {
            let (_, decoded_data) = abi
                .decode_log_from_slice(&[H256::from_str(log).unwrap()], &data)
                .expect("failed decoding log");
            decoded_data
        };

        match topic.to_string().as_str() {
            "0x726d…4890" => Event::BuyEvent(
                types::BuyEvent::from(decode_log(
                    &"0x726d161b78cf6b8052b856c14d2c21d3cfd1371760b4fa1472e9bc61be434890",
                )),
                tx_hash_string,
            ),
            "0x8f6e…b058" => Event::DrawImage(
                types::DrawImage::from(decode_log(
                    &"0x8f6e6256d8b6d91161e73f93b4a67134ea0b96d70a3c8c6d770db7e8d4d1b058",
                )),
                tx_hash_string,
            ),
            _ => panic!("Topic is not supported"),
        }
    } else {
        panic!("Failed to decode log");
    }
}

async fn get_sub_stream() -> web3::api::SubscriptionStream<WebSocket, web3::types::Log> {
    let web3 = get_web3_instance().await;
    let filter = get_board_filter();
    web3.eth_subscribe()
        .subscribe_logs(filter.clone())
        .await
        .unwrap()
}
async fn get_events() -> Vec<Event> {
    let web3 = get_web3_instance().await;
    let filter = get_board_filter();
    let base_filter = web3
        .eth_filter()
        .create_logs_filter(filter.clone())
        .await
        .unwrap();
    let logs = base_filter.logs().await.unwrap();
    logs.iter().map(|log| decode_log(log)).collect()
}

// monitors the contract for changes
// puts draw events into the db
pub async fn run(_tx: &Sender<Command>) {
    let events = get_events().await;
    events
        .iter()
        .enumerate()
        .for_each(|(i, event)| println!("Event {} {:?} ", i, event));
    let mut subscription_stream = get_sub_stream().await;
    loop {
        let new_log = subscription_stream.next().await.unwrap().ok().unwrap();
        println!("Got new log {:?} ", decode_log(&new_log.clone()));
    }
}
