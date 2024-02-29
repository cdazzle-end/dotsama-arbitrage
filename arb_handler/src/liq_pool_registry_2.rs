use std::io;
use std::{str, path::Path, fs::File, io::Read};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::RefCell;
use std::rc::Rc;
use crate::asset_registry_2::{AssetRegistry2, Asset};
use crate::token::{Token, AssetKeyType, TokenData};

type AssetPointer = Rc<RefCell<Asset>>;

pub struct LiqPoolRegistry2{
    pub liq_pools: Vec<LiqPool2>
}

#[derive(Clone, Debug)]
pub struct LiqPool2{
    pub chain_id: u64,
    pub assets: Vec<AssetPointer>,
    pub liquidity: Vec<u128>,
    pub contract_address: Option<String>,
    pub exchange: Option<String>,
    pub prices: Option<(u64, u64)>,
    pub price_decimals: Option<(u64,u64)>,
    pub a: Option<u64>,
    pub a_precision: Option<u64>,
    pub token_precisions: Option<Vec<String>>,
    // pub is_evm: bool
}
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct MyLpJson{
    chainId: u64,
    contractAddress: Option<String>,
    poolAssets: Vec<serde_json::Value>,
    liquidityStats: Vec<String>
}
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct CexLpJson{
    exchange: String,
    assetTicker: String,
    price: Vec<u64>,
    priceDecimals: Vec<u64>,
}
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct StableLpJson{
    chainId: u64,
    contractAddress: Option<String>,
    poolAssets: Vec<serde_json::Value>,
    liquidityStats: Vec<String>,
    tokenPrecisions: Vec<String>,
    swapFee: String,
    a: u64,
    aPrecision: u64,
    aBlock: String,
    futureA: String,
    futureABlock: String,
    totalSupply: String,
    poolPrecision: String,
}

impl LiqPoolRegistry2{
    pub fn build_liqpool_registry(asset_registry: &AssetRegistry2) -> LiqPoolRegistry2{
        let chains = vec![ "kar", "bnc", "movr", "hko", "kucoin", "mgx", "bsx"];
        // let chains = vec![ "kar", "bnc", "movr", "hko", "sdn", "kucoin", "mgx", "bsx"];
        let mut parsed_files = chains
                .into_iter()
                .map(|chain| {
                    let path_string = format!("../lps/{}/lps.json", chain);
                    let path = Path::new(&path_string);
                    let mut buf = vec![];
                    let mut file = File::open(path)?;
                    file.read_to_end(&mut buf)?;
                    let parsed = serde_json::from_str(str::from_utf8(&buf).unwrap())?;
                    Ok(parsed)
                })
                .collect::<Result<Vec<Value>, io::Error>>()
                .unwrap();

        parsed_files.append(&mut parse_stable_lps(vec!["kar"]));

        let lps = parsed_files.into_iter()
            .flat_map(|parsed| {
                parsed.as_array().unwrap().into_iter()
                    .filter_map(|lp| {
                        if lp.as_object().unwrap().contains_key("exchange"){
                            let lp_data: CexLpJson = serde_json::from_value(lp.clone()).map_err(|e| e).unwrap();
                            let asset = asset_registry.get_asset_by_key(&(lp_data.exchange.clone() + lp_data.assetTicker.as_str())).unwrap();
                            let usdt = asset_registry.get_asset_by_key(&(lp_data.exchange.clone() + "USDT")).unwrap();
                            Some(LiqPool2{
                                chain_id: 0,
                                exchange: Some(lp_data.exchange),
                                prices: Some((lp_data.price[0], lp_data.price[1])),
                                price_decimals: Some((lp_data.priceDecimals[0], lp_data.priceDecimals[1])),
                                contract_address: None,
                                assets: vec![asset, usdt],
                                liquidity: vec![],
                                a: None,
                                a_precision: None,
                                token_precisions: None
                            })
                        } else if lp.as_object().unwrap().contains_key("a") {
                            let lp_data: StableLpJson = serde_json::from_value(lp.clone()).map_err(|e| e).unwrap();
                            let chain_id = lp_data.chainId;
                            let contract_address = lp_data.contractAddress;
                            let pool_assets = lp_data.poolAssets;
                            let liquidity_stats = lp_data.liquidityStats.iter().map(
                                |x| x.as_str().parse().map_err(|e| e).unwrap()
                            ).collect();
                            let assets = pool_assets.into_iter().filter_map(|asset| {
                                let asset_id = serde_json::to_string(&asset).map_err(|e| e).unwrap();
                                asset_registry.get_asset_by_id(chain_id, asset_id.as_str())
                            }).collect::<Vec<_>>();

                            println!("Found stable ");
                            let pool = Some(LiqPool2 {
                                chain_id,
                                contract_address,
                                assets: assets,
                                liquidity: liquidity_stats,
                                a: Some(lp_data.a),
                                a_precision: Some(lp_data.aPrecision),
                                token_precisions: Some(lp_data.tokenPrecisions),
                                exchange: None,
                                prices: None,
                                price_decimals: None,
                            });
                            // println!("{:?}", pool.clone().unwrap().liquidity);
                            pool
                        } else {
                            let lp_data: MyLpJson = serde_json::from_value(lp.clone()).map_err(|e| e).unwrap();

                            let chain_id = lp_data.chainId;
                            let contract_address = lp_data.contractAddress;
                            let pool_assets = lp_data.poolAssets;
                            let liquidity_stats = lp_data.liquidityStats;
                            
                            let liquidity_0 = liquidity_stats[0].as_str().parse().map_err(|e| e).unwrap();
                            let liquidity_1 = liquidity_stats[1].as_str().parse().map_err(|e| e).unwrap();
                            let assets = pool_assets.into_iter().filter_map(|asset| {
                                let asset_id = serde_json::to_string(&asset).map_err(|e| e).unwrap();
                                asset_registry.get_asset_by_id(chain_id, asset_id.as_str())
                            }).collect::<Vec<_>>();
                            if assets.len() == 2 {
                                Some(LiqPool2 {
                                    chain_id,
                                    contract_address,
                                    assets: vec![assets[0].clone(), assets[1].clone()],
                                    liquidity: vec![liquidity_0, liquidity_1],
                                    exchange: None,
                                    prices: None,
                                    price_decimals: None,
                                    a: None,
                                    a_precision: None,
                                    token_precisions: None,
                                })
                            } else {
                                None
                            }
                        }
                        
                    }).collect::<Vec<_>>()
            }).collect::<Vec<_>>();
        LiqPoolRegistry2 { liq_pools: lps }
    }

    pub fn build_liqpool_registry_polkadot(asset_registry: &AssetRegistry2) -> LiqPoolRegistry2{
        let chains = vec![ "aca", "bnc", "glmr", "hdx", "para"];
        // let chains = vec![ "kar", "bnc", "movr", "hko", "sdn", "kucoin", "mgx", "bsx"];
        let mut parsed_files = chains
                .into_iter()
                .map(|chain| {
                    let path_string = format!("../../../polkadot_assets/lps/lp_registry/{}_lps.json", chain);
                    let path = Path::new(&path_string);
                    let mut buf = vec![];
                    let mut file = File::open(path)?;
                    file.read_to_end(&mut buf)?;
                    let parsed = serde_json::from_str(str::from_utf8(&buf).unwrap())?;
                    Ok(parsed)
                })
                .collect::<Result<Vec<Value>, io::Error>>()
                .unwrap();

        parsed_files.append(&mut parse_stable_lps_polkadot(vec!["aca"]));

        let lps = parsed_files.into_iter()
            .flat_map(|parsed| {
                parsed.as_array().unwrap().into_iter()
                    .filter_map(|lp| {
                        if lp.as_object().unwrap().contains_key("exchange"){
                            let lp_data: CexLpJson = serde_json::from_value(lp.clone()).map_err(|e| e).unwrap();
                            let asset = asset_registry.get_asset_by_key(&(lp_data.exchange.clone() + lp_data.assetTicker.as_str())).unwrap();
                            let usdt = asset_registry.get_asset_by_key(&(lp_data.exchange.clone() + "USDT")).unwrap();
                            Some(LiqPool2{
                                chain_id: 0,
                                exchange: Some(lp_data.exchange),
                                prices: Some((lp_data.price[0], lp_data.price[1])),
                                price_decimals: Some((lp_data.priceDecimals[0], lp_data.priceDecimals[1])),
                                contract_address: None,
                                assets: vec![asset, usdt],
                                liquidity: vec![],
                                a: None,
                                a_precision: None,
                                token_precisions: None
                            })
                        } else if lp.as_object().unwrap().contains_key("a") {
                            let lp_data: StableLpJson = serde_json::from_value(lp.clone()).map_err(|e| e).unwrap();
                            let chain_id = lp_data.chainId;
                            let contract_address = lp_data.contractAddress;
                            let pool_assets = lp_data.poolAssets;
                            let liquidity_stats = lp_data.liquidityStats.iter().map(
                                |x| x.as_str().parse().map_err(|e| e).unwrap()
                            ).collect();
                            let assets = pool_assets.into_iter().filter_map(|asset| {
                                let asset_id = serde_json::to_string(&asset).map_err(|e| e).unwrap();
                                asset_registry.get_asset_by_id(chain_id, asset_id.as_str())
                            }).collect::<Vec<_>>();

                            println!("Found stable ");
                            let pool = Some(LiqPool2 {
                                chain_id,
                                contract_address,
                                assets: assets,
                                liquidity: liquidity_stats,
                                a: Some(lp_data.a),
                                a_precision: Some(lp_data.aPrecision),
                                token_precisions: Some(lp_data.tokenPrecisions),
                                exchange: None,
                                prices: None,
                                price_decimals: None,
                            });
                            // println!("{:?}", pool.clone().unwrap().liquidity);
                            pool
                        } else {
                            let lp_data: MyLpJson = serde_json::from_value(lp.clone()).map_err(|e| e).unwrap();
                            println!("{:?}", lp_data.clone());
                            let chain_id = lp_data.chainId;
                            let contract_address = lp_data.contractAddress;
                            let pool_assets = lp_data.poolAssets;
                            let liquidity_stats = lp_data.liquidityStats.clone();
                            
                           
                            let liquidity_0 = liquidity_stats[0].as_str().parse().map_err(|e| e).unwrap();
                            println!("{}", liquidity_0);
                            let liquidity_1 = liquidity_stats[1].as_str().parse().map_err(|e| e).unwrap();
                            println!("{}", liquidity_1);
                            let assets = pool_assets.into_iter().filter_map(|asset| {
                                let asset_id = serde_json::to_string(&asset).map_err(|e| e).unwrap();
                                asset_registry.get_asset_by_id(chain_id, asset_id.as_str())
                            }).collect::<Vec<_>>();
                            if assets.len() == 2 {
                                Some(LiqPool2 {
                                    chain_id,
                                    contract_address,
                                    assets: vec![assets[0].clone(), assets[1].clone()],
                                    liquidity: vec![liquidity_0, liquidity_1],
                                    exchange: None,
                                    prices: None,
                                    price_decimals: None,
                                    a: None,
                                    a_precision: None,
                                    token_precisions: None,
                                })
                            } else {
                                None
                            }
                        }
                        
                    }).collect::<Vec<_>>()
            }).collect::<Vec<_>>();
        LiqPoolRegistry2 { liq_pools: lps }
    }

    pub fn display_liq_pools(&self){
        for pool in &self.liq_pools{
            pool.display_pool();
        }
    }
    pub fn display_stable_pools(&self){
        for pool in &self.liq_pools{
            // pool.display_stable_pool();
            if let Some(a) = pool.a{
                print!("pool: ");
                for asset in &pool.assets{
                    asset.borrow().display_asset();
                    print!(" | ");
                }
                println!("");
                print!("liquidity: ");
                for liquidity in &pool.liquidity{
                    print!("{} | ", liquidity);
                }
            }
        }
    }

    // pub fn 
}

pub fn parse_stable_lps(chains: Vec<&str>) -> Vec<Value>{
    chains
        .into_iter()
        .map(|chain| {
            let path_string = format!("../lps/{}/stablePools.json", chain);
            let path = Path::new(&path_string);
            let mut buf = vec![];
            let mut file = File::open(path)?;
            file.read_to_end(&mut buf)?;
            let parsed = serde_json::from_str(str::from_utf8(&buf).unwrap())?;
            Ok(parsed)
        })
        .collect::<Result<Vec<Value>, io::Error>>()
        .unwrap()

}
pub fn parse_stable_lps_polkadot(chains: Vec<&str>) -> Vec<Value>{
    chains
        .into_iter()
        .map(|chain| {
            let path_string = format!("../lps/polkadot/{}_stable_lps.json", chain);
            let path = Path::new(&path_string);
            let mut buf = vec![];
            let mut file = File::open(path)?;
            file.read_to_end(&mut buf)?;
            let parsed = serde_json::from_str(str::from_utf8(&buf).unwrap())?;
            Ok(parsed)
        })
        .collect::<Result<Vec<Value>, io::Error>>()
        .unwrap()

}

impl LiqPool2{
    pub fn display_pool(&self){
        if let Some(exchange) = &self.exchange{
            println!("Exchange: {}", exchange);
            for asset in &self.assets{
                print!("{}    ", asset.borrow().get_asset_name());
            }
            println!("");
            println!("Bid: {}    Ask: {}", self.prices.unwrap().0, self.prices.unwrap().1);
            println!("---------------------------------");
        } else {
            println!("Chain ID: {}", self.chain_id);
            println!("Contract Address: {}", self.contract_address.as_ref().map_or("N/A", |x| &x));
            for asset in &self.assets{
                print!("{}    ", asset.borrow().get_asset_name());
            }
            println!("");
            for liquidity in &self.liquidity{
                print!("{}    ", liquidity);
            }
            println!("");
            println!("---------------------------------");
        }
        
    }

    pub fn display_stable_pool(&self){

    }
}
