use std::{path::Path, fs::File, io::Read};
use std::str;
use std::collections::HashMap;
use serde_json::Value;

use crate::liq_pool::LiqPool;
use crate::token::Token;

pub struct EvmReader;

#[derive(Debug)]
struct EvmToken{
    pub chain: String,
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
}

// impl EvmReader{
//     pub fn get_token_registry(chain: &str) -> Vec<Token>{
//         let path_var = match chain {
//             // "Karura" => "kar",
//             "Moonbeam" => "glmr",
//             "Moonriver" => "movr",
//             "Astar" => "astr",
//             "Shiden" => "sdn",
//             _ => panic!("COULDNT MATCH CHAIN")
//         };
//         let path_string = r"..\".to_string() + path_var + r"\token_registry.txt";
//         let path = Path::new(path_string.as_str());
//         let mut buf: Vec<u8> = Vec::new();
//         let mut file = File::open(path).unwrap_or_else(|err| panic!("problem opening file: Error {:?}", err));
//         file.read_to_end(&mut buf).unwrap_or_else(|err| panic!("problem reading activity map into buffer {:?}", err));
//         let parsed: Value = serde_json::from_str(str::from_utf8(&buf).unwrap()).unwrap();

//         let mut token_list: Vec<Token> = Vec::new();
//         // let mut token_map = HashMap::new();
        

//         let mut x = 0;
//         while x < parsed.as_array().unwrap().len() {
//             // println!("{}", parsed[x]);
//             let token = Token::new(
//                 None, 
//                 parsed[x]["name"].as_str().unwrap().to_string(), 
//                 parsed[x]["symbol"].as_str().unwrap().to_string(), 
//                 parsed[x]["decimals"].as_u64().unwrap().try_into().unwrap(),
//                 0, 
//                 chain.to_string(), 
//                 Some(parsed[x]["address"].as_str().unwrap().to_string(),),
//             );

//             token_list.push(token);
//             x+=1;
//         }

//         println!("Tokens: {}", token_list.len());
//         token_list
        
//         // AssetRegistry {asset_list: token_list}
//     }

    // pub fn get_all_evm_liquidity_pools() -> Vec<LiqPool>{
    //     let mut liq_pools: Vec<LiqPool> = Vec::new();
    //     liq_pools.append(&mut EvmReader::get_liquidity_pools("Moonbeam"));
    //     liq_pools.append(&mut EvmReader::get_liquidity_pools("Moonriver"));
    //     liq_pools.append(&mut EvmReader::get_liquidity_pools("Astar"));
    //     liq_pools
    // }

//     pub fn get_liquidity_pools(chain: &str) -> Vec<LiqPool>{
//         let path_var = match chain {
//             // "Karura" => "kar",
//             "Moonbeam" => "glmr",
//             "Moonriver" => "movr",
//             "Astar" => "astr",
//             "Shiden" => "sdn",
//             _ => panic!("COULDNT MATCH CHAIN")
//         };
//         let path_string = r"..\".to_string() + path_var + r"\liq_pool_registry.txt";
//         // let path_string = r"..\glmr\liq_pool_registry.txt";
//         let path = Path::new(path_string.as_str());
//         let mut buf: Vec<u8> = Vec::new();
//         let mut file = File::open(path).unwrap_or_else(|err| panic!("problem opening file: Error {:?}", err));
//         file.read_to_end(&mut buf).unwrap_or_else(|err| panic!("problem reading activity map into buffer {:?}", err));
//         let parsed: Value = serde_json::from_str(str::from_utf8(&buf).unwrap()).unwrap();

//         let mut liq_pools: Vec<LiqPool> = Vec::new();
//         let mut x = 0;
//         while x < parsed.as_array().unwrap().len() {

//             let address = parsed[x]["address"].as_str().unwrap().to_string();
//             let reserves_1 = parsed[x]["reserves"][0].as_object().unwrap();
//             let reserves_2 = parsed[x]["reserves"][1].as_object().unwrap();
//             let token_1 = parsed[x]["token_1"].as_object().unwrap();
//             let token_2 = parsed[x]["token_2"].as_object().unwrap();

//             let hex_1 = reserves_1["hex"].as_str().unwrap().to_string();
//             let hex_1_parsed = hex_1.trim_start_matches("0x");
//             let hex_1_number = u128::from_str_radix(hex_1_parsed, 16).unwrap();

//             let hex_2 = reserves_2["hex"].as_str().unwrap().to_string();
//             let hex_2_parsed = hex_2.trim_start_matches("0x");
//             let hex_2_number = u128::from_str_radix(hex_2_parsed, 16).unwrap();

//             let token_1_obj = Token::new(
//                 None, 
//                 token_1["name"].as_str().unwrap().to_string(), 
//                 token_1["symbol"].as_str().unwrap().to_string(), 
//                 token_1["decimals"].as_u64().unwrap(), 
//                 0, 
//                 chain.to_string(), 
//                 Some(token_1["address"].as_str().unwrap().to_string()),
//             );

//             let token_2_obj = Token::new(
//                 None, 
//                 token_2["name"].as_str().unwrap().to_string(), 
//                 token_2["symbol"].as_str().unwrap().to_string(), 
//                 token_2["decimals"].as_u64().unwrap(), 
//                 0, 
//                 chain.to_string(), 
//                 Some(token_2["address"].as_str().unwrap().to_string()),
//             );

//             let tokens = vec![token_1_obj, token_2_obj];
//             let reserves = vec![hex_1_number, hex_2_number];
//             let liquidity_pool = LiqPool::new(chain.to_string(), Some(address), tokens, reserves);
//             // println!("{:?}", liquidity_pool);
//             // liquidity_pool.display_liq_pool();
//             liq_pools.push(liquidity_pool);
            
//             x+=1;
//         }
//         liq_pools
//     }
// }

// impl EvmToken{
//     pub fn new(chain: String, address: String, name: String, symbol: String, decimals: u32) -> EvmToken{
//         EvmToken {
//         chain, address, name, symbol, decimals
//         }
//     }
// }