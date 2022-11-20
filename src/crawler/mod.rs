use ethereum_abi::Abi;
use primitive_types::H256;

use num_bigint::BigUint;
use serde::Serialize;
use std::env;
use std::fs::File;
use std::ops::Add;
use std::os::macos::raw;
use std::str::FromStr;
use tokio::sync::mpsc::Sender;
use tokio::time::*;
use web3::contract::tokens::Tokenize;
use web3::contract::{Contract, Error, Options};
use web3::ethabi::{decode, ParamType, RawLog, Token};
use web3::futures::future::ok;
use web3::transports::WebSocket;
use web3::types::{Address, BlockNumber, FilterBuilder, H160, U256, U64};
use ParamType::*;

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
struct BuyEvent {}
async fn get_events() -> web3::contract::Result<()> {
    let web3 = get_web3_instance().await;

    let filter = FilterBuilder::default()
        .from_block(BlockNumber::Number(U64::from(0)))
        .to_block(BlockNumber::Latest)
        .address(vec![
            Address::from_str(&env::var("BOARD_ADDRESS").unwrap()).unwrap()
        ])
        .build();
    let t = web3.eth_filter().create_logs_filter(filter).await?;
    let logs = t.logs().await.unwrap();
    logs.iter().for_each(|log| {
        let ll = log.data.serialize(serde_json::value::Serializer).unwrap();
        let s = ll.as_str().unwrap();
        let data = hex::decode(&s[2..]).unwrap();
        let abi: Abi = {
            let file = File::open("src/crawler/Board.json").expect("failed to open ABI file");
            serde_json::from_reader(file).expect("failed to parse ABI")
        };

        let (_, decoded_data) = abi
            .decode_log_from_slice(
                &[H256::from_str(
                    "0x726d161b78cf6b8052b856c14d2c21d3cfd1371760b4fa1472e9bc61be434890",
                )
                .unwrap()],
                &data,
            )
            .expect("failed decoding log");
        let value1 = &decoded_data.get(1).unwrap().value;
        let param1 = &decoded_data.get(1).unwrap().param;
    });

    // FIXME

    /*         let data = (&data.as_str().unwrap()).as_bytes();
     */
    /*         let data: String = decode(data).unwrap();
     */

    // TODO also use the subscription for new events? image renderer could use it
    /*  let sub = web3.eth_subscribe().subscribe_logs(filter).await?;
    sub.for_each(|log| {
        println!("{:?}", log);
        future::ready(())
    })
    .await; */
    ok(()).await
}
// monitors the contract for changes
// puts draw events into the db

pub async fn run(tx: &Sender<Command>) {
    let mut i = 1;

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("Crawler step {} ", i);
        if i % 5 == 0 {
            println!("Crawler sends buy command ");
            let cmd = Command::Buy {
                from: "Dog".into(),
                price: 420,
            };
            tx.send(cmd).await.unwrap();
        }
        if i % 2 == 0 {
            get_events().await.unwrap();
        }

        call_board().await.unwrap();
        i += 1;
    }
}
