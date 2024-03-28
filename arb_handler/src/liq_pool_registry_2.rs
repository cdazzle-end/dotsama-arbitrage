use std::io;
use std::{str, path::Path, fs::File, io::Read};
use num::{BigInt, ToPrimitive, FromPrimitive, CheckedAdd, BigUint, CheckedMul, CheckedDiv, CheckedSub};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::RefCell;
use std::rc::Rc;
use crate::asset_registry_2::{AssetRegistry2, Asset};
// use crate::token::{Token, AssetKeyType, TokenData};

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
    pub pool_id: Option<String>, 
    pub exchange: Option<String>,
    pub prices: Option<(u64, u64)>,
    pub price_decimals: Option<(u64,u64)>,
    pub a: Option<u64>,
    pub a_precision: Option<u64>,
    pub token_precisions: Option<Vec<String>>,
    pub total_supply: Option<u128>,
    pub abi: Option<String>,
    pub dex_type: Option<String>,
    pub fee_rate: Option<String>,
    pub current_tick: Option<i64>,
    pub active_liquidity: Option<String>,
    pub initialized_ticks: Option<Vec<i128>>,
    pub lower_ticks: Option<Vec<TickData>>,
    pub upper_ticks: Option<Vec<TickData>>,
    // pub pool_id: Option<String>,
    pub share_issuance: Option<String>,
    pub swap_fee: Option<String>,
    pub pool_share_asset: Option<AssetPointer>,
    // pub is_evm: bool
}
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct MyLpJson{
    chainId: u64,
    dexType: String,
    contractAddress: Option<String>,
    abi: Option<String>,
    poolAssets: Vec<serde_json::Value>,
    liquidityStats: Vec<String>,
    feeRate: Option<String>,
    currentTick: Option<String>,
    activeLiquidity: Option<String>,
    initializedTicks: Option<Vec<i128>>,
    lowerTicks: Option<Vec<TickDataJson>>,
    upperTicks: Option<Vec<TickDataJson>>,

}
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TickDataJson {
    tick: i64,
    liquidityDelta: String,
    initialized: Option<bool>,
    liquidityTotal: String,
}
#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq, Eq)]
pub struct TickData{
    pub tick: i64,
    pub liquidity_delta: i128,
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
    a: String,
    aPrecision: u64,
    aBlock: String,
    futureA: String,
    futureABlock: String,
    totalSupply: String,
    poolPrecision: String,
    poolId: Option<String>,
    shareIssuance: Option<String>
}

impl LiqPoolRegistry2{
    pub fn build_liqpool_registry(asset_registry: &AssetRegistry2) -> LiqPoolRegistry2{
        let chains = vec![ "kar", "bnc_kusama", "movr", "hko", "mgx", "bsx"];
        // let chains = vec![ "kar", "bnc", "movr", "hko", "sdn", "kucoin", "mgx", "bsx"];
        let mut parsed_files = chains
                .into_iter()
                .map(|chain| {
                    // let path_string = format!("../lps/{}/lps.json", chain);
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
                                pool_id: None,
                                assets: vec![asset, usdt],
                                liquidity: vec![],
                                a: None,
                                a_precision: None,
                                token_precisions: None,
                                total_supply: None,
                                abi: None,
                                dex_type: None,
                                fee_rate: None,
                                current_tick: None,
                                active_liquidity: None,
                                initialized_ticks: None,
                                lower_ticks: None,
                                upper_ticks: None,
                                // pool_id: None,
                                share_issuance: None,
                                swap_fee: None,
                                pool_share_asset: None

                            })
                        } else if lp.as_object().unwrap().contains_key("a") {
                            let lp_data: StableLpJson = serde_json::from_value(lp.clone()).map_err(|e| e).unwrap();
                            let chain_id = lp_data.chainId;
                            let contract_address = lp_data.contractAddress;
                            // let lp_id = chain_id.clone().to_string() + &contract_address.clone();
                            let pool_assets = lp_data.poolAssets;
                            // let total_supply = lp_data.totalSupply;
                            let liquidity_stats = lp_data.liquidityStats.iter().map(
                                |x| x.as_str().parse().map_err(|e| e).unwrap()
                            ).collect();
                            let assets = pool_assets.into_iter().filter_map(|asset| {
                                let asset_id = serde_json::to_string(&asset).map_err(|e| e).unwrap();
                                asset_registry.get_asset_by_id(chain_id, asset_id.as_str())
                            }).collect::<Vec<_>>();

                            let pool_id = lp_data.poolId;
                            let pool_share_asset = if chain_id == 2034{
                                // println!("Chain id: {} | Pool ID: {}", chain_id.clone(), pool_id.clone().unwrap());
                                let asset = asset_registry.get_asset_by_id(chain_id, &serde_json::to_string(&pool_id.clone().unwrap()).unwrap()).unwrap();
                                Some(asset)
                            } else {
                                None
                            
                            };
                            let share_issuance = lp_data.shareIssuance;
                            let swap_fee = lp_data.swapFee;

                            // println!("Found stable ");
                            let pool = Some(LiqPool2 {
                                chain_id,
                                contract_address: contract_address.clone(),
                                pool_id: pool_id.clone(),
                                assets: assets,
                                liquidity: liquidity_stats,
                                a: Some(lp_data.futureA.parse().unwrap()),
                                a_precision: Some(lp_data.aPrecision),
                                token_precisions: Some(lp_data.tokenPrecisions),
                                exchange: None,
                                prices: None,
                                price_decimals: None,
                                total_supply: Some(lp_data.totalSupply.parse().map_err(|e| e).unwrap()),
                                abi: None,
                                dex_type: None,
                                fee_rate: None,
                                current_tick: None,
                                active_liquidity: None,
                                initialized_ticks: None,
                                lower_ticks: None,
                                upper_ticks: None,
                                // pool_id: pool_id,
                                share_issuance: share_issuance,
                                swap_fee: Some(swap_fee),
                                pool_share_asset: pool_share_asset
                            });
                            // println!("{:?}", pool.clone().unwrap().liquidity);
                            pool
                        } else if lp.as_object().unwrap().contains_key("currentTick") {
                            let lp_data: MyLpJson = serde_json::from_value(lp.clone()).map_err(|e| e).unwrap();

                            let chain_id = lp_data.chainId;
                            let dex_type = lp_data.dexType;
                            let contract_address = lp_data.contractAddress;
                            let abi = lp_data.abi;
                            let pool_assets = lp_data.poolAssets;
                            let liquidity_stats = lp_data.liquidityStats;
                            let fee_rate = lp_data.feeRate;
                            let current_tick = lp_data.currentTick;
                            let active_liquidity = lp_data.activeLiquidity;
                            let lower_tick_data = lp_data.lowerTicks;
                            let upper_tick_data = lp_data.upperTicks;
                            let lower_ticks = lower_tick_data.as_ref().map(|x| x.iter().map(|y| TickData{tick: y.tick.clone(), liquidity_delta: y.liquidityDelta.as_str().parse().map_err(|e| e).unwrap()}).collect());
                            let upper_ticks = upper_tick_data.as_ref().map(|x| x.iter().map(|y| TickData{tick: y.tick.clone(), liquidity_delta: y.liquidityDelta.as_str().parse().map_err(|e| e).unwrap()}).collect());
                            
                            let liquidity_0 = liquidity_stats[0].as_str().parse().map_err(|e| e).unwrap();
                            let liquidity_1 = liquidity_stats[1].as_str().parse().map_err(|e| e).unwrap();
                            let assets = pool_assets.into_iter().filter_map(|asset| {
                                let asset_id = serde_json::to_string(&asset).map_err(|e| e).unwrap();
                                asset_registry.get_asset_by_id(chain_id, asset_id.as_str())
                            }).collect::<Vec<_>>();
                            if assets.len() == 2 {
                                Some(LiqPool2 {
                                    chain_id,
                                    contract_address: contract_address.clone(),
                                    pool_id: contract_address.clone(),
                                    assets: vec![assets[0].clone(), assets[1].clone()],
                                    liquidity: vec![liquidity_0, liquidity_1],
                                    exchange: None,
                                    prices: None,
                                    price_decimals: None,
                                    a: None,
                                    a_precision: None,
                                    token_precisions: None,
                                    total_supply: None,
                                    abi: Some(abi.unwrap()),
                                    dex_type: Some(dex_type),
                                    fee_rate: fee_rate,
                                    current_tick: Some(current_tick.unwrap().parse().map_err(|e| e).unwrap()),
                                    initialized_ticks: lp_data.initializedTicks,
                                    active_liquidity: active_liquidity,
                                    lower_ticks: lower_ticks,
                                    upper_ticks: upper_ticks,
                                    // pool_id: None,
                                    share_issuance: None,
                                    swap_fee: None,
                                    pool_share_asset: None
                                })
                            } else {
                                None
                            }
                        } else {
                            let lp_data: MyLpJson = serde_json::from_value(lp.clone()).map_err(|e| e).unwrap();

                            let chain_id = lp_data.chainId;
                            let dex_type = lp_data.dexType;
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
                                    contract_address: contract_address.clone(),
                                    pool_id: contract_address.clone(),
                                    assets: vec![assets[0].clone(), assets[1].clone()],
                                    liquidity: vec![liquidity_0, liquidity_1],
                                    exchange: None,
                                    prices: None,
                                    price_decimals: None,
                                    a: None,
                                    a_precision: None,
                                    token_precisions: None,
                                    total_supply: None,
                                    abi: None,
                                    dex_type: dex_type.into(),
                                    fee_rate: None,
                                    current_tick: None,
                                    active_liquidity: None,
                                    initialized_ticks: None,
                                    lower_ticks: None,
                                    upper_ticks: None,
                                    // pool_id: None,
                                    share_issuance: None,
                                    swap_fee: None,
                                    pool_share_asset: None
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
        let chains = vec![ "aca", "bnc_polkadot", "glmr", "hdx", "para"];
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

        parsed_files.append(&mut parse_stable_lps_polkadot(vec!["aca", "hdx"]));

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
                                pool_id: None,
                                assets: vec![asset, usdt],
                                liquidity: vec![],
                                a: None,
                                a_precision: None,
                                token_precisions: None,
                                total_supply: None,
                                abi: None,
                                dex_type: None,
                                fee_rate: None,
                                current_tick: None,
                                active_liquidity: None,
                                initialized_ticks: None,
                                lower_ticks: None,
                                upper_ticks: None,
                                // pool_id: None,
                                share_issuance: None,
                                swap_fee: None,
                                pool_share_asset: None
                            })
                        } else if lp.as_object().unwrap().contains_key("a") {
                            let lp_data: StableLpJson = serde_json::from_value(lp.clone()).map_err(|e| e).unwrap();
                            let chain_id = lp_data.chainId;
                            let contract_address = lp_data.contractAddress;
                            let pool_assets = lp_data.poolAssets;
                            // let total_supply = lp_data.totalSupply;
                            let liquidity_stats = lp_data.liquidityStats.iter().map(
                                |x| x.as_str().parse().map_err(|e| e).unwrap()
                            ).collect();
                            let assets = pool_assets.into_iter().filter_map(|asset| {
                                let asset_id = serde_json::to_string(&asset).map_err(|e| e).unwrap();
                                asset_registry.get_asset_by_id(chain_id, asset_id.as_str())
                            }).collect::<Vec<_>>();

                            let pool_id = lp_data.poolId;
                            let pool_share_asset = if chain_id == 2034{
                                // println!("Chain id: {} | Pool ID: {}", chain_id.clone(), pool_id.clone().unwrap());
                                let asset = asset_registry.get_asset_by_id(chain_id, &serde_json::to_string(&pool_id.clone().unwrap()).unwrap()).unwrap();
                                Some(asset)
                            } else {
                                None
                            
                            };
                            let share_issuance = lp_data.shareIssuance;
                            let swap_fee = lp_data.swapFee;
                            // println!("Found stable ");
                            let pool = Some(LiqPool2 {
                                chain_id,
                                contract_address: contract_address.clone(),
                                pool_id: pool_id.clone(),
                                assets: assets,
                                liquidity: liquidity_stats,
                                a: Some(lp_data.futureA.parse().unwrap()),
                                a_precision: Some(lp_data.aPrecision),
                                token_precisions: Some(lp_data.tokenPrecisions),
                                exchange: None,
                                prices: None,
                                price_decimals: None,
                                total_supply: Some(lp_data.totalSupply.parse().map_err(|e| e).unwrap()),
                                abi: None,
                                dex_type: None,
                                fee_rate: None,
                                current_tick: None,
                                active_liquidity: None,
                                initialized_ticks: None,
                                lower_ticks: None,
                                upper_ticks: None,
                                // pool_id: pool_id,
                                share_issuance: share_issuance,
                                swap_fee: Some(swap_fee),
                                pool_share_asset

                            });
                            // println!("{:?}", pool.clone().unwrap().liquidity);
                            pool
                        }  else if lp.as_object().unwrap().contains_key("currentTick") {
                            
                            let lp_data: MyLpJson = serde_json::from_value(lp.clone()).map_err(|e| e).unwrap();

                            let chain_id = lp_data.chainId;

                            // println!("Adding {} LP", chain_id.clone());
                            let dex_type = lp_data.dexType;
                            let contract_address = lp_data.contractAddress;
                            let abi = lp_data.abi;
                            let pool_assets = lp_data.poolAssets;
                            let liquidity_stats = lp_data.liquidityStats;
                            let fee_rate = lp_data.feeRate;
                            let current_tick = lp_data.currentTick;
                            let active_liquidity = lp_data.activeLiquidity;
                            let lower_tick_data = lp_data.lowerTicks;
                            let upper_tick_data = lp_data.upperTicks;
                            // println!("Lower tick data: {:?}", lower_tick_data);
                            // println!("Upper tick data: {:?}", upper_tick_data);

                            let lower_ticks = lower_tick_data.as_ref().map(|x| x.iter().map(|y| TickData{tick: y.tick.clone(), liquidity_delta: y.liquidityDelta.as_str().parse().map_err(|e| e).unwrap()}).collect());
                            let upper_ticks = upper_tick_data.as_ref().map(|x| x.iter().map(|y| TickData{tick: y.tick.clone(), liquidity_delta: y.liquidityDelta.as_str().parse().map_err(|e| e).unwrap()}).collect());
                            
                            // let liquidity_0 = liquidity_stats[0].as_str().parse().map_err(|e| e).unwrap();
                            // let liquidity_1 = liquidity_stats[1].as_str().parse().map_err(|e| e).unwrap();
                            let assets = pool_assets.into_iter().filter_map(|asset| {
                                let asset_id = serde_json::to_string(&asset).map_err(|e| e).unwrap();
                                asset_registry.get_asset_by_id(chain_id, asset_id.as_str())
                            }).collect::<Vec<_>>();
                            if assets.len() == 2 {
                                Some(LiqPool2 {
                                    chain_id,
                                    contract_address: contract_address.clone(),
                                    pool_id: contract_address.clone(),
                                    assets: vec![assets[0].clone(), assets[1].clone()],
                                    liquidity: vec![0],
                                    exchange: None,
                                    prices: None,
                                    price_decimals: None,
                                    a: None,
                                    a_precision: None,
                                    token_precisions: None,
                                    total_supply: None,
                                    abi: Some(abi.unwrap()),
                                    dex_type: Some(dex_type),
                                    fee_rate: fee_rate,
                                    current_tick: Some(current_tick.unwrap().parse().map_err(|e| e).unwrap()),
                                    active_liquidity: active_liquidity,
                                    initialized_ticks: lp_data.initializedTicks,
                                    lower_ticks: lower_ticks,
                                    upper_ticks: upper_ticks,
                                    // pool_id: None,
                                    share_issuance: None,
                                    swap_fee: None,
                                    pool_share_asset: None
                                })
                            } else {
                                None
                            }
                        } else {
                            let lp_data: MyLpJson = serde_json::from_value(lp.clone()).map_err(|e| e).unwrap();
                            // println!("{:?}", lp_data.clone());
                            let chain_id = lp_data.chainId;
                            if chain_id == 2034{
                                // println!("{:?}", lp_data.clone());
                            }
                            // println!("Adding {} LP", chain_id.clone());
                            let dex_type = lp_data.dexType;
                            let contract_address = lp_data.contractAddress;
                            // let abi = lp_data.abi;
                            let pool_assets = lp_data.poolAssets;
                            let liquidity_stats = lp_data.liquidityStats;
  
                            let liquidity_0 = liquidity_stats[0].as_str().parse().map_err(|e| e).unwrap();
                            // println!("{}", liquidity_0);
                            let liquidity_1 = liquidity_stats[1].as_str().parse().map_err(|e| e).unwrap();
                            // println!("{}", liquidity_1);
                            let assets = pool_assets.into_iter().filter_map(|asset| {
                                let asset_id = serde_json::to_string(&asset).map_err(|e| e).unwrap();
                                let asset_by_id = asset_registry.get_asset_by_id(chain_id, asset_id.as_str());
                                
                                asset_by_id
                            }).collect::<Vec<_>>();
                            if assets.len() == 2 {
                                Some(LiqPool2 {
                                    chain_id,
                                    contract_address: contract_address.clone(),
                                    pool_id: contract_address.clone(),
                                    assets: vec![assets[0].clone(), assets[1].clone()],
                                    liquidity: vec![liquidity_0, liquidity_1],
                                    exchange: None,
                                    prices: None,
                                    price_decimals: None,
                                    a: None,
                                    a_precision: None,
                                    token_precisions: None,
                                    total_supply: None,
                                    abi: None,
                                    dex_type: Some(dex_type),
                                    fee_rate: None,
                                    current_tick: None,
                                    active_liquidity: None,
                                    initialized_ticks: None,
                                    lower_ticks: None,
                                    upper_ticks: None,
                                    // pool_id: None,
                                    share_issuance: None,
                                    swap_fee: None,
                                    pool_share_asset: None

                                })
                            } else {
                                // println!("ELSE");
                                None
                            }
                        }
                        
                    }).collect::<Vec<_>>()
            }).collect::<Vec<_>>();

            for pool in &lps{
                if pool.chain_id == 2034{
                    // pool.display_pool();
                }
            }

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
            let path_string = format!("../../../polkadot_assets/lps/lp_registry/{}_stable_pools.json", chain);
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
            let path_string = format!("../../../polkadot_assets/lps/lp_registry/{}_stable_lps.json", chain);
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
