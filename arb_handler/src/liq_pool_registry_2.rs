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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum Relay{
    Polkadot,
    Kusama
}



pub struct LiqPoolRegistry2{
    pub liq_pools: Vec<LiqPool2>,
    pub lp_registry_reworked: Vec<LiquidityPool>
}
pub struct LpRegistry2{
    pub lp_registry: Vec<LiquidityPool>
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TokenRate{
    pub numerator: u128,
    pub denominator: u128
}
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TokenRateJson{
    pub numerator: String,
    pub denominator: String
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BncStableData{
    pub chain_id: u64,
    pub pool_id: String,
    pub pool_assets: Vec<AssetPointer>,
    pub token_rates: Vec<TokenRate>,
    pub pool_liquidity: Vec<u128>,
    pub token_shares: Vec<u128>,
    pub token_precisions: Vec<u128>,
    pub swap_fee: u128,
    pub a: u128,
    pub a_precision: u128,
    pub fee_precision: u128,
    pub a_block: u128,
    pub future_a: u128,
    pub future_a_block: u128,
    pub total_supply: u128,
    pub pool_precision: u128,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DexData{
    pub chain_id: u64,
    pub dex_type: String,
    pub contract_address: Option<String>,
    pub pool_id: Option<String>,
    pub pool_assets: Vec<AssetPointer>,
    pub pool_liquidity: Vec<u128>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct StableData{
    pub chain_id: u64,
    pub pool_id: Option<String>,
    pub pool_assets: Vec<AssetPointer>,
    pub swap_fee: u128,
    pub share_asset: Option<AssetPointer>,
    pub share_issuance: Option<u128>,
    pub pool_liquidity: Vec<u128>,
    pub token_precisions: Vec<u128>,
    pub a: u128,
    pub a_precision: u128,
    pub fee_precision: u128,
    pub a_block: u128,
    pub future_a: u128,
    pub future_a_block: u128,
    pub total_supply: u128,
    pub token_shares: Option<Vec<u128>>,
    pub pool_precision: u128
}

#[derive(Debug, Clone, PartialEq)]
pub struct DexV3Data{
    pub chain_id: u64,
    pub dex_type: Option<String>,
    pub contract_address: String,
    pub abi: Option<String>,
    pub pool_id: Option<String>,
    pub pool_assets: Vec<AssetPointer>,
    pub active_liquidity: u128,
    pub current_tick: i64,
    pub fee_rate: u128,
    pub initialized_ticks: Vec<i128>,
    pub lower_ticks: Vec<TickData>,
    pub upper_ticks: Vec<TickData>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CexData{
    pub bid_price: u128,
    pub bid_decimals: u128,
    pub ask_price: u128,
    pub ask_decimals: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub enum LiquidityPool{
    BncStable(BncStableData),
    Cex(CexData),
    Dex(DexData),
    DexV3(DexV3Data),
    Stable(StableData)
}

pub enum PoolType{
    BncStable,
    Cex,
    Dex,
    DexV3,
    Stable
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
    pub token_rates: Option<Vec<TokenRate>>,
    pub token_shares: Option<Vec<u128>>,
    
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
    shareIssuance: Option<String>,
    tokenShares: Option<Vec<String>>,
    tokenRates: Option<Vec<TokenRateJson>>,
    feePrecision: String,
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

        let formatted_lps: Vec<LiquidityPool> = vec![];
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
                                pool_share_asset: None,
                                token_rates: None,
                                token_shares: None

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
                                pool_share_asset: pool_share_asset,
                                token_rates: None,
                                token_shares: None
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
                                    pool_share_asset: None,
                                    token_rates: None,
                                    token_shares: None
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
                                    pool_id: Some(dex_type.clone()),
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
                                    pool_share_asset: None,
                                    token_rates: None,
                                    token_shares: None
                                })
                            } else {
                                None
                            }
                        }
                        
                    }).collect::<Vec<_>>()
            }).collect::<Vec<_>>();
        LiqPoolRegistry2 { liq_pools: lps, lp_registry_reworked: formatted_lps}
    }

    pub fn build_liqpool_registry_polkadot(asset_registry: &AssetRegistry2) -> LiqPoolRegistry2{
        let chains = vec![ "aca", "bnc_polkadot", "glmr", "hdx", "para"];
        let relay = Relay::Polkadot;
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

        parsed_files.append(&mut parse_stable_lps_polkadot(vec!["aca", "hdx", "bnc_polkadot"]));

        let mut formatted_lps: Vec<LiquidityPool> = vec![];

        let lps = parsed_files.into_iter()
            .flat_map(|parsed| {
                parsed.as_array().unwrap().into_iter()
                    .filter_map(|lp| {
                        let mut pool_type: PoolType;

                        if lp.as_object().unwrap().contains_key("exchange"){
                            pool_type = PoolType::Cex;
                            // let lp_data: CexLpJson = serde_json::from_value(lp.clone()).map_err(|e| e).unwrap();
                            // let asset = asset_registry.get_asset_by_key(&(lp_data.exchange.clone() + lp_data.assetTicker.as_str())).unwrap();
                            // let usdt = asset_registry.get_asset_by_key(&(lp_data.exchange.clone() + "USDT")).unwrap();
                            // let cex_pool = Some(LiqPool2{
                            //     chain_id: 0,
                            //     exchange: Some(lp_data.exchange),
                            //     prices: Some((lp_data.price[0], lp_data.price[1])),
                            //     price_decimals: Some((lp_data.priceDecimals[0], lp_data.priceDecimals[1])),
                            //     contract_address: None,
                            //     pool_id: None,
                            //     assets: vec![asset, usdt],
                            //     liquidity: vec![],
                            //     a: None,
                            //     a_precision: None,
                            //     token_precisions: None,
                            //     total_supply: None,
                            //     abi: None,
                            //     dex_type: None,
                            //     fee_rate: None,
                            //     current_tick: None,
                            //     active_liquidity: None,
                            //     initialized_ticks: None,
                            //     lower_ticks: None,
                            //     upper_ticks: None,
                            //     // pool_id: None,
                            //     share_issuance: None,
                            //     swap_fee: None,
                            //     pool_share_asset: None,
                            //     token_rates: None,
                            //     token_shares: None
                            // });
                            None
                        } else if lp.as_object().unwrap().contains_key("a") {
                            pool_type = PoolType::Stable;
                            let lp_data: StableLpJson = serde_json::from_value(lp.clone()).map_err(|e| e).unwrap();
                            let chain_id = lp_data.chainId;

                            if relay == Relay::Polkadot && chain_id == 2030 {
                                let bnc_stable_pool: BncStableData = create_bnc_stable_pool(&lp_data, asset_registry, relay.clone());
                                formatted_lps.push(LiquidityPool::BncStable(bnc_stable_pool));

                            } else if relay == Relay::Polkadot && chain_id == 2000 {
                                let aca_stable_pool: BncStableData = create_bnc_stable_pool(&lp_data, asset_registry, relay.clone());
                                formatted_lps.push(LiquidityPool::BncStable(aca_stable_pool))

                            } else {
                                let stable_pool: StableData = create_stable_pool(&lp_data, asset_registry, relay.clone());
                                formatted_lps.push(LiquidityPool::Stable(stable_pool));
                            }
                            // let contract_address = lp_data.contractAddress;
                            // let pool_assets = lp_data.poolAssets;
                            // let token_shares = lp_data.tokenShares;
                            // let token_rates = lp_data.tokenRates;
                            // // let total_supply = lp_data.totalSupply;
                            // let liquidity_stats = lp_data.liquidityStats.iter().map(
                            //     |x| x.as_str().parse().map_err(|e| e).unwrap()
                            // ).collect();
                            // let assets = pool_assets.into_iter().filter_map(|asset| {
                            //     let asset_id = serde_json::to_string(&asset).map_err(|e| e).unwrap();
                            //     asset_registry.get_asset_by_id(chain_id, asset_id.as_str())
                            // }).collect::<Vec<_>>();

                            // let pool_id = lp_data.poolId;
                            // let pool_share_asset = if chain_id == 2034{
                            //     // println!("Chain id: {} | Pool ID: {}", chain_id.clone(), pool_id.clone().unwrap());
                            //     let asset = asset_registry.get_asset_by_id(chain_id, &serde_json::to_string(&pool_id.clone().unwrap()).unwrap()).unwrap();
                            //     Some(asset)
                            // } else {
                            //     None
                            
                            // };
                            // let share_issuance = lp_data.shareIssuance;
                            // let swap_fee = lp_data.swapFee;

                            // let formatted_token_rates: Option<Vec<TokenRate>> = match token_rates{
                            //     Some(token_rates) => Some(token_rates.iter().map(|y| TokenRate{numerator: y.numerator.parse().map_err(|e| e).unwrap(), denominator: y.denominator.parse().map_err(|e| e).unwrap()}).collect()),
                            //     None => None
                            // };
                            // let formatted_token_shares: Option<Vec<u128>> = match token_shares{
                            //     Some(token_shares) => Some(token_shares.iter().map(|y| y.parse().map_err(|e| e).unwrap()).collect()),
                            //     None => None
                            // };
                            
                            // let pool = Some(LiqPool2 {
                            //     chain_id,
                            //     contract_address: contract_address.clone(),
                            //     pool_id: pool_id.clone(),
                            //     assets: assets,
                            //     liquidity: liquidity_stats,
                            //     a: Some(lp_data.futureA.parse().unwrap()),
                            //     a_precision: Some(lp_data.aPrecision),
                            //     token_precisions: Some(lp_data.tokenPrecisions),
                            //     exchange: None,
                            //     prices: None,
                            //     price_decimals: None,
                            //     total_supply: Some(lp_data.totalSupply.parse().map_err(|e| e).unwrap()),
                            //     abi: None,
                            //     dex_type: None,
                            //     fee_rate: None,
                            //     current_tick: None,
                            //     active_liquidity: None,
                            //     initialized_ticks: None,
                            //     lower_ticks: None,
                            //     upper_ticks: None,
                            //     // pool_id: pool_id,
                            //     share_issuance: share_issuance,
                            //     swap_fee: Some(swap_fee),
                            //     pool_share_asset,
                            //     token_shares: formatted_token_shares,
                            //     token_rates: formatted_token_rates

                            // });
                            None
                        }  else if lp.as_object().unwrap().contains_key("currentTick") {
                            
                            
                            let lp_data: MyLpJson = serde_json::from_value(lp.clone()).map_err(|e| e).unwrap();

                            let mut asset_not_registered = false;
                            lp_data.poolAssets.iter().for_each(|asset| {
                                let asset_id = serde_json::to_string(&asset).map_err(|e| e).unwrap();
                                let asset_pointer = asset_registry.get_asset_by_id(lp_data.chainId, asset_id.as_str());
                                // If asset id not registered, skip
                                if asset_pointer.is_none(){
                                    asset_not_registered = true;
                                }
                            });
                            
                            if !asset_not_registered{
                                let dex_v3_pool: DexV3Data = create_dex_v3_pool(&lp_data, asset_registry, relay.clone());
                                formatted_lps.push(LiquidityPool::DexV3(dex_v3_pool));
                            } else {
                                println!("Filter GLMR erc pool")
                            }

                            None
                            // } else {
                            //     None
                            // }
                        } else {
                            let lp_data: MyLpJson = serde_json::from_value(lp.clone()).map_err(|e| e).unwrap();
                            // println!("{:?}", lp_data.clone());
                            let chain_id = lp_data.chainId;

                            // GLMR filter pools for ercs
                            if relay == Relay::Polkadot && chain_id == 2004 {
                                let mut asset_not_registered = false;
                                lp_data.poolAssets.iter().for_each(|asset| {
                                    let asset_id = serde_json::to_string(&asset).map_err(|e| e).unwrap();
                                    let asset_pointer = asset_registry.get_asset_by_id(chain_id, asset_id.as_str());
                                
                                    if asset_pointer.is_none(){
                                        asset_not_registered = true;
                                    }
                                });
                                if !asset_not_registered{
                                    let glmr_dex: DexData = create_dex_pool(&lp_data, asset_registry, relay.clone());
                                    formatted_lps.push(LiquidityPool::Dex(glmr_dex));
                                } else {
                                    println!("Filter GLMR erc pool")
                                }

                            } else {
                                let dex_pool = create_dex_pool(&lp_data, asset_registry, relay.clone());
                                formatted_lps.push(LiquidityPool::Dex(dex_pool));
                            }


                            // println!("Adding {} LP", chain_id.clone());
                            // let dex_type = lp_data.dexType;
                            // let contract_address = lp_data.contractAddress;
                            // // let abi = lp_data.abi;
                            // let pool_assets = lp_data.poolAssets;
                            // let liquidity_stats = lp_data.liquidityStats;
  
                            // let liquidity_0 = liquidity_stats[0].as_str().parse().map_err(|e| e).unwrap();
                            // // println!("{}", liquidity_0);
                            // let liquidity_1 = liquidity_stats[1].as_str().parse().map_err(|e| e).unwrap();
                            // // println!("{}", liquidity_1);
                            // let assets = pool_assets.into_iter().filter_map(|asset| {
                            //     let asset_id = serde_json::to_string(&asset).map_err(|e| e).unwrap();
                            //     let asset_by_id = asset_registry.get_asset_by_id(chain_id, asset_id.as_str());
                                
                            //     asset_by_id
                            // }).collect::<Vec<_>>();
                            // if assets.len() == 2 {
                            //     Some(LiqPool2 {
                            //         chain_id,
                            //         contract_address: contract_address.clone(),
                            //         pool_id: contract_address.clone(),
                            //         assets: vec![assets[0].clone(), assets[1].clone()],
                            //         liquidity: vec![liquidity_0, liquidity_1],
                            //         exchange: None,
                            //         prices: None,
                            //         price_decimals: None,
                            //         a: None,
                            //         a_precision: None,
                            //         token_precisions: None,
                            //         total_supply: None,
                            //         abi: None,
                            //         dex_type: Some(dex_type),
                            //         fee_rate: None,
                            //         current_tick: None,
                            //         active_liquidity: None,
                            //         initialized_ticks: None,
                            //         lower_ticks: None,
                            //         upper_ticks: None,
                            //         // pool_id: None,
                            //         share_issuance: None,
                            //         swap_fee: None,
                            //         pool_share_asset: None,
                            //         token_shares: None,
                            //         token_rates: None

                            //     })
                            // } else {
                                // println!("ELSE");
                                None
                            // }
                        }
                        
                    }).collect::<Vec<_>>()
            }).collect::<Vec<_>>();

        LiqPoolRegistry2 { liq_pools: lps, lp_registry_reworked: formatted_lps }
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

// GLMR filter out ercs that arent xc's
pub fn create_dex_pool(pool_json: &MyLpJson, asset_registry: &AssetRegistry2, relay: Relay) -> DexData{
    
    let chain_id = pool_json.chainId.clone();
    let dex_type = pool_json.dexType.clone();
    let contract_address = pool_json.contractAddress.clone();
    let pool_assets = pool_json.poolAssets.clone();
    let liquidity_stats = pool_json.liquidityStats.clone();
   
    let liquidity_0 = liquidity_stats[0].as_str().parse().map_err(|e| e).unwrap();
    let liquidity_1 = liquidity_stats[1].as_str().parse().map_err(|e| e).unwrap();
    let assets = pool_assets.clone().into_iter().filter_map(|asset| {
        let asset_id = serde_json::to_string(&asset).map_err(|e| e).unwrap();
        asset_registry.get_asset_by_id(chain_id, asset_id.as_str())
    }).collect::<Vec<_>>();



    let pool: DexData = DexData {
        chain_id,
        contract_address: contract_address.clone(),
        pool_id: contract_address.clone(),
        pool_assets: assets.clone(),
        pool_liquidity: vec![liquidity_0, liquidity_1],
        dex_type: dex_type.clone(),
    };

    if assets.len() < 2{
        println!("Chain ID: {}", chain_id);
        println!("Contract Address: {:?}", contract_address.clone());
        println!("Pool Assets: {:?}", pool_assets);
        println!("Liquidity Stats: {:?}", liquidity_stats);
        panic!("Not enough assets in pool. Chain Id: {} | Contract Address: {:?} | Pool Assets: {:?} | Liquidity Stats: {:?}", chain_id, contract_address.clone(), pool_assets, liquidity_stats);
    }

    pool

}

// REVIEW V3 pool dex types. Should  be DexV3, and abi should be further specification. And should be consistant across program.
pub fn create_dex_v3_pool(pool_json: &MyLpJson, asset_registry: &AssetRegistry2, relay: Relay) -> DexV3Data{
    
    let chain_id = pool_json.chainId;
    // println!("Adding {} LP", chain_id.clone());
    let dex_type = pool_json.dexType.clone();
    let contract_address = pool_json.contractAddress.clone();
    let abi = pool_json.abi.clone();
    let pool_assets = pool_json.poolAssets.clone();
    let liquidity_stats = pool_json.liquidityStats.clone();
    let fee_rate = pool_json.feeRate.clone();
    let current_tick = pool_json.currentTick.clone();
    let active_liquidity = pool_json.activeLiquidity.clone();
    let lower_tick_data = pool_json.lowerTicks.clone();
    let upper_tick_data = pool_json.upperTicks.clone();

    let lower_ticks: Option<Vec<TickData>> = lower_tick_data.as_ref().map(|x| x.iter().map(|y| TickData{tick: y.tick.clone(), liquidity_delta: y.liquidityDelta.as_str().parse().map_err(|e| e).unwrap()}).collect());
    let upper_ticks: Option<Vec<TickData>> = upper_tick_data.as_ref().map(|x| x.iter().map(|y| TickData{tick: y.tick.clone(), liquidity_delta: y.liquidityDelta.as_str().parse().map_err(|e| e).unwrap()}).collect());
    
    let assets = pool_assets.into_iter().filter_map(|asset| {
        let asset_id = serde_json::to_string(&asset).map_err(|e| e).unwrap();
        asset_registry.get_asset_by_id(chain_id, asset_id.as_str())
    }).collect::<Vec<_>>();

    let pool: DexV3Data = DexV3Data {
        chain_id,
        contract_address: pool_json.contractAddress.clone().unwrap(),
        pool_id: pool_json.contractAddress.clone(),
        pool_assets: assets,
        dex_type: Some(pool_json.dexType.clone()),
        abi: Some(pool_json.abi.clone().unwrap()),
        fee_rate: pool_json.feeRate.clone().unwrap().parse().map_err(|e| e).unwrap(),
        current_tick: pool_json.currentTick.clone().unwrap().parse().map_err(|e| e).unwrap(),
        active_liquidity: pool_json.activeLiquidity.clone().unwrap().parse().map_err(|e| e).unwrap(),
        initialized_ticks: pool_json.initializedTicks.clone().unwrap(),
        lower_ticks: lower_ticks.unwrap(),
        upper_ticks: upper_ticks.unwrap(),
    };
    pool

}

pub fn create_stable_pool(pool_json: &StableLpJson, asset_registry: &AssetRegistry2, relay: Relay ) -> StableData{
    // let lp_data: StableLpJson = serde_json::from_value(pool_json.clone()).map_err(|e| e).unwrap();
    let chain_id = pool_json.chainId;
    let contract_address = pool_json.contractAddress.clone();
    let pool_assets = pool_json.poolAssets.clone();
    let token_shares = pool_json.tokenShares.clone();
    let token_rates = pool_json.tokenRates.clone();
    let liquidity_stats = pool_json.liquidityStats.iter().map(
        |x| x.as_str().parse().map_err(|e| e).unwrap()
    ).collect();
    let assets = pool_assets.into_iter().filter_map(|asset| {
        let asset_id = serde_json::to_string(&asset).map_err(|e| e).unwrap();
        asset_registry.get_asset_by_id(chain_id, asset_id.as_str())
    }).collect::<Vec<_>>();

    let pool_id = pool_json.poolId.clone();
    let pool_share_asset = if relay == Relay::Polkadot && chain_id == 2034{
        // println!("Chain id: {} | Pool ID: {}", chain_id.clone(), pool_id.clone().unwrap());
        let asset = asset_registry.get_asset_by_id(chain_id, &serde_json::to_string(&pool_id.clone().unwrap()).unwrap()).unwrap();
        Some(asset)
    } else {
        None
    
    };
    let share_issuance = pool_json.shareIssuance.clone();
    let swap_fee = pool_json.swapFee.clone();

    let formatted_token_rates: Option<Vec<TokenRate>> = match token_rates{
        Some(token_rates) => Some(token_rates.iter().map(|y| TokenRate{numerator: y.numerator.parse().map_err(|e| e).unwrap(), denominator: y.denominator.parse().map_err(|e| e).unwrap()}).collect()),
        None => None
    };
    let formatted_token_shares: Option<Vec<u128>> = match token_shares{
        Some(token_shares) => Some(token_shares.iter().map(|y| y.parse().map_err(|e| e).unwrap()).collect()),
        None => None
    };
    
    // println!("Found stable ");
    let pool = StableData {
        chain_id,
        pool_id: pool_id.clone(),
        pool_assets: assets,
        swap_fee: pool_json.swapFee.parse().map_err(|e| e).unwrap(),
        share_asset: pool_share_asset,
        share_issuance: share_issuance.clone().map(|x| x.parse().map_err(|e| e).unwrap()),
        pool_liquidity: liquidity_stats,
        token_precisions: pool_json.tokenPrecisions.iter().map(|x| x.parse().map_err(|e| e).unwrap()).collect(),
        a: pool_json.a.parse().map_err(|e| e).unwrap(),
        a_precision: pool_json.aPrecision.clone().into(),
        a_block: pool_json.aBlock.parse().map_err(|e| e).unwrap(),
        future_a: pool_json.futureA.parse().map_err(|e| e).unwrap(),
        future_a_block: pool_json.futureABlock.parse().map_err(|e| e).unwrap(),
        total_supply: pool_json.totalSupply.parse().map_err(|e| e).unwrap(),
        fee_precision: pool_json.feePrecision.parse().map_err(|e| e).unwrap(),
        token_shares: formatted_token_shares,
        pool_precision: pool_json.poolPrecision.parse().map_err(|e| e).unwrap(),
    };
    pool

}
pub fn create_bnc_stable_pool(pool_json: &StableLpJson, asset_registry: &AssetRegistry2, relay: Relay ) -> BncStableData{
    // let lp_data: StableLpJson = serde_json::from_value(pool_json.clone()).map_err(|e| e).unwrap();
    let chain_id = pool_json.chainId;
    let pool_assets = pool_json.poolAssets.clone();


    let liquidity_stats = pool_json.liquidityStats.iter().map(
        |x| x.as_str().parse().map_err(|e| e).unwrap()
    ).collect();
    let assets = pool_assets.into_iter().filter_map(|asset| {
        let asset_id = serde_json::to_string(&asset).map_err(|e| e).unwrap();
        asset_registry.get_asset_by_id(chain_id, asset_id.as_str())
    }).collect::<Vec<_>>();


    let token_rates = pool_json.tokenRates.clone();
    let formatted_token_rates: Option<Vec<TokenRate>> = match token_rates{
        Some(token_rates) => Some(token_rates.iter().map(|y| TokenRate{numerator: y.numerator.parse().map_err(|e| e).unwrap(), denominator: y.denominator.parse().map_err(|e| e).unwrap()}).collect()),
        None => None
    };

    let token_shares = pool_json.tokenShares.clone();
    let formatted_token_shares: Option<Vec<u128>> = match token_shares{
        Some(token_shares) => Some(token_shares.iter().map(|y| y.parse().map_err(|e| e).unwrap()).collect()),
        None => None
    };
    
    // println!("Found stable ");
    let pool = BncStableData {
        chain_id,
        pool_id: pool_json.poolId.clone().unwrap(),
        pool_assets: assets,
        token_rates: formatted_token_rates.unwrap(),
        pool_liquidity: liquidity_stats,
        token_shares: formatted_token_shares.unwrap(),
        token_precisions: pool_json.tokenPrecisions.iter().map(|x| x.parse().map_err(|e| e).unwrap()).collect(),
        swap_fee: pool_json.swapFee.parse().map_err(|e| e).unwrap(),
        a: pool_json.a.parse().map_err(|e| e).unwrap(),
        a_precision: pool_json.aPrecision.clone().into(),
        a_block: pool_json.aBlock.parse().map_err(|e| e).unwrap(),
        future_a: pool_json.futureA.parse().map_err(|e| e).unwrap(),
        future_a_block: pool_json.futureABlock.parse().map_err(|e| e).unwrap(),
        total_supply: pool_json.totalSupply.parse().map_err(|e| e).unwrap(),
        pool_precision: pool_json.poolPrecision.parse().map_err(|e| e).unwrap(),
        fee_precision: pool_json.feePrecision.parse().map_err(|e| e).unwrap(),
    };

    pool

}