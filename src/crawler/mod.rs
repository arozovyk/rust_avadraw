use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use web3::contract::{Contract, Error, Options};
use web3::types::{Address, H160, U256};

fn wei_to_eth(wei_val: U256) -> f64 {
    let res = wei_val.as_u128() as f64;
    res / 1_000_000_000_000_000_000.0
}

// monitors the contract for changes
// puts draw events into the db
pub fn run() {
    let mut i = 0;
    loop {
        sleep(Duration::from_secs(2));
        println!("Crawler step {} ", i);
        i += 1;
    }
}
#[allow(dead_code)]
pub async fn call_board() -> Result<(), Error> {
    let websocket = web3::transports::WebSocket::new("ws://127.0.0.1:9650/ext/bc/C/ws")
        .await
        .unwrap();
    let web3s = web3::Web3::new(websocket);
    let mut accounts = web3s.eth().accounts().await?;
    accounts.push(H160::from_str("0x8db97C7cEcE249c2b98bDC0226Cc4C2A57BF52FC").unwrap());
    println!("Accounts: {:?}", accounts);

    for account in accounts {
        let balance = web3s.eth().balance(account, None).await?;
        println!("Eth balance of {:?}: {}", account, wei_to_eth(balance));
    }
    let board_addr = Address::from_str("0x4Ac1d98D9cEF99EC6546dEd4Bd550b0b287aaD6D").unwrap();
    let board_contract =
        Contract::from_json(web3s.eth(), board_addr, include_bytes!("Board.json")).unwrap();

    let tile: (U256, H160) = board_contract
        .query(
            "tiles",
            (U256::from_str("9").unwrap(), U256::from_str("2").unwrap()),
            None,
            Options::default(),
            None,
        )
        .await
        .unwrap();

    println!("{:?}", tile);
    Ok(())
}
