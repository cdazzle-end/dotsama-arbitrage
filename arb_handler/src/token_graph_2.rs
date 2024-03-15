// use crate::asset_registry::AssetLocation;
// use crate::token::{self, TokenData};
// use crate::{LiqPoolRegistry, asset_registry, liq_pool_registry};
// use crate::liq_pool_registry_2::LiqPoolRegistry2;
use num::{BigInt, BigUint, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, FromPrimitive, Num, One, Signed, ToPrimitive, Zero};
use num::bigint::{ToBigInt, ToBigUint};
use num::BigRational;
use num;
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
use crate::adjacency_table_2::{AdjacencyGroup, AdjacencyTable2, CexLp, DexLp, DexV3, GroupType, Liquidity, StableLp};
type AssetPointer = Rc<RefCell<Asset>>;
type GraphNodePointer = Rc<RefCell<GraphNode>>;

const MIN_TICK: i64 = -887272;
const MAX_TICK: i64 = 887272;
const MIN_TICK_DATA: TickData = TickData{tick: MIN_TICK, liquidity_delta: 0};
const MAX_TICK_DATA: TickData = TickData{tick: MAX_TICK, liquidity_delta: 0};

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
                            let path_value = calculate_stable_edge( &self, &current_node, &adjacent_pair, current_node.borrow().best_path_value.clone(), i);
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

                                let lp_id = stable_pair.get_lp_id(i);
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
        let asset_symbol = asset_symbol.to_uppercase();
        for node_bucket in self.node_map.values(){
            for node in node_bucket{
                if node.borrow().get_chain_id() == chain_id && node.borrow().get_asset_symbol().to_uppercase() == asset_symbol{
                    return Some(Rc::clone(node));
                }
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
            GroupType::Dex =>{
                //Find node that corresponds to adjacent asset. Add it to current node's adjacency list
                let bucket = node_map.get(&adj_group.adjacent_asset[0].borrow().get_map_key()).unwrap();
                for potential_adjacent_node in bucket{
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
    pub fn get_lp_id(&self, index: usize) -> Option<String>{
        match &self.liquidity{
            Liquidity::Stable(stable_lp) => stable_lp.lp_id.clone(),
            _ => panic!("Tried to get stable contract address from non-stable liquidity"),
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
#[derive(PartialEq, Debug)]
pub enum Color{
    White,
    Gray,
    Black
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

pub fn calculate_stable_edge(token_graph: &TokenGraph2, primary_node: &GraphNodePointer, adjacent_pair: &AdjacentNodePair2, input_amount: BigInt, target_index: usize) -> BigInt{

    match adjacent_pair{
        AdjacentNodePair2::StablePair(adj_pair) => {
            // println!(" BALANCES 3{:?}", adj_pair.liquidity);
            let (base_liquidity, base_token_precision, adjacent_liquidity, a, total_supply, adjacent_token_precisions) = match &adj_pair.liquidity {
                Liquidity::Stable(stableLp) => (stableLp.base_liquidity, stableLp.base_token_precision, stableLp.adjacent_liquidity.clone(), stableLp.a, stableLp.total_supply, stableLp.adjacent_token_precisions.clone()),
                _ => panic!("Tried to get dex liquidity from non-dex liquidity"),
            };
            // println!("------------------");
            // println!("STABLE SWAP");
            let mut balances: Vec<BigInt> = adjacent_liquidity.clone().into_iter().map(|x| x.to_bigint().unwrap()).collect();
            let mut token_precisions = adjacent_token_precisions.clone();
            balances.push(base_liquidity.clone().to_bigint().unwrap());
            token_precisions.push(base_token_precision.clone());

            let base_precision = base_token_precision.to_string().len() - 1;
            let primary_node_decimals = primary_node.borrow().get_asset_decimals();
            let primary_node_symbol = primary_node.borrow().get_asset_symbol();
            let total_node_decimals = base_precision as u128 + primary_node_decimals as u128;

            let input_amount = input_amount * f64::powi(10.0, base_precision as i32) as u128;
            // println!("Input amount: {}", input_amount);
            let input_formatted = input_amount.to_f64().unwrap() / f64::powi(10.0, total_node_decimals as i32);
            // println!("{} {} -> {}", primary_node.borrow().asset_key, input_formatted, adj_pair.adjacent_nodes[target_index].borrow().asset_key);
            // println!("Input unformatted: {}", input_amount);
            let swap_fee: BigInt;
            if a == 10000{
                swap_fee = BigInt::from(5000000)
            } else {
                swap_fee = BigInt::from(25000000)
            }
            let fee_precision =  "10000000000".parse::<BigInt>().unwrap();
            let fee_ration = BigRational::from(swap_fee.clone()) / BigRational::from(fee_precision.clone());

            // let old_d = getD(&balances, a).unwrap();
            let d = total_supply.clone();
            // println!("D: {}", d);

            let token_in_index = balances.len() - 1;
            balances[token_in_index] = balances[token_in_index].clone() + input_amount;
            let y = getY(&balances, target_index as u128, d, a, primary_node_symbol.clone()).unwrap();

            // println!("OLD Y: {}", old_y.to_f64().unwrap());

            // let y = get_y(&balances, target_index as u128, d, a).unwrap();

            // println!("NEW Y: {}", y.to_f64().unwrap());

            let mut dy = BigRational::from(balances[target_index].clone() - y);
            let fee_amount = (BigRational::from(dy.clone()) * fee_ration.clone());

            dy = dy - fee_amount;

            let mut total_output = BigInt::from(0);
            if a != 10000{
                if primary_node_symbol != "LKSM"{
                    let lksm_ratio = BigRational::from(balances[2].clone()).checked_div(&BigRational::from(balances[0].clone())).unwrap();
                    let real_output = lksm_ratio.checked_mul(&BigRational::from(dy.clone())).unwrap().round().numer().clone();
                    total_output = real_output;
                } else {
                    let lksm_ratio = BigRational::from(balances[2].clone()).checked_div(&BigRational::from(balances[3].clone())).unwrap();
                    let real_output = BigRational::from(dy.clone()).checked_div(&lksm_ratio).unwrap().round().numer().clone();
                    total_output = real_output;
                }
            } else{
                total_output = total_output.to_bigint().unwrap() + dy.round().numer();
            }
            let target_node = &adj_pair.adjacent_nodes[target_index];
            let target_decimals = target_node.borrow().get_asset_decimals();
            let target_precision = token_precisions[target_index];
            let target_precision_number = target_precision.to_string().len() - 1;
            let total_target_decimals = target_decimals as u128 + target_precision_number as u128;
            let total_output_formatted = total_output.to_f64().unwrap() / f64::powi(10.0, total_target_decimals as i32);
            // println!("Total output: {}", total_output_formatted);
            // println!("------------------");

            let total_output_minus_precision = total_output.to_f64().unwrap() / f64::powi(10.0, target_precision_number as i32);
            total_output_minus_precision.to_bigint().unwrap()
            
        },
        _ => panic!("Tried to get stable liquidity from non-stable liquidity")
    }
}

pub fn getD(balances: &Vec<BigInt>, a: u128) -> Option<u128> {
    let zero = BigInt::from(0u128);
    let one = BigInt::from(1u128);
    let two = BigInt::from(2u128);
    let mut sum = BigInt::from(0u128);
    let mut ann = BigInt::from(a);
    let mut balance_size = BigInt::from(balances.len());
    let mut a_precision = BigInt::from(100 as usize);
    let mut ann_formatted: BigRational;
    let mut balances = balances.clone();

    if ann == BigInt::from(10000u128){
        // ann = BigUint::from(100u128);
        a_precision = one.clone();
        ann_formatted = BigRational::new(BigInt::from(100), BigInt::from(1u128));
        // balance_size = BigInt::from(balances.len());
    } else {
        ann = BigInt::from(30u128);
        a_precision = one.clone();
        ann_formatted = BigRational::new(BigInt::from(30), BigInt::from(1u128));
        // balances = balances.split_at(2).0.to_vec();
        balances = vec![balances[0].clone(), balances[3].clone()];
        balance_size = BigInt::from(balances.len());
        
    }

    for x in balances.iter(){
        let balance: u128 = (*x).to_biguint().unwrap().to_u128().unwrap();
        sum = sum.checked_add(&balance.to_bigint()?)?;
        ann_formatted = ann_formatted.checked_mul(&BigRational::from(balance_size.clone()))?;
    }

    if sum == zero {
        return zero.to_u128();
    }
    
    let mut prev_d: BigInt;
    let mut d: BigInt = sum.clone();
    // println!("Start D: {}", d);
    for i in 0..255{
        let mut p_d = d.clone();
        for x in balances.iter(){
            let balance: u128 = (*x).to_u128()?;
            let div_op = BigInt::from(balance).checked_mul(&balance_size).unwrap();
            p_d = p_d.checked_mul(&d).unwrap().checked_div(&div_op).unwrap();
        }
        prev_d = d.clone();
        // D = (Ann * sum + pD * balance.length) * D / ((Ann - 1) * D + (balance.length + 1) * pD)
        let t_1 = ann_formatted.clone() * &sum + &p_d * &balance_size;
        
        let t_1_i: BigInt = t_1.round().numer().clone();
        
        let t_2 = ann_formatted.checked_sub(&one.clone().into())?.checked_mul(&d.clone().into())?;
        let t_2_i: BigInt = t_2.round().numer().clone();

        let t_3 = &balance_size.checked_add(&one.clone())?.checked_mul(&p_d.clone())?;

        let t_4 = t_2.checked_add(&t_3.clone().into())?;

        d = t_1.checked_mul(&d.clone().into())?.checked_div(&t_4)?.round().numer().clone();
        if &d > &prev_d {
            if &d - &prev_d <= one {
                break;
            }
        } else if &prev_d - &d <= one {
            break;
        }
    }
    return d.to_u128();
}

pub fn get_stable_swap_amount(swapFee: BigInt, totalSupply: BigInt, a: BigInt, precisions: Vec<BigInt>, balances: Vec<BigInt>){
    let swap_a = a.clone();
    let swap_d = totalSupply.clone();
    let fee_precisions = "10000000000".parse::<BigInt>().unwrap();
    let swap_balances = balances.clone();
}

pub fn get_y(balances: &Vec<BigInt>, token_index: u128, target_d: u128, amplitude: u128) -> Option<u128> {
    let one = BigInt::from(1);
    let two = BigInt::from(2);
    let mut c = BigInt::from(target_d);
    let mut sum = BigInt::from(0);
    let ann_initial = BigInt::from(amplitude);
    let balance_size = BigInt::from(balances.len());
    let mut ann = ann_initial.clone();
    let target_d_bigint = BigInt::from(target_d);
    let token_index_usize = token_index as usize;
    let a_precision = BigInt::from(100);
    let NUMBER_OF_ITERATIONS_TO_CONVERGE = 255;

    for (i, balance_ref) in balances.iter().enumerate() {
        ann = &ann * &balance_size;
        if i == token_index_usize {
            continue;
        }
        sum = sum.add(balance_ref);
        let div_op = balance_ref.checked_mul(&balance_size).unwrap();
        c = c.checked_mul(&target_d_bigint).unwrap().checked_div(&div_op).unwrap();
    }
    c = c.checked_mul(&target_d_bigint).unwrap().checked_mul(&a_precision).unwrap().checked_div(&ann.checked_mul(&balance_size).unwrap()).unwrap();
    // println!("C: {}", c);
    let b = sum.add(target_d_bigint.clone().mul(&a_precision).checked_div(&ann).unwrap());
    // let prev_y = BigInt::from(0);
    let mut y = target_d_bigint.clone();

    // println!("B: {}", b);
    // println!("Y Start: {}", y);

    for i in 0..NUMBER_OF_ITERATIONS_TO_CONVERGE {
        let prev_y = y.clone();
        y = y.clone().mul(&y).add(&c).checked_div(&y.mul(&two).add(&b).checked_sub(&target_d_bigint).unwrap()).unwrap();

        // let end = y.checked_sub(&prev_y).unwrap().abs().le(&one);

        if y.checked_sub(&prev_y).unwrap().abs().le(&one) {
            break;
        }
    }
    // println!("Y: {}", y);
    return y.to_u128()
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