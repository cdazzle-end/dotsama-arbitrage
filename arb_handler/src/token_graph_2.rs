// use crate::asset_registry::AssetLocation;
// use crate::token::{self, TokenData};
// use crate::{LiqPoolRegistry, asset_registry, liq_pool_registry};
// use crate::liq_pool_registry_2::LiqPoolRegistry2;
use num::{BigInt, BigUint, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, FromPrimitive, Num, One, Signed, ToPrimitive, Zero};
use num::bigint::{ToBigInt, ToBigUint};
use num::BigRational;
use num;
use bigdecimal::{BigDecimal};
// use std::borrow::{Borrow, BorrowMut};
// use num::BigRational::
use std::cell::RefCell;
use std::collections::{VecDeque, HashMap};
use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul};
use std::rc::Rc;
use std::str::FromStr;
use std::vec;
use crate::liq_pool_registry_2::TickData;
// use crate::{asset_registry::{AssetRegistry, Asset}};
use crate::AssetRegistry2;
use crate::asset_registry_2::{Asset, AssetLocation, TokenData};
use crate::adjacency_table_2::{AdjacencyGroup, AdjacencyTable2, CexLp, DexLp, DexV3, GroupType, Liquidity, StableLp, StableShareLp};
type AssetPointer = Rc<RefCell<Asset>>;
type GraphNodePointer = Rc<RefCell<GraphNode>>;

const MIN_TICK: i64 = -887272;
const MAX_TICK: i64 = 887272;
const MIN_TICK_DATA: TickData = TickData{tick: MIN_TICK, liquidity_delta: 0};
const MAX_TICK_DATA: TickData = TickData{tick: MAX_TICK, liquidity_delta: 0};
const SHARE_PRECISION: u64 = 18;
const MAX_Y_ITERATIONS_HDX: u64 = 128;
const MAX_Y_ITERATIONS_ACA: u64 = 255;
const MAX_D_ITERATIONS_HDX: u64 = 64;
const MAX_D_ITERATIONS_ACA: u64 = 255;

pub struct TokenGraph2{
    pub node_map: HashMap<String, Vec<GraphNodePointer>>,
    pub asset_registry: AssetRegistry2
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

        TokenGraph2{ node_map, asset_registry }
    }

    //Get node from asset key
    pub fn get_node(&self, asset_key: String) -> GraphNodePointer{
        // println!("asset key: {}", asset_key);
        let bucket = self.node_map.get(&asset_key).unwrap();
        for node in bucket{
            if node.borrow().asset_key == asset_key{
                return Rc::clone(node);
            }
        }
        panic!("Could not find node with asset key: {}", asset_key);
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
        for (key, buckets) in &self.node_map{
            for node in buckets{
                print!("Node: {} -> ", node.borrow().asset_key);
                for adj_node_2 in &node.borrow().adjacent_pairs2{
                    match adj_node_2 {
                        AdjacentNodePair2::StablePair(adj_node) => {
                            print!("(S)");
                            for node in &adj_node.adjacent_nodes{
                                node.borrow().asset.borrow().display_asset();
                                print!(" | ");
                            }
                            // print!(") ");
                            // print!(" | ");
                        },
                        AdjacentNodePair2::CexPair(adj_node) => {
                            print!("(C)");
                            adj_node.adjacent_node.borrow().asset.borrow().display_asset();
                            // print!(") ");
                            print!(" | ");
                        },
                        AdjacentNodePair2::DexPair(adj_node) => {
                            print!("(D)");
                            adj_node.adjacent_node.borrow().asset.borrow().display_asset();
                            // print!(") ");
                            print!(" | ");
                        },
                        AdjacentNodePair2::XcmPair(adj_node) => {
                            print!("(X)");
                            adj_node.adjacent_node.borrow().asset.borrow().display_asset();
                            // print!(") ");
                            print!(" | ");
                        },
                        _ => {}
                    }
                    // adj_node_2.adjacent_nodes.borrow().asset.borrow().display_asset();
                    // print!(" | ");
                }
                println!("");
            }

            println!("------------------------------------")
        }
    }

    pub fn display_stable_share_pairs(&self){
        for (key, buckets) in &self.node_map{
            for current_node in buckets{
                
                for adjacent_node_pair in &current_node.borrow().adjacent_pairs2{
                    match adjacent_node_pair {
                        AdjacentNodePair2::StableSharePair(pair) => {
                            print!("Node: {} -> ", current_node.borrow().asset_key);
                            print!("(Stable Share) ");
                            
                            for node in &pair.adjacent_nodes{
                                node.borrow().asset.borrow().display_asset();
                                print!(" | ");
                            }
                            println!("");
                            println!("------------------------------------");
                            
                            // print!(") ");
                            // print!(" | ");
                        },
                        // AdjacentNodePair2::StablePair(adj_node) => {
                        //     print!("(S)");
                        //     print!("Node: {} -> ", node.borrow().asset_key);
                        //     for node in &adj_node.adjacent_nodes{
                        //         node.borrow().asset.borrow().display_asset();
                        //         print!(" | ");
                        //     }
                        //     // print!(") ");
                        //     // print!(" | ");
                        // },
                        // AdjacentNodePair2::CexPair(adj_node) => {
                        //     print!("(C)");
                        //     adj_node.adjacent_node.borrow().asset.borrow().display_asset();
                        //     // print!(") ");
                        //     print!(" | ");
                        // },
                        // AdjacentNodePair2::DexPair(adj_node) => {
                        //     print!("(D)");
                        //     adj_node.adjacent_node.borrow().asset.borrow().display_asset();
                        //     // print!(") ");
                        //     print!(" | ");
                        // },
                        // AdjacentNodePair2::XcmPair(adj_node) => {
                        //     print!("(X)");
                        //     adj_node.adjacent_node.borrow().asset.borrow().display_asset();
                        //     // print!(") ");
                        //     print!(" | ");
                        // },
                        _ => {}
                    }
                    // adj_node_2.adjacent_nodes.borrow().asset.borrow().display_asset();
                    // print!(" | ");
                }
                println!("");
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

        println!("Input amount: {} | Output amount: {}", input_amount, output_formatted);
        //     println!("Output amount: {}", output_formatted);
        // } else {
        //     println!("No pair found");
        // } 
    }

    pub fn find_best_route(&self, asset_key_1: String, asset_key_2: String, input_amount: f64) -> (String, Vec<Rc<RefCell<GraphNode>>>) {

        let starting_node = &self.get_node(asset_key_1).clone();
        let formatted_input = &input_amount * f64::powi(10.0, starting_node.borrow().get_asset_decimals() as i32);
        starting_node.borrow_mut().best_path_value = formatted_input.to_bigint().unwrap();
        starting_node.borrow_mut().path_values.push(input_amount);

        let path_data: PathData = PathData{
            path_type: "Start".to_string(),
            lp_id: None,
        };

        starting_node.borrow_mut().path_value_types.push(0);
        starting_node.borrow_mut().path_datas.push(path_data);
        starting_node.borrow_mut().best_path.push(Rc::clone(&starting_node));

        let destination_node = &self.get_node(asset_key_2).clone();
        let destination_asset_location = destination_node.borrow().get_asset_location().unwrap();
        let all_destination_assets = &self.asset_registry.get_assets_at_location(destination_asset_location);
        let mut destination_nodes = vec![];
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
            for adjacent_pair in &current_node.borrow().adjacent_pairs2{
                match adjacent_pair {
                    AdjacentNodePair2::XcmPair(adjacent_pair) => {
                        if current_node.borrow().best_path_value > adjacent_pair.adjacent_node.borrow().best_path_value{
                            let mut current_path_contains_adjacent_node= false;
                            let mut is_destination_node = false;
                            for dest_node in &destination_nodes{
                                if dest_node.borrow().get_asset_key() == adjacent_pair.adjacent_node.borrow().get_asset_key(){
                                    is_destination_node = true;
                                }
                            }
                            for path_node in &current_node.borrow().best_path{
                                if path_node.borrow().get_asset_key() == adjacent_pair.adjacent_node.borrow().get_asset_key(){
                                    current_path_contains_adjacent_node = true;
                                }
                            }
                            
                            adjacent_pair.adjacent_node.borrow_mut().best_path_value = current_node.borrow().best_path_value.clone();
                            adjacent_pair.adjacent_node.borrow_mut().best_path = current_node.borrow().best_path.clone();
                            adjacent_pair.adjacent_node.borrow_mut().best_path.push(Rc::clone(&adjacent_pair.adjacent_node));
                            adjacent_pair.adjacent_node.borrow_mut().path_values = current_node.borrow().path_values.clone();
                            adjacent_pair.adjacent_node.borrow_mut().path_values.push(current_node.borrow().best_path_value_display(&self));
                            adjacent_pair.adjacent_node.borrow_mut().path_value_types = current_node.borrow().path_value_types.clone();
                            adjacent_pair.adjacent_node.borrow_mut().path_value_types.push(0);

                            let new_path_data: PathData = PathData{
                                path_type: "Xcm".to_string(),
                                lp_id: None,
                            };

                            adjacent_pair.adjacent_node.borrow_mut().path_datas = current_node.borrow().path_datas.clone();
                            adjacent_pair.adjacent_node.borrow_mut().path_datas.push(new_path_data);
                            if !current_path_contains_adjacent_node && !is_destination_node{
                                node_queue.push_back(Rc::clone(&adjacent_pair.adjacent_node));
                            }
                            
                        }
                    },
                    AdjacentNodePair2::DexPair(dex_pair) =>  {
                        let current_chain = current_node.borrow().get_chain_id();
                        
                        if current_chain == 2004{
                        
                        }

                        let path_value = calculate_dex_edge( adjacent_pair, current_node.borrow().best_path_value.clone());
                        if path_value > dex_pair.adjacent_node.borrow().best_path_value{
                            let mut test= false;
                            for path_node in &current_node.borrow().best_path{
                                if path_node.borrow().get_asset_key() == dex_pair.adjacent_node.borrow().get_asset_key(){
                                    test = true;
                                }
                            }
                            let lp_id = dex_pair.get_lp_id();
                            let mut is_destination_node = false;
                            for dest_node in &destination_nodes{
                                if dest_node.borrow().get_asset_key() == dex_pair.adjacent_node.borrow().get_asset_key(){
                                    is_destination_node = true;
                                }
                            }
                            dex_pair.adjacent_node.borrow_mut().best_path_value = path_value;
                            dex_pair.adjacent_node.borrow_mut().best_path = current_node.borrow().best_path.clone();
                            dex_pair.adjacent_node.borrow_mut().best_path.push(Rc::clone(&dex_pair.adjacent_node));
                            dex_pair.adjacent_node.borrow_mut().path_values = current_node.borrow().path_values.clone();
                            let formatted_path_value = dex_pair.adjacent_node.borrow().best_path_value_display(&self).clone();
                            dex_pair.adjacent_node.borrow_mut().path_values.push(formatted_path_value);
                            dex_pair.adjacent_node.borrow_mut().path_value_types = current_node.borrow().path_value_types.clone();
                            dex_pair.adjacent_node.borrow_mut().path_value_types.push(1);
                            
                            let new_path_data: PathData = PathData{
                                path_type: "Dex".to_string(),
                                lp_id: lp_id.clone(),
                            };

                            dex_pair.adjacent_node.borrow_mut().path_datas = current_node.borrow().path_datas.clone();
                            dex_pair.adjacent_node.borrow_mut().path_datas.push(new_path_data);
                            if !test && !is_destination_node{
                                node_queue.push_back(Rc::clone(&dex_pair.adjacent_node));
                            }
                            
                        }
                    },
                    AdjacentNodePair2::DexV3Pair(dex_pair) =>  {
                        let path_value = calculate_dex_edge( adjacent_pair, current_node.borrow().best_path_value.clone());

                        if path_value > dex_pair.adjacent_node.borrow().best_path_value{

                            let mut test= false;
                            for path_node in &current_node.borrow().best_path{
                                if path_node.borrow().get_asset_key() == dex_pair.adjacent_node.borrow().get_asset_key(){
                                    test = true;
                                }
                            }
                            let mut is_destination_node = false;
                            for dest_node in &destination_nodes{
                                if dest_node.borrow().get_asset_key() == dex_pair.adjacent_node.borrow().get_asset_key(){
                                    is_destination_node = true;
                                }
                            }
                            dex_pair.adjacent_node.borrow_mut().best_path_value = path_value;
                            dex_pair.adjacent_node.borrow_mut().best_path = current_node.borrow().best_path.clone();
                            dex_pair.adjacent_node.borrow_mut().best_path.push(Rc::clone(&dex_pair.adjacent_node));
                            dex_pair.adjacent_node.borrow_mut().path_values = current_node.borrow().path_values.clone();
                            let formatted_path_value = dex_pair.adjacent_node.borrow().best_path_value_display(&self).clone();
                            dex_pair.adjacent_node.borrow_mut().path_values.push(formatted_path_value);
                            dex_pair.adjacent_node.borrow_mut().path_value_types = current_node.borrow().path_value_types.clone();
                            dex_pair.adjacent_node.borrow_mut().path_value_types.push(1);

                            let lp_id = dex_pair.get_lp_id();
                            let new_path_data: PathData = PathData{
                                path_type: "DexV3".to_string(),
                                lp_id: lp_id.clone(),
                            };
                            dex_pair.adjacent_node.borrow_mut().path_datas = current_node.borrow().path_datas.clone();
                            dex_pair.adjacent_node.borrow_mut().path_datas.push(new_path_data);


                            if !test && !is_destination_node{
                                node_queue.push_back(Rc::clone(&dex_pair.adjacent_node));
                            }
                            
                        }
                    },
                    AdjacentNodePair2::CexPair(cex_pair) => {
                        let path_value = calculate_cex_edge( &self, &current_node, adjacent_pair, current_node.borrow().best_path_value.to_u128().unwrap());
                        if path_value > cex_pair.adjacent_node.borrow().best_path_value.to_u128().unwrap(){
                            let mut test= false;
                            for path_node in &current_node.borrow().best_path{
                                if path_node.borrow().get_asset_key() == cex_pair.adjacent_node.borrow().get_asset_key(){
                                    test = true;
                                }
                            }
                            let mut is_destination_node = false;
                            for dest_node in &destination_nodes{
                                if dest_node.borrow().get_asset_key() == cex_pair.adjacent_node.borrow().get_asset_key(){
                                    is_destination_node = true;
                                }
                            }
                            cex_pair.adjacent_node.borrow_mut().best_path_value = BigInt::from(path_value);
                            cex_pair.adjacent_node.borrow_mut().best_path = current_node.borrow().best_path.clone();
                            cex_pair.adjacent_node.borrow_mut().best_path.push(Rc::clone(&cex_pair.adjacent_node));
                            cex_pair.adjacent_node.borrow_mut().path_values = current_node.borrow().path_values.clone();
                            let formatted_path_value = cex_pair.adjacent_node.borrow().best_path_value_display(&self).clone();
                            cex_pair.adjacent_node.borrow_mut().path_values.push(formatted_path_value);
                            cex_pair.adjacent_node.borrow_mut().path_value_types = current_node.borrow().path_value_types.clone();
                            cex_pair.adjacent_node.borrow_mut().path_value_types.push(3);
                            
                            if !test && !is_destination_node{
                                node_queue.push_back(Rc::clone(&cex_pair.adjacent_node));
                            }
                            
                        }
                    },
                    AdjacentNodePair2::StablePair(stable_pair) => {
                        // for (i, adj_node) in stable_pair.adjacent_nodes.iter().enumerate(){
                        //     let path_value = calculate_stable_edge( &self, &current_node, &adjacent_pair, current_node.borrow().best_path_value, i);
                        // }
                        for i in 0..stable_pair.adjacent_nodes.len(){
                            let path_value = calculate_stable_edge(&current_node, &adjacent_pair, current_node.borrow().best_path_value.clone(), i).unwrap();
                            if path_value > stable_pair.adjacent_nodes[i].borrow().best_path_value{
                                let mut test= false;
                                for path_node in &current_node.borrow().best_path{
                                    if path_node.borrow().get_asset_key() == stable_pair.adjacent_nodes[i].borrow().get_asset_key(){
                                        test = true;
                                    }
                                }
                                let mut is_destination_node = false;
                                for dest_node in &destination_nodes{
                                    if dest_node.borrow().get_asset_key() == stable_pair.adjacent_nodes[i].borrow().get_asset_key(){
                                        is_destination_node = true;
                                    }
                                }
                                stable_pair.adjacent_nodes[i].borrow_mut().best_path_value = path_value;
                                stable_pair.adjacent_nodes[i].borrow_mut().best_path = current_node.borrow().best_path.clone();
                                stable_pair.adjacent_nodes[i].borrow_mut().best_path.push(Rc::clone(&stable_pair.adjacent_nodes[i]));
                                stable_pair.adjacent_nodes[i].borrow_mut().path_values = current_node.borrow().path_values.clone();
                                let formatted_path_value = stable_pair.adjacent_nodes[i].borrow().best_path_value_display(&self).clone();
                                stable_pair.adjacent_nodes[i].borrow_mut().path_values.push(formatted_path_value);
                                stable_pair.adjacent_nodes[i].borrow_mut().path_value_types = current_node.borrow().path_value_types.clone();
                                stable_pair.adjacent_nodes[i].borrow_mut().path_value_types.push(2);

                                let lp_id = stable_pair.get_lp_id();
                                let new_path_data: PathData = PathData{
                                    path_type: "Stable".to_string(),
                                    lp_id: lp_id.clone(),
                                };
    
                                stable_pair.adjacent_nodes[i].borrow_mut().path_datas = current_node.borrow().path_datas.clone();
                                stable_pair.adjacent_nodes[i].borrow_mut().path_datas.push(new_path_data);
                                if !test && !is_destination_node{
                                    node_queue.push_back(Rc::clone(&stable_pair.adjacent_nodes[i]));
                                }
                                
                            }
                        }
                    },
                    AdjacentNodePair2::StableSharePair(stable_pair) => {
                        // for (i, adj_node) in stable_pair.adjacent_nodes.iter().enumerate(){
                        //     let path_value = calculate_stable_edge( &self, &current_node, &adjacent_pair, current_node.borrow().best_path_value, i);
                        // }
                        for i in 0..stable_pair.adjacent_nodes.len(){
                            let path_value = calculate_stable_edge( &current_node, &adjacent_pair, current_node.borrow().best_path_value.clone(), i).unwrap();
                            if path_value > stable_pair.adjacent_nodes[i].borrow().best_path_value{
                                let mut test= false;
                                for path_node in &current_node.borrow().best_path{
                                    if path_node.borrow().get_asset_key() == stable_pair.adjacent_nodes[i].borrow().get_asset_key(){
                                        test = true;
                                    }
                                }
                                let mut is_destination_node = false;
                                for dest_node in &destination_nodes{
                                    if dest_node.borrow().get_asset_key() == stable_pair.adjacent_nodes[i].borrow().get_asset_key(){
                                        is_destination_node = true;
                                    }
                                }
                                stable_pair.adjacent_nodes[i].borrow_mut().best_path_value = path_value;
                                stable_pair.adjacent_nodes[i].borrow_mut().best_path = current_node.borrow().best_path.clone();
                                stable_pair.adjacent_nodes[i].borrow_mut().best_path.push(Rc::clone(&stable_pair.adjacent_nodes[i]));
                                stable_pair.adjacent_nodes[i].borrow_mut().path_values = current_node.borrow().path_values.clone();
                                let formatted_path_value = stable_pair.adjacent_nodes[i].borrow().best_path_value_display(&self).clone();
                                stable_pair.adjacent_nodes[i].borrow_mut().path_values.push(formatted_path_value);
                                stable_pair.adjacent_nodes[i].borrow_mut().path_value_types = current_node.borrow().path_value_types.clone();
                                stable_pair.adjacent_nodes[i].borrow_mut().path_value_types.push(2);

                                let lp_id = stable_pair.get_lp_id();
                                let new_path_data: PathData = PathData{
                                    path_type: "Stable".to_string(),
                                    lp_id: lp_id.clone(),
                                };
    
                                stable_pair.adjacent_nodes[i].borrow_mut().path_datas = current_node.borrow().path_datas.clone();
                                stable_pair.adjacent_nodes[i].borrow_mut().path_datas.push(new_path_data);
                                if !test && !is_destination_node{
                                    node_queue.push_back(Rc::clone(&stable_pair.adjacent_nodes[i]));
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
        let asset_id = asset_id.to_uppercase();
        for node_bucket in self.node_map.values(){
            for node in node_bucket{
                let current_chain_id = node.borrow().get_chain_id();
                let current_asset_id = node.borrow().get_local_id();

                if current_chain_id == chain_id && current_asset_id.eq(&asset_id){
                    return Some(Rc::clone(node));
                }
                // if node.borrow().get_chain_id() == chain_id && node.borrow().get_local_id().eq_ignore_ascii_case(&asset_id){
                //     return Some(Rc::clone(node));
                // }
            }
        }
        None
    }

    // pub fn calculate



    pub fn get_asset_decimals_for_kucoin_asset(&self, kucoin_node: &GraphNodePointer) -> u64 {
        self.asset_registry.get_kucoin_asset_decimals(kucoin_node.borrow().get_asset_location().unwrap())
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
            adjacent_pairs2: Vec::new(),
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
    let adjacent_assets = adjacency_table.get_adjacent_assets_2(current_node.borrow().asset.clone());
    for adj_group in adjacent_assets{
        match adj_group.group_type {
            GroupType::Stable => {
                let mut adjacent_nodes = vec![];
                for adjacent_asset in adj_group.adjacent_asset{
                    let bucket = node_map.get(&adjacent_asset.borrow().get_map_key()).unwrap();
                    for potential_adjacent_node in bucket{
                        if adjacent_asset.borrow().get_map_key() == potential_adjacent_node.borrow().asset_key{
                            adjacent_nodes.push(Rc::clone(&potential_adjacent_node));
                            // println!("Found stable pair")

                        }
                    }
                }
                let adjacent_node_2 = AdjacentNodePair2::StablePair(StablePair{adjacent_nodes: adjacent_nodes, liquidity: adj_group.liquidity.clone().unwrap()});
                
                current_node.borrow_mut().adjacent_pairs2.push(adjacent_node_2);
            },
            GroupType::StableShare => {
                    let mut adjacent_nodes = vec![];
                    let lp_data: StableShareLp = match &adj_group.liquidity.clone().unwrap() {
                        Liquidity::StableShare(stableLp) => stableLp.clone(),
                        _ => panic!("Tried to get dex liquidity from non-dex liquidity"),
                    };
                    let token_to_share = lp_data.token_to_share.unwrap();
                    let share_asset_key = &lp_data.share_asset.borrow().get_map_key();
                    let share_asset_node_bucket = node_map.get(&lp_data.share_asset.borrow().get_map_key()).unwrap();
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

                    for adjacent_asset in adj_group.adjacent_asset{
                        let bucket = node_map.get(&adjacent_asset.borrow().get_map_key()).unwrap();
                        for potential_adjacent_node in bucket{
                            if adjacent_asset.borrow().get_map_key() == potential_adjacent_node.borrow().asset_key{
                                adjacent_nodes.push(Rc::clone(&potential_adjacent_node));
                                // println!("Found stable pair")
    
                            }
                        }
                    }
                    let adjacent_node_2 = AdjacentNodePair2::StableSharePair(StableSharePair{share_asset_node: share_asset_node, token_to_share, adjacent_nodes: adjacent_nodes, liquidity: adj_group.liquidity.clone().unwrap()});
                    
                    current_node.borrow_mut().adjacent_pairs2.push(adjacent_node_2);
            },
            GroupType::Dex =>{
                //Find node that corresponds to adjacent asset. Add it to current node's adjacency list
                let bucket = node_map.get(&adj_group.adjacent_asset[0].borrow().get_map_key()).unwrap();
                for potential_adjacent_node in bucket {
                    if adj_group.adjacent_asset[0].borrow().get_map_key() == potential_adjacent_node.borrow().asset_key{
                        let dex_lp: DexLp = if let Liquidity::Dex(x) = adj_group.liquidity.clone().unwrap(){
                            x
                        } else {
                            panic!("Dex liquidity should be DexLp")
                        };
                        let adjacent_node_2 = AdjacentNodePair2::DexPair(DexPair{adjacent_node: Rc::clone(&potential_adjacent_node), liquidity: adj_group.liquidity.clone().unwrap()});
                        current_node.borrow_mut().adjacent_pairs2.push(adjacent_node_2);
                        // println!("found DEx")
                    }
                }
            },
            GroupType::DexV3 => {
                //Find node that corresponds to adjacent asset. Add it to current node's adjacency list
                let bucket = node_map.get(&adj_group.adjacent_asset[0].borrow().get_map_key()).unwrap();
                for potential_adjacent_node in bucket{
                    if adj_group.adjacent_asset[0].borrow().get_map_key() == potential_adjacent_node.borrow().asset_key{
                        let dex_lp: DexV3 = if let Liquidity::DexV3(x) = adj_group.liquidity.clone().unwrap(){
                            x
                        } else {
                            panic!("Dex liquidity should be DexLp")
                        };
                        let adjacent_node_2 = AdjacentNodePair2::DexV3Pair(DexV3Pair{adjacent_node: Rc::clone(&potential_adjacent_node), liquidity: adj_group.liquidity.clone().unwrap()});
                        current_node.borrow_mut().adjacent_pairs2.push(adjacent_node_2);
                    }
                }
            },
            GroupType::Cex => { },
            //     //Find node that corresponds to adjacent asset. Add it to current node's adjacency list
            //     let bucket = node_map.get(&adj_group.adjacent_asset[0].borrow().get_map_key()).unwrap();
            //     for potential_adjacent_node in bucket{
            //         if adj_group.adjacent_asset[0].borrow().get_map_key() == potential_adjacent_node.borrow().asset_key{
            //             let adjacent_node = AdjacentNodePair::new(&potential_adjacent_node, adj_group.liquidity.clone(), 3);
            //             current_node.borrow_mut().adjacent_pairs.push(adjacent_node);

            //             let adjacent_node_2 = AdjacentNodePair2::CexPair(CexPair{adjacent_node: Rc::clone(&potential_adjacent_node), liquidity: adj_group.liquidity.clone().unwrap()});
            //             current_node.borrow_mut().adjacent_pairs2.push(adjacent_node_2);
            //         }
            //     }
            // },
            _ => {}

        }
        
    }
}

//If asset is cross chain, get it's cross chain assets and add them as adjacent nodes
pub fn add_cross_chain_assets_2(current_node: GraphNodePointer, node_map: &HashMap<String, Vec<GraphNodePointer>>, asset_registry: &AssetRegistry2){
    let current_node_location = current_node.borrow().get_asset_location();
    if let Some(asset_location) = current_node_location{
        for cross_chain_asset in asset_registry.get_assets_at_location(asset_location){
            let bucket = node_map.get(&cross_chain_asset.borrow().get_map_key()).unwrap();
            for graph_node in bucket{
                if cross_chain_asset.borrow().get_map_key() == graph_node.borrow().asset.borrow().get_map_key(){
                    // current_node.borrow_mut().adjacent_pairs.push((Rc::clone(graph_node), ((0,0),(0,0))));
                    // let adjacent_node = AdjacentNodePair::new(&graph_node, None, 0);
                    // current_node.borrow_mut().adjacent_pairs.push(adjacent_node);

                    let adjacent_node_2 = AdjacentNodePair2::XcmPair(XcmPair{adjacent_node: Rc::clone(&graph_node)});
                    current_node.borrow_mut().adjacent_pairs2.push(adjacent_node_2);
                }
            }
        }
    }
}
//--------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub struct GraphNode{
    pub asset: AssetPointer,
    // pub adjacent_nodes: Vec<(GraphNodePointer, ((u128, u128), (u128,u128)))>,
    // pub adjacent_pairs: Vec<AdjacentNodePair>,
    pub adjacent_pairs2: Vec<AdjacentNodePair2>,
    pub asset_key: String,
    pub pred: Option<GraphNodePointer>,
    pub best_path_value: BigInt,
    pub best_path_value_display: f64,
    pub path_edges: Vec<((String,u128),(String, u128))>,
    pub best_path: Vec<GraphNodePointer>,
    pub path_values: Vec<f64>,
    pub path_value_types: Vec<u64>,
    pub path_datas: Vec<PathData>,

}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PathData{
    pub path_type: String,
    pub lp_id: Option<String>,
}
#[derive(Debug, PartialEq)]
pub struct AdjacentNodePair{
    pub adjacent_node: GraphNodePointer,
    pub liquidity: Option<Liquidity>,
    pub pair_type: u64,
}
#[derive(Debug, Clone, PartialEq)]
pub enum AdjacentNodePair2{
    DexPair(DexPair),
    DexV3Pair(DexV3Pair),
    CexPair(CexPair),
    StablePair(StablePair),
    StableSharePair(StableSharePair),
    XcmPair(XcmPair),
}
#[derive(Debug, Clone, PartialEq)]
pub struct DexPair{
    pub adjacent_node: GraphNodePointer,
    pub liquidity: Liquidity,
}
#[derive(Debug, Clone, PartialEq)]
pub struct DexV3Pair{
    pub adjacent_node: GraphNodePointer,
    pub liquidity: Liquidity,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CexPair{
    pub adjacent_node: GraphNodePointer,
    pub liquidity: Liquidity,
}
#[derive(Debug, Clone, PartialEq)]
pub struct StablePair{
    pub adjacent_nodes: Vec<GraphNodePointer>,
    pub liquidity: Liquidity,
}
#[derive(Debug, Clone, PartialEq)]
pub struct StableSharePair{
    pub share_asset_node: GraphNodePointer,
    pub token_to_share: bool,
    pub adjacent_nodes: Vec<GraphNodePointer>,
    pub liquidity: Liquidity,
}

#[derive(Debug, Clone, PartialEq)]
pub struct XcmPair{
    pub adjacent_node: GraphNodePointer,
}

impl DexPair{

    pub fn get_lp_id(&self) -> Option<String>{
        match &self.liquidity{
            Liquidity::Dex(dexLp) => dexLp.lp_id.clone(),
            _ => panic!("Tried to get dex contract address from non-dex liquidity"),
        }
    }
}
impl DexV3Pair {
    pub fn get_lp_id(&self) -> Option<String>{
        match &self.liquidity{
            Liquidity::DexV3(dexLp) => dexLp.lp_id.clone(),
            _ => panic!("Tried to get dex contract address from non-dex liquidity"),
        }
    }

}

impl StablePair{
    pub fn get_lp_id(&self) -> Option<String>{
        match &self.liquidity{
            Liquidity::Stable(stable_lp) => stable_lp.pool_id.clone(),
            _ => panic!("Tried to get stable contract address from non-stable liquidity"),
        }
    }
    pub fn get_swap_fee(&self) -> u128{
        match &self.liquidity{
            Liquidity::Stable(stable_lp) => stable_lp.swap_fee,
            _ => panic!("Tried to get stable contract address from non-stable liquidity"),
        }
    }
}

impl StableSharePair{
    pub fn get_lp_id(&self) -> Option<String>{
        match &self.liquidity{
            Liquidity::Stable(stable_lp) => stable_lp.pool_id.clone(),
            Liquidity::StableShare(stable_lp) => stable_lp.pool_id.clone(),
            _ => panic!("Tried to get stable lp id"),
        }
    }
    pub fn get_swap_fee(&self) -> u128{
        match &self.liquidity{
            Liquidity::Stable(stable_lp) => stable_lp.swap_fee,
            Liquidity::StableShare(stable_lp) => stable_lp.swap_fee,
            _ => panic!("Tried to get stable swap fee"),
        }
    }
}

impl AdjacentNodePair{
    pub fn new(adjacent_node: &GraphNodePointer, liquidity: Option<Liquidity>, pair_type: u64) -> AdjacentNodePair{
        AdjacentNodePair{
            adjacent_node: Rc::clone(adjacent_node),
            liquidity,
            pair_type
        }
    }
    pub fn get_dex_liquidity(&self) -> (u128, u128){
        match &self.liquidity.as_ref().unwrap(){
            Liquidity::Dex(dexLp) => (dexLp.base_liquidity, dexLp.adjacent_liquidity),
            _ => panic!("Tried to get dex liquidity from non-dex liquidity"),
        }
    }
    pub fn get_cex_liquidity_price(&self) -> (u128, u128){
        match &self.liquidity.as_ref().unwrap(){
            Liquidity::Cex(cexLp) => (cexLp.bid_price, cexLp.ask_price),
            _ => panic!("Tried to get cex liquidity from non-cex liquidity"),
        }
    }
    pub fn get_cex_liquidity_decimals(&self) -> (u128, u128){
        match &self.liquidity.as_ref().unwrap(){
            Liquidity::Cex(cexLp) => (cexLp.bid_decimals, cexLp.ask_decimals),
            _ => panic!("Tried to get cex liquidity from non-cex liquidity"),
        }
    }
}

impl GraphNode{
    // Get pool from key of adjacent node
    pub fn get_v3_lp_stats_from_pair(&self, adjacent_asset_key: String, contract_address: String) -> Option<DexV3> {
        for pair in &self.adjacent_pairs2{
            match pair{
                AdjacentNodePair2::DexV3Pair(dex_pair) => {
                    if dex_pair.adjacent_node.borrow().asset_key == adjacent_asset_key{
                        if let Liquidity::DexV3(dex_lp) = &dex_pair.liquidity{
                            if contract_address.eq_ignore_ascii_case(&dex_lp.lp_id.clone().unwrap()){
                                return Some(dex_lp.clone())
                            }
                            
                        }
                    }
                },
                AdjacentNodePair2::DexPair(dex_pair) => {
                    if dex_pair.adjacent_node.borrow().asset_key == adjacent_asset_key{
                        if let Liquidity::Dex(dex_lp) = &dex_pair.liquidity{

                        }
                    }
                },
                _ => {}
            }

        }
        None
    }

    pub fn get_stable_lp_stats_from_pair(&self, adjacent_asset_key: String, pool_id: String) -> Option<StableLp> {
        for pair in &self.adjacent_pairs2{
            match pair{
                AdjacentNodePair2::StablePair(stable_pair) => {
                    let adjacent_nodes = stable_pair.adjacent_nodes.clone();
                    for (index, stable_pool_node) in adjacent_nodes.iter().enumerate(){
                        // println!("Stable pool key: {}", stable_pool_node.borrow().get_asset_key());
                        // println!("Adjacent asset key: {}", adjacent_asset_key);
                        if stable_pool_node.borrow().get_asset_key().eq(&adjacent_asset_key){
                            if let Liquidity::Stable(stable_lp) = &stable_pair.liquidity{
                                println!("Stable pool id {:?}", stable_lp);

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

    pub fn get_stable_share_lp_stats_from_pair(&self, adjacent_asset_key: String, pool_id: String) -> Option<StableLp> {
        for pair in &self.adjacent_pairs2{
            match pair{
                AdjacentNodePair2::StableSharePair(stable_pair) => {
                    let adjacent_nodes = stable_pair.adjacent_nodes.clone();
                    for (index, stable_pool_node) in adjacent_nodes.iter().enumerate(){
                        if stable_pool_node.borrow().asset_key == adjacent_asset_key{
                            if let Liquidity::Stable(stable_lp) = &stable_pair.liquidity{
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

    pub fn get_v3_adjacent_node_pair(&self, adjacent_asset_key: String, contract_address: String) -> Option<AdjacentNodePair2>{
        for pair in &self.adjacent_pairs2{
            match pair{
                AdjacentNodePair2::DexV3Pair(dex_pair) => {
                    if dex_pair.adjacent_node.borrow().asset_key == adjacent_asset_key{
                        if let Liquidity::DexV3(dex_lp) = &dex_pair.liquidity{
                            if contract_address.eq_ignore_ascii_case(&dex_lp.lp_id.clone().unwrap()){
                                return Some(pair.clone());
                            }
                            
                        }
                    }
                },
                _ => {}
            }

        }
        None
    }

    pub fn get_stable_adjacent_node_pair(&self, adjacent_asset_key: String, pool_id: String) -> Option<(AdjacentNodePair2, usize)>{
        for pair in &self.adjacent_pairs2{
            match pair{
                AdjacentNodePair2::StablePair(stable_pair) => {
                    let adjacent_nodes = stable_pair.adjacent_nodes.clone();
                    for (index, stable_pool_node) in adjacent_nodes.iter().enumerate(){
                        if stable_pool_node.borrow().asset_key == adjacent_asset_key{
                            if let Liquidity::Stable(stable_lp) = &stable_pair.liquidity{
                                if pool_id.eq_ignore_ascii_case(&stable_lp.pool_id.clone().unwrap()){
                                    return Some((pair.clone(), index))
                                }
                            }
                        }
                    }
                },
                AdjacentNodePair2::StableSharePair(stable_pair) => {
                    let adjacent_nodes = stable_pair.adjacent_nodes.clone();
                    for (index, stable_pool_node) in adjacent_nodes.iter().enumerate(){
                        if stable_pool_node.borrow().asset_key == adjacent_asset_key{
                            if let Liquidity::Stable(stable_lp) = &stable_pair.liquidity{
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
    
    
    pub fn get_stable_share_adjacent_node_pair(&self, adjacent_asset_key: String, pool_id: String) -> Option<(AdjacentNodePair2, usize)>{
        println!("Getting stable share adjacent node pair: {}", &self.asset_key);
        println!("Adjacent asset key: {}", adjacent_asset_key);
        for pair in &self.adjacent_pairs2{
            match pair{
                AdjacentNodePair2::StableSharePair(stable_pair) => {
                    let adjacent_nodes = stable_pair.adjacent_nodes.clone();
                    // let base_node = stable_pair.
                    for (index, stable_pool_node) in adjacent_nodes.iter().enumerate(){
                        println!("Stable pool key: {}", stable_pool_node.borrow().get_asset_key());
                        if stable_pool_node.borrow().asset_key == adjacent_asset_key{
                            if let Liquidity::StableShare(stable_lp) = &stable_pair.liquidity{
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
    pub fn is_cex_token(&self) -> bool {
        self.asset.borrow().is_cex_token()
    }
    pub fn best_path_value_display(&self, token_graph: &TokenGraph2) -> f64 {
        match self.asset.borrow().token_data{
            TokenData::CexAsset { .. } => {
                if self.get_asset_symbol() == "USDT"{
                    let usdt_asset_decimals = 4;
                    self.best_path_value.to_f64().unwrap().clone() / f64::powi(10.0, usdt_asset_decimals as i32)
                } 
                else {
                    let kucoin_asset_decimals = token_graph.asset_registry.get_kucoin_asset_decimals(self.get_asset_location().unwrap());
                    self.best_path_value.to_f64().unwrap().clone() / f64::powi(10.0, kucoin_asset_decimals as i32)
                }
                
            },
            _ => {
                self.best_path_value.to_f64().unwrap().clone() / f64::powi(10.0, self.get_asset_decimals() as i32)
            }
        }
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
            path_string.push_str(&format!("{} {} {} ->", path_node.borrow().get_asset_key(), path_node.borrow().get_asset_name(), &self.path_values[i]));
        }
        path_string
    }
}

// pub fn calculate_edge_3(token_graph: &TokenGraph2, primary_node: &GraphNodePointer, adjacent_nodes: &AdjacentNodePair2, input: u128) -> u128{
//     match adjacent_nodes {
//         AdjacentNodePair2::DexPair(adj_pair) => {
//             let (base_liquidity, adjacent_liquidity) = match adj_pair.liquidity {
//                 Liquidity::Dex(dexLp) => (dexLp.base_liquidity, dexLp.adjacent_liquidity),
//                 _ => panic!("Tried to get dex liquidity from non-dex liquidity"),
//             };
//             // calculate_dex_edge(adjacent_nodes, base_liquidity, adjacent_liquidity, input)
//         0
            
//         },
//         AdjacentNodePair2::StablePair(adj_pair) => {
//             0
//         },
//         AdjacentNodePair2::CexPair(adj_pair) => {
//             calculate_cex_edge(token_graph, primary_node, adjacent_nodes, input)
//         },
//         _ => 0
//     }
// }

pub fn calculate_dex_edge(adjacent_node: &AdjacentNodePair2, input_amount: BigInt) -> BigInt{
    match adjacent_node{
        AdjacentNodePair2::DexPair(adj_pair) => {
            let (base_liquidity, adjacent_liquidity) = match adj_pair.liquidity.clone() {
                Liquidity::Dex(dexLp) => (dexLp.base_liquidity, dexLp.adjacent_liquidity),
                _ => panic!("Tried to get dex liquidity from non-dex liquidity"),
            };
            let base_liquidity = base_liquidity.to_bigint().unwrap();
            let adjacent_liquidity = adjacent_liquidity.to_bigint().unwrap();
            let increments = 5000;
            let token_1_increment = input_amount / BigInt::from(increments);
            let swap_fee = (token_1_increment.to_f64().unwrap().mul(0.003)) as u128;
            let token_1_increment_minus_swap = token_1_increment - swap_fee;
            
            let mut token_1_changing_liquidity = base_liquidity.clone();
            let mut token_2_changing_liquidity = adjacent_liquidity.clone();
            let mut total_slippage = BigInt::default();
            let mut total_token_2_output = BigInt::default();

            for _ in 0..increments {
                let token_2_out = (&token_2_changing_liquidity * token_1_increment_minus_swap.clone())
                    / (&token_1_changing_liquidity + token_1_increment_minus_swap.clone());
                let slip = (&token_2_out / &token_2_changing_liquidity) * &token_2_out;
                total_token_2_output += &token_2_out - &slip;
                token_2_changing_liquidity -= &token_2_out - &slip;
                token_1_changing_liquidity += token_1_increment_minus_swap.clone();
                total_slippage += &slip;
            }

            total_token_2_output
        },
        AdjacentNodePair2::DexV3Pair(adj_pair) => {
            let (contract_address, token_0, token_1, active_liquidity, current_tick, fee_rate, upper_ticks, lower_ticks) = match &adj_pair.liquidity {
                Liquidity::DexV3(dexLp) => (dexLp.lp_id.clone(), dexLp.token_0.clone(), dexLp.token_1.clone(), dexLp.active_liquidity, dexLp.current_tick, dexLp.fee_rate,  dexLp.upper_ticks.clone(), dexLp.lower_ticks.clone()),
                _ => panic!("Tried to get dex liquidity from non-dex liquidity"),
            };
            let contract_address_check = contract_address.clone().unwrap().to_uppercase();
            let target_contract = "0x17c19AefF4caBfe76633A20e6B0eD903Df48dD56".to_string().to_uppercase();

            let q96: BigInt = BigInt::from(2).pow(96);
            let one = BigRational::one();
            let zero = BigRational::zero();

            let adj_node_contract_address = adj_pair.adjacent_node.borrow().asset.borrow().get_asset_contract_address().clone();
            
            // Get CURRENT TICK | ACTIVE LIQUIDITY | FEE RATE? | UPPER/LOWER TICKS
            let mut current_tick = current_tick.to_bigint().unwrap();
            let active_liquidity = active_liquidity.to_bigint().unwrap();
            let fee_ratio: BigRational = BigRational::new(BigInt::from(fee_rate), BigInt::from(10).pow(6));

            // Base node is input, adjacent is output
            let input_token_index;
            if token_0.to_string().eq(&adj_node_contract_address.unwrap()) {
                input_token_index = 1;
            } else {
                input_token_index = 0;
            }

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
        },
        _ => panic!("Tried to get dex liquidity from non-dex liquidity"),
    }
    
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

pub fn calculate_cex_edge(token_graph: &TokenGraph2, primary_node: &GraphNodePointer, adjacent_pair: &AdjacentNodePair2, input_amount: u128) -> u128{
    match adjacent_pair {
        AdjacentNodePair2::CexPair(adj_pair) => {
            let (bid_price, bid_decimals, ask_price, ask_decimals ) = match adj_pair.liquidity {
                Liquidity::Cex(cexLp) => (cexLp.bid_price, cexLp.bid_decimals, cexLp.ask_price, cexLp.ask_decimals),
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
                let asset_decimals = token_graph.get_asset_decimals_for_kucoin_asset(&adj_pair.adjacent_node);

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

pub fn calculate_stable_edge(primary_node: &GraphNodePointer, adjacent_pair: &AdjacentNodePair2, input_amount: BigInt, target_index: usize) -> Option<BigInt>{

    match adjacent_pair{
        AdjacentNodePair2::StablePair(adj_pair) => {
            let lp_data: StableLp = match &adj_pair.liquidity {
                Liquidity::Stable(stableLp) => stableLp.clone(),
                _ => panic!("Tried to get dex liquidity from non-dex liquidity"),
            };
            let base_liquidity = lp_data.base_liquidity.to_bigint().unwrap();
            let base_token_precision = lp_data.base_token_precision;
            let adjacent_liquidity: Vec<BigInt> = lp_data.adjacent_liquidity.clone().into_iter().map(|x| x.to_bigint().unwrap()).collect();
            let a = lp_data.a;
            let pool_id: usize = lp_data.pool_id.clone().unwrap().parse().unwrap();
            let total_supply = lp_data.total_supply.to_bigint().unwrap();
            let adjacent_token_precisions = lp_data.adjacent_token_precisions.clone();
            let chain_id = lp_data.chain_id.clone();
            let input_asset_symbol = primary_node.borrow().get_asset_symbol();
            let output_asset_symbol = adj_pair.adjacent_nodes[target_index].borrow().get_asset_symbol();
            // println!("{} {} -> {}", chain_id, input_asset_symbol, output_asset_symbol);
            let a_precision = match lp_data.chain_id {
                2023 => 1,
                2000 => 100,
                _ => 1
            };
            let mut balances: Vec<BigInt> = adjacent_liquidity.clone().into_iter().map(|x| x.to_bigint().unwrap()).collect();
            let mut token_precisions = adjacent_token_precisions.clone();
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

            let swap_fee = BigInt::from(lp_data.swap_fee);
            let fee_precision =  "10000000000".parse::<BigInt>().unwrap();
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
                    // Add input to share reserves, calculate output and then later convert output to reserves
                    balances[token_in_index] = balances[token_in_index].clone() + input_amount.clone();
                }
            } else {
                // Pool just uses reserve balances
                balances[token_in_index] = balances[token_in_index].clone() + input_amount.clone();
            }

            let y = get_y(&balances, target_index as u128, d.clone(), a, a_precision, lp_data.chain_id.clone()).unwrap();


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
        AdjacentNodePair2::StableSharePair(adj_pair) => {
            // println!(" BALANCES 3{:?}", adj_pair.liquidity);
            // let (base_liquidity, base_token_precision, adjacent_liquidity, a, total_supply, adjacent_token_precisions) = match &adj_pair.liquidity {
            //     Liquidity::Stable(stableLp) => (stableLp.base_liquidity, stableLp.base_token_precision, stableLp.adjacent_liquidity.clone(), stableLp.a, stableLp.total_supply, stableLp.adjacent_token_precisions.clone()),
            //     _ => panic!("Tried to get dex liquidity from non-dex liquidity"),
            // };
            let lp_data: StableShareLp = match &adj_pair.liquidity {
                Liquidity::StableShare(stableLp) => stableLp.clone(),
                _ => panic!("Tried to get dex liquidity from non-dex liquidity"),
            };

            // println!("------------------");

            let a_precision = match lp_data.chain_id {
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
            let all_pool_balances: Vec<BigInt> = lp_data.pool_assets_liquidity.clone().into_iter().map(|x| x.to_bigint().unwrap()).collect();
            let share_issuance = BigInt::from(lp_data.share_issuance.unwrap());
            let token_precisions = lp_data.token_precisions.clone();

            let primary_node_decimals = primary_node.borrow().get_asset_decimals();
            let primary_node_symbol = primary_node.borrow().get_asset_symbol();
            
            // let fee_precision = BigInt::from(10000000000.to_u128().unwrap());
            let fee = adj_pair.get_swap_fee();
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

            if lp_data.token_to_share.unwrap(){
                // Base asset -> Share asset
                let base_asset_index = lp_data.base_asset_index.unwrap();
                let base_precision = token_precisions[base_asset_index];
                let share_asset = lp_data.share_asset.clone();

                // println!("Base precision: {}", base_precision);
                // println!("Primary node decimals: {}", primary_node_decimals);
                // println!("Input Amount: {}", input_amount);
                // println!("Total node decimals: {}", primary_node_decimals as u128);
                let total_node_decimals = base_precision as u128 + primary_node_decimals as u128;
                let input_asset_index = lp_data.base_asset_index.unwrap();
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


                

                let d_0 = get_d(&reserves_normalized, lp_data.a, lp_data.chain_id, a_precision).unwrap();
                let d_1 = get_d(&updated_reserves_normalized, lp_data.a, lp_data.chain_id, a_precision).unwrap();




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

                let adjusted_d = get_d(&adjusted_reserves, lp_data.a, lp_data.chain_id, a_precision).unwrap();

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
                let initial_d = get_d(&reserves_normalized, lp_data.a, lp_data.chain_id, a_precision).unwrap();
                let d_1 = initial_d.clone().add(input_amount.mul(&initial_d).checked_div(&share_issuance)?);
                let y = get_y(&reserves_normalized, target_index as u128, d_1.clone(), lp_data.a, a_precision, lp_data.chain_id).unwrap();

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

                let y_1 = get_y(&reserves_reduced, target_index as u128, d_1.clone(), lp_data.a, a_precision, lp_data.chain_id).unwrap();
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
        balances = vec![balances[0].clone(), balances[3].clone()];
    }

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

        // let end = y.checked_sub(&prev_y).unwrap().abs().le(&one);

        if y.checked_sub(&prev_y).unwrap().abs().le(&one) {
            // println!("Number of iterations: {}", i);
            break;
        }
    }
    // println!("Y End: {}", y);
    return Some(y);
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

pub fn calculate_edge_2(
    primary_node: &GraphNodePointer,
    adjacent_node: &AdjacentNodePair,
    input_amount: u128,
) -> u128 {
    let (base_liquidity, adjacent_liquidity) = adjacent_node.get_dex_liquidity();

    let base_liquidity = base_liquidity.to_bigint().unwrap();
    let adjacent_liquidity = adjacent_liquidity.to_bigint().unwrap();
    let increments = 5000;
    let token_1_increment = input_amount / increments;
    let swap_fee = (token_1_increment as f64 * 0.003) as u128;
    let token_1_increment_minus_swap = token_1_increment - swap_fee;
    
    let mut token_1_changing_liquidity = base_liquidity.clone();
    let mut token_2_changing_liquidity = adjacent_liquidity.clone();
    let mut total_slippage = BigInt::default();
    let mut total_token_2_output = BigInt::default();

    for _ in 0..increments {
         let token_2_out = (&token_2_changing_liquidity * token_1_increment_minus_swap)
            / (&token_1_changing_liquidity + token_1_increment_minus_swap);
        let slip = (&token_2_out / &token_2_changing_liquidity) * &token_2_out;
        total_token_2_output += &token_2_out - &slip;
        token_2_changing_liquidity -= &token_2_out - &slip;
        token_1_changing_liquidity += token_1_increment_minus_swap;
        total_slippage += &slip;
    }


    // (
    //     total_token_2_output.to_u128().unwrap(),
    //     (token_1_changing_liquidity.to_u128().unwrap(), token_2_changing_liquidity.to_u128().unwrap()),
    // )
    total_token_2_output.to_u128().unwrap()

}

fn calculate_kucoin_edge_2(token_graph: &TokenGraph2, primary_node: &GraphNodePointer, adjacent_pair: &AdjacentNodePair, input_amount: u128) -> u128{

    //Asset -> USDT
    let (bid_price, ask_price) = adjacent_pair.get_cex_liquidity_price();
    let (bid_decimals, ask_decimals) = adjacent_pair.get_cex_liquidity_decimals();
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
        let asset_decimals = token_graph.get_asset_decimals_for_kucoin_asset(&adjacent_pair.adjacent_node);

        //Convert number to normal, display value
        let converted_input = input_amount as f64 / f64::powi(10.0, usdt_token_decimals as i32);
        let converted_ask = bid_price.clone() as f64 / f64::powi(10.0, ask_decimals as i32);
        

        let asset_output = converted_input as f64 / converted_ask;
        let asset_output_converted = asset_output * f64::powi(10.0, asset_decimals as i32) ;
        asset_output_converted as u128
    }
}