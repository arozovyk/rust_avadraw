use std::str::FromStr;

use ethereum_abi::Value::{Tuple, *};
use ethereum_abi::{ DecodedParams, Value};
#[derive(Debug)]
pub enum Event {
    BuyEvent(BuyEvent),
    DrawImage(DrawImage),
}
#[derive(Debug)]
#[allow(dead_code)]
pub struct BuyEvent {
    zone: (u32, u32, u32, u32),
    price: std::string::String,
    url: std::string::String,
    buy_self: bool,
    owner: std::string::String,
}
impl BuyEvent {
    pub fn new(
        zone: (u32, u32, u32, u32),
        price: std::string::String,
        url: std::string::String,
        buy_self: bool,
        owner: std::string::String,
    ) -> Self {
        BuyEvent {
            zone,
            price,
            url,
            buy_self,
            owner,
        }
    }
}

impl From<DecodedParams> for BuyEvent {
    fn from(decoded_data: DecodedParams) -> Self {
        fn to_uint(v: Value) -> (primitive_types::U256, usize) {
            if let Uint(a, b) = v {
                (a, b)
            } else {
                (primitive_types::U256::from_str("-1").unwrap(), 1)
            }
        }
        if let Tuple(v) = &decoded_data.get(0).unwrap().value {
            let (x, y, dx, dy) = (
                to_uint(v.get(0).unwrap().1.clone()).0.as_u32(),
                to_uint(v.get(1).unwrap().1.clone()).0.as_u32(),
                to_uint(v.get(2).unwrap().1.clone()).0.as_u32(),
                to_uint(v.get(3).unwrap().1.clone()).0.as_u32(),
            );
            if let Uint(a, _) = &decoded_data.get(1).unwrap().value {
                let price = a.to_string();
                if let String(url) = &decoded_data.get(2).unwrap().value {
                    if let Bool(buy_self) = &decoded_data.get(3).unwrap().value {
                        if let Address(owner) = &decoded_data.get(4).unwrap().value {
                            let owner = owner;
                            return BuyEvent::new(
                                (x, y, dx, dy),
                                price,
                                url.clone(),
                                *buy_self,
                                owner.to_string(),
                            );
                        }
                    }
                }
            }
        }
        panic!("Failed to convert to BuyEvent struct");
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct DrawImage {
    x: u32,
    y: u32,
    url: std::string::String,
    overlay: bool,
}
impl DrawImage {
    pub fn new(x: u32, y: u32, url: std::string::String, overlay: bool) -> Self {
        DrawImage { x, y, url, overlay }
    }
}
impl From<DecodedParams> for DrawImage {
    fn from(decoded_data: DecodedParams) -> Self {
        if let Uint(x, _) = &decoded_data.get(0).unwrap().value {
            if let Uint(y, _) = &decoded_data.get(1).unwrap().value {
                if let String(url) = &decoded_data.get(2).unwrap().value {
                    if let Bool(overlay) = &decoded_data.get(3).unwrap().value {
                        return DrawImage::new(
                            x.as_u32(),
                            y.as_u32(),
                            url.clone(),
                            overlay.clone(),
                        );
                    }
                }
            }
        }

        panic!("Failed to convert to DrawImage struct");
    }
}