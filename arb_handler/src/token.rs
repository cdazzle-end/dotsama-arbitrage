
use std::fs::File;
use std::io::prelude::*;
use serde_json::{Value};
use std::str;
use std::path::Path;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token{
    pub local_id: Option<AssetKeyType>,
    pub name: String,
    pub symbol: String,
    pub decimals: u64,
    pub minimal_balance: u64,
    pub chain: String,
    pub contract_address: Option<String>
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenData{
    SubToken{
        local_id: AssetKeyType,
        name: String,
        symbol: String,
        decimals: u64,
        minimal_balance: u64,
        chain: String,
        contract_address: Option<String>
    },
    HeikoToken{
        local_id: String,
        name: String,
        symbol: String,
        decimals: u64,
        deposit: u64,
        isFrozen: bool,
        chain: String,
    },
    EvmToken{
        contract_address: String,
        name: String,
        symbol: String,
        decimals: u64,
        chain: String,
        cross_chain: bool,
        local_id: Option<String>,
    },
    KucoinToken{
        exchange: String,
        name: String,
        symbol: String,
        chain: String,
        precision: u32,
        contract_address: String,
        price_data: (u64, u64),
        price_decimals: (u64, u64)
    }

}

impl Token{
    pub fn new(asset_key: Option<AssetKeyType>, name: String, symbol: String, decimals: u64, minimal_balance: u64, chain: String, contract_address: Option<String>) -> Token{
        Token {
            local_id: asset_key, name, symbol, decimals, minimal_balance, chain, contract_address
        }
    }
    pub fn get_asset_key(&self){
        // self.asset_key

        match &self.local_id.clone().unwrap() {
            AssetKeyType::NativeAssetId(x) => println!("NAI {}", x),
            AssetKeyType::ForeignAssetId(x) => println!("FAI {}", x),
            AssetKeyType::StableAssetId(x) => println!("SAI {}", x),
            AssetKeyType::Erc20(x) => println!("ERC {}", x),
            _ => println!("FAILED TO MATCH KEY")
        }
    }
}

impl TokenData{
    pub fn new_sub(local_id: AssetKeyType, name: String, symbol: String, decimals: u64, minimal_balance: u64, chain: String, contract_address: Option<String>) -> TokenData{
        TokenData::SubToken { local_id, name, symbol, decimals, minimal_balance, chain, contract_address } 
    }
    pub fn new_evm(contract_address: String, name: String, symbol: String, decimals: u64, chain: String, cross_chain: bool, local_id: Option<String>) -> TokenData{
        TokenData::EvmToken { contract_address, name, symbol, decimals, chain, cross_chain, local_id }
    }
    pub fn new_kucoin( exchange: String, name: String, symbol: String, chain: String, precision: u32, contract_address: String, price_data: (u64,u64), price_decimals: (u64,u64) ) -> TokenData{
        TokenData::KucoinToken { exchange, name, symbol, chain, precision, contract_address, price_data, price_decimals}
    }

    pub fn get_chain(&self) -> String{
        // self.chain
        match self {
            TokenData::SubToken { chain,.. } => {
                return chain.to_string()
            }
            TokenData::EvmToken { chain, .. } => {
                return chain.to_string()
            }
            TokenData::KucoinToken { .. } => {
                panic!("Can't query kucoin token for chain")
            }
            TokenData::HeikoToken{ chain, .. } => {
                return chain.to_string()
            }
        }
    }

    pub fn get_contract_address(&self) -> Option<String>{
        // self.chain
        match self {
            TokenData::SubToken { contract_address,.. } => {
                return contract_address.clone()
            }
            TokenData::EvmToken { contract_address, .. } => {
                return Some(contract_address.to_string())
            }
            TokenData::KucoinToken { contract_address, ..}  => {
                panic!("Can't query kucoin token for contract address")
            }
            TokenData::HeikoToken{ .. } => {
                panic!("heiko");
            }
        }
    }

    pub fn get_asset_name(&self) -> String{
        // self.chain
        match self {
            TokenData::SubToken { name,.. } => {
                return name.clone()
            }
            TokenData::EvmToken { name, .. } => {
                return name.clone()
            }
            TokenData::KucoinToken { name, .. }  => {
                return name.clone()
            }
            TokenData::HeikoToken{ name, .. } => {
                return name.clone()
            }
        }
    }

    pub fn get_asset_decimals(&self) -> u64{
        match self {
            TokenData::SubToken {decimals, ..} => {
                // panic!("Trying to get EVM local id on a SUB token")
                decimals.clone()
                
            }
            TokenData::EvmToken {decimals, ..} => {
                decimals.clone()
            }
            
            TokenData::KucoinToken { .. } => {
                // panic!("Can't query kucoin token for decimals")
                0
            }
            TokenData::HeikoToken{decimals, .. } => {
                decimals.clone()
            }
        }
    }

    pub fn get_map_key(&self) -> String{
        // self.chain
        match self {
            TokenData::SubToken {chain, local_id, ..} => {
                // panic!("Trying to get EVM local id on a SUB token")
                chain.to_string() + &local_id.get_key_string()
                
            }
            TokenData::EvmToken { chain, contract_address,.. } => {
                chain.to_string() + &contract_address.to_string()
            }
            TokenData::KucoinToken { exchange, symbol, .. }  => {
                // panic("Can't query kucoin token for chain")
                exchange.to_string() + symbol
            }
            TokenData::HeikoToken{ local_id, chain, .. } => {
                chain.to_string() + &local_id
            }
        }
    }

    pub fn get_local_id_evm(&self) -> Option<String>{
        // self.chain
        match self {
            TokenData::SubToken {..} => {
                panic!("Trying to get EVM local id on a SUB token")
            }
            TokenData::EvmToken { local_id, .. } => {
                return local_id.clone()
            }
            TokenData::KucoinToken { .. }  => {
                panic!("Can't query kucoin token for local evm id")
            }
            TokenData::HeikoToken{ .. } => {
                panic!("heiko");
            }
        }
    }

    pub fn get_symbol(&self) -> String{
        // self.chain
        match self {
            TokenData::SubToken {symbol, ..} => {
                symbol.clone()
            }
            TokenData::EvmToken { symbol, .. } => {
                symbol.clone()
            }
            TokenData::KucoinToken { symbol, .. }  => {
                symbol.clone()
            }
            TokenData::HeikoToken{ symbol, .. } => {
                symbol.clone()
            }
        }
    }

    pub fn is_exchange_token(&self) -> bool{
        match self{
            TokenData::KucoinToken { .. } => true,
            _ => false
        }
    }

    //Returns true if asset is evm version of native substrate asset
    pub fn is_evm_sub_token(&self) -> bool{
        match self{
            TokenData::SubToken {..} => {
                false
            }
            TokenData::EvmToken { cross_chain, .. } => {
                cross_chain.clone()
            }
            TokenData::KucoinToken { .. }  => {
                false
            }
            TokenData::HeikoToken{ .. } => {
                false
            }
        }
    }

    //This is just for kucoin price data
    pub fn get_price_decimals(&self) -> (u64, u64){
        match self{
            TokenData::SubToken {..} => {
                panic!("Can't query price decimals on Sub token")
            }
            TokenData::EvmToken { cross_chain, .. } => {
                panic!("Can't query price decimals on EVM token")
            }
            TokenData::KucoinToken {price_decimals, .. }  => {
                price_decimals.clone()
            }
            TokenData::HeikoToken{ .. } => {
                panic!("heiko");
            }
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AssetKeyType{
    NativeAssetId(String),
    ForeignAssetId(String),
    StableAssetId(String),
    Erc20(String),
}

impl AssetKeyType{
    pub fn get_key_string(&self) -> String{
        match &self {
            AssetKeyType::NativeAssetId(x) => format!("{{NativeAssetId:{{{}}}", x),
            AssetKeyType::ForeignAssetId(x) => format!("{{ForeignAssetId:{}}}", x),
            AssetKeyType::StableAssetId(x) => format!("{{StableAssetId:{}}}", x),
            AssetKeyType::Erc20(x) => format!("{{Erc20:{}}}", x),
            _ => panic!("FAILED TO MATCH KEY")
        }
    }
}
