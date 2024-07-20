use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use num::ToPrimitive;

use crate::liq_pool_registry_2::{BncStableData, DexData, DexV3Data, LiqPoolRegistry2, LiquidityPool, StableData, TickData, TokenRate};
use crate::asset_registry_2::{Asset, TokenData};

//The Adjacency Table contains the connections between nodes in the token graph

type AssetPointer = Rc<RefCell<Asset>>;

// #[derive(Debug, Clone, PartialEq)]
// pub enum Liquidity{
//     Dex(DexData),
//     DexV3(DexV3Data),
//     Cex(CexData),
//     Stable(StableData),
//     BncStable(BncStableData),
//     StableShare(StableShareData)
// }
// #[derive(Debug, Clone, PartialEq)]
// pub struct DexData{
//     pub dex_type: Option<String>,
//     pub lp_id: Option<String>,
//     pub base_liquidity: u128,
//     pub adjacent_liquidity: u128,
// }
// #[derive(Debug, Clone, PartialEq)]
// pub struct DexV3Data{
//     pub dex_type: Option<String>,
//     pub lp_id: Option<String>,
//     pub token_0: String,
//     pub token_1: String,
//     pub active_liquidity: u128,
//     pub current_tick: i64,
//     pub fee_rate: u128,
//     pub lower_ticks: Vec<TickData>,
//     pub upper_ticks: Vec<TickData>,
// }

// #[derive(Debug, Copy, Clone, PartialEq)]
// pub struct CexData{
//     pub bid_price: u128,
//     pub bid_decimals: u128,
//     pub ask_price: u128,
//     pub ask_decimals: u128,
// }
// #[derive(Debug, Clone, PartialEq)]
// pub struct StableData{
//     pub chain_id: u128,
//     pub pool_id: Option<String>,
//     pub pool_assets: Vec<AssetPointer>,
//     pub token_to_share: Option<bool>,
//     pub swap_fee: u128,
//     pub share_issuance: Option<u128>,
//     pub base_liquidity: u128,
//     pub adjacent_liquidity: Vec<u128>,
//     pub a: u128,
//     pub total_supply: u128,
//     pub base_token_precision: u128,
//     pub adjacent_token_precisions: Vec<u128>,
//     pub token_shares: Option<Vec<u128>>,
//     pub token_rates: Option<Vec<TokenRate>>
// }
// #[derive(Debug, Clone, PartialEq)]
// pub struct BncStableData{
//     pub chain_id: u128,
//     pub pool_id: String,
//     pub swap_fee: u128,
//     pub base_asset: AssetPointer,
//     pub base_asset_index: usize,
//     pub pool_assets: Vec<AssetPointer>,
//     pub pool_liquidity: Vec<u128>,
//     pub a: u128,
//     pub total_supply: u128,
//     pub token_precisions: Vec<u128>,
//     pub token_shares: Vec<u128>,
//     pub token_rates: Vec<TokenRate>
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct StableShareData{
//     pub chain_id: u128,
//     pub pool_id: Option<String>,
//     pub token_to_share: Option<bool>,
//     pub share_asset: AssetPointer,
//     pub base_asset: Option<AssetPointer>,
//     pub base_asset_index: Option<usize>,
//     pub pool_assets: Vec<AssetPointer>,
//     pub pool_assets_liquidity: Vec<u128>,
//     pub share_issuance: Option<u128>,
//     pub total_supply: u128,
//     pub a: u128,
//     pub token_precisions: Vec<u128>,
//     pub swap_fee: u128,
// }

pub struct AdjacencyTable2{
    pub table_2: HashMap<String, Vec<AdjacencyList>>,
    // pub table_reworked: HashMap<String, Vec<AdjacencyGroupReworked>>
}

pub struct AdjacencyList{
    pub primary_asset: AssetPointer,
    pub list: Vec<AdjacencyGroup>
}
pub struct AdjacencyListReworked{
    pub primary_asset: AssetPointer,
    pub list: Vec<AdjacencyGroupReworked>
}
#[derive(Debug, Clone)]
pub enum AdjacencyGroup{
    Dex(DexGroup),
    DexV3(DexV3Group),
    Cex(CexGroup),
    Stable(StableGroup),
    BncStable(BncStableGroup),
    StableShare(StableShareGroup)
}

#[derive(Debug, Clone)]
pub struct DexGroup{
    // pub group_type: GroupType,
    // pub adjacent_asset: Vec<AssetPointer>,
    // pub liquidity: Option<LiquidityPool>
    pub group_type: GroupType,
    pub pool_assets: Vec<AssetPointer>,
    pub liquidity: LiquidityPool,
    pub base_asset: AssetPointer,
    pub base_asset_index: usize,
}

#[derive(Debug, Clone)]
pub struct DexV3Group{
    // pub group_type: GroupType,
    // pub adjacent_asset: Vec<AssetPointer>,
    // pub liquidity: Option<LiquidityPool>
    pub group_type: GroupType,
    pub pool_assets: Vec<AssetPointer>,
    pub liquidity: LiquidityPool,
    pub base_asset: AssetPointer,
    pub base_asset_index: usize,
}

#[derive(Debug, Clone)]
pub struct CexGroup{
    // pub group_type: GroupType,
    // pub adjacent_asset: Vec<AssetPointer>,
    // pub liquidity: Option<LiquidityPool>
    pub group_type: GroupType,
    pub pool_assets: Vec<AssetPointer>,
    pub liquidity: LiquidityPool,
    pub base_asset: AssetPointer,
    pub base_asset_index: usize,
}

#[derive(Debug, Clone)]
pub struct StableGroup{
    // pub group_type: GroupType,
    // pub adjacent_asset: Vec<AssetPointer>,
    // pub liquidity: Option<LiquidityPool>
    pub group_type: GroupType,
    pub pool_assets: Vec<AssetPointer>,
    pub liquidity: LiquidityPool,
    pub base_asset: AssetPointer,
    pub base_asset_index: usize,
}
#[derive(Debug, Clone)]
pub struct BncStableGroup{
    pub group_type: GroupType,
    pub pool_assets: Vec<AssetPointer>,
    pub liquidity: LiquidityPool,
    pub base_asset: AssetPointer,
    pub base_asset_index: usize,
}

#[derive(Debug, Clone)]
pub struct StableShareGroup{
    pub group_type: GroupType,
    pub pool_assets: Vec<AssetPointer>,
    pub share_asset: AssetPointer,
    pub liquidity: LiquidityPool,
    pub token_to_share: bool,
    // pub base_asset: Option<AssetPointer>,
    pub base_asset_index: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct AdjacencyGroupReworked{
    pub group_type: GroupType,
    pub pool_assets: Vec<AssetPointer>,
    pub liquidity: LiquidityPool,
    pub base_asset: AssetPointer,
    pub base_asset_index: usize,
}
#[derive(Debug, Clone)]
pub enum GroupType{
    Stable,
    BncStable,
    StableShare,
    Cex,
    Dex,
    DexV3,
    Xcm
}
impl AdjacencyTable2{
    pub fn build_table_2(lp_registry: &LiqPoolRegistry2) -> AdjacencyTable2{
        let mut adjacency_table = AdjacencyTable2 { table_2: HashMap::new() };
        for lp in &lp_registry.lp_registry_reworked{
            match lp {
                LiquidityPool::BncStable(pool_data) => {
                    for (base_asset_index, base_asset) in pool_data.pool_assets.iter().enumerate(){
                        adjacency_table.add_bnc_stable_pair_to_table_reworked(pool_data.clone(), base_asset.clone(), base_asset_index);
                    }
                },
                // For stable AND stable share
                LiquidityPool::Stable(pool_data) => {
                    for (base_asset_index, base_asset) in pool_data.pool_assets.iter().enumerate(){
                        adjacency_table.add_stable_pair_to_table_reworked(pool_data.clone(), base_asset.clone(), base_asset_index);
                    }
                },
                LiquidityPool::DexV3(pool_data) => {
                    for (base_asset_index, base_asset) in pool_data.pool_assets.iter().enumerate(){
                        adjacency_table.add_dex_v3_pair_to_table_reworked(pool_data.clone(), base_asset.clone(), base_asset_index);
                    }
                },
                LiquidityPool::Dex(pool_data) => {
                    for (base_asset_index, base_asset) in pool_data.pool_assets.iter().enumerate(){
                        adjacency_table.add_dex_pair_to_table_reworked(pool_data.clone(), base_asset.clone(), base_asset_index);
                    }
                },
                _ => ()

            }
        }
        // for lp in &lp_registry.liq_pools{   
        //     // println!("lp: {:?}", lp);
        //     let dex_type = lp.dex_type.clone();
        //     let lp_id = lp.pool_id.clone();
        //     let (asset_0, asset_1) = (Rc::clone(&lp.assets[0]), Rc::clone(&lp.assets[1]));
        //     // This is for cex pools (NOT IN OPERATION ATM)
        //     if let Some(x) = &lp.exchange{
        //         let (bid_price, ask_price) = (lp.prices.unwrap().0 as u128, lp.prices.unwrap().0 as u128);
        //         let (bid_decimals, ask_decimals) = (lp.price_decimals.unwrap().0 as u128, lp.price_decimals.unwrap().0 as u128);
        //         adjacency_table.add_cex_pair_to_table(
        //             Rc::clone(&asset_0),
        //             Rc::clone(&asset_1),
        //             bid_price, bid_decimals,
        //             ask_price, ask_decimals
        //         );
        //         adjacency_table.add_cex_pair_to_table(
        //             Rc::clone(&asset_1),
        //             Rc::clone(&asset_0),
        //             bid_price, bid_decimals,
        //             ask_price, ask_decimals
        //         );

        //     // This is for stable pools
        //     } else if let Some(x) = &lp.a {
        //         for (base_asset_index, base_asset) in lp.assets.iter().enumerate(){
        //             // println!("LIQUIDITY: {:?}", lp.liquidity);
        //             let chain_id: u128 = lp.chain_id.clone().to_u128().unwrap();
        //             let pool_id = lp.pool_id.clone();
                    
                    
        //             // let base_asset = Rc::clone(&base_asset);
        //             let relay = base_asset.borrow().get_relay_chain();

        //             let base_asset_index = lp.assets.iter().position(|x| Rc::ptr_eq(x, &base_asset)).unwrap();
        //             let base_asset_liquidity = lp.liquidity[base_asset_index];
        //             // let target_
        //             let token_precisions = lp.token_precisions.as_ref().unwrap().iter().map(|x| x.parse::<u128>().unwrap()).collect::<Vec<u128>>();
        //             let total_supply = lp.total_supply.unwrap(); 
        //             let base_token_precision = token_precisions[base_asset_index];
        //             let swap_fee: u128 = lp.swap_fee.as_ref().unwrap().parse().unwrap();
        //             let mut adjacent_assets = vec![];
        //             let mut adjacent_liquidity = vec![];
        //             let mut adjacent_token_precisions = vec![];
        //             let token_rates = lp.token_rates.clone();
        //             let token_shares = lp.token_shares.clone();
        //             for(i, asset) in lp.assets.iter().enumerate(){
        //                 if i != base_asset_index{
        //                     adjacent_assets.push(Rc::clone(asset));
        //                     // adjacent_liquidity.push(lp.liquidity[i]);
        //                     adjacent_token_precisions.push(token_precisions[i]);
        //                 }
        //             }
        //             for(i, liq) in lp.liquidity.iter().enumerate(){
        //                 if i != base_asset_index{
        //                     adjacent_liquidity.push(*liq);
        //                 }
        //             }

        //             let mut all_pool_assets = vec![];
        //             let mut all_asset_liquidity = vec![];
        //             let mut all_token_precisions = vec![];
        //             for (i, asset) in lp.assets.iter().enumerate(){
        //                 all_pool_assets.push(Rc::clone(asset));
        //                 all_asset_liquidity.push(lp.liquidity[i]);
        //                 all_token_precisions.push(token_precisions[i]);
                        
        //             }

        //             let n_pool_assets = lp.assets.len();
        //             if relay == "polkadot" && chain_id == 2030 {
        //                 adjacency_table.add_bnc_stable_pair_to_table(chain_id, pool_id.clone().unwrap(), swap_fee, base_asset.clone(), base_asset_index, all_pool_assets.clone(), all_asset_liquidity.clone(), lp.a.unwrap().into(), total_supply, token_precisions, token_shares.unwrap(), token_rates.unwrap());
        //                 adjacency_table.add_bnc_stable_pair_to_table_reworked(lp.clone(), base_asset.clone(), base_asset_index);
        //             } else {
        //                 adjacency_table.add_stable_pair_to_table(chain_id.clone(), pool_id.clone(), swap_fee, base_asset.clone(), base_asset_liquidity, base_token_precision, adjacent_assets.clone(), adjacent_liquidity.clone(), lp.a.unwrap().into(), total_supply, adjacent_token_precisions.clone(), token_rates.clone(), token_shares.clone());
        //             }
        //             if chain_id == 2034{
        //                 // println!("Adding stable share pairs to table");
        //                 let share_issuance = lp.share_issuance.clone().unwrap().parse::<u128>().unwrap();

        //                 // Check if share asset is already in the pool
        //                 let share_asset = lp.pool_share_asset.clone().unwrap();
        //                 let share_asset_key = share_asset.borrow().get_map_key();
        //                 let share_asset_table_bucket = adjacency_table
        //                     .table_2
        //                     .entry(share_asset_key.clone())
        //                     .or_insert(vec![AdjacencyList::new(&share_asset)]);

        //                 // Check if Share asset: share -> token group has been added
        //                 let mut share_inserted = false;
        //                 for adjacency_list in share_asset_table_bucket{
        //                     let adjacency_list_key = adjacency_list.primary_asset.borrow().get_map_key();
        //                     if share_asset_key == adjacency_list_key{
        //                         share_inserted = adjacency_list.check_share_asset(&share_asset, n_pool_assets);
        //                     }
        //                 }

        //                 // Add share -> token group
        //                 if !share_inserted {
        //                     // println!("Adding share -> token group");
        //                     adjacency_table.add_stable_share_to_table_new(
        //                         false, // TOKEN TO SHARE
        //                         chain_id, 
        //                         pool_id.clone(),
        //                         swap_fee,
        //                         Rc::clone(&share_asset),
        //                         share_issuance,
        //                         None,
        //                         None,
        //                         all_asset_liquidity.clone(),
        //                         all_pool_assets.clone(),
        //                         lp.a.unwrap().into(), 
        //                         total_supply, 
        //                         all_token_precisions.clone()
        //                     );
                        
        //                 } else {
        //                     // println!("Share -> token group already added");
        //                 }
                    

        //                 // Add group for token -> share
        //                 adjacency_table.add_stable_share_to_table_new(
        //                     true, // TOKEN TO SHARE 
        //                     chain_id, 
        //                     pool_id.clone(),
        //                     swap_fee,
        //                     share_asset.clone(),
        //                     share_issuance, 
        //                     Some(base_asset.clone()),
        //                     Some(base_asset_index),
        //                     all_asset_liquidity.clone(),
        //                     all_pool_assets.clone(),
        //                     lp.a.unwrap().into(), 
        //                     total_supply, 
        //                     all_token_precisions.clone()
        //                 );
        //             }
                
        //         }
        //         // After adding stable pairs, add stable share pairs if on hdx



        //     // This is for regular dex pools, which is most of what were working with
        //     } else if let Some(x) = &lp.current_tick{
        //         // println!("Active Liquidity: {:?}", lp.active_liquidity);
        //         let active_liquidity = &lp.active_liquidity.clone().unwrap().as_str().parse::<u128>().unwrap();
        //         let contract_address = lp.contract_address.clone();
        //         let token_0 = asset_0.borrow().get_asset_contract_address().unwrap();
        //         let token_1 = asset_1.borrow().get_asset_contract_address().unwrap();
        //         adjacency_table.add_dex_3_to_table(
        //             dex_type.clone(),
        //             lp_id.clone(),
        //             Rc::clone(&asset_0),
        //             Rc::clone(&asset_1),
        //             token_0.clone(),
        //             token_1.clone(),
        //             *active_liquidity,
        //             lp.fee_rate.clone().unwrap().parse::<u128>().unwrap(),
        //             lp.current_tick.clone().unwrap().into(),
        //             lp.lower_ticks.clone().unwrap(),
        //             lp.upper_ticks.clone().unwrap()
        //         );
        //         adjacency_table.add_dex_3_to_table(
        //             dex_type.clone(),
        //             lp_id.clone(),
        //             Rc::clone(&asset_1),
        //             Rc::clone(&asset_0),
        //             token_0,
        //             token_1,
        //             *active_liquidity,
        //             lp.fee_rate.clone().unwrap().parse::<u128>().unwrap(),
        //             lp.current_tick.clone().unwrap().into(),
        //             lp.lower_ticks.clone().unwrap(),
        //             lp.upper_ticks.clone().unwrap()
        //         );
        //     } else {
        //         let (liquidity_0, liquidity_1) = (lp.liquidity[0], lp.liquidity[1]);
        //         adjacency_table.add_dex_pair_to_table(
        //             dex_type.clone(),
        //             lp_id.clone(),
        //             Rc::clone(&asset_0),
        //             liquidity_0,
        //             Rc::clone(&asset_1),
        //             liquidity_1,
        //         );
        //         adjacency_table.add_dex_pair_to_table(dex_type, lp_id, asset_1, liquidity_1, asset_0, liquidity_0);
        //     }
        // }


        adjacency_table
    }


    pub fn add_bnc_stable_pair_to_table_reworked(
        &mut self, 
        pool_data: BncStableData,
        base_asset: AssetPointer,
        base_asset_index: usize,

    ){
        let primary_node_asset = base_asset.clone();
        let primary_node_key = primary_node_asset.borrow().get_map_key();

        let pool_assets = pool_data.pool_assets.clone();

        //Get bucket from base asset key. If none exists yet, create new one
        let table_bucket = self
            .table_2
            .entry(primary_node_key.clone())
            .or_insert(vec![AdjacencyList::new(&primary_node_asset)]);

        //Find base_asset adjacency list and add new asset
        let mut inserted = false;

        //Loop through lists, find one that corresponds to primary asset (share asset or one of pool assets)
        for adjacency_list in table_bucket{
            let adjacency_list_key = adjacency_list.primary_asset.borrow().get_map_key();
            if primary_node_key == adjacency_list_key{
                // let pair = AdjacencyGroup::new_stable_pair(pool_id, share_issuance, adjacent_assets, base_liquidity, base_token_precision, adjacent_liquidity, a, total_supply, adjacent_token_precisions);
                

                let pair = AdjacencyGroup::BncStable(BncStableGroup{
                    group_type: GroupType::BncStable,
                    pool_assets: pool_assets.clone(),
                    liquidity: LiquidityPool::BncStable(pool_data.clone()),
                    base_asset: primary_node_asset.clone(),
                    base_asset_index
                });
                
                adjacency_list.list.push(pair);
                
                inserted = true;
                break;
            }
        }
        
            //If base_asset has no corresponding adjacency list in the table, create one
            //This should only happen if there is a hashmap collision, i.e. 2 different asset keys hash to the same value.
            if !inserted{
                let new_adjacency_list = AdjacencyList::new(&primary_node_asset);
                self.table_2.entry(primary_node_key.clone()).and_modify(|e| e.push(new_adjacency_list));
            }
    }


    pub fn add_stable_pair_to_table_reworked(
        &mut self,
        pool_data: StableData,
        base_asset: AssetPointer,
        base_asset_index: usize,
    ) {
        let primary_node_asset = base_asset.clone();
        let primary_node_key = primary_node_asset.borrow().get_map_key();

        //Get bucket from base asset key. If none exists yet, create new one
        let table_bucket = self
            .table_2
            .entry(primary_node_key.clone())
            .or_insert(vec![AdjacencyList::new(&primary_node_asset)]);

        //Find base_asset adjacency list and add new asset
        let mut inserted = false;

        //Loop through lists, find one that corresponds to primary asset (share asset or one of pool assets)
        for adjacency_list in table_bucket{
            let adjacency_list_key = adjacency_list.primary_asset.borrow().get_map_key();
            if primary_node_key == adjacency_list_key{
                
                let pair = AdjacencyGroup::Stable(StableGroup{
                    group_type: GroupType::Stable,
                    pool_assets: pool_data.pool_assets.clone(),
                    liquidity: LiquidityPool::Stable(pool_data.clone()),
                    base_asset: primary_node_asset.clone(),
                    base_asset_index
                });

                adjacency_list.list.push(pair);
                
                inserted = true;
                break;
            }
        }

        //If base_asset has no corresponding adjacency list in the table, create one
        //This should only happen if there is a hashmap collision, i.e. 2 different asset keys hash to the same value.
        if !inserted{
            let new_adjacency_list = AdjacencyList::new(&primary_node_asset);
            self.table_2.entry(primary_node_key.clone()).and_modify(|e| e.push(new_adjacency_list));
        }

        // If pool has share assets, HDX chain 2034
        match &pool_data.share_asset {
            Some(share_asset) => {
                let share_asset_key = share_asset.borrow().get_map_key();

                let share_asset_table_bucket = self
                    .table_2
                    .entry(share_asset_key.clone())
                    .or_insert(vec![AdjacencyList::new(&share_asset)]);

                // Check if Share asset: share -> token group has been added
                let mut share_inserted = false;

                let pool_id = pool_data.pool_id.clone();

                // Getting entry in hashmap with MAP KEY
                // let share_asset_list = share_asset_table_bucket.iter().find(|table_bucket_list| table_bucket_list.primary_asset.borrow().get_map_key() == share_asset_key);

                // match share_asset_list{
                //     Some(share_asset_list) => {
                //         // println!("FOUND SHARE LIST: MAP KEY")
                //         // share_inserted = share_asset_list.check_share_asset(&share_asset, pool_data.pool_assets.len());
                //     },
                //     None => panic!("Could not find share list with map key")
                // }

                // Getting entry in hashmap with RC COMPARE
                let share_asset_list = share_asset_table_bucket.iter().find(|table_bucket_list| Rc::ptr_eq(&table_bucket_list.primary_asset, &share_asset));
                let share_asset_list = match share_asset_list{
                    Some(share_asset_list) => {
                        // println!("FOUND SHARE LIST: RC POINTER EQ");
                        share_inserted = share_asset_list.check_share_asset(&share_asset, pool_id.clone().unwrap());
                    },
                    None => panic!("Could not find share list with rc pointer eq")
                };


                // for adjacency_list in share_asset_table_bucket{
                //     let adjacency_list_key = adjacency_list.primary_asset.borrow().get_map_key();
                //     if share_asset_key == adjacency_list_key{
                //         share_inserted = adjacency_list.check_share_asset(&share_asset, pool_id.clone().unwrap());
                //     }
                // }

                // Add share -> token group ** JUST ONCE
                if !share_inserted {
                    self.add_stable_share_pair_to_table_reworked(pool_data.clone(), false, None, None);
                }

                // Add group for token -> share
                self.add_stable_share_pair_to_table_reworked(pool_data.clone(), true, Some(base_asset.clone()), Some(base_asset_index));

                
            },
            None => ()
        };
    }
    
    pub fn add_stable_share_pair_to_table_reworked(
        &mut self,
        pool_data: StableData,
        token_to_share: bool,
        base_asset: Option<AssetPointer>,
        base_asset_index: Option<usize>,
    ){
        let share_asset = pool_data.share_asset.clone().unwrap();
        let all_pool_assets = pool_data.pool_assets.clone();
        // let mut primary_node_adjacent_assets = vec![];
        let primary_node_asset = if token_to_share { base_asset.clone().unwrap() } else { share_asset.clone() };
        let primary_node_key = primary_node_asset.borrow().get_map_key();

        //Get bucket from base asset key. If none exists yet, create new one
        let table_bucket = self
            .table_2
            .entry(primary_node_key.clone())
            .or_insert(vec![AdjacencyList::new(&primary_node_asset)]);
        

        //Find base_asset adjacency list and add new asset
        let mut inserted = false;

        //Loop through lists, find one that corresponds to primary asset (share asset or one of pool assets)
        for adjacency_list in table_bucket{
            let adjacency_list_key = adjacency_list.primary_asset.borrow().get_map_key();
            if primary_node_key == adjacency_list_key{
                // let pair = AdjacencyGroup::new_stable_pair(pool_id, share_issuance, adjacent_assets, base_liquidity, base_token_precision, adjacent_liquidity, a, total_supply, adjacent_token_precisions);
                


                let pair = AdjacencyGroup::StableShare(StableShareGroup{
                    group_type: GroupType::StableShare,
                    pool_assets: all_pool_assets.clone(),
                    liquidity: LiquidityPool::Stable(pool_data.clone()),
                    token_to_share,
                    share_asset,
                    base_asset_index

                });
                
                adjacency_list.list.push(pair);
                
                inserted = true;
                break;
            }
        }

        //If base_asset has no corresponding adjacency list in the table, create one
        //This should only happen if there is a hashmap collision, i.e. 2 different asset keys hash to the same value.
        if !inserted{
            let new_adjacency_list = AdjacencyList::new(&primary_node_asset);
            self.table_2.entry(primary_node_key.clone()).and_modify(|e| e.push(new_adjacency_list));
        }
    }

    pub fn add_dex_pair_to_table_reworked(
        &mut self,
        pool_data: DexData,
        base_asset: AssetPointer,
        base_asset_index: usize,
    ) {
        let primary_node_asset = base_asset.clone();
        let primary_node_key = primary_node_asset.borrow().get_map_key();

        //Get bucket from base asset key. If none exists yet, create new one
        let table_bucket = self
            .table_2
            .entry(primary_node_key.clone())
            .or_insert(vec![AdjacencyList::new(&primary_node_asset)]);

        //Find base_asset adjacency list and add new asset
        let mut inserted = false;

        //Loop through lists, find one that corresponds to primary asset (share asset or one of pool assets)
        for adjacency_list in table_bucket{
            let adjacency_list_key = adjacency_list.primary_asset.borrow().get_map_key();
            if primary_node_key == adjacency_list_key{
                
                let pair = AdjacencyGroup::Dex(DexGroup{
                    group_type: GroupType::Dex,
                    pool_assets: pool_data.pool_assets.clone(),
                    liquidity: LiquidityPool::Dex(pool_data.clone()),
                    base_asset: primary_node_asset.clone(),
                    base_asset_index
                });

                adjacency_list.list.push(pair);
                
                inserted = true;
                break;
            }
        }

        //If base_asset has no corresponding adjacency list in the table, create one
        //This should only happen if there is a hashmap collision, i.e. 2 different asset keys hash to the same value.
        if !inserted{
            let new_adjacency_list = AdjacencyList::new(&primary_node_asset);
            self.table_2.entry(primary_node_key.clone()).and_modify(|e| e.push(new_adjacency_list));
        }
    }

    pub fn add_dex_v3_pair_to_table_reworked(
        &mut self,
        pool_data: DexV3Data,
        base_asset: AssetPointer,
        base_asset_index: usize,
    ) {
        let primary_node_asset = base_asset.clone();
        let primary_node_key = primary_node_asset.borrow().get_map_key();

        //Get bucket from base asset key. If none exists yet, create new one
        let table_bucket = self
            .table_2
            .entry(primary_node_key.clone())
            .or_insert(vec![AdjacencyList::new(&primary_node_asset)]);

        //Find base_asset adjacency list and add new asset
        let mut inserted = false;

        //Loop through lists, find one that corresponds to primary asset (share asset or one of pool assets)
        for adjacency_list in table_bucket{
            let adjacency_list_key = adjacency_list.primary_asset.borrow().get_map_key();
            if primary_node_key == adjacency_list_key{
                
                let pair = AdjacencyGroup::DexV3(DexV3Group{
                    group_type: GroupType::Dex,
                    pool_assets: pool_data.pool_assets.clone(),
                    liquidity: LiquidityPool::DexV3(pool_data.clone()),
                    base_asset: primary_node_asset.clone(),
                    base_asset_index
                });

                adjacency_list.list.push(pair);
                
                inserted = true;
                break;
            }
        }

        //If base_asset has no corresponding adjacency list in the table, create one
        //This should only happen if there is a hashmap collision, i.e. 2 different asset keys hash to the same value.
        if !inserted{
            let new_adjacency_list = AdjacencyList::new(&primary_node_asset);
            self.table_2.entry(primary_node_key.clone()).and_modify(|e| e.push(new_adjacency_list));
        }
    }
    
    // fn add_dex_3_to_table(
    //     &mut self,
    //     dex_type: Option<String>,
    //     lp_id: Option<String>,
    //     base_asset: AssetPointer,
    //     adjacent_asset: AssetPointer,
    //     token_0: String,
    //     token_1: String,
    //     active_liquidity: u128,
    //     fee_rate: u128,
    //     current_tick: i64,
    //     lower_ticks: Vec<TickData>,
    //     upper_ticks: Vec<TickData>,
    // ){
    //     let base_asset_key = base_asset.borrow().get_map_key();

    //     let table_bucket = self
    //         .table_2
    //         .entry(base_asset_key.clone())
    //         .or_insert(vec![AdjacencyList::new(&base_asset)]);

    //     //Find base_asset adjacency list and add new asset
    //     let mut inserted = false;

    //     //Loop through lists, find one that corresponds to base asset
    //     for adjacency_list in table_bucket{

    //         //The first asset in an adjacency list is the primary asset (the rest are adjacent to the primary)
    //         let adjacency_list_key = adjacency_list.primary_asset.borrow().get_map_key();
    //         if base_asset_key == adjacency_list_key{
    //             adjacency_list.add_dex_3_pair(adjacent_asset, dex_type, lp_id, token_0, token_1, active_liquidity, fee_rate, current_tick, lower_ticks, upper_ticks);
    //             inserted = true;
    //             break;
    //         }
    //     }

    //     //If base_asset has no corresponding adjacency list in the table, that means there is a list at that index which isn't the one we're looking for
    //     //This should only happen if there is a hashmap collision, i.e. 2 different asset keys hash to the same value.
    //     if !inserted{
    //         let new_adjacency_list = AdjacencyList::new(&base_asset);
    //         self.table_2.entry(base_asset_key.clone()).and_modify(|e| e.push(new_adjacency_list));
    //     }

    // }

    // fn add_dex_pair_to_table(
    //     &mut self,
    //     dex_type: Option<String>,
    //     lp_id: Option<String>,
    //     base_asset: AssetPointer,
    //     base_liquidity: u128,
    //     adjacent_asset: AssetPointer,
    //     adjacent_liquidity: u128,
    // ){
    //     let base_asset_key = base_asset.borrow().get_map_key();

    //     let table_bucket = self
    //         .table_2
    //         .entry(base_asset_key.clone())
    //         .or_insert(vec![AdjacencyList::new(&base_asset)]);

    //     //Find base_asset adjacency list and add new asset
    //     let mut inserted = false;

    //     //Loop through lists, find one that corresponds to base asset
    //     for adjacency_list in table_bucket{

    //         //The first asset in an adjacency list is the primary asset (the rest are adjacent to the primary)
    //         let adjacency_list_key = adjacency_list.primary_asset.borrow().get_map_key();
    //         if base_asset_key == adjacency_list_key{
    //             adjacency_list.add_dex_pair(adjacent_asset, dex_type, lp_id, base_liquidity, adjacent_liquidity);
    //             inserted = true;
    //             break;
    //         }
    //     }

    //     //If base_asset has no corresponding adjacency list in the table, that means there is a list at that index which isn't the one we're looking for
    //     //This should only happen if there is a hashmap collision, i.e. 2 different asset keys hash to the same value.
    //     if !inserted{
    //         let new_adjacency_list = AdjacencyList::new(&base_asset);
    //         self.table_2.entry(base_asset_key.clone()).and_modify(|e| e.push(new_adjacency_list));
    //     }
    // }

    // pub fn add_cex_pair_to_table(
    //     &mut self,
    //     base_asset: AssetPointer,
    //     adjacent_asset: AssetPointer,
    //     bid_price: u128, bid_decimals: u128,
    //     ask_price: u128, ask_decimals: u128
    // ){
    //     // let table = &mut self.table;
    //     let base_asset_key = base_asset.borrow().get_map_key();

    //     //Get bucket from base asset key. If none exists yet, create new one
    //     let table_bucket = self
    //         .table_2
    //         .entry(base_asset_key.clone())
    //         .or_insert(vec![AdjacencyList::new(&base_asset)]);
        

    //     //Find base_asset adjacency list and add new asset
    //     let mut inserted = false;

    //     //Loop through lists, find one that corresponds to base asset
    //     for adjacency_list in table_bucket{

    //         //The first asset in an adjacency list is the primary asset (the rest are adjacent to the primary)
    //         let adjacency_list_key = adjacency_list.primary_asset.borrow().get_map_key();
    //         if base_asset_key == adjacency_list_key{
    //             adjacency_list.add_cex_pair(adjacent_asset, bid_price, bid_decimals, ask_price, ask_decimals);
    //             inserted = true;
    //             break;
    //         }
    //     }

    //     //If base_asset has no corresponding adjacency list in the table, create one
    //     //This should only happen if there is a hashmap collision, i.e. 2 different asset keys hash to the same value.
    //     if !inserted{
    //         let new_adjacency_list = AdjacencyList::new(&base_asset);
    //         self.table_2.entry(base_asset_key.clone()).and_modify(|e| e.push(new_adjacency_list));
    //     }
    // }

    // pub fn add_stable_pair_to_table(
    //     &mut self,
    //     chain_id: u128,
    //     pool_id: Option<String>,
    //     swap_fee: u128,
    //     base_asset: AssetPointer,
    //     base_liquidity: u128,
    //     base_token_precision: u128,
    //     adjacent_assets: Vec<AssetPointer>,
    //     adjacent_liquidity: Vec<u128>,
    //     a: u128,
    //     total_supply: u128,
    //     adjacent_token_precisions: Vec<u128>, 
    //     token_rates: Option<Vec<TokenRate>>, 
    //     token_shares: Option<Vec<u128>>
        
    // ){
    //     // println!("BASE LIQ {:?}", base_liquidity);
    //     // println!("ADJ LiQ: {:?}", adjacent_liquidity);

    //     // let table = &mut self.table;
    //     let base_asset_key = base_asset.borrow().get_map_key();

    //     //Get bucket from base asset key. If none exists yet, create new one
    //     let table_bucket = self
    //         .table_2
    //         .entry(base_asset_key.clone())
    //         .or_insert(vec![AdjacencyList::new(&base_asset)]);
        

    //     //Find base_asset adjacency list and add new asset
    //     let mut inserted = false;

    //     //Loop through lists, find one that corresponds to base asset
    //     for adjacency_list in table_bucket{
    //         let adjacency_list_key = adjacency_list.primary_asset.borrow().get_map_key();
    //         if base_asset_key == adjacency_list_key{
    //             adjacency_list.add_stable_pair(chain_id, pool_id, swap_fee, adjacent_assets, base_liquidity, base_token_precision, adjacent_liquidity, a, total_supply, adjacent_token_precisions, token_rates, token_shares);
    //             inserted = true;
    //             break;
    //         }
    //     }

    //     //If base_asset has no corresponding adjacency list in the table, create one
    //     //This should only happen if there is a hashmap collision, i.e. 2 different asset keys hash to the same value.
    //     if !inserted{
    //         let new_adjacency_list = AdjacencyList::new(&base_asset);
    //         self.table_2.entry(base_asset_key.clone()).and_modify(|e| e.push(new_adjacency_list));
    //     }
    // }

    // pub fn add_stable_share_pair_to_table(
    //     &mut self,
    //     token_to_share: bool,
    //     chain_id: u128,
    //     pool_id: Option<String>,
    //     swap_fee: u128,
    //     share_asset: AssetPointer,
    //     share_issuance: u128,
    //     base_asset: AssetPointer,
    //     base_liquidity: u128,
    //     base_token_precision: u128,
    //     mut adjacent_assets: Vec<AssetPointer>,
    //     adjacent_liquidity: Vec<u128>,
    //     a: u128,
    //     total_supply: u128,
    //     adjacent_token_precisions: Vec<u128>,
    // ) {
    //     let base_asset_key = match token_to_share{
    //         true => {
    //             adjacent_assets.push(Rc::clone(&share_asset));
    //             base_asset.borrow().get_map_key()
    //         },
    //         false => share_asset.borrow().get_map_key()
    //     };
        

    //     //Get bucket from base asset key. If none exists yet, create new one
    //     let table_bucket = self
    //         .table_2
    //         .entry(base_asset_key.clone())
    //         .or_insert(vec![AdjacencyList::new(&base_asset)]);
        

    //     //Find base_asset adjacency list and add new asset
    //     let mut inserted = false;

    //     //Loop through lists, find one that corresponds to base asset
    //     for adjacency_list in table_bucket{
    //         let adjacency_list_key = adjacency_list.primary_asset.borrow().get_map_key();
    //         if base_asset_key == adjacency_list_key{
    //             // let pair = AdjacencyGroup::new_stable_pair(pool_id, share_issuance, adjacent_assets, base_liquidity, base_token_precision, adjacent_liquidity, a, total_supply, adjacent_token_precisions);
                
    //             // let pair = AdjacencyGroup{
    //             //     group_type: GroupType::StableShare,
    //             //     adjacent_asset: adjacent_assets,
    //             //     liquidity: Some(Liquidity::StableShare(
    //             //         StableShareLp{
    //             //             chain_id,
    //             //             pool_id,
    //             //             share_asset: Rc::clone(&share_asset),
    //             //             token_to_share: Some(token_to_share),
    //             //             swap_fee: swap_fee,
    //             //             share_issuance: Some(share_issuance),
    //             //             base_liquidity: base_liquidity,
    //             //             base_token_precision: base_token_precision,
    //             //             pool_assets_liquidity: adjacent_liquidity,
    //             //             a: a,
    //             //             total_supply: total_supply,
    //             //             // base_token_precision: base_token_precision,
    //             //             adjacent_token_precisions: adjacent_token_precisions
    //             //         }
    //             //     ))
    //             // };
                
    //             // adjacency_list.list.push(pair);
                
    //             inserted = true;
    //             break;
    //         }
    //     }

    //     //If base_asset has no corresponding adjacency list in the table, create one
    //     //This should only happen if there is a hashmap collision, i.e. 2 different asset keys hash to the same value.
    //     if !inserted{
    //         let new_adjacency_list = AdjacencyList::new(&base_asset);
    //         self.table_2.entry(base_asset_key.clone()).and_modify(|e| e.push(new_adjacency_list));
    //     }


    // }

    // pub fn add_stable_share_to_table_new(
    //     &mut self,
    //     token_to_share: bool,
    //     chain_id: u128,
    //     pool_id: Option<String>,
    //     swap_fee: u128,
    //     share_asset: AssetPointer,
    //     share_issuance: u128,
    //     base_asset: Option<AssetPointer>,
    //     base_asset_index: Option<usize>,
    //     all_pool_assets_liquidity: Vec<u128>,
    //     all_pool_assets: Vec<AssetPointer>,
    //     a: u128,
    //     total_supply: u128,
    //     all_pool_token_precisions: Vec<u128>,
    // ) {
    //     let mut primary_node_adjacent_assets = vec![];
    //     let primary_node_asset = match token_to_share{
    //         true => {
    //             // adjacent_assets.push(Rc::clone(&share_asset));
    //             primary_node_adjacent_assets.push(Rc::clone(&share_asset));
    //             base_asset.clone().unwrap()
    //         },
    //         false => {
    //             primary_node_adjacent_assets = all_pool_assets.clone();
    //             share_asset.clone()
    //         }
    //     };
    //     let primary_node_key = primary_node_asset.borrow().get_map_key();

    //     //Get bucket from base asset key. If none exists yet, create new one
    //     let table_bucket = self
    //         .table_2
    //         .entry(primary_node_key.clone())
    //         .or_insert(vec![AdjacencyList::new(&primary_node_asset)]);
        

    //     //Find base_asset adjacency list and add new asset
    //     let mut inserted = false;

    //     //Loop through lists, find one that corresponds to primary asset (share asset or one of pool assets)
    //     for adjacency_list in table_bucket{
    //         let adjacency_list_key = adjacency_list.primary_asset.borrow().get_map_key();
    //         if primary_node_key == adjacency_list_key{
    //             // let pair = AdjacencyGroup::new_stable_pair(pool_id, share_issuance, adjacent_assets, base_liquidity, base_token_precision, adjacent_liquidity, a, total_supply, adjacent_token_precisions);
                


    //             let pair = AdjacencyGroup{
    //                 group_type: GroupType::StableShare,
    //                 adjacent_asset: primary_node_adjacent_assets,
    //                 liquidity: Some(Liquidity::StableShare(
    //                     StableShareData{
    //                         chain_id,
    //                         pool_id,
    //                         token_to_share: Some(token_to_share),
    //                         share_asset: Rc::clone(&share_asset),
    //                         share_issuance: Some(share_issuance),
    //                         base_asset,
    //                         base_asset_index,
    //                         pool_assets: all_pool_assets,
    //                         pool_assets_liquidity: all_pool_assets_liquidity,
    //                         token_precisions: all_pool_token_precisions,
    //                         a: a,
    //                         total_supply: total_supply,
    //                         swap_fee: swap_fee,
    //                     }
    //                 ))
    //             };
                
    //             adjacency_list.list.push(pair);
                
    //             inserted = true;
    //             break;
    //         }
    //     }

    //     //If base_asset has no corresponding adjacency list in the table, create one
    //     //This should only happen if there is a hashmap collision, i.e. 2 different asset keys hash to the same value.
    //     if !inserted{
    //         let new_adjacency_list = AdjacencyList::new(&primary_node_asset);
    //         self.table_2.entry(primary_node_key.clone()).and_modify(|e| e.push(new_adjacency_list));
    //     }
    // }

    // pub fn add_bnc_stable_pair_to_table(
    //     &mut self,
    //     chain_id: u128,
    //     pool_id: String,
    //     swap_fee: u128,
    //     base_asset: AssetPointer,
    //     base_asset_index: usize,
    //     pool_assets: Vec<AssetPointer>,
    //     pool_assets_liquidity: Vec<u128>,
    //     a: u128,
    //     total_supply: u128,
    //     token_precisions: Vec<u128>,
    //     token_shares: Vec<u128>,
    //     token_rates: Vec<TokenRate>
    // ){

    //     let primary_node_asset = base_asset.clone();
    //     let primary_node_key = primary_node_asset.borrow().get_map_key();
    //     let mut adjacent_assets = vec![];
    //     for(i, asset) in pool_assets.iter().enumerate(){
    //         if i != base_asset_index{
    //             adjacent_assets.push(Rc::clone(asset));
    //         }
    //     }
    //     //Get bucket from base asset key. If none exists yet, create new one
    //     let table_bucket = self
    //         .table_2
    //         .entry(primary_node_key.clone())
    //         .or_insert(vec![AdjacencyList::new(&primary_node_asset)]);
        

    //     //Find base_asset adjacency list and add new asset
    //     let mut inserted = false;

    //     //Loop through lists, find one that corresponds to primary asset (share asset or one of pool assets)
    //     for adjacency_list in table_bucket{
    //         let adjacency_list_key = adjacency_list.primary_asset.borrow().get_map_key();
    //         if primary_node_key == adjacency_list_key{
    //             // let pair = AdjacencyGroup::new_stable_pair(pool_id, share_issuance, adjacent_assets, base_liquidity, base_token_precision, adjacent_liquidity, a, total_supply, adjacent_token_precisions);
                


    //             let pair = AdjacencyGroup{
    //                 group_type: GroupType::BncStable,
    //                 adjacent_asset: adjacent_assets,
    //                 liquidity: Some(Liquidity::BncStable(
    //                     BncStableData{
    //                         chain_id,
    //                         pool_id,
    //                         base_asset: primary_node_asset.clone(),
    //                         base_asset_index,
    //                         pool_assets: pool_assets.clone(),
    //                         pool_liquidity: pool_assets_liquidity.clone(),
    //                         token_precisions: token_precisions.clone(),
    //                         a: a,
    //                         total_supply: total_supply,
    //                         swap_fee: swap_fee,
    //                         token_rates,
    //                         token_shares
    //                     }
    //                 ))
    //             };
                
    //             adjacency_list.list.push(pair);
                
    //             inserted = true;
    //             break;
    //         }
    //     }

    //     //If base_asset has no corresponding adjacency list in the table, create one
    //     //This should only happen if there is a hashmap collision, i.e. 2 different asset keys hash to the same value.
    //     if !inserted{
    //         let new_adjacency_list = AdjacencyList::new(&primary_node_asset);
    //         self.table_2.entry(primary_node_key.clone()).and_modify(|e| e.push(new_adjacency_list));
    //     }
    // }



    pub fn get_adjacency_groups_for_asset(&self, input_asset: AssetPointer) -> Vec<AdjacencyGroup>{
        let list_key = input_asset.borrow().get_map_key();
        let table_bucket = self.table_2.get(&list_key);
        let mut adjacency_groups: Vec<AdjacencyGroup> = Vec::new();
        if let Some(bucket) = table_bucket{
            for list in bucket{
                if list.primary_asset.borrow().get_map_key() == list_key{
                    adjacency_groups = list.get_adjacency_groups();
                }
            }
        }
        adjacency_groups
    }

    pub fn display_table(&self){
        for (key, bucket) in &self.table_2{
            for list in bucket{
                // list.primary_asset.borrow().display_asset();
                print!("Base asset: ( "); list.primary_asset.borrow().display_asset(); print!(" ) - Adjacent assets: ");
                for adjacency_group in &list.list{
                    adjacency_group.display_group_assets();
                }
                println!("");
                println!("--------------------------------");
            }
            // println!("Primary Asset: {}", list.primary_asset.borrow().get_map_key());
            // for adjacency_group in &list.list{
            //     println!("Adjacent Asset: {}", adjacency_group.adjacent_asset.borrow().get_map_key());
            // }
        }
    }

}

impl AdjacencyList{
    pub fn new(primary_asset: &AssetPointer) -> AdjacencyList{
        AdjacencyList{
            primary_asset: Rc::clone(primary_asset),
            list: Vec::new()
        }
    }

    // pub fn add_dex_pair(&mut self, adjacent_assets: AssetPointer, dex_type: Option<String>, lp_id: Option<String>, base_liquidity: u128, adjacent_liquidity: u128){
    //     let pair = AdjacencyGroup::new_dex_pair(Rc::clone(&adjacent_assets),dex_type, lp_id, base_liquidity, adjacent_liquidity);
    //     self.list.push(pair);
    // }

    // pub fn add_dex_3_pair(&mut self, adjacent_assets: AssetPointer, dex_type: Option<String>, lp_id: Option<String>, token_0: String, token_1: String, active_liquidity: u128, fee_rate: u128, current_tick: i64, lower_ticks: Vec<TickData>, upper_ticks: Vec<TickData>){
    //     let pair = AdjacencyGroup::new_dex_3_pair(Rc::clone(&adjacent_assets), dex_type, lp_id, token_0, token_1, active_liquidity, fee_rate, current_tick, lower_ticks, upper_ticks);
    //     self.list.push(pair);
    // }

    // pub fn add_cex_pair(&mut self, adjacent_asset: AssetPointer, bid_price: u128, bid_decimals: u128, ask_price: u128, ask_decimals: u128){
    //     let pair = AdjacencyGroup::new_cex_pair(Rc::clone(&adjacent_asset), bid_price, bid_decimals, ask_price, ask_decimals);
    //     self.list.push(pair);
    // }

    // pub fn  add_stable_pair(&mut self, chain_id: u128, pool_id: Option<String>, swap_fee: u128, adjacent_assets: Vec<AssetPointer>, base_liquidity: u128, base_token_precision: u128, adjacent_liquidity: Vec<u128>, a: u128, total_supply: u128, adjacent_token_precisions: Vec<u128>, token_rates: Option<Vec<TokenRate>>, token_shares: Option<Vec<u128>>){
    //     let pair = AdjacencyGroup::new_stable_pair(chain_id, pool_id, swap_fee, adjacent_assets, base_liquidity, base_token_precision, adjacent_liquidity, a, total_supply, adjacent_token_precisions, token_rates, token_shares);
    //     self.list.push(pair);
    // }

    pub fn get_adjacency_groups(&self) -> Vec<AdjacencyGroup>{
        let mut adjacent_groups: Vec<AdjacencyGroup> = Vec::new();
        for pair in &self.list{
            adjacent_groups.push(pair.clone());
        }
        adjacent_groups
    }

    pub fn get_pool_assets(&self) -> Vec<AdjacencyGroup> {
        let mut adjacent_assets: Vec<AdjacencyGroup> = Vec::new();
        for pair in &self.list{
            
            // let pair_copy = AdjacencyGroup{
            //     adjacent_assets: Rc::clone(&pair.adjacent_asset),
            //     liquidity: pair.liquidity.clone()
            // };
            adjacent_assets.push(pair.clone());
        }
        adjacent_assets
    }

    pub fn check_share_asset(&self, share_asset: &AssetPointer, share_pool_id: String) -> bool{

        //Check each group/pair in list
        for adjacency_group in &self.list{
            match adjacency_group {
                AdjacencyGroup::StableShare(share_group) => {
                    let share_pool_lp = match &share_group.liquidity {
                        LiquidityPool::Stable(share_lp) => share_lp,
                        _ => panic!("Error: Expected StableShare liquidity type")
                    };
                    let pool_id = share_pool_lp.pool_id.clone().unwrap();
                    if Rc::ptr_eq(&share_pool_lp.share_asset.clone().unwrap(), share_asset) && share_pool_lp.pool_id.clone().unwrap() == share_pool_id{
                        return true;
                    }
                },
                _ => {}
            }

            // // Look for StableShare grouptype
            // match &pair.liquidity{
            //     Some(Liquidity::StableShare(share_lp)) => {

            //         // Check for share asset and if group is share -> token
            //         if Rc::ptr_eq(&share_lp.share_asset, share_asset) && share_lp.token_to_share.unwrap() == false{

            //             // Check that adjacent assets are all present. Should be enough to confirm stable share pair has been added
            //             if pair.adjacent_asset.len() == n_assets{
            //                 return true;
            //             }
            //         }
            //     },
            //     _ => {}
            // }
        }
        false
    }
}

// impl AdjacencyListReworked{
//     pub fn new(primary_asset: &AssetPointer) -> AdjacencyList{
//         AdjacencyList{
//             primary_asset: Rc::clone(primary_asset),
//             list: Vec::new()
//         }
//     }

//     // pub fn add_dex_pair(&mut self, adjacent_assets: AssetPointer, dex_type: Option<String>, lp_id: Option<String>, base_liquidity: u128, adjacent_liquidity: u128){
//     //     let pair = AdjacencyGroup::new_dex_pair(Rc::clone(&adjacent_assets),dex_type, lp_id, base_liquidity, adjacent_liquidity);
//     //     self.list.push(pair);
//     // }

//     // pub fn add_dex_3_pair(&mut self, adjacent_assets: AssetPointer, dex_type: Option<String>, lp_id: Option<String>, token_0: String, token_1: String, active_liquidity: u128, fee_rate: u128, current_tick: i64, lower_ticks: Vec<TickData>, upper_ticks: Vec<TickData>){
//     //     let pair = AdjacencyGroup::new_dex_3_pair(Rc::clone(&adjacent_assets), dex_type, lp_id, token_0, token_1, active_liquidity, fee_rate, current_tick, lower_ticks, upper_ticks);
//     //     self.list.push(pair);
//     // }

//     // pub fn add_cex_pair(&mut self, adjacent_asset: AssetPointer, bid_price: u128, bid_decimals: u128, ask_price: u128, ask_decimals: u128){
//     //     let pair = AdjacencyGroup::new_cex_pair(Rc::clone(&adjacent_asset), bid_price, bid_decimals, ask_price, ask_decimals);
//     //     self.list.push(pair);
//     // }

//     // pub fn  add_stable_pair(&mut self, chain_id: u128, pool_id: Option<String>, swap_fee: u128, adjacent_assets: Vec<AssetPointer>, base_liquidity: u128, base_token_precision: u128, adjacent_liquidity: Vec<u128>, a: u128, total_supply: u128, adjacent_token_precisions: Vec<u128>, token_rates: Option<Vec<TokenRate>>, token_shares: Option<Vec<u128>>){
//     //     let pair = AdjacencyGroup::new_stable_pair(chain_id, pool_id, swap_fee, adjacent_assets, base_liquidity, base_token_precision, adjacent_liquidity, a, total_supply, adjacent_token_precisions, token_rates, token_shares);
//     //     self.list.push(pair);
//     // }

//     // pub fn get_adjacent_assets(&self) -> Vec<AdjacencyGroup>{
//     //     let mut adjacent_assets: Vec<AdjacencyGroup> = Vec::new();
//     //     for pair in &self.list{
//     //         // let pair_copy = AdjacencyGroup{
//     //         //     adjacent_assets: Rc::clone(&pair.adjacent_asset),
//     //         //     liquidity: pair.liquidity.clone()
//     //         // };
//     //         adjacent_assets.push(pair.clone());
//     //     }
//     //     adjacent_assets
//     // }

//     // pub fn check_share_asset(&self, share_asset: &AssetPointer, n_assets: usize) -> bool{

//     //     //Check each group/pair in list
//     //     for pair in &self.list{

//     //         // Look for StableShare grouptype
//     //         match &pair.liquidity{
//     //             Some(Liquidity::StableShare(share_lp)) => {

//     //                 // Check for share asset and if group is share -> token
//     //                 if Rc::ptr_eq(&share_lp.share_asset, share_asset) && share_lp.token_to_share.unwrap() == false{

//     //                     // Check that adjacent assets are all present. Should be enough to confirm stable share pair has been added
//     //                     if pair.adjacent_asset.len() == n_assets{
//     //                         return true;
//     //                     }
//     //                 }
//     //             },
//     //             _ => {}
//     //         }
//     //     }
//     //     false
//     // }
// }

impl AdjacencyGroup{

    pub fn display_group_assets(&self){
        let mut pool_assets: Vec<AssetPointer> = vec![];
        match &self{
            // AdjacencyGroup::Dex(dex_group) => {
            //     print!("(D)"); dex_group.adjacent_asset.borrow().display_asset(); print!(" | "); 
            // },
            // AdjacencyGroup::Cex(cex_group) => {
            //     print!("(C)"); cex_group.adjacent_asset.borrow().display_asset(); print!(" | "); 
            // },
            // AdjacencyGroup::Stable(stable_group) => {
            //     print!("(S)"); stable_group.adjacent_asset.borrow().display_asset(); print!(" | ");
            // },
            // AdjacencyGroup::StableShare(share_group) => {
            //     print!("(S)"); share_group.adjacent_asset.borrow().display_asset(); print!(" | ");
            // },
            // AdjacencyGroup::DexV3(dex_v3_group) => {
            //     print!("(D3)"); dex_v3_group.adjacent_asset.borrow().display_asset(); print!(" | ");
            // },
            // AdjacencyGroup::BncStable(bnc_stable_group) => {
            //     print!("(BNC)"); bnc_stable_group.adjacent_asset.borrow().display_asset(); print!(" | ");
            // }
            // GroupType::Xcm => adjacency_group.adjacent_asset.[0]borrow().display_asset(),
            // _ => {}
            AdjacencyGroup::Dex(dex_group) => {
                print!("(Dex)"); 
                pool_assets = dex_group.pool_assets.clone();
            },
            AdjacencyGroup::DexV3(dex_v3_group) => {
                print!("(DV3): ( ");
                pool_assets = dex_v3_group.pool_assets.clone();
            },
            AdjacencyGroup::Stable(stable_group) => {
                print!("(Stable): ( ");
                pool_assets = stable_group.pool_assets.clone();
            },
            AdjacencyGroup::StableShare(share_group) => {
                let lp_data = match &share_group.liquidity{
                    LiquidityPool::Stable(lp_data) => lp_data,
                    _ => panic!("Error: Expected StableShare liquidity type")
                };
                let share_asset = lp_data.share_asset.clone().unwrap();
                if share_group.token_to_share{
                    print!("(S Share: Token -> Share) ");
                    // share_asset.borrow().display_asset();
                    pool_assets.push(Rc::clone(&share_asset));
                } else {
                    print!("(S Share: Share -> Token) ");
                    // share_asset.borrow().display_asset();
                    
                    pool_assets = share_group.pool_assets.clone();
                }
            },
            AdjacencyGroup::BncStable(bnc_stable_group) => {
                print!("(BNC Stable): (");
                pool_assets = bnc_stable_group.pool_assets.clone();
            },
            _ => {}
        }
        pool_assets.iter().for_each(|asset| {
            asset.borrow().display_asset();
            print!(" | ");
        });
        println!("");
    }
    // pub fn new_dex_pair(adjacent_asset: AssetPointer, dex_type: Option<String>, lp_id: Option<String>, base_liquidity: u128, adjacent_liquidity: u128) -> AdjacencyGroup{
    //     AdjacencyGroup{
    //         group_type: GroupType::Dex,
    //         adjacent_asset: vec![Rc::clone(&adjacent_asset)],
    //         liquidity: Some(Liquidity::Dex(DexData{
    //             dex_type,
    //             lp_id: lp_id,
    //             base_liquidity: base_liquidity,
    //             adjacent_liquidity: adjacent_liquidity
    //         }))
    //     }
    // }

    // pub fn new_dex_3_pair(adjacent_asset: AssetPointer, dex_type: Option<String>, lp_id: Option<String>, token_0: String, token_1: String, active_liquidity: u128, fee_rate: u128, current_tick: i64, lower_ticks: Vec<TickData>, upper_ticks: Vec<TickData>) -> AdjacencyGroup{
    //     AdjacencyGroup{
    //         group_type: GroupType::DexV3,
    //         adjacent_asset: vec![Rc::clone(&adjacent_asset)],
    //         liquidity: Some(Liquidity::DexV3(DexV3Data{
    //             dex_type: dex_type,
    //             lp_id,
    //             token_0: token_0,
    //             token_1: token_1,
    //             current_tick: current_tick,
    //             active_liquidity: active_liquidity,
    //             fee_rate: fee_rate,
    //             lower_ticks: lower_ticks,
    //             upper_ticks: upper_ticks
    //         }))
    //     }
    // }

    // pub fn new_cex_pair(adjacent_asset: AssetPointer, bid_price: u128, bid_decimals: u128, ask_price: u128, ask_decimals: u128) -> AdjacencyGroup{
    //     AdjacencyGroup{
    //         group_type: GroupType::Cex,
    //         adjacent_asset: vec![Rc::clone(&adjacent_asset)],
    //         liquidity: Some(Liquidity::Cex(CexData{
    //             bid_price: bid_price, bid_decimals: bid_decimals,
    //             ask_price: ask_price, ask_decimals: ask_decimals
    //         }))
    //     }
    // }

    // pub fn new_stable_pair(chain_id: u128, pool_id: Option<String>, swap_fee: u128, adjacent_assets: Vec<AssetPointer>, base_liquidity: u128, base_token_precision: u128, adjacent_liquidity: Vec<u128>, a: u128, total_supply: u128, adjacent_token_precisions: Vec<u128>, token_rates: Option<Vec<TokenRate>>, token_shares: Option<Vec<u128>>) -> AdjacencyGroup{
    //     // let num: u32 = 46874416394179827106165590000000000000;
        
    //     AdjacencyGroup{
    //         group_type: GroupType::Stable,
    //         adjacent_asset: adjacent_assets,
    //         liquidity: Some(Liquidity::Stable(
    //             StableData{
    //                 chain_id: chain_id,
    //                 token_to_share: None,
    //                 swap_fee: swap_fee,
    //                 share_issuance: None,
    //                 pool_id,
    //                 base_liquidity: base_liquidity,
    //                 base_token_precision: base_token_precision,
    //                 adjacent_liquidity: adjacent_liquidity,
    //                 a: a,
    //                 total_supply: total_supply,
    //                 adjacent_token_precisions: adjacent_token_precisions,
    //                 token_shares: token_shares,
    //                 token_rates: token_rates
    //             }
    //         ))
    //     }
    // }


    // pub fn new_xcm_pair(adjacent_asset: AssetPointer) -> AdjacencyGroup{
    //     AdjacencyGroup{
    //         group_type: GroupType::Xcm,
    //         adjacent_asset: vec![Rc::clone(&adjacent_asset)],
    //         liquidity: None
    //     }
    // }

    // pub fn display_dex_pair(&self){
    //     print!("DexPair: ( ");
    //     for asset in &self.adjacent_asset{
    //         asset.borrow().display_asset();
    //         print!(" | ");
    //     }
    //     println!(" )");
    //     println!("");

    // }

    // pub fn display_stable_adjacenct(&self) {
    //     print!("StablePool: ( ");
    //     for asset in &self.adjacent_asset{
    //         asset.borrow().display_asset();
    //         print!(" | ");
    //     }
    //     print!(" )");
    //     // println!("");
    // }
}