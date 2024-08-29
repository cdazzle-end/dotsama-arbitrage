// use crate::asset_registry::AssetLocation;
// use crate::token::{self, TokenData};
// use crate::{LiqPoolRegistry, asset_registry, liq_pool_registry};
// use crate::liq_pool_registry_2::LiqPoolRegistry2;
use serde_json::Value;
use std::fs;
use num::{BigInt, BigUint, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, FromPrimitive, Num, One, Signed, ToPrimitive, Zero};
use num::bigint::{ToBigInt, ToBigUint};
use num::BigRational;
use num;
use bigdecimal::{BigDecimal};
// use std::borrow::{Borrow, BorrowMut};
// use num::BigRational::
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::{path::Path, fs::File, io::Read};
use serde::{Deserialize, Deserializer, Serialize};
use std::ops::{Add, Mul};
use std::rc::Rc;
use std::str::FromStr;
use std::vec;
use crate::liq_pool_registry_2::{LiquidityPool, TickData, TokenRate, BncStableData, CexData, DexData, DexV3Data, StableData};
// use crate::{asset_registry::{AssetRegistry, Asset}};
use crate::AssetRegistry2;
use crate::asset_registry_2::{Asset, AssetLocation, TokenData};
use crate::adjacency_table_2::{AdjacencyGroup, AdjacencyTable2,  GroupType, };
use crate::fee_book::{TransferDepositFeeBook, XcmFeeData};

use std::default::Default;
use std::hash::{Hasher, Hash};
use std::str;
use std::io;
use serde::de::{Error, Visitor};
type AssetPointer = Rc<RefCell<Asset>>;
type GraphNodePointer = Rc<RefCell<GraphNode>>;

const MIN_TICK: i64 = -887272;
const MAX_TICK: i64 = 887272;
const MIN_TICK_DATA: TickData = TickData{tick: MIN_TICK, liquidity_delta: 0};
const MAX_TICK_DATA: TickData = TickData{tick: MAX_TICK, liquidity_delta: 0};
const SHARE_PRECISION: u64 = 18;
const MAX_Y_ITERATIONS_HDX: u64 = 128;
const MAX_Y_ITERATIONS_ACA: u64 = 255;
const MAX_Y_ITERATIONS_BNC: u64 = 255;
const MAX_D_ITERATIONS_HDX: u64 = 64;
const MAX_D_ITERATIONS_ACA: u64 = 255;



pub struct TokenGraph2{
    pub node_map: HashMap<String, Vec<GraphNodePointer>>,
    pub asset_registry: AssetRegistry2,
    pub fee_book: TransferDepositFeeBook,
}
impl TokenGraph2{

    pub fn build_graph_2(asset_registry: AssetRegistry2, adjacency_table: AdjacencyTable2) -> TokenGraph2{
        let graph_nodes: Vec<GraphNodePointer> = create_graph_nodes(&asset_registry);
        let node_map: HashMap<String, Vec<GraphNodePointer>> = create_node_map(&graph_nodes);

        //For each node, get adjacent assets. Look up nodes for those assets. Add these nodes to the current node's adjacency list
        for current_node in graph_nodes{
            add_adjacent_assets_2(Rc::clone(&current_node), &node_map, &adjacency_table);
            add_cross_chain_assets_2(current_node, &node_map, &asset_registry);
        }
        let xcm_transfer_and_deposit_fee_book = format!("./../../../xcm-test/data/newEventFeeBook.json");
        let xcm_transfer_and_deposit_fees = fs::read_to_string(xcm_transfer_and_deposit_fee_book.clone()).unwrap();
        let fee_book: TransferDepositFeeBook = serde_json::from_str(&xcm_transfer_and_deposit_fees).unwrap();

        TokenGraph2{ node_map, asset_registry, fee_book }
    }

    //Get node from asset key
    pub fn get_node(&self, asset_key: String) -> GraphNodePointer{
        let bucket = self.node_map.get(&asset_key)
            .expect(&format!("Failed to find bucket for asset_key: {:?}", asset_key));
        for node in bucket{
            if node.borrow().asset_key == asset_key{
                return Rc::clone(node);
            }
        }
        panic!("Could not find node with asset key: {}", asset_key);
    }
    pub fn get_asset_origin_node(&self, node: GraphNodePointer) -> Option<GraphNodePointer>{
        let node_chain_id = node.borrow().get_chain_id();
        let node_origin_chain_id = node.borrow().get_origin_chain_id();
        if node_chain_id == node_origin_chain_id{
            return Some(node.clone())
        }
        let asset_location = node.borrow().get_asset_location().unwrap();
        let all_xcm_assets = &self.asset_registry.get_assets_at_location(asset_location);
        for xcm_asset in all_xcm_assets{
            if xcm_asset.borrow().get_chain_id().unwrap() == node_origin_chain_id{
                let origin_xcm_node = self.get_node(xcm_asset.borrow().get_map_key()); 
                return Some(origin_xcm_node);
            }
        }
        panic!("Cant find origin asset node for {}", node.borrow().get_asset_key());
    }

    pub fn display_all_nodes(&self){
        let mut node_strings = vec![];
        for (key, buckets) in &self.node_map{
            for node in buckets{
                node_strings.push(node.borrow().get_asset_key())
            }
        }
        node_strings.sort();
        for node in node_strings{
            println!("{}", node);
        }
    }


    //Display graph with stable pools added
    pub fn display_graph_3(&self){

        let mut sorted_keys: Vec<&String> = self.node_map.keys().collect();

        sorted_keys.sort();

        for key in sorted_keys{
            let buckets = self.node_map.get(key).unwrap();
            for node in buckets{
                print!("Node: {} {} -> ", node.borrow().asset_key, node.borrow().get_asset_symbol());
                for node_pool in &node.borrow().adjacent_node_pools{
                    match node_pool {
                        NodePool::Stable(node_pool) => {
                            print!("(S)");
                            for (index, adjacent_node) in node_pool.pool_nodes.iter().enumerate(){
                                if index == node_pool.base_asset_index{
                                    continue;
                                }
                                print!("{} {}", adjacent_node.borrow().asset_key, adjacent_node.borrow().get_asset_symbol());
                                print!(" | ");
                            }
                        },
                        NodePool::StableShare(node_pool) => {
                            print!("(S Share) ");
                            if node_pool.token_to_share{
                                print!("Token -> Share ");
                                let share_asset = node_pool.share_asset_node.clone();
                                // share_asset.borrow().display_asset();
                                print!("{} {}", share_asset.borrow().asset_key, share_asset.borrow().get_asset_symbol());
                                print!(" | ");
                            } else {
                                print!("Share -> Token ");
                                for (index, adjacent_node) in node_pool.pool_nodes.iter().enumerate(){
                                    print!("{} {}", adjacent_node.borrow().asset_key, adjacent_node.borrow().get_asset_symbol());
                                    print!(" | ");
                                }
                            }
                            
                            // print!(") ");
                            // print!(" | ");
                        },
                        NodePool::BncStable(node_pool) => {
                            print!("(BncS)");
                            for (index, adjacent_node) in node_pool.pool_nodes.iter().enumerate(){
                                if index == node_pool.base_asset_index{
                                    continue;
                                }
                                print!("{} {}", adjacent_node.borrow().asset_key, adjacent_node.borrow().get_asset_symbol());
                                print!(" | ");
                            }
                            // print!(") ");
                            // print!(" | ");
                        },
                        NodePool::Cex(node_pool) => {
                            print!("(C)");
                            node_pool.pool_nodes.borrow().display_asset();
                            // print!(") ");
                            print!(" | ");
                        },
                        NodePool::Dex(node_pool) => {
                            print!("(D2)");
                            for (index, adjacent_node) in node_pool.pool_nodes.iter().enumerate(){
                                if index == node_pool.base_asset_index{
                                    continue;
                                }
                                print!("{} {}", adjacent_node.borrow().asset_key, adjacent_node.borrow().get_asset_symbol());
                                print!(" | ");
                            }
                            // print!(") ");
                        },
                        NodePool::DexV3(node_pool) => {
                            print!("(D3)");
                            for (index, adjacent_node) in node_pool.pool_nodes.iter().enumerate(){
                                if index == node_pool.base_asset_index{
                                    continue;
                                }
                                print!("{} {}", adjacent_node.borrow().asset_key, adjacent_node.borrow().get_asset_symbol());
                                print!(" | ");
                            }
                            
                            // print!(") ");
                        },
                        NodePool::Xcm(node_pool) => {
                            print!("(X)");
                            node_pool.xcm_node.borrow().display_asset();
                            let adjacent_node = node_pool.xcm_node.clone();
                            print!("{} {}", adjacent_node.borrow().asset_key, adjacent_node.borrow().get_asset_symbol());
                            print!(" | ");
                        },
                        _ => {}
                    }
                    // adj_node_2.adjacent_nodes.borrow().asset.borrow().display_asset();
                    // print!(" | ");
                }
                println!("");
            }
        }

    
    }

    // Make sure fee keys translate to node keys, visa versa
    pub fn test_all_fees(&self){

        for (key, buckets) in &self.node_map{
            for node in buckets{
                // println!("************************************");
                // println!("Getting fee for node: {}", node.borrow().asset_key);
                self.fee_book.get_transfer_fee_data(node.clone());

            }
        }
        for (key, buckets) in &self.node_map{
            for node in buckets{
                // println!("************************************");
                // println!("Getting fee for node: {}", node.borrow().asset_key);
                self.fee_book.get_deposit_fee_data(node.clone());

            }
        }

        let transfer_fees = self.fee_book.get_all_transfer_fee_data();
        for (chain, chain_transfer_data) in transfer_fees{
            let chain_id: u64 = chain.parse().unwrap();
            print!("Chain: {} | ", chain_id);
            for(asset_key, asset_data) in chain_transfer_data.assets.iter(){
                println!("Asset: {}", asset_key);
                let node = self.get_asset_by_chain_and_id(chain_id, asset_key.clone());
                match node {
                    Some(node) => node.borrow().display_asset(),
                    None => panic!("Cant find node for transfer fee key")
                }
            }
            // println!("Transfer Fees: {:?}", chain_transfer_data);

        }

        let deposit_fees = self.fee_book.get_all_deposit_fee_data();
        for(chain, chain_deposit_data) in deposit_fees{
            let chain_id: u64 = chain.parse().unwrap();
            print!("Chain: {} | ", chain_id);
            for(asset_key, asset_data) in chain_deposit_data.assets.iter(){
                println!("Asset: {}", asset_key);
                let node = self.get_asset_by_chain_and_id(chain_id, asset_key.clone());
                match node {
                    Some(node) => node.borrow().display_asset(),
                    None => panic!("Cant find node for deposit fee key")
                }
            }
        }

    }



    pub fn display_stable_share_pairs(&self){
        for (key, buckets) in &self.node_map{
            for current_node in buckets{
                
                for adjacent_node_pair in &current_node.borrow().adjacent_node_pools{
                    match adjacent_node_pair {
                        NodePool::StableShare(pair) => {
                            print!("Node: {} -> ", current_node.borrow().asset_key);
                            print!("(Stable Share) ");
                            
                            for node in &pair.pool_nodes{
                                node.borrow().asset.borrow().display_asset();
                                print!(" | ");
                            }
                            println!("");
                            println!("------------------------------------");
                        },
                        _ => {}
                    }
                }
                println!("");
            }

            // println!("------------------------------------")
        }
    }

    pub fn display_bnc_stable_pairs(&self){
        for (key, buckets) in &self.node_map{
            for current_node in buckets{
                
                for node_pool in &current_node.borrow().adjacent_node_pools{
                    match node_pool {
                        NodePool::BncStable(node_pool) => {
                            print!("Node: {} {} -> ", current_node.borrow().asset_key, current_node.borrow().get_asset_symbol());
                            print!("(Bnc Stable Pair) ");
                            
                            for node in &node_pool.pool_nodes{
                                print!("{} {} ", node.borrow().asset.borrow().get_map_key(), node.borrow().asset.borrow().get_asset_symbol());
                                print!(" | ");
                            }
                            println!("");
                            println!("------------------------------------");
                        },
                        _ => {}
                    }
                }
            }

            // println!("------------------------------------")
        }
    }

    // Cannot travel to node if it already exists in current path



    pub fn calculate_v3_swap(&self, asset_key_1: String, asset_key_2: String, contract_address: String, input_amount: f64){
        let base_node: &GraphNodePointer = &self.get_asset_by_chain_and_symbol(2004, "GLMR".to_string()).unwrap();
        let adjacent_node: &GraphNodePointer = &self.get_asset_by_chain_and_symbol(2004, "XCACA".to_string()).unwrap();
        let base_node_decimals = base_node.borrow().get_asset_decimals();
        let adjacent_node_decimals = adjacent_node.borrow().get_asset_decimals();
        
        let formatted_input = &input_amount * f64::powi(10.0, base_node_decimals as i32);
        let input_amount = formatted_input.to_bigint().unwrap();

        let adjacent_key = adjacent_node.borrow().get_asset_key();

        let lp_stats = base_node.borrow().get_v3_lp_stats_from_pair(adjacent_key.clone(), contract_address.clone());
        let adjacent_pair = base_node.borrow().get_v3_adjacent_node_pair(adjacent_key.clone(), contract_address);

        if let Some(pair) = adjacent_pair{
            let output_amount = calculate_dex_edge(&pair, input_amount);
            let output_bigint = BigRational::new(BigInt::from(output_amount), BigInt::one());
            let output_formatted = output_bigint / (BigInt::from(10).pow(adjacent_node_decimals as u32));
            let output_formatted = output_formatted.to_f64().unwrap();
            println!("Output amount: {}", output_formatted);
        } else {
            println!("No pair found");
        }

    }

    pub fn calculate_bnc_stable_swap(&self, asset_key_1: String, asset_key_2: String, chain_id: u64, pool_id: String, input_amount: f64){

        // let asset_key_1_string = serde_json::to_string(&asset_key_1).unwrap();
        // let asset_key_2_string = serde_json::to_string(&asset_key_2).unwrap();
        let asset_key_1_string = asset_key_1;
        let asset_key_2_string = asset_key_2;

        let base_node: &GraphNodePointer = &self.get_asset_by_chain_and_id(chain_id, asset_key_1_string).unwrap();
        let adjacent_node: &GraphNodePointer = &self.get_asset_by_chain_and_id(chain_id, asset_key_2_string).unwrap();

        // let base_node: &GraphNodePointer = &self.get_asset_by_chain_and_symbol(chain_id, asset_key_1_string).unwrap();
        // let adjacent_node: &GraphNodePointer = &self.get_asset_by_chain_and_symbol(chain_id, asset_key_2_string).unwrap();
        let base_node_decimals = base_node.borrow().get_asset_decimals();
        let adjacent_node_decimals = adjacent_node.borrow().get_asset_decimals();

        let formatted_input = &input_amount * f64::powi(10.0, base_node_decimals as i32);
        let input_amount_big = formatted_input.to_bigint().unwrap();

        let adjacent_key = adjacent_node.borrow().get_asset_key();

        // let lp_stats = base_node.borrow().get_stable_share_lp_stats_from_pair(adjacent_key.clone(), pool_id.clone());
        let adjacent_pair = base_node.borrow().get_stable_adjacent_node_pair(adjacent_key.clone(), pool_id);
        
        // println!("{:?}", adjacent_pair);

        // let adjacent_pair = base_node.borrow().get_stable_share_adjacent_node_pair(adjacent_key.clone(), pool_id);
        let (adjecent_node_pool, index) = adjacent_pair.unwrap();

        // if let Some(pair, index) = adjacent_pair.unwrap(){
        println!("Calculating swap");
        let output_amount = calculate_stable_edge(base_node, &adjecent_node_pool, input_amount_big, index).unwrap();
        let output_bigint = BigRational::new(output_amount, BigInt::one());
        let output_formatted = output_bigint / (BigInt::from(10).pow(adjacent_node_decimals as u32));
        let output_formatted = output_formatted.to_f64().unwrap();
        println!("Input amount: {} | Output amount: {}", input_amount, output_formatted);
        // println!("Input amount: {} | Output amount: {}", input_amount, output_formatted);
        //     println!("Output amount: {}", output_formatted);
        // } else {
        //     println!("No pair found");
        // } 
    }

    pub fn calculate_aca_stable_swap(&self, asset_key_1: String, asset_key_2: String, chain_id: u64, pool_id: String, input_amount: f64){
        let asset_key_1_string = asset_key_1;
        let asset_key_2_string = asset_key_2;

        // let base_node: &GraphNodePointer = &self.get_asset_by_chain_and_id(chain_id, asset_key_1_string).unwrap();
        // let adjacent_node: &GraphNodePointer = &self.get_asset_by_chain_and_id(chain_id, asset_key_2_string).unwrap();

        let base_node: &GraphNodePointer = &self.get_asset_by_chain_and_symbol(chain_id, asset_key_1_string).unwrap();
        let adjacent_node: &GraphNodePointer = &self.get_asset_by_chain_and_symbol(chain_id, asset_key_2_string).unwrap();
        let base_node_decimals = base_node.borrow().get_asset_decimals();
        let adjacent_node_decimals = adjacent_node.borrow().get_asset_decimals();

        let formatted_input = &input_amount * f64::powi(10.0, base_node_decimals as i32);
        let input_amount_big = formatted_input.to_bigint().unwrap();

        let adjacent_key = adjacent_node.borrow().get_asset_key();

        // let lp_stats = base_node.borrow().get_stable_share_lp_stats_from_pair(adjacent_key.clone(), pool_id.clone());
        let (node_pool, target_index) = base_node.borrow().get_stable_adjacent_node_pair(adjacent_key.clone(), pool_id).unwrap();
        


        if let NodePool::Stable(node_pool) = node_pool.clone(){
            let pool_data = match &node_pool.liquidity{
                LiquidityPool::Stable(data) => data,
                _ => panic!("Invalid pool data")
            };

            for (index, node) in node_pool.pool_nodes.iter().enumerate(){
                node.borrow().display_asset(); println!(" | {}", pool_data.pool_liquidity[index]);
            }
        }


        // println!("{:?}", adjacent_pair);

        // let adjacent_pair = base_node.borrow().get_stable_share_adjacent_node_pair(adjacent_key.clone(), pool_id);
        // let (adjecent_node_pool, index) = node_pool.unwrap();

        // if let Some(pair, index) = adjacent_pair.unwrap(){
        println!("Calculating swap");
        let output_amount = calculate_stable_edge(base_node, &node_pool, input_amount_big, target_index).unwrap();
        let output_bigint = BigRational::new(output_amount, BigInt::one());
        let output_formatted = output_bigint / (BigInt::from(10).pow(adjacent_node_decimals as u32));
        let output_formatted = output_formatted.to_f64().unwrap();
        println!("Input amount: {} | Output amount: {}", input_amount, output_formatted);
    }

    pub fn calculate_stable_swap(&self, asset_key_1: String, asset_key_2: String, chain_id: u64, pool_id: String, input_amount: f64){

        // let asset_key_1_string = serde_json::to_string(&asset_key_1).unwrap();
        // let asset_key_2_string = serde_json::to_string(&asset_key_2).unwrap();
        let asset_key_1_string = asset_key_1;
        let asset_key_2_string = asset_key_2;

        // let base_node: &GraphNodePointer = &self.get_asset_by_chain_and_id(chain_id, asset_key_1_string).unwrap();
        // let adjacent_node: &GraphNodePointer = &self.get_asset_by_chain_and_id(chain_id, asset_key_2_string).unwrap();

        let base_node: &GraphNodePointer = &self.get_asset_by_chain_and_symbol(chain_id, asset_key_1_string).unwrap();
        let adjacent_node: &GraphNodePointer = &self.get_asset_by_chain_and_symbol(chain_id, asset_key_2_string).unwrap();
        let base_node_decimals = base_node.borrow().get_asset_decimals();
        let adjacent_node_decimals = adjacent_node.borrow().get_asset_decimals();

        let formatted_input = &input_amount * f64::powi(10.0, base_node_decimals as i32);
        let input_amount_big = formatted_input.to_bigint().unwrap();

        let adjacent_key = adjacent_node.borrow().get_asset_key();

        // let lp_stats = base_node.borrow().get_stable_share_lp_stats_from_pair(adjacent_key.clone(), pool_id.clone());
        let adjacent_pair = base_node.borrow().get_stable_adjacent_node_pair(adjacent_key.clone(), pool_id);
        
        // let adjacent_pair = base_node.borrow().get_stable_share_adjacent_node_pair(adjacent_key.clone(), pool_id);
        let (adjecent_node_pool, index) = adjacent_pair.unwrap();

        // if let Some(pair, index) = adjacent_pair.unwrap(){
        let output_amount = calculate_stable_edge(base_node, &adjecent_node_pool, input_amount_big, index).unwrap();
        let output_bigint = BigRational::new(output_amount, BigInt::one());
        let output_formatted = output_bigint / (BigInt::from(10).pow(adjacent_node_decimals as u32));
        let output_formatted = output_formatted.to_f64().unwrap();

        // println!("Input amount: {} | Output amount: {}", input_amount, output_formatted);
        //     println!("Output amount: {}", output_formatted);
        // } else {
        //     println!("No pair found");
        // } 
    }

    pub fn convert_transfer_fee_amount_to_current_node(&self, fee_node: GraphNodePointer, current_node: GraphNodePointer, fee_amount: BigInt) -> BigInt{
        // println!("Converting Fee Node: {} to Current Node: {} | Input amount: {}", fee_node.borrow().get_asset_key_and_symbol(),  current_node.borrow().get_asset_key_and_symbol(), fee_amount);
        let mut conversion_amount = self.find_immediate_edge_between_nodes(fee_node.clone(), current_node.clone(), fee_amount.clone());
        if conversion_amount.is_some(){
            if conversion_amount.clone().unwrap().is_zero(){
                return BigInt::one();
            } else {
                return conversion_amount.unwrap();
            };
        } else {
            conversion_amount = self.find_edge_between_nodes(fee_node.clone(), current_node.clone(), fee_amount.clone());
            if conversion_amount.is_some(){
                if conversion_amount.clone().unwrap().is_zero(){
                    return BigInt::one();
                } else {
                    return conversion_amount.unwrap();
                };
            } else {
                conversion_amount = self.find_path_between_nodes_on_chain(fee_node.clone(), current_node.clone(), fee_amount);
                if conversion_amount.is_some(){
                    return conversion_amount.unwrap();
                } else {
                    panic!("No edge found between nodes. Fee node: {} | Current node: {}", fee_node.borrow().get_asset_key_and_symbol(), current_node.borrow().get_asset_key_and_symbol());
                }
            }
        }

        // If no edge on current chain exists, find  one

    }

    pub fn find_best_route(&self, asset_key_1: String, asset_key_2: String, input_amount: BigDecimal) -> (String, Vec<Rc<RefCell<GraphNode>>>) {
        // println!("STARTING INPUT AMOUNT: {}", input_amount);
        let starting_node = &self.get_node(asset_key_1.clone()).clone();
        let relay = starting_node.borrow().get_relay_chain();
        // let formatted_input = &input_amount * f64::powi(10.0, starting_node.borrow().get_asset_decimals() as i32);
        // let decimal_place_multiplier = 
        let formatted_input = &input_amount * BigDecimal::from_f64(f64::powi(10.0, starting_node.borrow().get_asset_decimals() as i32)).unwrap();
        // println!("Formatted input: {}", formatted_input);
        starting_node.borrow_mut().best_path_value = formatted_input.to_bigint().unwrap();
        // println!("Starting node best path value: {}", starting_node.borrow().best_path_value);
        starting_node.borrow_mut().path_values.push(input_amount);

        let path_data: PathData = PathData{
            path_type: "Start".to_string(),
            lp_id: None,
            ..Default::default()
        };

        starting_node.borrow_mut().path_value_types.push(PathType::Xcm);
        starting_node.borrow_mut().path_datas.push(path_data);
        starting_node.borrow_mut().best_path.push(Rc::clone(&starting_node));

        let destination_node = &self.get_node(asset_key_2).clone();
        let destination_asset_location = destination_node.borrow().get_asset_location().unwrap();
        let all_destination_assets = &self.asset_registry.get_assets_at_location(destination_asset_location);
        let mut destination_nodes = vec![];

        // Just input destination node ************
        // destination_nodes.push(Rc::clone(destination_node));

        // All Destination nodes ***************
        for dest_asset in all_destination_assets{
            if(!dest_asset.borrow().is_cex_token()){
                let dest_node = &self.get_node(dest_asset.borrow().get_map_key()).clone();
                destination_nodes.push(Rc::clone(&dest_node));
            }
        }


        let mut node_queue = VecDeque::new();
        node_queue.push_back(Rc::clone(starting_node));

        while !node_queue.is_empty() {
            let current_node = node_queue.pop_front().unwrap();
            for node_pool in &current_node.borrow().adjacent_node_pools{
                match node_pool {
                    NodePool::Xcm(adjacent_pair) => {
                        let xcm_node = adjacent_pair.xcm_node.clone();
                        let mut xcm_input_amount = current_node.borrow().best_path_value.clone();
                        
                        // println!("****************************************");
                        // println!("Xcm transferring {} -> {}", current_node.borrow().asset_key, adjacent_pair.xcm_node.borrow().asset_key);
                        // Ignore when adjacent node is the current node
                        if current_node.as_ptr().eq(&xcm_node.as_ptr()){
                            // println!("SKIP");
                            continue;
                        }

                        // ******** Transfer fee data ********
                        // Transfer Data: (fee) When fee asset is native, subtract fee from output amount
                        // Transfer Data: (reserve) subtract as fee, but also log
                        let transfer_fee_data = &self.fee_book.get_transfer_fee_data(current_node.clone());
                        let mut start_node_transfer_reserve_amount = BigInt::zero();
                        let mut start_node_transfer_fee_amount = BigInt::zero();
                        // let mut fee_amount_to_subtract = BigInt::zero();

                        // If transfer fee exists in fee book
                        if let Some(fee_data) = transfer_fee_data {
                            let fee_node = &self.get_asset_by_chain_and_id(current_node.borrow().get_chain_id(), fee_data.get_fee_asset_id()).unwrap();
                            start_node_transfer_fee_amount = BigInt::from_str(fee_data.clone().feeAmount.unwrap().as_str()).unwrap();

                            // println!("Fee node: {} | Current node: {}", fee_node.borrow().asset_key, current_node.borrow().asset_key);
                            if fee_node.as_ptr().eq(&current_node.as_ptr()){
                                // start_node_fee_amount = fee_amount.clone();
                                xcm_input_amount = xcm_input_amount.clone() - start_node_transfer_fee_amount.clone();
                            } else {
                            // transfer asset != fee asset
                                start_node_transfer_reserve_amount = self.convert_transfer_fee_amount_to_current_node(fee_node.clone(), current_node.clone(), start_node_transfer_fee_amount.clone());
                                xcm_input_amount = xcm_input_amount.clone() - start_node_transfer_reserve_amount.clone();
                            }
                        }
                        // println!("1.(T) fee: {} | reserve: {}", fee_amount_to_subtract, start_node_reserve_amount);

                        let asset_origin_node = self.get_asset_origin_node(current_node.clone()).unwrap();
                        let (
                            mut xcm_output_amount, 
                            middle_node_transfer_reserve_amount, 
                            middle_node_transfer_fee_amount, 
                            middle_node_deposit_reserve_amount, 
                            middle_node_deposit_fee_amount
                         ) = calculate_origin_xcm_edge(
                            &self,
                            &self.fee_book, 
                            current_node.clone(), 
                            asset_origin_node.clone(), 
                            adjacent_pair, 
                            xcm_input_amount.clone()
                        );

                        // 

                        // ************** Deposit fee data **************
                        // Get deposit fee data and subtract it from total output
                        // FUCKen unless deposit fee is different asset, ex Transferring multiassets from hydra -> asset hub
                        // Then needs to be treated similar to transfer fee data when fee is different
                        // let deposit_fee_data = &self.fee_book.get_deposit_fee_data(adjacent_pair.xcm_node.clone());
                        // let mut deposit_fee_amount = BigInt::zero();
                        // if let Some(deposit_fee_data) = deposit_fee_data{
                        //     deposit_fee_amount = BigInt::from_str(deposit_fee_data.clone().feeAmount.unwrap().as_str()).unwrap();
                        //     xcm_output_amount = xcm_output_amount.clone() - deposit_fee_amount.clone();
                        // }

                        let mut destination_deposit_fee_amount = BigInt::from(0);
                        let mut destination_deposit_reserve_amount = BigInt::from(0);

                        let deposit_fee_data = &self.fee_book.get_deposit_fee_data(adjacent_pair.xcm_node.clone());
                        if let Some(fee_data) = deposit_fee_data {
                            let deposit_fee_node_option = &self.get_asset_by_chain_and_id(adjacent_pair.xcm_node.clone().borrow().get_chain_id(), fee_data.get_fee_asset_id());
                            let deposit_fee_node = match deposit_fee_node_option {
                                Some(node) => node,
                                None => panic!("Token graph cannot find asset node for Chain ID(Origin): {} | ID(fee_asset): {}", adjacent_pair.xcm_node.clone().borrow().get_chain_id(), fee_data.get_fee_asset_id()),
                            };
                            destination_deposit_fee_amount = BigInt::from_str(fee_data.feeAmount.clone().unwrap().as_str()).unwrap();

                            if deposit_fee_node.as_ptr().eq(&adjacent_pair.xcm_node.clone().as_ptr()){
                                xcm_output_amount = xcm_output_amount.clone() - destination_deposit_fee_amount.clone();
                            } else {
                                destination_deposit_reserve_amount = self.convert_transfer_fee_amount_to_current_node(deposit_fee_node.clone(), current_node.clone(), destination_deposit_fee_amount.clone());
                                xcm_output_amount = xcm_output_amount.clone() - destination_deposit_reserve_amount.clone();
                            }
                        }
                        // println!("1. (D) fee: {}", deposit_fee_amount);
                        // ********************************************

                        // if current_node.borrow().best_path_value > adjacent_pair.adjacent_node.borrow().best_path_value{
                        if xcm_output_amount > adjacent_pair.xcm_node.borrow().best_path_value{
                            let mut current_path_contains_adjacent_node= false;
                            let mut is_destination_node = false;
                            for dest_node in &destination_nodes{
                                if dest_node.borrow().get_asset_key() == adjacent_pair.xcm_node.borrow().get_asset_key(){
                                    is_destination_node = true;
                                }
                            }
                            for path_node in &current_node.borrow().best_path{
                                if path_node.borrow().get_asset_key() == adjacent_pair.xcm_node.borrow().get_asset_key(){
                                    current_path_contains_adjacent_node = true;
                                }
                            }
                            
                            adjacent_pair.xcm_node.borrow_mut().best_path_value = xcm_output_amount.clone();
                            adjacent_pair.xcm_node.borrow_mut().best_path = current_node.borrow().best_path.clone();
                            adjacent_pair.xcm_node.borrow_mut().best_path.push(Rc::clone(&adjacent_pair.xcm_node));
                            adjacent_pair.xcm_node.borrow_mut().path_values = current_node.borrow().path_values.clone();
                            adjacent_pair.xcm_node.borrow_mut().path_values.push(current_node.borrow().best_path_value_display(&self));
                            adjacent_pair.xcm_node.borrow_mut().path_value_types = current_node.borrow().path_value_types.clone();
                            adjacent_pair.xcm_node.borrow_mut().path_value_types.push(PathType::Xcm);

                            // let reserve_string = reserve_amount.to_string();
                            let transfer_fee_amounts = vec![start_node_transfer_fee_amount.to_string(), middle_node_transfer_fee_amount.to_string()];
                            let transfer_reserve_amounts = vec![start_node_transfer_reserve_amount.to_string(), middle_node_transfer_reserve_amount.to_string()];
                            let deposit_fee_amounts = vec![middle_node_deposit_fee_amount.to_string(), destination_deposit_fee_amount.to_string()];
                            let deposit_reserve_amounts = vec![middle_node_deposit_reserve_amount.to_string(), destination_deposit_reserve_amount.to_string()];
                            let new_path_data: PathData = PathData{
                                path_type: "Xcm".to_string(),
                                lp_id: None,
                                xcm_transfer_fee_amounts: transfer_fee_amounts,
                                xcm_transfer_reserve_amounts: transfer_reserve_amounts,
                                xcm_deposit_fee_amounts: deposit_fee_amounts,
                                xcm_deposit_reserve_amounts: deposit_reserve_amounts
                            };

                            adjacent_pair.xcm_node.borrow_mut().path_datas = current_node.borrow().path_datas.clone();
                            adjacent_pair.xcm_node.borrow_mut().path_datas.push(new_path_data);
                            if !current_path_contains_adjacent_node && !is_destination_node{
                                node_queue.push_back(Rc::clone(&adjacent_pair.xcm_node));
                            }
                            // println!("****************************************");
                            
                        }
                    },
                    NodePool::Dex(dex_pool) =>  {
                        let current_chain = current_node.borrow().get_chain_id();
                        let input_index = dex_pool.base_asset_index;
                        let output_index = match input_index {
                            0 => 1,
                            1 => 0,
                            _ => panic!("Invalid index")
                        };
                        let path_value = calculate_dex_edge( node_pool, current_node.borrow().best_path_value.clone());

                        let adjacent_node = dex_pool.pool_nodes[output_index].clone();
                        if path_value > adjacent_node.borrow().best_path_value{
                            let mut test= false;
                            for path_node in &current_node.borrow().best_path{
                                if path_node.borrow().get_asset_key() == adjacent_node.borrow().get_asset_key(){
                                    test = true;
                                }
                            }

                            let mut is_destination_node = false;
                            for dest_node in &destination_nodes{
                                if dest_node.borrow().get_asset_key() == adjacent_node.borrow().get_asset_key(){
                                    is_destination_node = true;
                                }
                            }
                            adjacent_node.borrow_mut().best_path_value = path_value;
                            adjacent_node.borrow_mut().best_path = current_node.borrow().best_path.clone();
                            adjacent_node.borrow_mut().best_path.push(Rc::clone(&adjacent_node));
                            adjacent_node.borrow_mut().path_values = current_node.borrow().path_values.clone();
                            let formatted_path_value = adjacent_node.borrow().best_path_value_display(&self).clone();
                            adjacent_node.borrow_mut().path_values.push(formatted_path_value);
                            adjacent_node.borrow_mut().path_value_types = current_node.borrow().path_value_types.clone();
                            
                            let dex_type = dex_pool.get_dex_type();
                            let pool_id = dex_pool.get_pool_id();
                            let dex_type = match dex_type{
                                Some(dex_id) => {
                                    if dex_id.contains("solar"){ // All normal dexes are set to solar
                                        "Dex".to_string()
                                    } else if dex_id.contains("omnipool"){ // Need this for hydraDx
                                        "Omnipool".to_string()
                                    } else {
                                        dex_id.to_string()
                                    }
                                },
                                None => "Dex".to_string()
                            };
                            
                            let path_value_type = match dex_type.as_str(){
                                "Dex" => PathType::Dex,
                                "Omnipool" => PathType::Omnipool,
                                _ => PathType::Xcm
                            };

                            adjacent_node.borrow_mut().path_value_types.push(path_value_type);

                            let new_path_data: PathData = PathData{
                                path_type: dex_type,
                                lp_id: pool_id.clone(), // Just for contract address on evm
                                ..Default::default()
                            };

                            adjacent_node.borrow_mut().path_datas = current_node.borrow().path_datas.clone();
                            adjacent_node.borrow_mut().path_datas.push(new_path_data);
                            if !test && !is_destination_node{
                                node_queue.push_back(Rc::clone(&adjacent_node));
                            }
                            
                        }
                    },
                    NodePool::DexV3(dex_pool) =>  {
                        let path_value = calculate_dex_edge( node_pool, current_node.borrow().best_path_value.clone());

                        let input_index = dex_pool.base_asset_index;
                        let output_index = match input_index {
                            0 => 1,
                            1 => 0,
                            _ => panic!("Invalid index")
                        };
                        let adjacent_node = dex_pool.pool_nodes[output_index].clone();
                        if path_value > adjacent_node.borrow().best_path_value{

                            let mut test= false;
                            for path_node in &current_node.borrow().best_path{
                                if path_node.borrow().get_asset_key() == adjacent_node.borrow().get_asset_key(){
                                    test = true;
                                }
                            }
                            let mut is_destination_node = false;
                            for dest_node in &destination_nodes{
                                if dest_node.borrow().get_asset_key() == adjacent_node.borrow().get_asset_key(){
                                    is_destination_node = true;
                                }
                            }
                            adjacent_node.borrow_mut().best_path_value = path_value;
                            adjacent_node.borrow_mut().best_path = current_node.borrow().best_path.clone();
                            adjacent_node.borrow_mut().best_path.push(Rc::clone(&adjacent_node));
                            adjacent_node.borrow_mut().path_values = current_node.borrow().path_values.clone();
                            let formatted_path_value = adjacent_node.borrow().best_path_value_display(&self).clone();
                            adjacent_node.borrow_mut().path_values.push(formatted_path_value);
                            adjacent_node.borrow_mut().path_value_types = current_node.borrow().path_value_types.clone();
                            adjacent_node.borrow_mut().path_value_types.push(PathType::DexV3);

                            // REVIEW Dex type is == abi, so for V3 its algebra or uni3
                            // Dex type should be PathType::DexV3
                            // abi should be type of v3 pool
                            // PathData.path_type should be abi
                            let dex_type = dex_pool.get_dex_type().unwrap();
                            let dex_abi = dex_pool.get_abi().unwrap();
                            let pool_id = dex_pool.get_pool_id();
                            let new_path_data: PathData = PathData{
                                path_type: dex_abi,
                                lp_id: pool_id.clone(),
                                ..Default::default()
                            };
                            adjacent_node.borrow_mut().path_datas = current_node.borrow().path_datas.clone();
                            adjacent_node.borrow_mut().path_datas.push(new_path_data);


                            if !test && !is_destination_node{
                                node_queue.push_back(Rc::clone(&adjacent_node));
                            }
                            
                        }
                    },
                    NodePool::Cex(cex_pair) => {
                        let path_value = calculate_cex_edge( &self, &current_node, node_pool, current_node.borrow().best_path_value.to_u128().unwrap());
                        if path_value > cex_pair.pool_nodes.borrow().best_path_value.to_u128().unwrap(){
                            let mut test= false;
                            for path_node in &current_node.borrow().best_path{
                                if path_node.borrow().get_asset_key() == cex_pair.pool_nodes.borrow().get_asset_key(){
                                    test = true;
                                }
                            }
                            let mut is_destination_node = false;
                            for dest_node in &destination_nodes{
                                if dest_node.borrow().get_asset_key() == cex_pair.pool_nodes.borrow().get_asset_key(){
                                    is_destination_node = true;
                                }
                            }
                            cex_pair.pool_nodes.borrow_mut().best_path_value = BigInt::from(path_value);
                            cex_pair.pool_nodes.borrow_mut().best_path = current_node.borrow().best_path.clone();
                            cex_pair.pool_nodes.borrow_mut().best_path.push(Rc::clone(&cex_pair.pool_nodes));
                            cex_pair.pool_nodes.borrow_mut().path_values = current_node.borrow().path_values.clone();
                            let formatted_path_value = cex_pair.pool_nodes.borrow().best_path_value_display(&self).clone();
                            cex_pair.pool_nodes.borrow_mut().path_values.push(formatted_path_value);
                            cex_pair.pool_nodes.borrow_mut().path_value_types = current_node.borrow().path_value_types.clone();
                            cex_pair.pool_nodes.borrow_mut().path_value_types.push(PathType::Cex);
                            
                            if !test && !is_destination_node{
                                node_queue.push_back(Rc::clone(&cex_pair.pool_nodes));
                            }
                            
                        }
                    },
                    NodePool::Stable(stable_pool) => {
                        // for (i, adj_node) in stable_pair.adjacent_nodes.iter().enumerate(){
                        //     let path_value = calculate_stable_edge( &self, &current_node, &adjacent_pair, current_node.borrow().best_path_value, i);
                        // }
                        let input_index = stable_pool.base_asset_index;
                        for (i, target_pool_node) in stable_pool.pool_nodes.iter().enumerate(){
                            if i == input_index{
                                continue;
                            }
                            
                            let adjacent_node = stable_pool.pool_nodes[i].clone();

                            let path_value = calculate_stable_edge(&current_node, &node_pool, current_node.borrow().best_path_value.clone(), i).unwrap();
                            
                            if path_value > adjacent_node.borrow().best_path_value{
                                let mut test= false;
                                for path_node in &current_node.borrow().best_path{
                                    if path_node.borrow().get_asset_key() == adjacent_node.borrow().get_asset_key(){
                                        test = true;
                                    }
                                }
                                let mut is_destination_node = false;
                                for dest_node in &destination_nodes{
                                    if dest_node.borrow().get_asset_key() == adjacent_node.borrow().get_asset_key(){
                                        is_destination_node = true;
                                    }
                                }
                                adjacent_node.borrow_mut().best_path_value = path_value;
                                adjacent_node.borrow_mut().best_path = current_node.borrow().best_path.clone();
                                adjacent_node.borrow_mut().best_path.push(Rc::clone(&adjacent_node));
                                adjacent_node.borrow_mut().path_values = current_node.borrow().path_values.clone();
                                let formatted_path_value = adjacent_node.borrow().best_path_value_display(&self).clone();
                                adjacent_node.borrow_mut().path_values.push(formatted_path_value);
                                adjacent_node.borrow_mut().path_value_types = current_node.borrow().path_value_types.clone();
                                adjacent_node.borrow_mut().path_value_types.push(PathType::Stable);

                                let pool_id = stable_pool.get_pool_id();
                                let new_path_data: PathData = PathData{
                                    path_type: "Stable".to_string(),
                                    lp_id: pool_id.clone(),
                                    ..Default::default()
                                };
    
                                adjacent_node.borrow_mut().path_datas = current_node.borrow().path_datas.clone();
                                adjacent_node.borrow_mut().path_datas.push(new_path_data);
                                if !test && !is_destination_node{
                                    node_queue.push_back(Rc::clone(&adjacent_node));
                                }
                                
                            }
                        }
                    },
                    NodePool::BncStable(stable_pair) => {
                        let input_index = stable_pair.base_asset_index;
                        for (i, adjacent_node) in stable_pair.pool_nodes.iter().enumerate(){
                            if i == input_index{
                                continue;
                            }

                            // let adjacent_node = stable_pair.pool_nodes[i].clone();

                            let path_value = calculate_stable_edge(&current_node, &node_pool, current_node.borrow().best_path_value.clone(), i).unwrap();
                            if path_value > adjacent_node.borrow().best_path_value{
                                let mut test= false;
                                for path_node in &current_node.borrow().best_path{
                                    if path_node.borrow().get_asset_key() == adjacent_node.borrow().get_asset_key(){
                                        test = true;
                                    }
                                }
                                let mut is_destination_node = false;
                                for dest_node in &destination_nodes{
                                    if dest_node.borrow().get_asset_key() == adjacent_node.borrow().get_asset_key(){
                                        is_destination_node = true;
                                    }
                                }
                                adjacent_node.borrow_mut().best_path_value = path_value;
                                adjacent_node.borrow_mut().best_path = current_node.borrow().best_path.clone();
                                adjacent_node.borrow_mut().best_path.push(Rc::clone(&adjacent_node));
                                adjacent_node.borrow_mut().path_values = current_node.borrow().path_values.clone();
                                let formatted_path_value = adjacent_node.borrow().best_path_value_display(&self).clone();
                                adjacent_node.borrow_mut().path_values.push(formatted_path_value);
                                adjacent_node.borrow_mut().path_value_types = current_node.borrow().path_value_types.clone();
                                adjacent_node.borrow_mut().path_value_types.push(PathType::Stable);

                                let pool_id = stable_pair.get_pool_id();
                                let new_path_data: PathData = PathData{
                                    path_type: "Stable".to_string(),
                                    lp_id: pool_id.clone(),
                                    ..Default::default()
                                };
    
                                adjacent_node.borrow_mut().path_datas = current_node.borrow().path_datas.clone();
                                adjacent_node.borrow_mut().path_datas.push(new_path_data);
                                if !test && !is_destination_node{
                                    node_queue.push_back(Rc::clone(&adjacent_node));
                                }
                                
                            }
                        }


                    }
                    NodePool::StableShare(stable_share_pool) => {

                        // 

                        if stable_share_pool.token_to_share{
                            let path_value = calculate_stable_edge( &current_node, &node_pool, current_node.borrow().best_path_value.clone(), 0).unwrap();
                            let adjacent_node = stable_share_pool.share_asset_node.clone();
                                if path_value > adjacent_node.borrow().best_path_value{
                                    let mut test= false;
                                    for path_node in &current_node.borrow().best_path{
                                        if path_node.borrow().get_asset_key() == adjacent_node.borrow().get_asset_key(){
                                            test = true;
                                        }
                                    }
                                    let mut is_destination_node = false;
                                    for dest_node in &destination_nodes{
                                        if dest_node.borrow().get_asset_key() == adjacent_node.borrow().get_asset_key(){
                                            is_destination_node = true;
                                        }
                                    }
                                    adjacent_node.borrow_mut().best_path_value = path_value;
                                    adjacent_node.borrow_mut().best_path = current_node.borrow().best_path.clone();
                                    adjacent_node.borrow_mut().best_path.push(Rc::clone(&adjacent_node));
                                    adjacent_node.borrow_mut().path_values = current_node.borrow().path_values.clone();
                                    let formatted_path_value = adjacent_node.borrow().best_path_value_display(&self).clone();
                                    adjacent_node.borrow_mut().path_values.push(formatted_path_value);
                                    adjacent_node.borrow_mut().path_value_types = current_node.borrow().path_value_types.clone();
                                    adjacent_node.borrow_mut().path_value_types.push(PathType::Stable);
    
                                    let pool_id = stable_share_pool.get_pool_id();
                                    let new_path_data: PathData = PathData{
                                        path_type: "StableShare".to_string(),
                                        lp_id: pool_id.clone(),
                                        ..Default::default()
                                    };
        
                                    adjacent_node.borrow_mut().path_datas = current_node.borrow().path_datas.clone();
                                    adjacent_node.borrow_mut().path_datas.push(new_path_data);
                                    if !test && !is_destination_node{
                                        node_queue.push_back(Rc::clone(&adjacent_node));
                                    }
                                    
                                }
                        } else {
                            for (i, target_pool_node) in stable_share_pool.pool_nodes.iter().enumerate(){
                                let path_value = calculate_stable_edge( &current_node, &node_pool, current_node.borrow().best_path_value.clone(), i).unwrap();
                                
                                let adjacent_node = stable_share_pool.pool_nodes[i].clone();
                                if path_value > adjacent_node.borrow().best_path_value{
                                    let mut test= false;
                                    for path_node in &current_node.borrow().best_path{
                                        if path_node.borrow().get_asset_key() == adjacent_node.borrow().get_asset_key(){
                                            test = true;
                                        }
                                    }
                                    let mut is_destination_node = false;
                                    for dest_node in &destination_nodes{
                                        if dest_node.borrow().get_asset_key() == adjacent_node.borrow().get_asset_key(){
                                            is_destination_node = true;
                                        }
                                    }
                                    adjacent_node.borrow_mut().best_path_value = path_value;
                                    adjacent_node.borrow_mut().best_path = current_node.borrow().best_path.clone();
                                    adjacent_node.borrow_mut().best_path.push(Rc::clone(&adjacent_node));
                                    adjacent_node.borrow_mut().path_values = current_node.borrow().path_values.clone();
                                    let formatted_path_value = adjacent_node.borrow().best_path_value_display(&self).clone();
                                    adjacent_node.borrow_mut().path_values.push(formatted_path_value);
                                    adjacent_node.borrow_mut().path_value_types = current_node.borrow().path_value_types.clone();
                                    adjacent_node.borrow_mut().path_value_types.push(PathType::Stable);
    
                                    let pool_id = stable_share_pool.get_pool_id();
                                    let new_path_data: PathData = PathData{
                                        path_type: "StableShare".to_string(),
                                        lp_id: pool_id.clone(),
                                        ..Default::default()
                                    };
        
                                    adjacent_node.borrow_mut().path_datas = current_node.borrow().path_datas.clone();
                                    adjacent_node.borrow_mut().path_datas.push(new_path_data);
                                    if !test && !is_destination_node{
                                        node_queue.push_back(Rc::clone(&adjacent_node));
                                    }
                                    
                                }
                            }
                        
                        }


                    
                    }
                }
            }
        }

        let mut possible_destination_nodes = vec![];

        for node in destination_nodes{
            // node.borrow().display_path();
            let best_path = node.borrow().best_path.clone();
            if best_path.len() > 0 {
                possible_destination_nodes.push(Rc::clone(&node));
            }
        }
        let mut highest_value: Option<BigInt> = None;
        let mut highest_value_node: Option<Rc<RefCell<GraphNode>>> = None;

        for possible_node in possible_destination_nodes{
            let best_path_value = possible_node.borrow().best_path_value.clone();
            match highest_value {
                None => {
                    highest_value = Some(best_path_value);
                    highest_value_node = Some(Rc::clone(&possible_node));
                },
                Some(current_highest) if best_path_value > current_highest => {
                    highest_value = Some(best_path_value);
                    highest_value_node = Some(Rc::clone(&possible_node));
                },
                _ => {} // If the current node's value isn't higher, do nothing.
            }
        }

        let highest_value_path = highest_value_node.clone().unwrap().borrow().best_path.clone();
        let return_string = highest_value_node.unwrap().borrow().get_display_path().to_string();

        let path_and_display: (String, Vec<Rc<RefCell<GraphNode>>>) = (return_string, highest_value_path);
        path_and_display
    }

    pub fn get_asset_keys(&self, asset_key_2: String){
        let destination_node = &self.get_node(asset_key_2).clone();
        let destination_asset_location = destination_node.borrow().get_asset_location().unwrap();
        let all_destination_assets = &self.asset_registry.get_assets_at_location(destination_asset_location);
        let mut destination_nodes = vec![];
        for dest_asset in all_destination_assets{
            if(!dest_asset.borrow().is_cex_token()){
                let dest_node = &self.get_node(dest_asset.borrow().get_map_key()).clone();
                println!("{}", dest_node.borrow().get_asset_key());
                destination_nodes.push(Rc::clone(&dest_node));
            }
            
        }
    }

    pub fn get_asset_by_chain_and_symbol(&self, chain_id: u64, asset_symbol: String) -> Option<GraphNodePointer>{
        // let asset_symbol = &asset_symbol;
        println!("Chaind id: {} | Asset symbol: {}", chain_id, asset_symbol);
        for node_bucket in self.node_map.values(){
            for node in node_bucket{
                let current_chain_id = node.borrow().get_chain_id();
                let current_asset_symbol = node.clone().borrow().get_asset_symbol().clone();
                // let string = "ASEED";
                // println!("{}", string);

                if current_chain_id == chain_id{
                    println!("Current chain id: {} | Current asset symbol: {} --- {}", current_chain_id, current_asset_symbol.clone().as_str(), asset_symbol);
                    if asset_symbol.eq_ignore_ascii_case(&current_asset_symbol.as_str()){
                        return Some(Rc::clone(node));
                    }
                }
            }
        }
        None
    }

    pub fn get_asset_by_chain_and_id(&self, chain_id: u64, asset_id: String) -> Option<GraphNodePointer>{
        // println!("Chaind id: {} | Asset symbol: {}", chain_id, asset_id);
        let asset_id = asset_id.to_uppercase();
        for node_bucket in self.node_map.values(){
            for node in node_bucket{
                let current_chain_id = node.borrow().get_chain_id();
                let current_asset_id = node.borrow().get_local_id();
                let current_asset_symbol = node.clone().borrow().get_asset_symbol().clone();
                
                if current_chain_id == chain_id{
                    // println!("Current chain id: {} | Current asset id: {} --- {}", current_chain_id, current_asset_id.clone(), asset_id);
                    if asset_id.eq_ignore_ascii_case(&current_asset_id.as_str()){
                        return Some(Rc::clone(node));
                    }
                }
                // if node.borrow().get_chain_id() == chain_id && node.borrow().get_local_id().eq_ignore_ascii_case(&asset_id){
                //     return Some(Rc::clone(node));
                // }
            }
        }
        None
    }

    // finds an edge between the two node on the current chain
    pub fn find_immediate_edge_between_nodes(&self, primary_node: GraphNodePointer, adjacent_node: GraphNodePointer, fee_amount: BigInt) -> Option<BigInt>{
        // println!("Finding immediate edge...");
        for (index, node_pool) in primary_node.borrow().adjacent_node_pools.iter().enumerate(){
            match node_pool {
                NodePool::Dex(dex_pool) => {
                    for (index, pool_node) in dex_pool.pool_nodes.iter().enumerate(){
                        if pool_node.borrow().asset_key == adjacent_node.borrow().asset_key{
                            // return Some(((self.asset_key.clone(), index as u128), (pool_node.borrow().asset_key.clone(), index as u128)))
                            return Some(calculate_dex_edge(node_pool, fee_amount))
                        }
                    }
                },
                NodePool::DexV3(dex_pool) => {
                    for (index, pool_node) in dex_pool.pool_nodes.iter().enumerate(){
                        if pool_node.borrow().asset_key == adjacent_node.borrow().asset_key{
                            // return Some(((self.asset_key.clone(), index as u128), (pool_node.borrow().asset_key.clone(), index as u128)))
                            return Some(calculate_dex_edge(node_pool, fee_amount))
                        }
                    }
                },
                NodePool::Stable(stable_pool) => {
                    for (index, pool_node) in stable_pool.pool_nodes.iter().enumerate(){
                        if pool_node.borrow().asset_key == adjacent_node.borrow().asset_key{
                            // return Some(((self.asset_key.clone(), index as u128), (pool_node.borrow().asset_key.clone(), index as u128)))
                            return Some(calculate_stable_edge(&primary_node, node_pool, fee_amount, index).unwrap())
                        }
                    }
                },
                NodePool::BncStable(stable_pool) => {
                    for (index, pool_node) in stable_pool.pool_nodes.iter().enumerate(){
                        if pool_node.borrow().asset_key == adjacent_node.borrow().asset_key{
                            // return Some(((self.asset_key.clone(), index as u128), (pool_node.borrow().asset_key.clone(), index as u128)))
                            return Some(calculate_stable_edge(&primary_node, node_pool, fee_amount, index).unwrap())
                        }
                    }
                },
                NodePool::StableShare(stable_share_pool) => {
                    for (index, pool_node) in stable_share_pool.pool_nodes.iter().enumerate(){
                        if pool_node.borrow().asset_key == adjacent_node.borrow().asset_key{
                            // return Some(((self.asset_key.clone(), index as u128), (pool_node.borrow().asset_key.clone(), index as u128)))
                            return Some(calculate_stable_edge(&primary_node, node_pool, fee_amount, index).unwrap())
                        }
                    }
                },
                _ => {}
            }
        }
        None
    }

    // Gets every asset for each asset location, and finds an edge between the two nodes somewhere
    pub fn find_edge_between_nodes(&self, primary_node: GraphNodePointer, adjacent_node: GraphNodePointer, fee_amount: BigInt) -> Option<BigInt>{
        // Get fee asset location, get all assets at that location.
        // println!("Finding immediate edge on any chain...");
        let primary_asset_location = primary_node.borrow().get_asset_location().unwrap();
        let all_primary_assets = &self.asset_registry.get_assets_at_location(primary_asset_location);
        let all_primary_asset_nodes = all_primary_assets.iter().map(|asset| self.get_node(asset.borrow().get_map_key())).collect::<Vec<GraphNodePointer>>();

        // Get current node location and all assets at that location
        let adjacent_node_location = adjacent_node.borrow().get_asset_location().unwrap();
        let all_adjacent_assets = &self.asset_registry.get_assets_at_location(adjacent_node_location);
        let all_adjacent_asset_nodes = all_adjacent_assets.iter().map(|asset| self.get_node(asset.borrow().get_map_key())).collect::<Vec<GraphNodePointer>>();

        for primary_asset_node in all_primary_asset_nodes{
            for adjacent_asset_node in &all_adjacent_asset_nodes{
                if(primary_asset_node.borrow().get_chain_id() == adjacent_asset_node.borrow().get_chain_id()){
                    let edge = self.find_immediate_edge_between_nodes(primary_asset_node.clone(), adjacent_asset_node.clone(), fee_amount.clone());
                    if edge.is_some(){
                        return edge;
                    }
                }
            }
        }
        None

    }

    //
    pub fn find_path_between_nodes_on_chain(&self, primary_node: GraphNodePointer, adjacent_node: GraphNodePointer, fee_amount: BigInt) -> Option<BigInt>{
        // Get fee asset location, get all assets at that location.
        // println!("Finding path between nodes on any chain...");
        let primary_asset_location = primary_node.borrow().get_asset_location().unwrap();
        let all_primary_assets = &self.asset_registry.get_assets_at_location(primary_asset_location);
        let all_primary_asset_nodes = all_primary_assets.iter().map(|asset| self.get_node(asset.borrow().get_map_key())).collect::<Vec<GraphNodePointer>>();

        // Get current node location and all assets at that location
        let adjacent_node_location = adjacent_node.borrow().get_asset_location().unwrap();
        let all_adjacent_assets = &self.asset_registry.get_assets_at_location(adjacent_node_location);
        let all_adjacent_asset_nodes = all_adjacent_assets.iter().map(|asset| self.get_node(asset.borrow().get_map_key())).collect::<Vec<GraphNodePointer>>();

        // Get both nodes from the same chain and search for path
        for primary_asset_node in all_primary_asset_nodes{
            let current_chain_id = primary_asset_node.borrow().get_chain_id();
            let mut chain_contains_fee_node = false;
            for adjacent_asset_node in &all_adjacent_asset_nodes{
                if adjacent_asset_node.borrow().get_chain_id() == current_chain_id{
                    chain_contains_fee_node = true;
                    let traversed_path = vec![primary_asset_node.clone()];
                    let node_queue: Vec<GraphNodePointer> = vec![];
                    // println!("Finding path between {} and {}", primary_asset_node.borrow().get_asset_key_and_symbol(),  adjacent_asset_node.borrow().get_asset_key_and_symbol());
                    // let conversion_value = self.find_path_bfs(primary_asset_node.clone(), adjacent_asset_node.clone(), traversed_path, node_queue, fee_amount.clone());
                    let conversion_value = self.get_path_bfs(primary_asset_node.clone(), adjacent_asset_node.clone(), fee_amount.clone());
                    match conversion_value{
                        Some(value) => {
                            return Some(value);
                        },
                        None => ()
                        
                    }
                }
            }


            
        }
        None
    }

    // pub fn find_path_bfs(&self, current_node: GraphNodePointer, target_node: GraphNodePointer, traversed_nodes: Vec<GraphNodePointer>, bfs_node_queue: Vec<GraphNodePointer>, input_amount: BigInt) -> Option<BigInt>{
    //     let mut traversed_nodes = traversed_nodes.clone();
    //     traversed_nodes.push(Rc::clone(&current_node));

    //     let mut node_queue = bfs_node_queue.clone();
    
    //     println!("Current node: {} {} | Target node: {} {}", current_node.borrow().get_asset_key(), current_node.borrow().get_asset_symbol(), target_node.borrow().get_asset_key(), target_node.borrow().get_asset_symbol());

    //     // First check all adjacent nodes to find target, if not found move on to next node
    //     for node_pool in current_node.borrow().adjacent_node_pools.iter(){
    //         let pool_node = match node_pool {
    //             NodePool::Xcm(x) => (),
    //             NodePool::Dex(dex_pool) =>  {
    //                 // println!("Dex: {:?}", dex_pool.get_pool_id());
    //                 for pool_node in dex_pool.pool_nodes.iter(){
    //                     let mut traversed = false;
    //                     for traversed_node in &traversed_nodes{
    //                         if traversed_node.borrow().get_asset_key() == pool_node.borrow().get_asset_key(){
    //                             traversed = true
    //                         }
    //                     }

    //                     if !traversed{
    //                         node_queue.push(Rc::clone(&pool_node));
    //                         traversed_nodes.push(Rc::clone(&pool_node));
    //                     }
    //                     if pool_node.borrow().get_asset_key() == target_node.borrow().get_asset_key(){
    //                         return Some(calculate_dex_edge(node_pool, input_amount.clone()));
    //                     }
    //                 }
    //             },
    //             NodePool::DexV3(dex_pool) => {
    //                 // println!("Dex V3: {:?}", dex_pool.get_pool_id());
    //                 for pool_node in dex_pool.pool_nodes.iter(){
    //                     print!("{} | ", pool_node.borrow().get_asset_key());
    //                     let mut traversed = false;
    //                     for traversed_node in &traversed_nodes{
    //                         if traversed_node.borrow().get_asset_key() == pool_node.borrow().get_asset_key(){
    //                             traversed = true
    //                         }
    //                     }
    //                     if !traversed{
    //                         node_queue.push(Rc::clone(&pool_node));
    //                         traversed_nodes.push(Rc::clone(&pool_node));
    //                     }
    //                     if pool_node.borrow().get_asset_key() == target_node.borrow().get_asset_key(){
    //                         return Some(calculate_dex_edge(node_pool, input_amount));
    //                     }
    //                 }
    //                 println!("");
    //             },
    //             NodePool::Stable(stable_pool) => {
    //                 // println!("Stable: {:?}", stable_pool.get_pool_id());
    //                 for (index, pool_node) in stable_pool.pool_nodes.iter().enumerate(){
    //                     println!("{} | ", pool_node.borrow().get_asset_key());
    //                     let mut traversed = false;
    //                     for traversed_node in &traversed_nodes{
    //                         if traversed_node.borrow().get_asset_key() == pool_node.borrow().get_asset_key(){
    //                             traversed = true
    //                         }
    //                     }
    //                     if !traversed{
    //                         node_queue.push(Rc::clone(&pool_node));
    //                         traversed_nodes.push(Rc::clone(&pool_node));
    //                     }
    //                     if pool_node.borrow().get_asset_key() == target_node.borrow().get_asset_key(){
    //                         return Some(calculate_stable_edge(&current_node, node_pool, input_amount, index).unwrap());
    //                     }
    //                 }
    //             },
    //             NodePool::BncStable(stable_pool) => {
                    
    //                 for (index, pool_node) in stable_pool.pool_nodes.iter().enumerate(){
    //                     let mut traversed = false;
    //                     for traversed_node in &traversed_nodes{
    //                         if traversed_node.borrow().get_asset_key() == pool_node.borrow().get_asset_key(){
    //                             traversed = true
    //                         }
    //                     }
    //                     if !traversed{
    //                         node_queue.push(Rc::clone(&pool_node));
    //                         traversed_nodes.push(Rc::clone(&pool_node));
    //                     }
    //                     if pool_node.borrow().get_asset_key() == target_node.borrow().get_asset_key(){
    //                         return Some(calculate_stable_edge(&current_node, node_pool, input_amount, index).unwrap());
    //                     }
    //                 }
    //             },
    //             NodePool::StableShare(stable_share_pool) => {
    //                 for (index, pool_node) in stable_share_pool.pool_nodes.iter().enumerate(){
    //                     let mut traversed = false;
    //                     for traversed_node in &traversed_nodes{
    //                         if traversed_node.borrow().get_asset_key() == pool_node.borrow().get_asset_key(){
    //                             traversed = true
    //                         }
    //                     }
    //                     if !traversed{
    //                         node_queue.push(Rc::clone(&pool_node));
    //                         traversed_nodes.push(Rc::clone(&pool_node));
    //                     }
    //                     if pool_node.borrow().get_asset_key() == target_node.borrow().get_asset_key(){
    //                         return Some(calculate_stable_edge(&current_node, node_pool, input_amount, index).unwrap());
    //                     }
    //                 }
    //             },
    //             _ => ()
                
                    
    //         };
    //     }
    //     // If traversed all node edges on chain, return none
    //     if node_queue.len() == 0{
    //         // println!("No more nodes to traverse. Returning None");
    //         return None
    //     }

    //     // Move on to next node. Convert input amount via node edge
    //     let next_node = node_queue.pop().unwrap();
    //     let converted_input_value = self.calculate_edge_between_nodes(current_node.clone(), next_node.clone(), input_amount.clone());

    //     let converted_input_unwrap = match converted_input_value{
    //         Some(value) => {
    //             value
    //         },
    //         None => {
    //             panic!("Cant calculate edge between nodes. Current Node: {} | Next Node {}", current_node.borrow().get_asset_key_and_symbol(), next_node.borrow().get_asset_key_and_symbol());
    //         }
    //     };

    //     return self.find_path_bfs(next_node, target_node, traversed_nodes, node_queue, converted_input_unwrap);
        
    // }

    // For fee conversion, start node = fee node. and find amount equivelant to target node
    // Cases like IBTC, where the fee amount is smaller than the smallest unit of the transferred node, it will return 0, so round to 1 
    pub fn get_path_bfs(&self, start_node: GraphNodePointer, target_node: GraphNodePointer, input_amount: BigInt)-> Option<BigInt>{
        // println!("Finding path between {} and {}", start_node.borrow().get_asset_key_and_symbol(), target_node.borrow().get_asset_key_and_symbol());
        let mut node_queue: VecDeque<(GraphNodePointer, BigInt)> = VecDeque::new();
        node_queue.push_back((Rc::clone(&start_node), input_amount.clone()));
    
        // Set to keep track of visited nodes
        let mut traversed_node_map: HashMap<String, Vec<GraphNodePointer>> = HashMap::new();
        traversed_node_map.insert(start_node.borrow().get_asset_key(), vec![Rc::clone(&start_node)]);

        while let Some((current_node, current_amount)) = node_queue.pop_front() {
            // println!("*** Next node in queue: {} | Input Amount: {}", current_node.borrow().get_asset_key_and_symbol(), current_amount);
            for node_pool in current_node.borrow().adjacent_node_pools.iter() {
                match node_pool {
                    NodePool::Dex(dex_pool) => {
                        for pool_node in dex_pool.pool_nodes.iter() {
                            let pool_node_asset_key = pool_node.borrow().get_asset_key();
                            // println!("Node compare: (adjacent pool node) {} == (target node) {}", pool_node_asset_key, target_node.borrow().get_asset_key());
                            if pool_node_asset_key == target_node.borrow().get_asset_key() {
                                // println!("Found target node: {}", pool_node.borrow().get_asset_key_and_symbol());
                                let conversion_amount = calculate_dex_edge(node_pool, current_amount.clone());
                                if conversion_amount.is_zero(){
                                    return Some(BigInt::from(1));
                                }
                                return Some(conversion_amount);
                            }

                            // Enter node into traversed map if it doesn't exist
                            let mut bucket = traversed_node_map.entry(pool_node.borrow().asset_key.clone()).or_insert(Vec::new());
                            let mut node_is_inserted = false;
                            for traversed_node in bucket.clone() {
                                if Rc::ptr_eq(&traversed_node, pool_node) {
                                    node_is_inserted = true;
                                    break;
                                }
                            }
                            if !node_is_inserted {
                                let converted_input_amount = calculate_dex_edge(node_pool, current_amount.clone());
                                node_queue.push_back((Rc::clone(pool_node), converted_input_amount));
                                bucket.push(Rc::clone(pool_node));
                            }
                        }
                    },
                    NodePool::DexV3(dex_pool) => {
                        for pool_node in dex_pool.pool_nodes.iter() {
                            let pool_node_asset_key = pool_node.borrow().get_asset_key();
                            if pool_node_asset_key == target_node.borrow().get_asset_key() {
                                let conversion_amount = calculate_dex_edge(node_pool, current_amount.clone());
                                if conversion_amount.is_zero(){
                                    return Some(BigInt::from(1));
                                }
                                return Some(conversion_amount);
                            }

                            let mut bucket = traversed_node_map.entry(pool_node_asset_key.clone()).or_insert(Vec::new());
                            let mut node_is_inserted = false;
                            for traversed_node in bucket.clone() {
                                if Rc::ptr_eq(&traversed_node, pool_node) {
                                    node_is_inserted = true;
                                    break;
                                }
                            }
                            if !node_is_inserted {
                                let converted_input_amount = calculate_dex_edge(node_pool, current_amount.clone());
                                node_queue.push_back((Rc::clone(pool_node), converted_input_amount));
                                bucket.push(Rc::clone(pool_node));
                            }
                        }
                    },
                    NodePool::Stable(stable_pool) => {
                        for (index, pool_node) in stable_pool.pool_nodes.iter().enumerate() {
                            let pool_node_asset_key = pool_node.borrow().get_asset_key();
                            if pool_node_asset_key == target_node.borrow().get_asset_key() {
                                let conversion_amount = calculate_stable_edge(&current_node, node_pool, current_amount.clone(), index).unwrap();
                                if conversion_amount.is_zero(){
                                    return Some(BigInt::from(1));
                                }
                                return Some(conversion_amount);
                            }

                            let bucket = traversed_node_map.entry(pool_node_asset_key.clone()).or_insert(Vec::new());
                            let mut node_is_inserted = false;
                            for traversed_node in bucket.clone() {
                                if Rc::ptr_eq(&traversed_node, pool_node) {
                                    node_is_inserted = true;
                                    break;
                                }
                            }
                            if !node_is_inserted {
                                let converted_input_amount = calculate_stable_edge(&current_node, node_pool, current_amount.clone(), index).unwrap();
                                node_queue.push_back((Rc::clone(pool_node), converted_input_amount));
                                bucket.push(Rc::clone(pool_node));
                            }
                        }
                    },
                    NodePool::BncStable(stable_pool) => {
                        for (index, pool_node) in stable_pool.pool_nodes.iter().enumerate() {
                            let pool_node_asset_key = pool_node.borrow().get_asset_key();
                            if pool_node_asset_key == target_node.borrow().get_asset_key() {
                                let mut conversion_amount = calculate_stable_edge(&current_node, node_pool, current_amount.clone(), index).unwrap();
                                if conversion_amount.is_zero(){
                                    return Some(BigInt::from(1));
                                }
                                return Some(conversion_amount);
                            }

                            let bucket = traversed_node_map.entry(pool_node_asset_key.clone()).or_insert(Vec::new());
                            let mut node_is_inserted = false;
                            for traversed_node in bucket.clone() {
                                if Rc::ptr_eq(&traversed_node, pool_node) {
                                    node_is_inserted = true;
                                    break;
                                }
                            }
                            if !node_is_inserted {
                                let converted_input_amount = calculate_stable_edge(&current_node, node_pool, current_amount.clone(), index).unwrap();
                                node_queue.push_back((Rc::clone(pool_node), converted_input_amount));
                                bucket.push(Rc::clone(pool_node));
                            }
                        }
                    },
                    NodePool::StableShare(stable_share_pool) => {
                        for (index, pool_node) in stable_share_pool.pool_nodes.iter().enumerate() {
                            let pool_node_asset_key = pool_node.borrow().get_asset_key();
                            if pool_node_asset_key == target_node.borrow().get_asset_key() {
                                let mut conversion_amount = calculate_stable_edge(&current_node, node_pool, current_amount.clone(), index).unwrap();
                                if conversion_amount.is_zero(){
                                    conversion_amount = BigInt::from(1);
                                }
                                return Some(conversion_amount);
                            }

                            let bucket = traversed_node_map.entry(pool_node_asset_key.clone()).or_insert(Vec::new());
                            let mut node_is_inserted = false;
                            for traversed_node in bucket.clone() {
                                if Rc::ptr_eq(&traversed_node, pool_node) {
                                    node_is_inserted = true;
                                    break;
                                }
                            }
                            if !node_is_inserted {
                                let converted_input_amount = calculate_stable_edge(&current_node, node_pool, current_amount.clone(), index).unwrap();
                                node_queue.push_back((Rc::clone(pool_node), converted_input_amount));
                                bucket.push(Rc::clone(pool_node));
                            }
                        }
                    },
                    _ => (),
                }
            }
        }

        return None;
    }

    pub fn get_asset_decimals_for_kucoin_asset(&self, kucoin_node: &GraphNodePointer) -> u64 {
        self.asset_registry.get_kucoin_asset_decimals(kucoin_node.borrow().get_asset_location().unwrap())
    }

        // Finds the first instance of a node pool with specified node and calculates edge
    pub fn calculate_edge_between_nodes(&self, current_node: GraphNodePointer, target_node: GraphNodePointer, input_amount: BigInt) -> Option<BigInt>{
        for node_pool in current_node.borrow().adjacent_node_pools.iter(){
            match node_pool{
                NodePool::Dex(dex_pool) => {
                    for pool_node in dex_pool.pool_nodes.iter(){
                        if pool_node.borrow().get_asset_key() == target_node.borrow().get_asset_key(){
                            let edge = calculate_dex_edge(node_pool, input_amount.clone());
                            return Some(edge);
                            // return self.find_path_bfs(next_node, target_node, traversed_nodes, node_queue, converted_value);
                        }
                    }
                },
                NodePool::DexV3(dex_pool) => {
                    for pool_node in dex_pool.pool_nodes.iter(){
                        if pool_node.borrow().get_asset_key() == target_node.borrow().get_asset_key(){
                            let edge = calculate_dex_edge(node_pool, input_amount.clone());
                            // return self.find_path_bfs(next_node, target_node, traversed_nodes, node_queue, converted_value);
                            return Some(edge);
                        }
                    }
                },
                NodePool::Stable(stable_pool) => {
                    for (index, pool_node) in stable_pool.pool_nodes.iter().enumerate(){
                        if pool_node.borrow().get_asset_key() == target_node.borrow().get_asset_key(){
                            let edge = calculate_stable_edge(&current_node, node_pool, input_amount.clone(), index).unwrap();
                            // return self.find_path_bfs(next_node, target_node, traversed_nodes, node_queue, converted_value);
                            return Some(edge);
                        }
                    }
                },
                NodePool::BncStable(stable_pool) => {
                    for (index, pool_node) in stable_pool.pool_nodes.iter().enumerate(){
                        if pool_node.borrow().get_asset_key() == target_node.borrow().get_asset_key(){
                            let edge = calculate_stable_edge(&current_node, node_pool, input_amount.clone(), index).unwrap();
                            // return self.find_path_bfs(next_node, target_node, traversed_nodes, node_queue, converted_value);
                            return Some(edge);
                        }
                    }
                },
                NodePool::StableShare(stable_pool) => {
                    for (index, pool_node) in stable_pool.pool_nodes.iter().enumerate(){
                        if pool_node.borrow().get_asset_key() == target_node.borrow().get_asset_key(){
                            let edge = calculate_stable_edge(&current_node, node_pool, input_amount.clone(), index).unwrap();
                            // return self.find_path_bfs(next_node, target_node, traversed_nodes, node_queue, converted_value);
                            return Some(edge);
                        }
                    }
                },
                _ => return None
            };
        }
        None
    }
}
//Helper functions for TokenGraph ----------------------------------------------
pub fn create_graph_nodes(asset_registry: &AssetRegistry2) -> Vec<GraphNodePointer>{
    let mut graph_nodes = vec![];
    for asset in &asset_registry.get_all_assets(){
        let new_node = Rc::new(RefCell::new(GraphNode{
            asset: Rc::clone(&asset),
            // adjacent_nodes: Vec::new(),
            // adjacent_pairs: Vec::new(),
            adjacent_node_pools: Vec::new(),
            asset_key: asset.borrow().get_map_key(),
            pred: None,
            best_path_value: BigInt::zero(),
            best_path_value_display: 0.0,
            path_edges: Vec::new(),
            best_path: Vec::new(),
            path_values: Vec::new(),
            path_value_types: Vec::new(),
            path_datas: Vec::new(),
        }));
        graph_nodes.push(Rc::clone(&new_node));
    }
    graph_nodes
}
pub fn create_node_map(graph_nodes: &[GraphNodePointer]) -> HashMap<String, Vec<GraphNodePointer>>{
    let mut node_map = HashMap::new();

    for node in graph_nodes {
        let bucket = node_map.entry(node.borrow().asset_key.clone()).or_insert(Vec::new());
        bucket.push(node.clone());
    }
    node_map
}

//Get adjacent assets & liquidity for current node
pub fn add_adjacent_assets_2(current_node: GraphNodePointer, node_map: &HashMap<String, Vec<GraphNodePointer>>, adjacency_table: &AdjacencyTable2){
    let adjacent_assets = adjacency_table.get_adjacency_groups_for_asset(current_node.borrow().asset.clone());
    for adj_group in adjacent_assets{
        match adj_group{
            AdjacencyGroup::Stable(adjacency_group) => {
                let pool_data = match &adjacency_group.liquidity.clone(){
                    LiquidityPool::Stable(x) => x.clone(),
                    _ => panic!("Tried to get stable liquidity from non-stable liquidity"),
                };
                let mut all_pool_nodes = vec![];
                for pool_asset in pool_data.pool_assets.clone(){
                    let bucket = node_map.get(&pool_asset.borrow().get_map_key()).unwrap();
                    for potential_adjacent_node in bucket{
                        if pool_asset.borrow().get_map_key() == potential_adjacent_node.borrow().asset_key{
                            all_pool_nodes.push(Rc::clone(&potential_adjacent_node));

                        }
                    }
                }
                
                let stable_pool = StablePool{
                    pool_nodes: all_pool_nodes, 
                    liquidity: adjacency_group.liquidity.clone(), 
                    base_asset_index: adjacency_group.base_asset_index.clone()
                };
                let node_pool = NodePool::Stable(stable_pool);
                
                current_node.borrow_mut().adjacent_node_pools.push(node_pool);
            },
            AdjacencyGroup::Dex(adjacency_group) => {
                let pool_data = match &adjacency_group.liquidity.clone(){
                    LiquidityPool::Dex(x) => x.clone(),
                    _ => panic!("Tried to get stable liquidity from non-stable liquidity"),
                };
                let mut all_pool_nodes = vec![];
                for pool_asset in pool_data.pool_assets.clone(){
                    let bucket = node_map.get(&pool_asset.borrow().get_map_key()).unwrap();
                    for potential_adjacent_node in bucket{
                        if pool_asset.borrow().get_map_key() == potential_adjacent_node.borrow().asset_key{
                            all_pool_nodes.push(Rc::clone(&potential_adjacent_node));

                        }
                    }
                }
                
                let dex_pool = DexPool{
                    pool_nodes: all_pool_nodes, 
                    liquidity: adjacency_group.liquidity.clone(), 
                    base_asset_index: adjacency_group.base_asset_index.clone()
                };
                let node_pool = NodePool::Dex(dex_pool);
                
                current_node.borrow_mut().adjacent_node_pools.push(node_pool);
            },
            AdjacencyGroup::DexV3(adjacency_group) => {
                let pool_data = match &adjacency_group.liquidity.clone(){
                    LiquidityPool::DexV3(x) => x.clone(),
                    _ => panic!("Tried to get stable liquidity from non-stable liquidity"),
                };
                let mut all_pool_nodes = vec![];
                for pool_asset in pool_data.pool_assets.clone(){
                    let bucket = node_map.get(&pool_asset.borrow().get_map_key()).unwrap();
                    for potential_adjacent_node in bucket{
                        if pool_asset.borrow().get_map_key() == potential_adjacent_node.borrow().asset_key{
                            all_pool_nodes.push(Rc::clone(&potential_adjacent_node));

                        }
                    }
                }
                
                let dex_v3_pool = DexV3Pool{
                    pool_nodes: all_pool_nodes, 
                    liquidity: adjacency_group.liquidity.clone(), 
                    base_asset_index: adjacency_group.base_asset_index.clone()
                };
                let node_pool = NodePool::DexV3(dex_v3_pool);
                
                current_node.borrow_mut().adjacent_node_pools.push(node_pool);
            },
            AdjacencyGroup::Cex(group) => {

            },
            AdjacencyGroup::StableShare(adjacency_group) => {
                let lp_data = match &adjacency_group.liquidity.clone(){
                    LiquidityPool::Stable(stableLp) => stableLp.clone(),
                    _ => panic!("Tried to get stable share liquidity from non-stable share liquidity"),
                };
                
                let token_to_share = adjacency_group.token_to_share.clone();
                let share_asset_key = &lp_data.share_asset.clone().unwrap().borrow().get_map_key();
                let share_asset_node_bucket = node_map.get(&lp_data.share_asset.unwrap().borrow().get_map_key()).unwrap();
                let mut share_asset_node_option: Option<GraphNodePointer> = None;
                for potential_share_node in share_asset_node_bucket{
                    if share_asset_key.eq(potential_share_node.borrow().asset_key.as_str()){
                        share_asset_node_option = Some(Rc::clone(&potential_share_node));
                    }
                }

                if share_asset_node_option == None{
                    panic!("Could not find share asset node")
                }
                let share_asset_node = share_asset_node_option.unwrap();

                let mut all_pool_nodes = vec![];
                for adjacent_asset in adjacency_group.pool_assets.clone(){
                    let bucket = node_map.get(&adjacent_asset.borrow().get_map_key()).unwrap();
                    for potential_adjacent_node in bucket{
                        if adjacent_asset.borrow().get_map_key() == potential_adjacent_node.borrow().asset_key{
                            all_pool_nodes.push(Rc::clone(&potential_adjacent_node));
                        }
                    }
                }
                let node_pool = NodePool::StableShare(StableSharePool{
                    share_asset_node, 
                    token_to_share, 
                    pool_nodes: all_pool_nodes,
                    base_asset_index: adjacency_group.base_asset_index.clone(),
                    liquidity: adjacency_group.liquidity.clone(),
                });
                
                current_node.borrow_mut().adjacent_node_pools.push(node_pool);
            },
            AdjacencyGroup::BncStable(adjacency_group) => {
                let pool_data = match &adjacency_group.liquidity.clone(){
                    LiquidityPool::BncStable(x) => x.clone(),
                    _ => panic!("Tried to get bnc stable liquidity from non-bnc stable liquidity"),
                };
                let pool_assets = pool_data.pool_assets.clone();
                let mut pool_asset_nodes = vec![];
                for (index, pool_asset) in pool_assets.iter().enumerate(){
                    let bucket = node_map.get(&pool_asset.borrow().get_map_key()).unwrap();
                    for potential_adjacent_node in bucket{
                        if pool_asset.borrow().get_map_key() == potential_adjacent_node.borrow().asset_key{
                            pool_asset_nodes.push(Rc::clone(&potential_adjacent_node));
                        }
                    }
                }
                let node_pool = NodePool::BncStable(BncStablePool{
                    liquidity: adjacency_group.liquidity.clone(), 
                    pool_nodes: pool_asset_nodes,
                    base_asset_index: adjacency_group.base_asset_index.clone(),
                });
                
                current_node.borrow_mut().adjacent_node_pools.push(node_pool);
            },
            _ => {}

        };

        // let adj_group = match adj_group{
        //     AdjacencyGroup::Stable(group) => group,
        //     AdjacencyGroup::Dex(group) => group,
        //     AdjacencyGroup::DexV3(group) => group,
        //     AdjacencyGroup::Cex(group) => group,
        //     AdjacencyGroup::StableShare(group) => group,
        //     AdjacencyGroup::BncStable(group) => group,
        //     AdjacencyGroup::Xcm(group) => group,
        // };

        // match adj_group.group_type {
        //     GroupType::Stable => {
        //         let pool_data = match &adj_group.liquidity.clone().unwrap(){
        //             Liquidity::Stable(x) => x.clone(),
        //             _ => panic!("Tried to get stable liquidity from non-stable liquidity"),
        //         };
        //         let mut adjacent_nodes = vec![];
        //         for adjacent_asset in adj_group.adjacent_asset{
        //             let bucket = node_map.get(&adjacent_asset.borrow().get_map_key()).unwrap();
        //             for potential_adjacent_node in bucket{
        //                 if adjacent_asset.borrow().get_map_key() == potential_adjacent_node.borrow().asset_key{
        //                     adjacent_nodes.push(Rc::clone(&potential_adjacent_node));
        //                     // println!("Found stable pair")

        //                 }
        //             }
        //         }
        //         let stable_pool = StablePool{adjacent_nodes: adjacent_nodes, liquidity: adj_group.liquidity.clone().unwrap()};
        //         let adjacent_node_2 = NodePool::Stable(StablePool{adjacent_nodes: adjacent_nodes, liquidity: adj_group.liquidity.clone().unwrap()});
                
        //         current_node.borrow_mut().adjacent_node_pools.push(adjacent_node_2);
        //     },
        //     GroupType::BncStable => {
        //         let mut adjacent_nodes = vec![];
        //         let pool_data = match &adj_group.liquidity.clone().unwrap(){
        //             Liquidity::BncStable(x) => x.clone(),
        //             _ => panic!("Tried to get bnc stable liquidity from non-bnc stable liquidity"),
        //         };
        //         let pool_assets = pool_data.pool_assets.clone();
        //         let mut pool_asset_nodes = vec![];
        //         for (index, pool_asset) in pool_assets.iter().enumerate(){
        //             // Get graph node for the asset
        //             let bucket = node_map.get(&pool_asset.borrow().get_map_key()).unwrap();
        //             for potential_adjacent_node in bucket{
        //                 if pool_asset.borrow().get_map_key() == potential_adjacent_node.borrow().asset_key{
        //                     pool_asset_nodes.push(Rc::clone(&potential_adjacent_node));
        //                     // println!("Found stable pair")

        //                 }
        //             }
        //         }

        //         // for adjacent_asset in adj_group.adjacent_asset{
        //         //     let bucket = node_map.get(&adjacent_asset.borrow().get_map_key()).unwrap();
        //         //     for potential_adjacent_node in bucket{
        //         //         if adjacent_asset.borrow().get_map_key() == potential_adjacent_node.borrow().asset_key{
        //         //             adjacent_nodes.push(Rc::clone(&potential_adjacent_node));
        //         //             // println!("Found stable pair")

        //         //         }
        //         //     }
        //         // }
        //         let adjacent_node_2 = NodePool::BncStable(BncStablePool{
        //             liquidity: adj_group.liquidity.clone(), 
        //             pool_nodes: pool_asset_nodes,
        //             base_asset_index: pool_data.base_asset_index,
        //         });
                
        //         current_node.borrow_mut().adjacent_node_pools.push(adjacent_node_2);
        //     },
        //     GroupType::StableShare => {
        //             let mut adjacent_nodes = vec![];
        //             let lp_data: StableShareData = match &adj_group.liquidity.clone().unwrap() {
        //                 Liquidity::StableShare(stableLp) => stableLp.clone(),
        //                 _ => panic!("Tried to get stable share liquidity from non-stable share liquidity"),
        //             };
        //             let token_to_share = lp_data.token_to_share.unwrap();
        //             let share_asset_key = &lp_data.share_asset.borrow().get_map_key();
        //             let share_asset_node_bucket = node_map.get(&lp_data.share_asset.borrow().get_map_key()).unwrap();
        //             let mut share_asset_node_option: Option<GraphNodePointer> = None;
        //             for potential_share_node in share_asset_node_bucket{
        //                 if share_asset_key.eq(potential_share_node.borrow().asset_key.as_str()){
        //                     share_asset_node_option = Some(Rc::clone(&potential_share_node));
        //                 }
        //             }

        //             if share_asset_node_option == None{
        //                 panic!("Could not find share asset node")
        //             }
        //             let share_asset_node = share_asset_node_option.unwrap();

        //             for adjacent_asset in adj_group.adjacent_asset{
        //                 let bucket = node_map.get(&adjacent_asset.borrow().get_map_key()).unwrap();
        //                 for potential_adjacent_node in bucket{
        //                     if adjacent_asset.borrow().get_map_key() == potential_adjacent_node.borrow().asset_key{
        //                         adjacent_nodes.push(Rc::clone(&potential_adjacent_node));
        //                         // println!("Found stable pair")
    
        //                     }
        //                 }
        //             }
        //             let adjacent_node_2 = NodePool::StableShare(StableSharePool{share_asset_node: share_asset_node, token_to_share, adjacent_nodes: adjacent_nodes, liquidity: adj_group.liquidity.clone().unwrap()});
                    
        //             current_node.borrow_mut().adjacent_node_pools.push(adjacent_node_2);
        //     },
        //     GroupType::Dex =>{
        //         //Find node that corresponds to adjacent asset. Add it to current node's adjacency list
        //         let bucket = node_map.get(&adj_group.adjacent_asset[0].borrow().get_map_key()).unwrap();
        //         for potential_adjacent_node in bucket {
        //             if adj_group.adjacent_asset[0].borrow().get_map_key() == potential_adjacent_node.borrow().asset_key{
        //                 let dex_lp: DexData = if let Liquidity::Dex(x) = adj_group.liquidity.clone().unwrap(){
        //                     x
        //                 } else {
        //                     panic!("Dex liquidity should be DexLp")
        //                 };
        //                 let adjacent_node_2 = NodePool::Dex(DexPool{adjacent_node: Rc::clone(&potential_adjacent_node), liquidity: adj_group.liquidity.clone().unwrap()});
        //                 current_node.borrow_mut().adjacent_node_pools.push(adjacent_node_2);
        //                 // println!("found DEx")
        //             }
        //         }
        //     },
        //     GroupType::DexV3 => {
        //         //Find node that corresponds to adjacent asset. Add it to current node's adjacency list
        //         let bucket = node_map.get(&adj_group.adjacent_asset[0].borrow().get_map_key()).unwrap();
        //         for potential_adjacent_node in bucket{
        //             if adj_group.adjacent_asset[0].borrow().get_map_key() == potential_adjacent_node.borrow().asset_key{
        //                 let dex_lp: DexV3Data = if let Liquidity::DexV3(x) = adj_group.liquidity.clone().unwrap(){
        //                     x
        //                 } else {
        //                     panic!("Dex liquidity should be DexLp")
        //                 };
        //                 let adjacent_node_2 = NodePool::DexV3(DexV3Pool{adjacent_node: Rc::clone(&potential_adjacent_node), liquidity: adj_group.liquidity.clone().unwrap()});
        //                 current_node.borrow_mut().adjacent_node_pools.push(adjacent_node_2);
        //             }
        //         }
        //     },
        //     GroupType::Cex => { },
        //     //     //Find node that corresponds to adjacent asset. Add it to current node's adjacency list
        //     //     let bucket = node_map.get(&adj_group.adjacent_asset[0].borrow().get_map_key()).unwrap();
        //     //     for potential_adjacent_node in bucket{
        //     //         if adj_group.adjacent_asset[0].borrow().get_map_key() == potential_adjacent_node.borrow().asset_key{
        //     //             let adjacent_node = AdjacentNodePair::new(&potential_adjacent_node, adj_group.liquidity.clone(), 3);
        //     //             current_node.borrow_mut().adjacent_pairs.push(adjacent_node);

        //     //             let adjacent_node_2 = AdjacentNodePair2::CexPair(CexPair{adjacent_node: Rc::clone(&potential_adjacent_node), liquidity: adj_group.liquidity.clone().unwrap()});
        //     //             current_node.borrow_mut().adjacent_pairs2.push(adjacent_node_2);
        //     //         }
        //     //     }
        //     // },
        //     _ => {}

        // }
        
    }
}

//If asset is cross chain, get it's cross chain assets and add them as adjacent nodes
pub fn add_cross_chain_assets_2(current_node: GraphNodePointer, node_map: &HashMap<String, Vec<GraphNodePointer>>, asset_registry: &AssetRegistry2){
    let current_node_location = current_node.borrow().get_asset_location();
    if let Some(asset_location) = current_node_location{
        for cross_chain_asset in asset_registry.get_assets_at_location(asset_location){
            let bucket = node_map.get(&cross_chain_asset.borrow().get_map_key()).unwrap();
            for xcm_adjacent_node in bucket{
                if cross_chain_asset.borrow().get_map_key() == xcm_adjacent_node.borrow().asset.borrow().get_map_key(){

                    // Handle cases where xcm channel is not available
                    // INTR/IBTC can not make it to Parallel
                    // let xcm_node_key = xcm_adjacent_node.borrow().get_asset_symbol();
                    // if xcm_node_key == "INTR" || xcm_node_key == "IBTC"{
                    //     continue;
                    // }

                    let node_pool = NodePool::Xcm(Xcm{
                        xcm_node: Rc::clone(&xcm_adjacent_node)
                    });
                    current_node.borrow_mut().adjacent_node_pools.push(node_pool);
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GraphNode{
    pub asset: AssetPointer,
    // pub adjacent_nodes: Vec<(GraphNodePointer, ((u128, u128), (u128,u128)))>,
    // pub adjacent_pairs: Vec<AdjacentNodePair>,
    pub adjacent_node_pools: Vec<NodePool>,
    pub asset_key: String,
    pub pred: Option<GraphNodePointer>,
    pub best_path_value: BigInt,
    pub best_path_value_display: f64,
    pub path_edges: Vec<((String,u128),(String, u128))>,
    pub best_path: Vec<GraphNodePointer>,
    pub path_values: Vec<BigDecimal>,
    // ** Maybe remove. 0 = Xcm, 1 = Dex,  2 = Stable (All forms of stable), 3 = DexV3 (PathData.path_type = pool name like uni3 or algebra), 4 = Omnipool, 100 = Cex (Not in use atm), 
    pub path_value_types: Vec<PathType>, // logged as path_identifer
    pub path_datas: Vec<PathData>,

}
//--------------------------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PathType{
    Xcm,
    Dex,
    Stable,
    DexV3,
    Omnipool,
    Cex
}
// This is the object that we log at the end
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PathNode{
    pub node_key: String,
    pub asset_name: String,
    pub path_value: String,
    pub path_type: PathType, // Path traverseal type. Start, Dex, DexV3, Stable, Cex, Omnipool
    pub path_data: PathData,
    // pub path_id: String, // Any extra info like pool ID
}


// Removing path value types (that indicate path traversal type)
// REVIEW Structure of this could be better broken down into different types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PathData {
    pub path_type: String,
    pub lp_id: Option<String>,
    #[serde(default)]
    pub xcm_transfer_fee_amounts: Vec<String>,
    #[serde(default)]
    pub xcm_transfer_reserve_amounts: Vec<String>,
    #[serde(default)]
    pub xcm_deposit_fee_amounts: Vec<String>,
    #[serde(default)]
    pub xcm_deposit_reserve_amounts: Vec<String>,
}
impl Default for PathData {
    fn default() -> Self {
        PathData {
            path_type: String::new(),
            lp_id: None,
            xcm_transfer_fee_amounts: Vec::new(),
            xcm_transfer_reserve_amounts: Vec::new(),
            xcm_deposit_fee_amounts: Vec::new(),
            xcm_deposit_reserve_amounts: Vec::new(),
        }
    }
}
// #[derive(Debug, PartialEq)]
// pub struct AdjacentNodePair{
//     pub adjacent_node: GraphNodePointer,
//     pub liquidity: Option<Liquidity>,
//     pub pair_type: u64,
// }
#[derive(Debug, Clone, PartialEq)]
pub enum NodePool{
    Dex(DexPool),
    DexV3(DexV3Pool),
    Cex(CexPool),
    Stable(StablePool),
    BncStable(BncStablePool),
    StableShare(StableSharePool),
    Xcm(Xcm),
}
#[derive(Debug, Clone, PartialEq)]
pub struct StablePool{
    pub pool_nodes: Vec<GraphNodePointer>,
    pub base_asset_index: usize,
    pub liquidity: LiquidityPool,
}
#[derive(Debug, Clone, PartialEq)]
pub struct BncStablePool{
    pub pool_nodes: Vec<GraphNodePointer>,
    pub base_asset_index: usize,
    pub liquidity: LiquidityPool,
}
#[derive(Debug, Clone, PartialEq)]
pub struct StableSharePool{
    pub pool_nodes: Vec<GraphNodePointer>,
    pub base_asset_index: Option<usize>,
    pub share_asset_node: GraphNodePointer,
    pub token_to_share: bool,
    pub liquidity: LiquidityPool,
}
#[derive(Debug, Clone, PartialEq)]
pub struct DexPool{
    pub pool_nodes: Vec<GraphNodePointer>,
    pub base_asset_index: usize,
    pub liquidity: LiquidityPool,
}
#[derive(Debug, Clone, PartialEq)]
pub struct DexV3Pool{
    pub pool_nodes: Vec<GraphNodePointer>,
    pub base_asset_index: usize,
    pub liquidity: LiquidityPool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CexPool{
    pub pool_nodes: GraphNodePointer,
    pub liquidity: LiquidityPool,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Xcm{
    pub xcm_node: GraphNodePointer,
}

pub trait Pool {

    fn get_liquidity(&self) -> &LiquidityPool;

    fn get_pool_id(&self) -> Option<String> {
        match self.get_liquidity() {
            LiquidityPool::Dex(dex_lp) => dex_lp.pool_id.clone(),
            LiquidityPool::DexV3(dex_lp) => dex_lp.pool_id.clone(),
            LiquidityPool::Stable(stable_lp) => stable_lp.pool_id.clone(),
            LiquidityPool::BncStable(stable_lp) => Some(stable_lp.pool_id.clone()),
            _ => panic!("Tried to get LP ID from unsupported liquidity type"),
        }
    }

    fn get_swap_fee(&self) -> u128{
        match &self.get_liquidity(){
            LiquidityPool::Dex(dex) => 0,
            LiquidityPool::DexV3(dex) => dex.fee_rate,
            LiquidityPool::Stable(stable_lp) => stable_lp.swap_fee,
            LiquidityPool::BncStable(stable_lp) => stable_lp.swap_fee,
            _ => panic!("Tried to get stable swap fee"),
        }
    }
    fn get_dex_type(&self) -> Option<String>{
        match &self.get_liquidity(){
            LiquidityPool::Dex(pool_data) => Some(pool_data.dex_type.clone()),
            LiquidityPool::DexV3(pool_data) => pool_data.dex_type.clone(),
            _ => panic!("Tried to get dex type from non-dex or dex3 liquidity"),
        }
    }

    // REVIEW Should reformat GlmrLp to make dexType be DexV2 and include an abi like in dex3
    fn get_abi(&self) -> Option<String>{
        match &self.get_liquidity(){
            LiquidityPool::Dex(pool_data) => Some(pool_data.dex_type.clone()),
            LiquidityPool::DexV3(pool_data) => pool_data.abi.clone(),
            _ => panic!("Tried to get dex type from non-dex or dex3 liquidity"),
        }
    }

    fn get_pool_nodes(&self) -> Vec<GraphNodePointer>;

    // fn get_swap_fee(&self) -> Option<u128> {
    //     match self.get_liquidity() {
    //         LiquidityPool::Stable(stable_lp) => Some(stable_lp.swap_fee),
    //         LiquidityPool::StableShare(stable_lp) => Some(stable_lp.swap_fee),
    //         _ => None,
    //     }
    // }
}

impl Pool for DexPool{

    fn get_liquidity(&self) -> &LiquidityPool {
        &self.liquidity
    }

    fn get_pool_nodes(&self) -> Vec<GraphNodePointer> {
        self.pool_nodes.clone()
    }
}
impl Pool for DexV3Pool {
    fn get_liquidity(&self) -> &LiquidityPool {
        &self.liquidity
    }
    fn get_pool_nodes(&self) -> Vec<GraphNodePointer> {
        self.pool_nodes.clone()
    }

}

impl Pool for StablePool{
    fn get_liquidity(&self) -> &LiquidityPool {
        &self.liquidity
    }
    fn get_pool_nodes(&self) -> Vec<GraphNodePointer> {
        self.pool_nodes.clone()
    }
}

impl Pool for BncStablePool{
    fn get_liquidity(&self) -> &LiquidityPool {
        &self.liquidity
    }
    fn get_pool_nodes(&self) -> Vec<GraphNodePointer> {
        self.pool_nodes.clone()
    }
}

impl Pool for StableSharePool{
    fn get_liquidity(&self) -> &LiquidityPool {
        &self.liquidity
    }
    fn get_pool_nodes(&self) -> Vec<GraphNodePointer> {
        self.pool_nodes.clone()
    }
}

// impl AdjacentNodePair{
//     pub fn new(adjacent_node: &GraphNodePointer, liquidity: Option<Liquidity>, pair_type: u64) -> AdjacentNodePair{
//         AdjacentNodePair{
//             adjacent_node: Rc::clone(adjacent_node),
//             liquidity,
//             pair_type
//         }
//     }
//     pub fn get_dex_liquidity(&self) -> (u128, u128){
//         match &self.liquidity.as_ref().unwrap(){
//             Liquidity::Dex(dexLp) => (dexLp.base_liquidity, dexLp.adjacent_liquidity),
//             _ => panic!("Tried to get dex liquidity from non-dex liquidity"),
//         }
//     }
//     pub fn get_cex_liquidity_price(&self) -> (u128, u128){
//         match &self.liquidity.as_ref().unwrap(){
//             Liquidity::Cex(cexLp) => (cexLp.bid_price, cexLp.ask_price),
//             _ => panic!("Tried to get cex liquidity from non-cex liquidity"),
//         }
//     }
//     pub fn get_cex_liquidity_decimals(&self) -> (u128, u128){
//         match &self.liquidity.as_ref().unwrap(){
//             Liquidity::Cex(cexLp) => (cexLp.bid_decimals, cexLp.ask_decimals),
//             _ => panic!("Tried to get cex liquidity from non-cex liquidity"),
//         }
//     }
// }

// impl PartialEq for GraphNode {
//     fn eq(&self, other: &Self) -> bool {
//         self.id == other.id
//     }
// }

impl GraphNode{
    // Get pool from key of adjacent node
    pub fn get_v3_lp_stats_from_pair(&self, adjacent_asset_key: String, contract_address: String) -> Option<DexV3Data> {
        for adjacent_node_pool in &self.adjacent_node_pools{
            match adjacent_node_pool{
                NodePool::DexV3(node_pool) => {
                    for pool_node in &node_pool.pool_nodes{
                        if pool_node.borrow().asset_key == adjacent_asset_key{
                            if let LiquidityPool::DexV3(pool_data) = &node_pool.liquidity{
                                if contract_address.eq_ignore_ascii_case(&pool_data.pool_id.clone().unwrap()){
                                    return Some(pool_data.clone())
                                }
                                
                            } 
                        }
                    }
                },
                _ => {}
            }

        }
        None
    }

    pub fn get_stable_lp_stats_from_pair(&self, adjacent_asset_key: String, pool_id: String) -> Option<StableData> {
        for adjacent_node_pool in &self.adjacent_node_pools{

            match adjacent_node_pool{
                NodePool::Stable(stable_pool) => {
                    for (index, stable_pool_node) in stable_pool.pool_nodes.iter().enumerate(){
                        if stable_pool_node.borrow().get_asset_key().eq(&adjacent_asset_key){
                            if let LiquidityPool::Stable(stable_pool_data) = &stable_pool.liquidity{
                                println!("Stable pool id {:?}", stable_pool_data);
                                if pool_id.eq_ignore_ascii_case(&stable_pool_data.pool_id.clone().unwrap()){
                                    return Some(stable_pool_data.clone())
                                }
                            }
                        }
                    }
                },
                _ => {}
            }

        }
        None
    }

    pub fn get_stable_share_lp_stats_from_pair(&self, adjacent_asset_key: String, pool_id: String) -> Option<StableData> {
        for adjacent_node_pool in &self.adjacent_node_pools{
            match adjacent_node_pool{
                NodePool::StableShare(stable_pair) => {
                    for (index, stable_pool_node) in stable_pair.pool_nodes.iter().enumerate(){
                        if stable_pool_node.borrow().asset_key == adjacent_asset_key{
                            if let LiquidityPool::Stable(stable_lp) = &stable_pair.liquidity{
                                if pool_id.eq_ignore_ascii_case(&stable_lp.pool_id.clone().unwrap()){
                                    return Some(stable_lp.clone())
                                }
                            }
                        }
                    }
                },
                _ => {}
            }

        }
        None
    }

    pub fn get_v3_adjacent_node_pair(&self, adjacent_asset_key: String, contract_address: String) -> Option<NodePool>{
        for pool in &self.adjacent_node_pools{
            match pool{
                NodePool::DexV3(dex_pool) => {
                    for (index, dex_pool_node) in dex_pool.pool_nodes.iter().enumerate(){
                        if dex_pool_node.borrow().asset_key == adjacent_asset_key{
                            if let LiquidityPool::DexV3(pool_data) = &dex_pool.liquidity{
                                if contract_address.eq_ignore_ascii_case(&pool_data.pool_id.clone().unwrap()){
                                    return Some(pool.clone());
                                }
                                
                            }
                        }
                    }
                },
                _ => {}
            }

        }
        None
    }

    pub fn get_stable_adjacent_node_pair(&self, adjacent_asset_key: String, pool_id: String) -> Option<(NodePool, usize)>{
        for pair in &self.adjacent_node_pools{
            match pair{
                NodePool::Stable(stable_pool) => {
                    // let pool_nodes = stable_pair.clone();
                    for (index, stable_pool_node) in stable_pool.pool_nodes.iter().enumerate(){
                        if stable_pool_node.borrow().asset_key == adjacent_asset_key{
                            if let LiquidityPool::Stable(stable_lp) = &stable_pool.liquidity{
                                if pool_id.eq_ignore_ascii_case(&stable_lp.pool_id.clone().unwrap()){
                                    return Some((pair.clone(), index))
                                }
                            }
                        }
                    }
                    
                },
                NodePool::BncStable(stable_pool) => {
                    // let pool_nodes = stable_pool.pool_nodes.clone();
                    for (index, stable_pool_node) in stable_pool.pool_nodes.iter().enumerate(){
                        if stable_pool_node.borrow().asset_key == adjacent_asset_key{
                            if let LiquidityPool::BncStable(pool_data) = &stable_pool.liquidity{
                                if pool_id.eq_ignore_ascii_case(&pool_data.pool_id.clone()){
                                    println!("Getting BNC node pool");
                                    return Some((pair.clone(), index))
                                }
                            }
                        }
                    }
                },
                NodePool::StableShare(stable_pool) => {
                    // let adjacent_nodes = stable_pool.pool_nodes.clone();
                    for (index, stable_pool_node) in stable_pool.pool_nodes.iter().enumerate(){
                        if stable_pool_node.borrow().asset_key == adjacent_asset_key{
                            if let LiquidityPool::Stable(stable_lp) = &stable_pool.liquidity{
                                if pool_id.eq_ignore_ascii_case(&stable_lp.pool_id.clone().unwrap()){
                                    return Some((pair.clone(), index))
                                }
                            }
                        }
                    }
                },
                _ => {}
            }
        }
        None
    }
    
    
    pub fn get_stable_share_adjacent_node_pair(&self, adjacent_asset_key: String, pool_id: String) -> Option<(NodePool, usize)>{
        println!("Getting stable share adjacent node pair: {}", &self.asset_key);
        println!("Adjacent asset key: {}", adjacent_asset_key);
        for pair in &self.adjacent_node_pools{
            match pair{
                NodePool::StableShare(stable_share_pool) => {
                    // let adjacent_nodes = stable_share_pool.pool_nodes.clone();
                    // let base_node = stable_pair.
                    for (index, stable_pool_node) in stable_share_pool.pool_nodes.iter().enumerate(){
                        println!("Stable pool key: {}", stable_pool_node.borrow().get_asset_key());
                        println!("Token -> Share: {}", stable_share_pool.token_to_share);
                        if stable_pool_node.borrow().asset_key == adjacent_asset_key{
                            if let LiquidityPool::Stable(stable_lp) = &stable_share_pool.liquidity{
                                if pool_id.eq_ignore_ascii_case(&stable_lp.pool_id.clone().unwrap()){
                                    return Some((pair.clone(), index))
                                }
                            }
                        }
                    }
                },
                _ => {}
            }
        }
        None
    }

    pub fn get_asset_name(&self) -> String{
        self.asset.borrow().get_asset_name().to_string()
    }

    pub fn get_asset_key(&self) -> String{
        self.asset.borrow().get_map_key()
    }

    pub fn get_asset_decimals(&self) -> u64{
        self.asset.borrow().get_asset_decimals()
    }

    pub fn get_asset_location(&self) -> Option<AssetLocation>{
        self.asset.borrow().asset_location.clone()
    }

    pub fn get_asset_symbol(&self) -> String{
        self.asset.borrow().get_asset_symbol().to_string()
    }

    pub fn get_asset_key_and_symbol(&self) -> String{
        format!("{} {}", self.get_asset_key(), self.get_asset_symbol())
    }

    pub fn get_local_id(&self) -> String{
        self.asset.borrow().get_local_id().unwrap().to_string()
    }

    pub fn get_dex_contract_address(){

    }
    pub fn get_asset_contract_address(&self) -> String{
        self.asset.borrow().get_asset_contract_address().unwrap()
    }
    pub fn get_chain_id(&self) -> u64 {
        self.asset.borrow().get_chain_id().clone().unwrap()
    }
    pub fn get_origin_chain_id(&self) -> u64 {
        self.asset.borrow().get_origin_chain_id().clone().unwrap()
    }
    pub fn get_relay_chain(&self) -> String {
        self.asset.borrow().get_relay_chain()
    }
    pub fn is_cex_token(&self) -> bool {
        self.asset.borrow().is_cex_token()
    }
    pub fn best_path_value_display(&self, token_graph: &TokenGraph2) -> BigDecimal {
        match self.asset.borrow().token_data{
            TokenData::CexAsset { .. } => {
                if self.get_asset_symbol() == "USDT"{
                    let usdt_asset_decimals = 4;
                    BigDecimal::from(self.best_path_value.clone()) / BigDecimal::from(BigInt::from(10).pow(usdt_asset_decimals as u32))
                    // self.best_path_value.to_f64().unwrap().clone() / f64::powi(10.0, usdt_asset_decimals as i32)
                } 
                else {
                    let kucoin_asset_decimals = token_graph.asset_registry.get_kucoin_asset_decimals(self.get_asset_location().unwrap());
                    // self.best_path_value.to_f64().unwrap().clone() / f64::powi(10.0, kucoin_asset_decimals as i32)
                    BigDecimal::from(self.best_path_value.clone()) / BigDecimal::from(BigInt::from(10).pow(kucoin_asset_decimals as u32))
                }
                
            },
            _ => {
                // self.best_path_value.to_f64().unwrap().clone() / f64::powi(10.0, self.get_asset_decimals() as i32)
                BigDecimal::from(self.best_path_value.clone()) / BigDecimal::from(BigInt::from(10).pow(self.get_asset_decimals() as u32))
            }
        }
    }

    pub fn display_asset(&self){
        self.asset.borrow().display_asset();
    }

    pub fn display_path(&self){
        println!("Node: {} {} {}", &self.get_asset_key(), &self.get_asset_name(), &self.get_asset_decimals());
        print!("Path: ");
        for (i, path_node) in self.best_path.iter().enumerate(){
            println!("{} {} {} ->", path_node.borrow().get_asset_key(), path_node.borrow().get_asset_name(), &self.path_values[i]);
        }
    }

    pub fn get_display_path(&self) -> String{
        let mut path_string = String::new();
        for (i, path_node) in self.best_path.iter().enumerate(){
            let pool_id = match &self.path_datas[i].lp_id{
                Some(id) => id.clone(),
                None => "".to_string(),
            };
            path_string.push_str(&format!("{} {} {} ({})->", path_node.borrow().get_asset_key(), path_node.borrow().get_asset_name(), &self.path_values[i], &pool_id));
            
        }
        path_string
    }


}

// pub fn filter_dex

pub fn calculate_v2_dex_swap_formula(dex_pool: &DexPool, input_amount: BigInt) -> BigInt {
    // println!("Calculating V2 DEX swap");
    // dex_pool.get_pool_nodes().iter().for_each(|node| {
    //     println!("Node: {}", node.borrow().get_asset_key_and_symbol());
    // });

    let input_asset_index = dex_pool.base_asset_index;
    let output_asset_index = match input_asset_index{
        0 => 1,
        1 => 0,
        _ => panic!("Invalid asset index"),
    };   
    // println!("Input asset index: {} | Output asset index: {}", input_asset_index, output_asset_index);
    let (base_liquidity, target_liquidity) = match dex_pool.liquidity.clone() {
        LiquidityPool::Dex(pool_data) => {
            (pool_data.pool_liquidity[input_asset_index], pool_data.pool_liquidity[output_asset_index])
        }
        _ => panic!("Tried to get dex liquidity from non-dex liquidity"),
    };

    let input_reserve = BigInt::from(base_liquidity);
    let output_reserve = BigInt::from(target_liquidity);

    // println!("Input amount: {}", input_amount);
    // println!("Input reserve: {} | Output reserve: {}", input_reserve, output_reserve);

    // let slip_multiplier = BigInt::from(10000) - BigInt::from(100); // 1% // MOONBEAM FEE RATE
    // let slip_multiplier = BigInt::from(9970); // 0.3% // BIFROST FEE RATE
    let slip_multiplier = BigInt::from(9960); // 0.4% // Minimum for Moonbeam algebra swaps

    let amount_in_with_slippage = input_amount.clone() * slip_multiplier.clone();
    let slip_numerator = amount_in_with_slippage.clone() * output_reserve.clone();
    let slip_denominator = (input_reserve.clone() * BigInt::from(10000)) + amount_in_with_slippage.clone();
    let total_amount_out = slip_numerator.clone() / slip_denominator.clone();
    // println!("Amount in with slippage: {} | Slip num: {} | Slip den: {} | Total out: {} ", amount_in_with_slippage, slip_numerator, slip_denominator, total_amount_out);
    total_amount_out
}

pub fn calculate_v3_dex_swap(dex_v3_pool: &DexV3Pool, input_amount: BigInt) -> BigInt{
    let (contract_address, tokens, active_liquidity, current_tick, fee_rate, upper_ticks, lower_ticks) = match &dex_v3_pool.liquidity {
        LiquidityPool::DexV3(pool_data) => (pool_data.pool_id.clone(), pool_data.pool_assets.clone(), pool_data.active_liquidity, pool_data.current_tick, pool_data.fee_rate,  pool_data.upper_ticks.clone(), pool_data.lower_ticks.clone()),
        _ => panic!("Tried to get dex liquidity from non-dex liquidity"),
    };
    // println!("V3 Contract Address: {:?}", contract_address);
    let input_token_index = dex_v3_pool.base_asset_index;

    let (token_0, token_1) = (tokens[0].clone(), tokens[1].clone());

    // let contract_address_check = contract_address.clone().unwrap().to_uppercase();
    // let target_contract = "0x17c19AefF4caBfe76633A20e6B0eD903Df48dD56".to_string().to_uppercase();

    let q96: BigInt = BigInt::from(2).pow(96);
    let one = BigRational::one();
    let zero = BigRational::zero();

    // let target_node_contract_address = dex_v3_pool.pool_nodes.borrow().asset.borrow().get_asset_contract_address().clone();
    
    // Get CURRENT TICK | ACTIVE LIQUIDITY | FEE RATE? | UPPER/LOWER TICKS
    let mut current_tick = current_tick.to_bigint().unwrap();
    let active_liquidity = active_liquidity.to_bigint().unwrap();

    let additional_fee_rate = BigInt::from(100); // An additional 0.01% fee rate
    let final_fee_rate = BigInt::from(fee_rate).checked_add(&additional_fee_rate).unwrap();

    let fee_ratio: BigRational = BigRational::new(final_fee_rate, BigInt::from(10).pow(6));

    if lower_ticks.len() == 0 || upper_ticks.len() == 0 || active_liquidity == BigInt::zero() {
        return BigInt::zero()
    }

    // Set RETURN VALUES
    let mut total_token_in = BigInt::from(0);
    let mut total_token_out = BigInt::from(0);

    // Set PRICE RANGE LIQUIDITY
    let mut price_range_liquidity = active_liquidity.clone();

    // Set INPUT AMOUNT            
    let mut token_in_amount_remaining = BigRational::new(BigInt::from(input_amount), BigInt::one());
    token_in_amount_remaining = token_in_amount_remaining * (one.clone() - fee_ratio);
    
    // Set index for initialized ticks upper/lower to 0
    let mut tick_index = 0;

    while token_in_amount_remaining.gt(&zero){
        

        // Get CURRENT TICK BOUNDARIES
        let lower_tick = match lower_ticks.get(tick_index){
            Some(tick_lower) => tick_lower.clone(),
            None => MIN_TICK_DATA.clone()
        };
        let upper_tick = match upper_ticks.get(tick_index){
            Some(tick_upper) => tick_upper.clone(),
            None => MAX_TICK_DATA.clone()
        };
        tick_index += 1;

        //Get CURRENT PRICE | UPPER PRICE | LOWER PRICE
        let current_sqrt_price_x96 = get_sqrt_ratio_at_tick(current_tick.to_i32().unwrap());
        let current_sqrt_price = BigRational::new(current_sqrt_price_x96.clone(), q96.clone());
        let sqrt_price_upper_x96 = get_sqrt_ratio_at_tick(upper_tick.tick.to_i32().unwrap());
        let sqrt_price_lower_x96 = get_sqrt_ratio_at_tick(lower_tick.tick.to_i32().unwrap());
        
        if input_token_index == 0 {
            // Swapping 0 -> 1

            // Get TARGET PRICE from PRICE CHANGE
            let change_in_price_reciprocal = token_in_amount_remaining.clone() / price_range_liquidity.clone();
            let change_in_price = (one.clone() / current_sqrt_price.clone()) + change_in_price_reciprocal.clone();
            let target_sqrt_price = change_in_price.clone().pow(-1);
            let target_sqrt_price_x96 = target_sqrt_price.clone() * q96.clone();
  
            // Check TARGET PRICE exceeds TICK BOUNDRY
            let price_exceeds_range = target_sqrt_price_x96.to_integer().lt(&sqrt_price_lower_x96.clone()); 
            if price_exceeds_range {

                // Calculate AMOUNT IN | AMOUNT OUT
                let sqrt_price_lower = BigRational::new(sqrt_price_lower_x96.clone(), q96.clone());
                let amount_token_0_in = calculate_amount_0(price_range_liquidity.clone(), sqrt_price_lower.clone(), current_sqrt_price.clone());
                let amount_token_1_out = calculate_amount_1(price_range_liquidity.clone(), sqrt_price_lower.clone(), current_sqrt_price.clone());

                // Accumulate TOKENS IN/OUT
                token_in_amount_remaining -= amount_token_0_in.clone();
                total_token_in += amount_token_0_in;
                total_token_out += amount_token_1_out;

                // Get DELTA LIQUIDITY
                let delta_liquidity = lower_tick.liquidity_delta.clone();
             
                // ****** When crossing a lower tick range, subtract. Look at glmr_lp registry and examine delta liquidity.

                // Apply DELTA LIQUIDITY to ACTIVE LIQUIDITY
                price_range_liquidity = price_range_liquidity - delta_liquidity;

                // Set CURRENT TICK to LOWER TICK
                current_tick = BigInt::from(lower_tick.tick.clone());

                // Check ACTIVE LIQUIDITY
                if price_range_liquidity == BigInt::zero() {
                    // println!("Active liquidity is zero");
                    break;
                }

            } else {
                let amount_token_0_in = calculate_amount_0(price_range_liquidity.clone(), target_sqrt_price.clone(), current_sqrt_price.clone());  
                let amount_token_1_out = calculate_amount_1(price_range_liquidity.clone(), target_sqrt_price.clone(), current_sqrt_price.clone());

                token_in_amount_remaining = zero.clone();
                total_token_in += amount_token_0_in;
                total_token_out += amount_token_1_out;
            }
        } else {
        // 1 -> 0

            // Get TARGET PRICE from PRICE CHANGE
            let change_in_sqrt_price = token_in_amount_remaining.clone() / price_range_liquidity.clone();
            let target_sqrt_price_x96 = (current_sqrt_price.clone() + change_in_sqrt_price.clone()) * q96.clone();
            
            // Check TARGET PRICE exceeds TICK BOUNDRY
            let price_exceeds_range = target_sqrt_price_x96.to_integer().gt(&sqrt_price_upper_x96);
            if price_exceeds_range{
                
                let sqrt_price_upper = BigRational::new(sqrt_price_upper_x96.clone(), q96.clone());
                let amount_token_1_in = calculate_amount_1(price_range_liquidity.clone(), sqrt_price_upper.clone(), current_sqrt_price.clone());
                let amount_token_0_out = calculate_amount_0(price_range_liquidity.clone(), sqrt_price_upper.clone(), current_sqrt_price.clone());

                token_in_amount_remaining -= amount_token_1_in.clone();
                total_token_in += amount_token_1_in;
                total_token_out += amount_token_0_out;

                // Get DELTA LIQUIDITY
                let delta_liquidity = upper_tick.liquidity_delta.clone();

                // ****** When crossing a upper tick range, add. Look at glmr_lp registry and examine delta liquidity.

                // Apply DELTA LIQUIDITY to ACTIVE LIQUIDITY
                price_range_liquidity = price_range_liquidity + delta_liquidity;

                // Set CURRENT TICK to UPPER TICK
                current_tick = BigInt::from(upper_tick.tick.clone());
                
                // Check ACTIVE LIQUIDITY
                if price_range_liquidity == BigInt::zero() {
                    break;
                }
            } else {
                let target_sqrt_price = BigRational::new(target_sqrt_price_x96.to_integer().clone(), q96.clone());
                let amount_token_1_in = calculate_amount_1(price_range_liquidity.clone(), target_sqrt_price.clone(), current_sqrt_price.clone());
                let amount_token_0_out = calculate_amount_0(price_range_liquidity.clone(), target_sqrt_price.clone(), current_sqrt_price.clone());

                token_in_amount_remaining = zero.clone();
                total_token_in += amount_token_1_in;
                total_token_out += amount_token_0_out;
            
            }
        }
    }
    // println!("{:?}", total_token_out);
    total_token_out
}

pub fn calculate_dex_edge(adjacent_node: &NodePool, input_amount: BigInt) -> BigInt{
    // println!("DEX input amount: {}", input_amount);
    match adjacent_node{
        NodePool::Dex(dex_pool) => {
            calculate_v2_dex_swap_formula(&dex_pool, input_amount)

        },
        NodePool::DexV3(dex_v3_pool) => {
            calculate_v3_dex_swap(&dex_v3_pool, input_amount)
        },
        _ => panic!("Tried to get dex liquidity from non-dex liquidity"),
    }
    
}

pub fn calculate_bnc_polkadot_stable_edge(adjacent_node: &NodePool, input_amount: BigInt) -> BigInt{
    match adjacent_node{
        NodePool::Stable(stable_pair) => {
            // let (pool_id, swap_fee) = match &stable_pair.liquidity {
            //     Liquidity::Stable(stable_lp) => (stable_lp.pool_id.clone(), stable_lp.swap_fee),
            //     _ => panic!("Tried to get stable liquidity from non-stable liquidity"),
            // };
            // let swap_fee = BigInt::from(swap_fee);
            // let swap_fee_ratio = BigRational::new(swap_fee, BigInt::from(10000));
            // let input_amount = BigRational::new(input_amount, BigInt::one());
            // let output_amount = input_amount.clone() * (BigInt::one() - swap_fee_ratio.clone());
            // output_amount.to_integer()
            let temp = BigInt::from(9);
            temp
        },
        _ => panic!("Tried to get stable liquidity from non-stable liquidity"),
    }
}

// Calculate fees and output for edge to and from home chain
pub fn calculate_origin_xcm_edge(
    token_graph: &TokenGraph2, 
    fee_book: &TransferDepositFeeBook, 
    current_node: GraphNodePointer, 
    origin_node: GraphNodePointer, 
    adjacent_pair: &Xcm, 
    input_amount: BigInt
    ) -> (BigInt, BigInt, BigInt, BigInt, BigInt){ // Returns total_xcm_output (after fee deduction), transfer_reserve_amount, transfer_fee_amount, deposit_reserve_amount, deposit_fee_amount
    let relay_chain = current_node.borrow().get_relay_chain();
    let start_chain = current_node.borrow().get_chain_id();
    let dest_chain = adjacent_pair.xcm_node.borrow().get_chain_id();
    let asset_symbol = current_node.borrow().get_asset_symbol();
    let asset_origin_chain = current_node.borrow().asset.borrow().get_origin_chain_id().unwrap();
    let adjacent_node = adjacent_pair.xcm_node.clone();

    // Check if transfer is to home chain or away from home chain
    // Is start chain == origin chain?
    // yes: transfer is away from home chain
    // no: is dest chain == origin chain?
    // -- yes: transfer to home chain
    // -- no: transfer to home then dest

    let mut total_fees = BigInt::from(0);
    let mut deposit_fee_amount = BigInt::from(0);
    let mut deposit_reserve_amount = BigInt::from(0);
    let mut transfer_fee_amount = BigInt::from(0);
    let mut transfer_reserve_amount = BigInt::from(0);

    // Account for fees from transfer through home chain
    // If not transferring through home chain then skip fee deductions for this section
    if start_chain != asset_origin_chain && dest_chain != asset_origin_chain {
        // Get deposit data. If fee asset is different then transferred asset, calculate reserve. Else subtract fee normally from transferred amount
        // If no deposit fee data then skip
        let deposit_fee_data = fee_book.get_deposit_fee_data(origin_node.clone());
        if let Some(fee_data) = deposit_fee_data {
            let deposit_fee_node_option = token_graph.get_asset_by_chain_and_id(origin_node.borrow().get_chain_id(), fee_data.get_fee_asset_id());
            let deposit_fee_node = match deposit_fee_node_option {
                Some(node) => node,
                None => panic!("Token graph cannot find asset node for Chain ID(Origin): {} | ID(fee_asset): {}", origin_node.borrow().get_chain_id(), fee_data.get_fee_asset_id()),
            };
            deposit_fee_amount = BigInt::from_str(fee_data.feeAmount.unwrap().as_str()).unwrap();

            if deposit_fee_node.as_ptr().eq(&origin_node.as_ptr()){
                // fee_amount_to_subtract = transfer_fee_amount.clone();
                total_fees += deposit_fee_amount.clone();
            } else {
                deposit_reserve_amount = token_graph.convert_transfer_fee_amount_to_current_node(deposit_fee_node.clone(), current_node.clone(), deposit_fee_amount.clone());
                total_fees += deposit_reserve_amount.clone();
            }
        }


        // let deposit_fee_amount: BigInt = match fee_book.get_deposit_fee_data(origin_node.clone()) {
        //     Some(fee_data) => {
        //         BigInt::from_str(fee_data.feeAmount.unwrap().as_str()).unwrap()
        //     }, 
        //     None => BigInt::from(0)
        // };
        // total_fees += deposit_fee_amount;

        // If no transfer fee data then skip
        let transfer_fee_data = fee_book.get_transfer_fee_data(origin_node.clone());
        if let Some(fee_data) = transfer_fee_data {
            let transfer_fee_node_option = token_graph.get_asset_by_chain_and_id(origin_node.borrow().get_chain_id(), fee_data.get_fee_asset_id());
            let transfer_fee_node = match transfer_fee_node_option {
                Some(node) => node,
                None => panic!("Token graph cannot find asset node for Chain ID(Origin): {} | ID(fee_asset): {}", origin_node.borrow().get_chain_id(), fee_data.get_fee_asset_id()),
            };
            transfer_fee_amount = BigInt::from_str(fee_data.feeAmount.unwrap().as_str()).unwrap();

            
            if transfer_fee_node.as_ptr().eq(&origin_node.as_ptr()){
                // fee_amount_to_subtract = transfer_fee_amount.clone();
                total_fees += transfer_fee_amount.clone();
            } else {
                transfer_reserve_amount = token_graph.convert_transfer_fee_amount_to_current_node(transfer_fee_node.clone(), current_node.clone(), transfer_fee_amount.clone());
                total_fees += transfer_reserve_amount.clone();
            }
        };

    }



    // if start_chain == asset_origin_chain || dest_chain == asset_origin_chain {
    //     // Transfer is away from home chain, or to home chain
    //     // DO NOTHING

    // }

    //     println!("2. (T) fee: {} | reserve: {}", fee_amount_to_subtract, reserve_amount);
    // }


    let xcm_output = input_amount.checked_sub(&total_fees).unwrap();
    (xcm_output, transfer_reserve_amount, transfer_fee_amount, deposit_reserve_amount, deposit_fee_amount)
}

pub fn get_sqrt_ratio_at_tick(tick: i32) -> BigInt {
    assert!(tick >= -887272 && tick <= 887272, "TICK");


    let abs_tick = tick.abs() as u32;

    let max_uint256: BigInt = BigInt::from_str_radix("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff", 16).unwrap();
    let q32: BigInt = BigInt::from(2).pow(32);
    let one: BigInt = BigInt::from(1);
    let zero: BigInt = BigInt::from(0);
    let mut ratio: BigInt = if (abs_tick & 0x1) != 0 {
        BigInt::from_str_radix("fffcb933bd6fad37aa2d162d1a594001", 16).unwrap()
    } else {
        BigInt::from_str_radix("100000000000000000000000000000000", 16).unwrap()
    };

    if (abs_tick & 0x2) != 0 { ratio = mul_shift(&ratio, "fff97272373d413259a46990580e213a"); }
    if (abs_tick & 0x4) != 0 { ratio = mul_shift(&ratio, "fff2e50f5f656932ef12357cf3c7fdcc"); }
    if (abs_tick & 0x8) != 0 { ratio = mul_shift(&ratio, "ffe5caca7e10e4e61c3624eaa0941cd0"); }
    if (abs_tick & 0x10) != 0 { ratio = mul_shift(&ratio, "ffcb9843d60f6159c9db58835c926644"); }
    if (abs_tick & 0x20) != 0 { ratio = mul_shift(&ratio, "ff973b41fa98c081472e6896dfb254c0"); }
    if (abs_tick & 0x40) != 0 { ratio = mul_shift(&ratio, "ff2ea16466c96a3843ec78b326b52861"); }
    if (abs_tick & 0x80) != 0 { ratio = mul_shift(&ratio, "fe5dee046a99a2a811c461f1969c3053"); }
    if (abs_tick & 0x100) != 0 { ratio = mul_shift(&ratio, "fcbe86c7900a88aedcffc83b479aa3a4"); }
    if (abs_tick & 0x200) != 0 { ratio = mul_shift(&ratio, "f987a7253ac413176f2b074cf7815e54"); }
    if (abs_tick & 0x400) != 0 { ratio = mul_shift(&ratio, "f3392b0822b70005940c7a398e4b70f3"); }
    if (abs_tick & 0x800) != 0 { ratio = mul_shift(&ratio, "e7159475a2c29b7443b29c7fa6e889d9"); }
    if (abs_tick & 0x1000) != 0 { ratio = mul_shift(&ratio, "d097f3bdfd2022b8845ad8f792aa5825"); }
    if (abs_tick & 0x2000) != 0 { ratio = mul_shift(&ratio, "a9f746462d870fdf8a65dc1f90e061e5"); }
    if (abs_tick & 0x4000) != 0 { ratio = mul_shift(&ratio, "70d869a156d2a1b890bb3df62baf32f7"); }
    if (abs_tick & 0x8000) != 0 { ratio = mul_shift(&ratio, "31be135f97d08fd981231505542fcfa6"); }
    if (abs_tick & 0x10000) != 0 { ratio = mul_shift(&ratio, "9aa508b5b7a84e1c677de54f3e99bc9"); }
    if (abs_tick & 0x20000) != 0 { ratio = mul_shift(&ratio, "5d6af8dedb81196699c329225ee604"); }
    if (abs_tick & 0x40000) != 0 { ratio = mul_shift(&ratio, "2216e584f5fa1ea926041bedfe98"); }
    if (abs_tick & 0x80000) != 0 { ratio = mul_shift(&ratio, "48a170391f7dc42444e8fa2"); }

    if tick > 0 {
        ratio = &max_uint256 / &ratio;
    }

    if &ratio % &q32 > zero {
        ratio = (&ratio / &q32) + &one;
    } else {
        ratio = &ratio / &q32;
    }
    ratio
}

fn mul_shift(val: &BigInt, mul_by: &str) -> BigInt {
    let multiplier = BigInt::from_str_radix(mul_by, 16).expect("Invalid BigInt string");
    let result = val * multiplier;
    let shift_amount = BigInt::from(2).pow(128);
    result / shift_amount
}

pub fn calculate_amount_0(liq: BigInt, pa: BigRational, pb: BigRational) -> BigInt {
    let mut p_a = pa.clone();
    let mut p_b = pb.clone();
    if pa > pb {
        p_a = pb;
        p_b = pa;
    }
    // println!("Liquidity {}", liq.to_u128().unwrap());
    // println!("HIGHER VALUE {}", p_b.to_f64().unwrap());
    // println!("LOWER VALUE {}", p_a.to_f64().unwrap());

    // let t1 = p_b.clone() - p_a.clone();
    // let t2 = t1.clone() / p_b.clone();
    // let t3 = t2.clone() / p_a.clone();
    // let t4 = BigRational::new(liq.clone(), BigInt::one()) * t3.clone();

    // println!("T1: {}", t1.to_f64().unwrap());
    // println!("T2: {}", t2.to_f64().unwrap());
    // println!("T3: {}", t3.to_f64().unwrap());
    // println!("T4: {}", t4.to_f64().unwrap());

    let amount = BigRational::new(liq, BigInt::one()) * ((p_b.clone() - p_a.clone()) / p_b.clone() / p_a.clone()); 
    // println!("CALCLULATED AMOUNT OUT {}", amount.to_integer());
    amount.to_integer()
}

pub fn calculate_amount_1(liq: BigInt, pa: BigRational, pb: BigRational) -> BigInt {
    let mut p_a = pa.clone();
    let mut p_b = pb.clone();
    if pa > pb {
        p_a = pb;
        p_b = pa;
    }
    // println!("HIGHER VALUE {}", p_b.to_f64().unwrap());
    // println!("LOWER VALUE {}", p_a.to_f64().unwrap());
    let amount = BigRational::new(liq, BigInt::one()) * (p_b.clone() - p_a.clone());
    amount.to_integer()
}

pub fn calculate_cex_edge(token_graph: &TokenGraph2, primary_node: &GraphNodePointer, adjacent_pair: &NodePool, input_amount: u128) -> u128{
    match adjacent_pair {
        NodePool::Cex(adj_pair) => {
            let (bid_price, bid_decimals, ask_price, ask_decimals ) = match adj_pair.liquidity {
                LiquidityPool::Cex(cexLp) => (cexLp.bid_price, cexLp.bid_decimals, cexLp.ask_price, cexLp.ask_decimals),
                _ => panic!("Tried to get dex liquidity from non-dex liquidity"),
            };
            if primary_node.borrow().get_asset_symbol() != "USDT"{
                let usdt_token_decimals = 4;
                let asset_decimals = token_graph.get_asset_decimals_for_kucoin_asset(primary_node);

                //Convert number to normal, display value
                let converted_input = input_amount as f64 / f64::powi(10.0, asset_decimals as i32);
                let converted_bid = bid_price.clone() as f64 / f64::powi(10.0, bid_decimals as i32);

                let asset_output = converted_input * converted_bid;
                let asset_output_converted = asset_output * f64::powi(10.0, usdt_token_decimals as i32) ;
                asset_output_converted as u128
            }
            //USDT -> Asset
            else {
                let usdt_token_decimals = 6;
                let asset_decimals = token_graph.get_asset_decimals_for_kucoin_asset(&adj_pair.pool_nodes);

                //Convert number to normal, display value
                let converted_input = input_amount as f64 / f64::powi(10.0, usdt_token_decimals as i32);
                let converted_ask = bid_price.clone() as f64 / f64::powi(10.0, ask_decimals as i32);
                

                let asset_output = converted_input as f64 / converted_ask;
                let asset_output_converted = asset_output * f64::powi(10.0, asset_decimals as i32) ;
                asset_output_converted as u128
            }    
        },
        _ => panic!("Tried to get dex liquidity from non-dex liquidity"),
    }
}

pub fn calculate_stable_edge(primary_node: &GraphNodePointer, node_pool: &NodePool, input_amount: BigInt, target_index: usize) -> Option<BigInt>{

    match node_pool{
        NodePool::Stable(node_pool) => {
            // println!("Calc stable swap");
            let relay = primary_node.borrow().get_relay_chain();
            let chain_id = primary_node.borrow().get_chain_id().to_u128().unwrap();

            // Bifrost Polkadot is modified
            if relay == "polkadot" && chain_id == 2030 {
                panic!("Calling calculate stable swap on BNC Polkadot is not allowed");
            }
            let input_index = node_pool.base_asset_index;

            let pool_data: StableData = match &node_pool.liquidity {
                LiquidityPool::Stable(pool_data) => pool_data.clone(),
                _ => panic!("Tried to get dex liquidity from non-dex liquidity"),
            };
            let base_liquidity = pool_data.pool_liquidity[input_index].to_bigint().unwrap();
            let base_token_precision = pool_data.token_precisions[input_index];
            let pool_liquidity: Vec<BigInt> = pool_data.pool_liquidity.clone().into_iter().map(|x| x.to_bigint().unwrap()).collect();
            let a = pool_data.a;
            let pool_id: usize = pool_data.pool_id.clone().unwrap().parse().unwrap();
            let total_supply = pool_data.total_supply.to_bigint().unwrap();
            let token_precisions = pool_data.token_precisions.clone();

            let input_asset_symbol = primary_node.borrow().get_asset_symbol();
            let output_asset_symbol = node_pool.pool_nodes[target_index].borrow().get_asset_symbol();
            // println!("{} {} -> {}", chain_id, input_asset_symbol, output_asset_symbol);
            let a_precision = match pool_data.chain_id {
                2023 => 1,
                2000 => 100,
                _ => 1
            };
            let mut balances: Vec<BigInt> = pool_liquidity.clone().into_iter().map(|x| x.to_bigint().unwrap()).collect();
            let mut token_precisions = token_precisions.clone();
            balances.push(base_liquidity.clone().to_bigint().unwrap());
            token_precisions.push(base_token_precision.clone());


            // KARURA is fucked up. The token precision has already been added to the number that is shown as the balance in the pool. So don't multiply them for pool 1 in karura
            if chain_id != 2000 && pool_id != 1 {
                // println!("Skipping TOKEN PRECISION for karura pool 1");
                balances = balances.iter().enumerate().map(|(index, balance)| {
                    let precision;
                    if index >= token_precisions.len(){
                        precision = 1;
                    } else {
                        precision = token_precisions[index];
                    }
                    let updated_balance = balance * precision;
                    // println!("Balance: {}", updated_balance);
                    updated_balance
                }).collect::<Vec<BigInt>>();

            }


            let base_precision = base_token_precision.to_string().len() - 1;
            let primary_node_symbol = primary_node.borrow().get_asset_symbol();

            let input_amount = input_amount * f64::powi(10.0, base_precision as i32) as u128;

            let swap_fee = BigInt::from(pool_data.swap_fee);
            // let fee_precision =  "10000000000".parse::<BigInt>().unwrap();
            let fee_precision = BigInt::from(pool_data.fee_precision.clone());
            let fee_ration = BigRational::from(swap_fee.clone()) / BigRational::from(fee_precision.clone());

            // println!("Balances: {:?}", balances);
            let d = get_d(&balances, a, chain_id.clone(), a_precision).unwrap();


            let token_in_index = balances.len() - 1;

            // Convert input amount to shares on certain pools
            let converted_a = BigInt::from(a).checked_div(&BigInt::from(a_precision)).unwrap();
            if converted_a == BigInt::from(30) {

                // Convert input to shares then add input to share reserves and calculate output
                if input_asset_symbol.eq_ignore_ascii_case("LDOT") || input_asset_symbol.eq_ignore_ascii_case("LKSM"){
                    let input_asset_reserves = balances[2].clone();
                    let input_asset_shares = balances[3].clone();
                    let input_percent = BigRational::from(input_amount.clone()).checked_div(&BigRational::from(input_asset_reserves.clone())).unwrap();
                    let input_amount_converted_to_shares = input_percent.checked_mul(&BigRational::from(input_asset_shares.clone())).unwrap();
                    
                    // actual_input_amount = input_amount_converted_to_shares.to_integer();
                    balances[token_in_index] = balances[token_in_index].clone() + input_amount_converted_to_shares.to_integer();
                } else {
                    // println!("Dot input");
                    // Add input to share reserves, calculate output and then later convert output to reserves
                    balances[token_in_index] = balances[token_in_index].clone() + input_amount.clone();
                }
            } else {
                // Pool just uses reserve balances
                balances[token_in_index] = balances[token_in_index].clone() + input_amount.clone();
            }

            let y = get_y(&balances, target_index as u128, d.clone(), a, a_precision, pool_data.chain_id.clone() as u128).unwrap();


            let dy = BigRational::from(balances[target_index].clone() - y.clone());


            let mut total_output = BigInt::from(0);
            if a == 3000{
                // println!("DOT/LDOT");
                // Output is already actual amount
                if primary_node_symbol == "LKSM" || primary_node_symbol == "LDOT"{
                    total_output = dy.round().numer().to_bigint().unwrap();
                } else {
                // Output is in shares, and need to convert to reserve amount
                    let output_asset_reserves = balances[2].clone();
                    let output_asset_shares = balances[0].clone();
                    let output_amount_shares_ratio = BigRational::from(dy.clone()).checked_div(&BigRational::from(output_asset_shares.clone())).unwrap();
                    let output_amount_converted_to_reserves = output_amount_shares_ratio.checked_mul(&BigRational::from(output_asset_reserves.clone())).unwrap();
                    total_output = output_amount_converted_to_reserves.to_integer();
                }
            } else{
                total_output = total_output.to_bigint().unwrap() + dy.round().numer();
            }

            let fee_amount = BigRational::from(total_output.clone()) * fee_ration.clone();
            // println!("FEE AMOUNT: {}", fee_amount.round().numer().to_bigint().unwrap());
            total_output = total_output - fee_amount.round().numer();
            // println!("TOTAL OUTPUT: {}", total_output);

            let target_precision = token_precisions[target_index];
            // let target_precision_number = target_precision.to_string().len() - 1;

            // println!("TARGET PRECISION: {}", target_precision.to_f64().unwrap());
            // println!("D: {}", d);
            // println!("Y: {}", y); 
            // println!("DY: {}", dy.to_integer());

            let total_output_minus_precision = total_output.checked_div(&BigInt::from(target_precision)).unwrap();
            // println!("TOTAL OUTPUT MINUS PRECISION: {}", total_output_minus_precision);
            total_output_minus_precision.to_bigint()
            
        },
        NodePool::BncStable(adj_pair) => {
            let stable_pool_data: BncStableData = match &adj_pair.liquidity {
                LiquidityPool::BncStable(stable_pool_data) => stable_pool_data.clone(),
                _ => panic!("Tried to get dex liquidity from non-dex liquidity"),
            };

            let pool_liquidity: Vec<BigInt> = stable_pool_data.pool_liquidity.clone().into_iter().map(|x| x.to_bigint().unwrap()).collect();
            let mut pool_shares: Vec<BigInt> = stable_pool_data.token_shares.clone().into_iter().map(|x| x.to_bigint().unwrap()).collect();
            
            // println!("POOL SHARES: {:?}", pool_shares.clone());
            // println!("POOL LIQUIDITY: {:?}", pool_liquidity.clone());

            let token_precisions = stable_pool_data.token_precisions.clone();
            let amplification = stable_pool_data.a.clone();
            let swap_fee = stable_pool_data.swap_fee.clone();
            let fee_denominator = BigInt::from(stable_pool_data.fee_precision.clone());
            let token_rates: Vec<TokenRate> = stable_pool_data.token_rates.clone();

            // Shouldn't need to calculate D because total supply is accurate
            let total_supply = BigInt::from(stable_pool_data.total_supply.clone());

            let input_index = adj_pair.base_asset_index.clone();
            let target_index = target_index.clone();

            let input_asset_liquidity = BigInt::from(pool_liquidity[input_index].clone());
            let output_asset_liquidity = BigInt::from(pool_liquidity[target_index].clone());

            // println!("Input Index: {} | Output Index {}", input_index, target_index);

            let input_numerator = BigInt::from(token_rates[input_index].numerator);
            let input_denominator = BigInt::from(token_rates[input_index].denominator);

            let input_percent: BigRational = BigRational::from(input_amount.clone()).checked_div(&BigRational::from(input_asset_liquidity.clone())).unwrap();
            let input_amount_as_shares = (BigRational::from(BigInt::from(pool_shares[input_index].clone())).checked_mul(&input_percent).unwrap()).floor().to_integer();
        
            pool_shares[input_index] = pool_shares[input_index].clone().add(input_amount_as_shares.clone().mul(token_precisions[input_index]));
            // pool_liquidity[input_index] = pool_liquidity[input_index].clone().add(input_amount.clone());
        
            // console.log(`${d.toFixed()} -- D NEW`)
            // let yShares = calculateYBifrost(poolShares, outputAssetIndex, dNew, amplification)
            let y_shares = get_y_bnc(&pool_shares, target_index, total_supply.clone(), amplification).unwrap();
            // println!("Y SHARES: {}", y_shares.clone());

            let mut dy_shares = pool_shares[target_index].clone()  // THIS is out calculated output amount
                .checked_sub(&y_shares).unwrap()
                .checked_sub(&BigInt::from(1)).unwrap()
                .checked_div(&BigInt::from(token_precisions[target_index].clone())).unwrap();

            let fee_amount_shares = dy_shares.clone()
                .mul(swap_fee)
                .checked_div(&fee_denominator).unwrap(); 

            dy_shares = dy_shares.checked_sub(&fee_amount_shares).unwrap();

            let output_percent = BigRational::from(dy_shares.clone()).checked_div(&BigRational::from(pool_shares[target_index].clone())).unwrap();
            // let output_amount = output_asset_liquidity.mul(output_percent).integerValue(BigNumber.ROUND_DOWN)
            let output_amount = output_percent.mul(output_asset_liquidity).floor().to_integer();
            // println!("Input amount: {} | Output amount: {}", input_amount, output_amount);
            // println!("Input Shares: {} | Output Shares: {}", input_amount_as_shares, dy_shares);
            Some(output_amount)
            // .mul(output_percent).integerValue(BigNumber.ROUND_DOWN)
            

            // let dNew = get_d(pool_shares, amplification).integerValue(BigNumber.ROUND_DOWN)

            // let base_liquidity = lp_data.base_liquidity.to_bigint().unwrap();
            // let base_token_precision = lp_data.base_token_precision;
            // let adjacent_liquidity: Vec<BigInt> = lp_data.adjacent_liquidity.clone().into_iter().map(|x| x.to_bigint().unwrap()).collect();
            // let a = lp_data.a;
            // let a_precision = 100;
            // let pool_id: usize = lp_data.pool_id.clone().unwrap().parse().unwrap();
            // let total_supply = lp_data.total_supply.to_bigint().unwrap();
            // let adjacent_token_precisions = lp_data.adjacent_token_precisions.clone();
            // let token_rates: Vec<TokenRate> = lp_data.token_rates.unwrap();
            // let token_shares: Vec<BigInt> = lp_data.token_shares.unwrap().into_iter().map(|x| x.to_bigint().unwrap()).collect();
            // let mut balances: Vec<BigInt> = adjacent_liquidity.clone().into_iter().map(|x| x.to_bigint().unwrap()).collect();
            // let mut token_precisions = adjacent_token_precisions.clone();
            // balances.push(base_liquidity.clone().to_bigint().unwrap());
            // let swap_fee = BigInt::from(lp_data.swap_fee);
            // let fee_denominator = BigInt::from_str("10000000000").unwrap();



            // return BigInt::from_str("000").ok();
        },
        NodePool::StableShare(stable_share_pool) => {
            // println!(" BALANCES 3{:?}", adj_pair.liquidity);
            // let (base_liquidity, base_token_precision, adjacent_liquidity, a, total_supply, adjacent_token_precisions) = match &adj_pair.liquidity {
            //     Liquidity::Stable(stableLp) => (stableLp.base_liquidity, stableLp.base_token_precision, stableLp.adjacent_liquidity.clone(), stableLp.a, stableLp.total_supply, stableLp.adjacent_token_precisions.clone()),
            //     _ => panic!("Tried to get dex liquidity from non-dex liquidity"),
            // };
            let token_to_share = stable_share_pool.token_to_share;
            let stable_pool_data: StableData = match &stable_share_pool.liquidity {
                LiquidityPool::Stable(stable_pool_data) => stable_pool_data.clone(),
                _ => panic!("Tried to get stable liquidity from non-dex liquidity"),
            };

            // println!("------------------");

            let a_precision = match stable_pool_data.chain_id {
                2023 => 1,
                2000 => 100,
                _ => 1
            };
            
            // println!("------------------");
            // println!("STABLE SWAP");
            // let token_to_share = lp_data.token_to_share.unwrap();



            // let mut balances: Vec<BigInt> = lp_data.pool_assets_liquidity.clone().into_iter().map(|x| x.to_bigint().unwrap()).collect();
            
            // balances.push(lp_data.base_liquidity.clone().to_bigint().unwrap());
            // token_precisions.push(lp_data.base_token_precision.clone());
            let all_pool_balances: Vec<BigInt> = stable_pool_data.pool_liquidity.clone().into_iter().map(|x| x.to_bigint().unwrap()).collect();
            let share_issuance = BigInt::from(stable_pool_data.share_issuance.unwrap());
            let token_precisions = stable_pool_data.token_precisions.clone();

            let primary_node_decimals = primary_node.borrow().get_asset_decimals();
            let primary_node_symbol = primary_node.borrow().get_asset_symbol();
            
            // let fee_precision = BigInt::from(10000000000.to_u128().unwrap());
            let fee = stable_share_pool.get_swap_fee();
            let fee_precision =  "10000000000".parse::<BigInt>().unwrap();

            // println!("all_pool_balances: {:?}", all_pool_balances);

            let reserves_normalized = all_pool_balances
                .iter()
                .enumerate()
                .map(|(index, &ref x)|{
                    let precision = token_precisions[index];
                    let normalized = x.mul(precision.clone().to_bigint().unwrap());
                    normalized
                }).collect::<Vec<BigInt>>();
            
            // println!("RESERVES NORMALIZED: {:?}", reserves_normalized);
            
            // let fee_formatted = fee * fee_precision.to_f64().unwrap();
            // let fee_ration = BigRational::from(BigInt::from(fee_formatted.round().to_u128().unwrap())) / BigRational::from(fee_precision.clone());
            let fee_ration = BigRational::from_u128(fee).unwrap() / BigRational::from(BigInt::from(fee_precision.clone()));

            if stable_share_pool.token_to_share{
                // Base asset -> Share asset
                let input_asset_index = stable_share_pool.base_asset_index.unwrap();
                let input_asset_precision = token_precisions[input_asset_index];
                let share_asset = stable_pool_data.share_asset.clone();

                // println!("Base precision: {}", base_precision);
                // println!("Primary node decimals: {}", primary_node_decimals);
                // println!("Input Amount: {}", input_amount);
                // println!("Total node decimals: {}", primary_node_decimals as u128);
                let total_node_decimals = input_asset_precision as u128 + primary_node_decimals as u128;
                // let input_asset_index = stable_share_data.base_asset_index.unwrap();
                // let input_amount = input_amount * f64::powi(10.0, base_precision as i32) as u128;
                // let input_amount = input_amount * base_precision;
                // let input_formatted = input_amount.checked_mul(&BigInt::from(10).pow(primary_node_decimals as u32)).unwrap();
                // let input_formatted = input_amount.to_f64().unwrap() / f64::powi(10.0, total_node_decimals as i32);
                let mut updated_reserves = all_pool_balances.clone();

                // updated_reserves[input_asset_index] = ;
                // println!("Input Amount: {}", input_amount);
                // println!("Input Formatted: {}", input_formatted);
                // println!("Reserves: {:?}", reserves_normalized);
                // println!("Updated Reserves: {:?}", updated_reserves);
                updated_reserves[input_asset_index] = updated_reserves[input_asset_index].clone() + input_amount;
                
                // println!("Updated Reserves: {:?}", updated_reserves);
                let updated_reserves_normalized = updated_reserves
                    .iter()
                    .enumerate()
                    .map(|(index, &ref x)|{
                        let precision = token_precisions[index];
                        let normalized = x.mul(precision.clone().to_bigint().unwrap());
                        normalized
                    }).collect::<Vec<BigInt>>();


                

                let d_0 = get_d(&reserves_normalized, stable_pool_data.a, stable_pool_data.chain_id.into(), a_precision).unwrap();
                let d_1: BigInt = get_d(&updated_reserves_normalized, stable_pool_data.a, stable_pool_data.chain_id.into(), a_precision).unwrap();




                let initial_input_reserve = reserves_normalized[input_asset_index].clone();
                let updated_input_reserve = updated_reserves_normalized[input_asset_index].clone();
                let mut adjusted_reserves = reserves_normalized.clone() ;

                let ideal_balance = d_1.checked_mul(&initial_input_reserve)?.checked_div(&d_0)?;
                let diff = updated_input_reserve.checked_sub(&ideal_balance)?.abs();
                let fee_amount = fee_ration.checked_mul(&BigRational::new(diff, BigInt::one()))?;
                let fee_amount_int = fee_amount.round().numer().to_bigint().unwrap();
                let adjusted_input_reserve = updated_input_reserve.checked_sub(&fee_amount_int)?.to_bigint().unwrap();
                adjusted_reserves[input_asset_index] = adjusted_input_reserve.clone();

                updated_reserves[input_asset_index] = adjusted_input_reserve.clone();

                let adjusted_d = get_d(&adjusted_reserves, stable_pool_data.a, stable_pool_data.chain_id.into(), a_precision).unwrap();

                // println

                // println!("D0: {}", d_0);
                // println!("D1: {}", d_1);
                // println!("Adjusted D: {}", adjusted_d);
                // println!("Fee Amount: {}", fee_amount_int);


                let d_diff = adjusted_d.checked_sub(&d_0)?;

                // println!("D Diff: {}", d_diff);

                let share_amount = d_diff.checked_mul(&share_issuance)?.checked_div(&d_0)?;
                let share_amount_formatted = share_amount.checked_div(&BigInt::from(SHARE_PRECISION));

                // println!("Share Amount: {:?}", share_amount_formatted);
                // println!("Share Amount: {}", share_amount);

                return Some(share_amount)
            } else {
                // let input_shares_amount = input_amount
                // Target asset is base asset
                let initial_d = get_d(&reserves_normalized, stable_pool_data.a, stable_pool_data.chain_id.into(), a_precision).unwrap();
                let d_1 = initial_d.clone().add(input_amount.mul(&initial_d).checked_div(&share_issuance)?);
                let y = get_y(&reserves_normalized, target_index as u128, d_1.clone(), stable_pool_data.a, a_precision, stable_pool_data.chain_id.into()).unwrap();

                // println!("Initial D: {} | D1: {} | Y: {}", initial_d, d_1, y);

                let mut reserves_reduced: Vec<BigInt> = vec![];
                for (index, &ref x) in reserves_normalized.iter().enumerate() {
                    let dx_expected = if index == target_index {
                        x.checked_mul(&d_1)?.checked_div(&initial_d)?.checked_sub(&y)?
                    } else {
                        x.checked_sub(&x.checked_mul(&d_1)?.checked_div(&initial_d)?)?
                    };
                    let fee_amount = fee_ration
                        .checked_mul(&BigRational::new(dx_expected.clone(), BigInt::one()))?
                        .round()
                        .numer()
                        .to_bigint()
                        .unwrap();

                    let reduced = x.checked_sub(&fee_amount)?;
                    reserves_reduced.push(reduced);
                }

                let y_1 = get_y(&reserves_reduced, target_index as u128, d_1.clone(), stable_pool_data.a, a_precision, stable_pool_data.chain_id as u128).unwrap();
                let dy = reserves_reduced[target_index].checked_sub(&y_1)?;
                let dy_0 = reserves_normalized[target_index].checked_sub(&y)?;

                // println!("dy: {}", dy);
                // println!("dy_0: {}", dy_0);

                let calculated_fee = dy_0.checked_sub(&dy)?;
                let dy_minus_fee = dy.checked_sub(&calculated_fee)?;
                // println!("Calculated Fee: {}", calculated_fee);

                let base_precision = token_precisions[target_index];
                let calculated_amount_out = dy_minus_fee.checked_div(&BigInt::from(base_precision))?;
                // println!("Calculated Amount Out: {}", calculated_amount_out);

                // let amount_minus_fee = calculated_amount_out.checked_sub(&calculated_fee)?;

                // println!("base precision: {}", base_precision);
                // println!("Amount Minus Fee: {}", amount_minus_fee);

                return Some(calculated_amount_out.abs())
            }
            
        },
        _ => panic!("Tried to get stable liquidity from non-stable liquidity")
    }
}

// pub fn normalize_liquidity_reserves(balances: Vec<BigInt>, assets: Vec<>) -> Vec<BigInt> {
//     let mut balances = balances.clone();
    
// }

pub fn get_d(balances: &Vec<BigInt>, a: u128, chain_id: u128, a_precision: u128) -> Option<BigInt> {
    let zero = BigInt::from(0u128);
    let one = BigInt::from(1u128);
    let two = BigInt::from(2u128);
    let mut sum = BigInt::from(0u128);
    let mut ann = BigInt::from(a);
    let mut balance_size = BigInt::from(balances.len());
    let mut a_precision = BigInt::from(a_precision);
    let mut ann_formatted: BigRational;
    let mut balances = balances.clone();
    
    let max_iterations = match chain_id {
        2034 => MAX_D_ITERATIONS_HDX,
        _ => MAX_D_ITERATIONS_ACA,
    };
    ann = ann.checked_div(&a_precision).unwrap();

    // For KAR stables. ann = futureA / a_precision
    // Like the second stable pool, futureA is 10000, precision is 100. so the amplification is 100
    // Also I dont fully understand the mechanics of the kar pool. The first pool has 4 liquidity stats for 2 assets,
    // KSM and LKSM balance is 2 and 3
    // 0 and 1 are balances of KSM and LKSM, The LKSM balance is different than the actual LKSM in the pool account.
    // Total supply is 0 + 1. So i think thats what we use to calculate D. and that will corrspond to 0 and 3 after we take primary node balance out and add it to the end
    // For HDX stables, a precision is 1


    // Kar the amplification is logged as a * a_precision, to avoid a decimal number.So to get a just divide by a_precision


    // println!("ANN: {}", ann);
    if ann == BigInt::from(30u128){
        balances = vec![balances[0].clone(), balances[3].clone()];
    }

    let n_coins = BigInt::from(balances.len());

    // println!("Balances: {:?}", balances);

    for reserve in balances.iter(){
        sum = sum.checked_add(&reserve)?;
        ann = ann.checked_mul(&n_coins)?;
        // ann_formatted = ann_formatted.checked_mul(&n_coins)?;
    }

    if sum == zero {
        return Some(zero);
    }
    
    // let mut prev_d: BigInt;
    let mut d: BigInt = sum.clone();
    for i in 0..max_iterations{
        let mut p_d: BigInt = d.clone();
        for balance in balances.iter(){
            let div_op = balance.checked_mul(&n_coins)?;
            p_d = p_d.checked_mul(&d)?.checked_div(&div_op)?;
        }
        let prev_d = d.clone();

        // println!("ANN: {} | SUM: {} | P_D: {} | N_COINS: {}", ann, sum, p_d, n_coins);
// 
        let t_1 = ann.checked_mul(&sum).unwrap().checked_add(&p_d.checked_mul(&n_coins).unwrap())?;
        let t_2 = ann.checked_sub(&one)?.checked_mul(&d)?;
        let t_3 = n_coins.checked_add(&one)?.checked_mul(&p_d)?;
        let t_4 = t_2.checked_add(&t_3)?;

        // println!("T1: {}", t_1);
        // println!("T2: {}", t_2);
        // println!("T3: {}", t_3);
        // println!("T4: {}", t_4);


        d = t_1.checked_mul(&d)?.checked_div(&t_4)?.clone();

        if has_converged(&prev_d, &d){
            // println!("Number of convergences: {}", i);
            break;
        }
    }
    return Some(d);
}

pub fn has_converged(v_0: &BigInt, v_1: &BigInt) -> bool{
    let diff = v_0.checked_sub(&v_1).unwrap().abs();
    if v_1.gt(&v_0) && diff.le(&BigInt::from(1)){
        return true;
    } else if v_1.le(&v_0) && diff.le(&BigInt::from(1)){
        return true;
    }
    return false;
}

pub fn get_stable_swap_amount(swapFee: BigInt, totalSupply: BigInt, a: BigInt, precisions: Vec<BigInt>, balances: Vec<BigInt>){
    let swap_a = a.clone();
    let swap_d = totalSupply.clone();
    let fee_precisions = "10000000000".parse::<BigInt>().unwrap();
    let swap_balances = balances.clone();
}

pub fn get_y(balances: &Vec<BigInt>, target_index: u128, target_d: BigInt, amplitude: u128, a_precision: u128, chain_id: u128) -> Option<BigInt> {
    // println!("get_y");
    // balances.iter().for_each(|x| println!("{}", x));

    let one = BigInt::from(1);
    let two = BigInt::from(2);
    let mut c = BigRational::from(target_d.clone());
    let mut sum = BigInt::from(0);
    let ann_initial = BigInt::from(amplitude);
    
    let mut ann = ann_initial.clone();
    let target_d_bigint = BigInt::from(target_d.clone());
    let token_index_usize = target_index as usize;
    // let a_precision = BigInt::from(100);
    let a_precision = BigInt::from(a_precision);
    let mut balances = balances.clone();


    let max_iterations = match chain_id {
        2034 => MAX_Y_ITERATIONS_HDX,
        _ => MAX_Y_ITERATIONS_ACA,
    };

    ann = ann.checked_div(&a_precision).unwrap();
    if ann == BigInt::from(30u128){
        // println!("ANN: {}", ann);
        balances = vec![balances[0].clone(), balances[3].clone()];
    }

    // println!("Balances should be shares: ");
    // println!("Balances: {:?}", balances);

    let n_coins = BigInt::from(balances.len());

    for (i, balance_ref) in balances.iter().enumerate() {
        ann = &ann * &n_coins;
        if i == token_index_usize {
            continue;
        }
        sum = sum.add(balance_ref);
        let div_op = balance_ref.checked_mul(&n_coins).unwrap();
        c = c.checked_mul(&BigRational::from(target_d_bigint.clone())).unwrap().checked_div(&BigRational::from(div_op.clone())).unwrap();
        // println!("{} --- Sum: {} | C: {} | Current Balance: {} | Div Op: {}", i, sum, c, balance_ref, div_op);
    }
    // println!("SUM: {} | C: {} | Target D {} | A Precision {} | N Coins: {} | Ann {}", sum, c.to_f64().unwrap(), target_d_bigint,  a_precision, n_coins, ann);
    c = c.checked_mul(&BigRational::from(target_d_bigint.clone())).unwrap().checked_mul(&BigRational::from(BigInt::one())).unwrap().checked_div(&BigRational::from(ann.checked_mul(&n_coins).unwrap())).unwrap();
    // c = c.checked_mul(&BigRational::from(target_d_bigint.clone())).unwrap().checked_mul(&BigRational::from(a_precision.clone())).unwrap().checked_div(&BigRational::from(ann.checked_mul(&n_coins).unwrap())).unwrap();
    
    let c_numer: BigDecimal = BigDecimal::from_str(&c.numer().to_string()).unwrap();
    let c_denom: BigDecimal = BigDecimal::from_str(&c.denom().to_string()).unwrap();
    let c_decimal: BigDecimal = c_numer / c_denom;

    // let b = sum.add(target_d_bigint.clone().mul(&a_precision).checked_div(&ann).unwrap());
    let b = sum.add(target_d_bigint.clone().mul(BigInt::one()).checked_div(&ann).unwrap());
    // println!("B: {}", b);   
    // let prev_y = BigInt::from(0);
    let mut y = target_d_bigint.clone();

    // println!("B: {}", b);
    // println!("Y Start: {}", y);

    for i in 0..max_iterations {
        let prev_y = y.clone();
        y = y.clone().mul(&y).add(&c.round().numer().to_bigint().unwrap()).checked_div(&y.mul(&two).add(&b).checked_sub(&target_d_bigint).unwrap()).unwrap();
        // println!("Y {} | {}", i, y.clone());
        // let end = y.checked_sub(&prev_y).unwrap().abs().le(&one);

        if y.checked_sub(&prev_y).unwrap().abs().le(&one) {
            // println!("Number of iterations: {}", i);
            break;
        }
    }
    // println!("Y End: {}", y);
    return Some(y);
}

pub fn get_y_bnc(balances: &Vec<BigInt>, target_index: usize, target_d: BigInt, amplitude: u128) -> Option<BigInt> {
    let one = BigInt::from(1);
    let two = BigInt::from(2);
    let mut c = BigRational::from(target_d.clone());
    let mut sum = BigInt::from(0);
    let mut ann = BigInt::from(amplitude);
    let balance_size = BigInt::from(balances.len());
    let target_d_u256 = BigInt::from(target_d);
    let a_precision_u256 = BigInt::from(100); // needs to be variable



    for (i, balance_ref) in balances.iter().enumerate() {
        // println!("{} Balance: {}", i, balance_ref.clone());
        ann = ann.checked_mul(&balance_size).unwrap();
        if i == target_index as usize {
            continue;
        }
        sum = sum.add(balance_ref);
        let div_op = balance_ref.checked_mul(&balance_size).unwrap();
        c = c.checked_mul(&BigRational::from(target_d_u256.clone())).unwrap().checked_div(&BigRational::from(div_op)).unwrap().floor();
    }

    // println!("Sum: {} | C: {} | Target D: {} | A Precision: {} | Ann: {} | Balance Size: {}", sum, c, target_d_u256, a_precision_u256, ann, balance_size);

    c = c
        .mul(target_d_u256.clone())
        .mul(a_precision_u256.clone())
        .checked_div(&BigRational::from(ann.clone().mul(balance_size))).unwrap().floor();

    // println!("C: {}", c);

    let b = sum.add(target_d_u256.clone().mul(a_precision_u256.clone()).checked_div(&ann.clone()).unwrap());
    let mut prev_y: BigInt;
    let mut y: BigInt = target_d_u256.clone();

    // println!("Y Start: {}", y.clone());

    for i in 0..MAX_Y_ITERATIONS_BNC {
        prev_y = y.clone();
        y = y.clone()
            .mul(&y)
            .add(&c.round().numer().to_bigint().unwrap())
            .checked_div(&y.mul(&two).add(&b).checked_sub(&target_d_u256).unwrap()).unwrap();

        // println!("Y {} | {}", i, y.clone());

        let diff = y.checked_sub(&prev_y).unwrap().abs();
        if y.gt(&prev_y) {
            if y.checked_sub(&prev_y).unwrap().abs().le(&one) {
                break;
            }
        } else if prev_y.checked_sub(&y).unwrap().abs().le(&one) {
            break;
        }
    }

    // let result: bn = new bn(y).integerValue(BigNumber.ROUND_DOWN)
    let result: BigInt = BigInt::from(y.clone());
    Some(result)

}

pub fn getY(balances: &Vec<BigInt>, target_index: u128, D: u128, a: u128, primary_node: String) -> Option<u128>{
    let one: BigInt = BigInt::from(1u128);
    let two: BigInt = BigInt::from(2u128);
    let mut c = BigInt::from(D);
    let mut sum = BigInt::from(0u128);
    let mut ann = BigInt::from(a);
    let mut balance_size = BigInt::from(balances.len());
    let target_d = BigInt::from(D);
    let a_precision = BigInt::from(100 as usize);
    let fee_precision = BigInt::from(10000000000 as usize);
    let mut ann_formatted: BigRational;
    
    let mut balances = balances.clone();
    let mut lksmBalances: Vec<BigInt> = vec![];

    if ann == BigInt::from(10000u128){
        ann_formatted = BigRational::new(BigInt::from(100), BigInt::from(1u128));
    } else {
        ann_formatted = BigRational::new(BigInt::from(30), BigInt::from(1u128));
        lksmBalances = vec![balances[1].clone(), balances[2].clone()];
        balances = vec![balances[0].clone(), balances[3].clone()];
        balance_size = BigInt::from(balances.len());

    }

    for (i, balance_ref) in balances.iter().enumerate() {
			let balance: BigInt = BigInt::from(balance_ref.clone());
			ann = ann.checked_mul(&balance_size)?; // ****
            ann_formatted = ann_formatted.checked_mul(&BigRational::from(balance_size.clone()))?;
			let token_index_usize = target_index as usize;
			if i == token_index_usize {
				continue;
			}
			sum = sum.checked_add(&balance)?;
			let div_op: BigInt = balance.checked_mul(&balance_size)?;
			c = c.checked_mul(&target_d)?.checked_div(&div_op)?;
		}

    // c = c * D / (ann * balances.len() as u128);
    let t_1 = BigRational::from(target_d.clone()).checked_div(&ann_formatted.checked_mul(&BigRational::from(balance_size.clone()))?)?;
    c = c.checked_mul(&t_1.round().numer())?;
    
    // let mut b = s + D / ann;
    let target_d_ratio = BigRational::from(target_d.clone());
    let b = sum.checked_add(&target_d_ratio.checked_div(&ann_formatted)?.round().numer())?;
    let x = target_d_ratio.checked_div(&ann_formatted)?;
    let mut prev_y: BigInt;
    let mut y = target_d.clone();

    for _i in 0..255 {
        prev_y = y.clone();
        y = y.checked_mul(&y)?.checked_add(&c)?.checked_div(&y.checked_mul(&two)?.checked_add(&b)?.checked_sub(&target_d)?)?;
        if y > prev_y {
            if y.clone() - prev_y <= one {
                break;
            }
        } else if prev_y - y.clone() <= one {
            break;
        }
    }
    
    return y.to_u128();

    // for i in 0..255{

    // }

    
}

// pub fn calculate_edge_2(
//     primary_node: &GraphNodePointer,
//     adjacent_node: &AdjacentNodePair,
//     input_amount: u128,
// ) -> u128 {
//     let (base_liquidity, adjacent_liquidity) = adjacent_node.get_dex_liquidity();

//     let base_liquidity = base_liquidity.to_bigint().unwrap();
//     let adjacent_liquidity = adjacent_liquidity.to_bigint().unwrap();
//     let increments = 5000;
//     let token_1_increment = input_amount / increments;
//     let swap_fee = (token_1_increment as f64 * 0.003) as u128;
//     let token_1_increment_minus_swap = token_1_increment - swap_fee;
    
//     let mut token_1_changing_liquidity = base_liquidity.clone();
//     let mut token_2_changing_liquidity = adjacent_liquidity.clone();
//     let mut total_slippage = BigInt::default();
//     let mut total_token_2_output = BigInt::default();

//     for _ in 0..increments {
//          let token_2_out = (&token_2_changing_liquidity * token_1_increment_minus_swap)
//             / (&token_1_changing_liquidity + token_1_increment_minus_swap);
//         let slip = (&token_2_out / &token_2_changing_liquidity) * &token_2_out;
//         total_token_2_output += &token_2_out - &slip;
//         token_2_changing_liquidity -= &token_2_out - &slip;
//         token_1_changing_liquidity += token_1_increment_minus_swap;
//         total_slippage += &slip;
//     }


//     // (
//     //     total_token_2_output.to_u128().unwrap(),
//     //     (token_1_changing_liquidity.to_u128().unwrap(), token_2_changing_liquidity.to_u128().unwrap()),
//     // )
//     total_token_2_output.to_u128().unwrap()

// }

// fn calculate_kucoin_edge_2(token_graph: &TokenGraph2, primary_node: &GraphNodePointer, adjacent_pair: &AdjacentNodePair, input_amount: u128) -> u128{

    //Asset -> USDT
//     let (bid_price, ask_price) = adjacent_pair.get_cex_liquidity_price();
//     let (bid_decimals, ask_decimals) = adjacent_pair.get_cex_liquidity_decimals();
//     if primary_node.borrow().get_asset_symbol() != "USDT"{
//         let usdt_token_decimals = 4;
//         let asset_decimals = token_graph.get_asset_decimals_for_kucoin_asset(primary_node);

//         //Convert number to normal, display value
//         let converted_input = input_amount as f64 / f64::powi(10.0, asset_decimals as i32);
//         let converted_bid = bid_price.clone() as f64 / f64::powi(10.0, bid_decimals as i32);

//         let asset_output = converted_input * converted_bid;
//         let asset_output_converted = asset_output * f64::powi(10.0, usdt_token_decimals as i32) ;
//         asset_output_converted as u128
//     }
//     //USDT -> Asset
//     else {
//         let usdt_token_decimals = 6;
//         let asset_decimals = token_graph.get_asset_decimals_for_kucoin_asset(&adjacent_pair.adjacent_node);

//         //Convert number to normal, display value
//         let converted_input = input_amount as f64 / f64::powi(10.0, usdt_token_decimals as i32);
//         let converted_ask = bid_price.clone() as f64 / f64::powi(10.0, ask_decimals as i32);
        

//         let asset_output = converted_input as f64 / converted_ask;
//         let asset_output_converted = asset_output * f64::powi(10.0, asset_decimals as i32) ;
//         asset_output_converted as u128
//     }
// }