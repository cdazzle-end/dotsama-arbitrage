
use std::fs::File;
use std::io::prelude::*;
use serde_json::{Value};
use std::str;
use std::path::Path;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Token{
    pub asset_key: AssetKeyType,
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
    pub minimal_balance: u64
}

impl Token{
    pub fn new(asset_key: AssetKeyType, name: String, symbol: String, decimals: u32, minimal_balance: u64) -> Token{
        Token {
            asset_key, name, symbol, decimals, minimal_balance
        }
    }
    pub fn get_asset_key(&self){
        // self.asset_key

        match &self.asset_key {
            AssetKeyType::NativeAssetId{token: x} => println!("NAI {}", x),
            AssetKeyType::ForeignAssetId(x) => println!("FAI {}", x),
            AssetKeyType::StableAssetId(x) => println!("SAI {}", x),
            AssetKeyType::Erc20(x) => println!("ERC {}", x),
            _ => println!("FAILED TO MATCH KEY")
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AssetKeyType{
    NativeAssetId{token: String},
    ForeignAssetId(String),
    StableAssetId(String),
    Erc20(String),
}

impl AssetKeyType{
    fn get_key_string(&self) -> String{
        match &self {
            AssetKeyType::NativeAssetId{token: x} => format!("{{NativeAssetId:{{Token:{}}}", x),
            AssetKeyType::ForeignAssetId(x) => format!("{{ForeignAssetId:{}}}", x),
            AssetKeyType::StableAssetId(x) => format!("{{StableAssetId:{}}}", x),
            AssetKeyType::Erc20(x) => format!("{{Erc20:{}}}", x),
            _ => panic!("FAILED TO MATCH KEY")
        }
    }
}

// impl<AssetKeyType> PartialEq for AssetKeyType{
//     fn eq(&self, other: &Self) -> bool {
//         let asset_key_value = match &self {
//             AssetKeyType::NativeAssetId{token: x} => x,
//             AssetKeyType::ForeignAssetId(x) => x,
//             AssetKeyType::StableAssetId(x) => x,
//             AssetKeyType::Erc20(x) => x,
//             _ => println!("FAILED TO MATCH KEY 1")
//         };

//         let other_key_value = match &self {
//             AssetKeyType::NativeAssetId{token: x} => x,
//             AssetKeyType::ForeignAssetId(x) => x,
//             AssetKeyType::StableAssetId(x) => x,
//             AssetKeyType::Erc20(x) => x,
//             _ => println!("FAILED TO MATCH KEY")
//         };


//         asset_key_value == other_key_value;
//     }
// }