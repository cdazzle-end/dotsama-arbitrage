use std::{str, path::Path, fs::File, io::Read};

use serde_json::Value;
use std::cell::RefCell;
use std::rc::Rc;
use crate::{asset_registry::{AssetRegistry, Asset}};
use crate::token::{Token, AssetKeyType, TokenData};

// use crate::asset_registry::AssetRegistry;

// use crate::asset_registry::AssetRegistry;
type AssetPointer = Rc<RefCell<Asset>>;

pub struct LiqPoolRegistry{
    pub liq_pools: Vec<LiqPool>
}

#[derive(Debug)]
pub struct LiqPool{
    pub chain: String,
    pub exchange: Option<String>,
    pub prices: Option<(u64, u64)>,
    pub price_decimals: Option<(u64,u64)>,
    pub contract_address: Option<String>,
    pub assets: Vec<AssetPointer>,
    pub liquidity: Vec<u128>,
    pub is_evm: bool
}

//TO DO: maybe make chain a custom ENUM

impl LiqPool{
    pub fn new(chain: String, exchange: Option<String>, prices: Option<(u64, u64)>, price_decimals: Option<(u64,u64)>, contract_address: Option<String>, assets: Vec<AssetPointer>, liquidity: Vec<u128>, is_evm: bool) -> LiqPool{
        LiqPool {
            chain,
            exchange,
            prices,
            price_decimals,
            contract_address,
            assets,
            liquidity,
            is_evm
        }
    }

    pub fn get_formatted_price(&self) -> (f64, f64){
        let decimals = self.price_decimals.unwrap();
        let bid = self.prices.unwrap().0 as f64/ f64::powi(10.0, decimals.0 as i32);
        let ask = self.prices.unwrap().1 as f64/ f64::powi(10.0, decimals.1 as i32);
        (bid, ask)
    }

    pub fn display_liq_pool(&self){
        if self.is_evm{
            println!("Pool address: {}", self.contract_address.clone().unwrap());
            println!("{:?} {} -- {:?} {}", self.assets[0].borrow().token_data.get_map_key(), self.assets[0].borrow().token_data.get_asset_name(), self.assets[1].borrow().token_data.get_map_key(), self.assets[1].borrow().token_data.get_asset_name());
            println!("{} -- {}", self.liquidity[0], self.liquidity[1])
        } else if self.exchange != None{
            println!("Kucoin LP");
            println!("{:?} {} -- {:?} {}", self.assets[0].borrow().token_data.get_map_key(), self.assets[0].borrow().token_data.get_asset_name(), self.assets[1].borrow().token_data.get_map_key(), self.assets[1].borrow().token_data.get_asset_name());
            let formatted_prices = self.get_formatted_price();
            println!("Bid: {} -- Ask: {}", formatted_prices.0, formatted_prices.1)
            
        }else{
            println!("SUB");
            println!("{} -- {}", self.assets[0].borrow().token_data.get_map_key(), self.assets[1].borrow().token_data.get_map_key());
            println!("{} -- {}", self.liquidity[0] , self.liquidity[1]);
            println!("Contract: {:?}", self.contract_address)
        }
    }
}
// "kar", "bnc", "movr"
impl LiqPoolRegistry{
    pub fn build_all_liqpool_registry(asset_registry: &AssetRegistry) -> LiqPoolRegistry{
        let chains = vec!["heiko"];
        // let chain_ids = vec!["2000", "2001"]; // NEED TO ADD CHAIN ID TO JSON FILE
        let mut liq_pools = Vec::new();
        let mut parsed_files = Vec::new();
        for chain in chains{
            let path_string = r"..\".to_owned() + chain + r"\liq_pool_registry";
            let mut buf: Vec<u8> = Vec::new();
            let mut file = File::open(Path::new(&path_string)).unwrap_or_else(|err| panic!("problem opening file: Error {:?}", err));
            file.read_to_end(&mut buf).unwrap_or_else(|err| panic!("problem reading activity map into buffer {:?}", err));
            let parsed: Value = serde_json::from_str(str::from_utf8(&buf).unwrap()).unwrap();
            parsed_files.push(parsed);
        }
        // println!("{:?}", parsed_files);
        for (i, file) in parsed_files.iter().enumerate(){
            // let chain_id = chain_ids[i];
            for liq_pool in file.as_array().unwrap(){
                // println!("{:?}", liq_pool);
                println!("{:?}", liq_pool);
                let chain_id = liq_pool["chainId"].as_str().unwrap();
                let assets = liq_pool["poolAssets"].as_array().unwrap();
                let liquidity_stats = liq_pool["liquidityStats"].as_array().unwrap();
                
                let liquidity_0: u128 = liquidity_stats[0].as_str().unwrap().parse().unwrap();
                let liquidity_1: u128 = liquidity_stats[1].as_str().unwrap().parse().unwrap();
                // println!("{} -- {}", liquidity_0, liquidity_1)
                let contract_address = liq_pool["contractAddress"].as_str().unwrap().to_string();

                if contract_address == "None".to_string(){
                    let asset_0 = asset_registry.asset_map_lookup(chain_id.to_string(), parse_asset_key_type(&assets[0]).get_key_string());
                    let asset_1 = asset_registry.asset_map_lookup(chain_id.to_string(), parse_asset_key_type(&assets[1]).get_key_string());
                    let liq_pool = LiqPool::new(chain_id.to_string(),None, None, None, None, vec![asset_0, asset_1], vec![liquidity_0, liquidity_1], false);
                    liq_pools.push(liq_pool);
                } else {
                    let asset_0 = asset_registry.asset_map_lookup(chain_id.to_string(), assets[0].as_str().unwrap().to_string().to_ascii_lowercase());
                    let asset_1 = asset_registry.asset_map_lookup(chain_id.to_string(), assets[1].as_str().unwrap().to_string().to_ascii_lowercase());
                    let liq_pool = LiqPool::new(chain_id.to_string(), None, None, None, Some(contract_address), vec![asset_0, asset_1], vec![liquidity_0, liquidity_1], true);
                    liq_pools.push(liq_pool);
                }
            }
        }

        LiqPoolRegistry { liq_pools }
    }

    //Abberviated list, only substrate tokens and important evm tokens
    pub fn build_sub_liqpool_registry(asset_registry: &AssetRegistry) -> LiqPoolRegistry{
        let chains = vec![ "kar", "bnc", "movr", "heiko"];
        let sub_evm_addresses = asset_registry.get_substrate_evm_tokens();
        let mut liq_pools = Vec::new();
        let mut parsed_files = Vec::new();
        for chain in chains{
            let path_string = r"..\".to_owned() + chain + r"\liq_pool_registry";
            let mut buf: Vec<u8> = Vec::new();
            let mut file = File::open(Path::new(&path_string)).unwrap_or_else(|err| panic!("problem opening file: Error {:?}", err));
            file.read_to_end(&mut buf).unwrap_or_else(|err| panic!("problem reading activity map into buffer {:?}", err));
            let parsed: Value = serde_json::from_str(str::from_utf8(&buf).unwrap()).unwrap();
            parsed_files.push(parsed);
        }
        // println!("{:?}", parsed_files);
        for (i, file) in parsed_files.iter().enumerate(){
            // let chain_id = chain_ids[i];
            for liq_pool in file.as_array().unwrap(){
                let chain_id = liq_pool["chainId"].as_str().unwrap();
                let assets = liq_pool["poolAssets"].as_array().unwrap();
                let liquidity_stats = liq_pool["liquidityStats"].as_array().unwrap();
                
                let liquidity_0: u128 = liquidity_stats[0].as_str().unwrap().parse().unwrap();
                let liquidity_1: u128 = liquidity_stats[1].as_str().unwrap().parse().unwrap();
                let contract_address = liq_pool["contractAddress"].as_str().unwrap().to_string();

                if contract_address == "None".to_string(){
                    let asset_0 = asset_registry.asset_map_lookup(chain_id.to_string(), parse_asset_key_type(&assets[0]).get_key_string());
                    let asset_1 = asset_registry.asset_map_lookup(chain_id.to_string(), parse_asset_key_type(&assets[1]).get_key_string());
                    let liq_pool = LiqPool::new(chain_id.to_string(), None, None, None, None, vec![asset_0, asset_1], vec![liquidity_0, liquidity_1], false);
                    liq_pools.push(liq_pool);
                } else if contract_address == "heiko" {
                    println!("ASSET 0: {:?}", assets[0]);
                    let asset_0 = asset_registry.asset_map_lookup(chain_id.to_string(), assets[0].as_str().unwrap().to_string().to_ascii_lowercase());
                    println!("ASSET 1: {:?}", assets[1]);
                    let asset_1 = asset_registry.asset_map_lookup(chain_id.to_string(), assets[1].as_str().unwrap().to_string().to_ascii_lowercase());
                    let liq_pool = LiqPool::new(chain_id.to_string(), None, None, None, None, vec![asset_0, asset_1], vec![liquidity_0, liquidity_1], false);
                    liq_pools.push(liq_pool);

                } else {
                    let asset_0 = asset_registry.asset_map_lookup(chain_id.to_string(), assets[0].as_str().unwrap().to_string().to_ascii_lowercase());
                    let asset_1 = asset_registry.asset_map_lookup(chain_id.to_string(), assets[1].as_str().unwrap().to_string().to_ascii_lowercase());
                     //Check if either asset is a sub_evm token
                    if asset_0.borrow().token_data.is_evm_sub_token() || asset_1.borrow().token_data.is_evm_sub_token() {
                        let liq_pool = LiqPool::new(chain_id.to_string(), None, None, None,Some(contract_address), vec![asset_0, asset_1], vec![liquidity_0, liquidity_1], true);
                        liq_pools.push(liq_pool);
                    }
                }
            }
        }
        add_kucoin_pairs(asset_registry, &mut liq_pools);

        LiqPoolRegistry { liq_pools }
    }

    pub fn display_all_pools(&self){
        for pool in &self.liq_pools{
            pool.display_liq_pool();
        }
    }

    pub fn display_kucoin_pools(&self){
        for pool in &self.liq_pools{
            if pool.exchange != None{
                pool.display_liq_pool()
            }
        }
    }
}

fn add_kucoin_pairs(asset_registry: &AssetRegistry, liq_pools: &mut Vec<LiqPool>){
    let kucoin_assets = asset_registry.get_kucoin_tokens();
    // let liq_pools = mut liq_pools
    //Get usdt asset
    let mut usdt_asset = asset_registry.get_kucoin_usdt();
    // let mut liq_pools = Vec::new();
    for asset in &kucoin_assets{
        match &asset.borrow().token_data{
            
            TokenData::KucoinToken { exchange, name, symbol, chain, precision, contract_address, price_data, price_decimals } => {
                if symbol != "USDT"{
                    let assets = vec![Rc::clone(&asset), Rc::clone(&usdt_asset)];
                    let liq_pool = LiqPool::new(
                        "None".to_string(), 
                        Some(exchange.to_string()), 
                        Some(price_data.clone()), 
                        Some(price_decimals.clone()), 
                        None, 
                        assets,
                        vec![], 
                        false
                    );
                    liq_pools.push(liq_pool);
                }
            },
            _ => panic!("Expected kucoin token")
        }
        
    }
    // liq_pools
}
fn parse_asset_key_type(value: &Value) -> AssetKeyType{
    let mut keys = value.as_object().unwrap().keys();
    
    match keys.next() {
        Some(x) if x == "NativeAssetId" => {
            let native_id = value["NativeAssetId"].as_object().unwrap();
            let native_key = native_id.keys().next().unwrap();
            let native_id_value = native_id.get_key_value(native_key).unwrap().1;
            let native_string = "\"".to_string() + &native_key.to_string() + "\":" + &native_id_value.to_string();
            // println!("{}", native_string);
            AssetKeyType::NativeAssetId(native_string)
        },
        Some(x) if x == "ForeignAssetId" => AssetKeyType::ForeignAssetId(value["ForeignAssetId"].as_str().unwrap().to_string()),
        Some(x) if x == "StableAssetId" => AssetKeyType::StableAssetId(value["StableAssetId"].as_str().unwrap().to_string()),
        Some(x) if x == "Erc20" => AssetKeyType::Erc20(value["Erc20"].as_str().unwrap().to_string()),
        _ => panic!("Error matching asset key type")
    }
}