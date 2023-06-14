use crate::asset_registry::AssetLocation;
use crate::token::{self, TokenData};
// use crate::adj_list_node::{AdjListNode, AdjListNodeOption, TokenNodeIterator};
use crate::{LiqPoolRegistry, asset_registry, liq_pool_registry};
use num::{BigInt, ToPrimitive};
use num::bigint::ToBigInt;
use num::BigRational;
use num;
// use num::rational::BigRational;


use std::cell::RefCell;
use std::collections::{VecDeque, HashMap};
// use std::intrinsics::powf64;
use std::rc::Rc;
use crate::{asset_registry::{AssetRegistry, Asset}};
use crate::adjacency_table::AdjacencyTable;

type AssetPointer = Rc<RefCell<Asset>>;
// type TableBucket = Vec<Vec<AssetPointer>>;
type GraphNodePointer = Rc<RefCell<GraphNode>>;

pub struct TokenGraph{
    pub node_map: HashMap<String, Vec<GraphNodePointer>>,
    pub asset_registry: AssetRegistry
}

impl TokenGraph{
    pub fn build_graph(asset_registry: AssetRegistry, adjacency_table: AdjacencyTable) -> TokenGraph{
        let mut graph_nodes: Vec<GraphNodePointer> = Vec::new();
        //Map of all graph nodes
        let mut node_map: HashMap<String, Vec<GraphNodePointer>> = HashMap::new();
        // let assets = asset_registry.get_all_assets();

        //Create a graph node for every asset
        for asset in &asset_registry.get_all_assets(){
            let new_node = Rc::new(RefCell::new(GraphNode{
                asset: Rc::clone(&asset),
                adjacent_nodes: Vec::new(),
                asset_key: asset.borrow().token_data.get_map_key(),
                pred: None,
                distance: 0,
                time_d: 0,
                time_f: 0,
                color: Color::White,
                best_path_value: 0,
                best_path_value_display: 0.0,
                path_edges: Vec::new(),
                best_path: Vec::new(),
                path_values: Vec::new(),
            }));
            graph_nodes.push(Rc::clone(&new_node));
            let bucket = node_map.entry(new_node.borrow().asset_key.clone()).or_insert(Vec::new());
            bucket.push(new_node);
        }

        //For each node, get adjacent assets. Find the node that corresponds to each adjacent asset. Add adjacent asset's node to current node
        for current_node in graph_nodes{

            let node_asset = current_node.borrow().asset.clone();
            //Get adjacent assets & liquidity for current node
            for adjacent_asset in adjacency_table.get_adjacent_assets(node_asset){
                //Find node that corresponds to adjacent asset. Add it to current node's adjacency list
                let bucket = node_map.get(&adjacent_asset.0.borrow().token_data.get_map_key()).unwrap();
                for potential_adjacent_node in bucket{
                    if adjacent_asset.0.borrow().token_data.get_map_key() == potential_adjacent_node.borrow().asset_key{
                        // println!("{} - {}", adjacent_asset.1.0, adjacent_asset.1.1);
                        current_node.borrow_mut().adjacent_nodes.push((Rc::clone(potential_adjacent_node), (adjacent_asset.1.0, adjacent_asset.1.1)));
                    }
                }
            }

            //If asset is cross chain, get it's cross chain assets and add them as adjacent nodes
            let current_node_location = current_node.borrow().asset.borrow().asset_location.clone();
            match current_node_location{
                Some(asset_location) => {
                    for cross_chain_asset in asset_registry.get_assets_at_location(asset_location){
                        let bucket = node_map.get(&cross_chain_asset.borrow().token_data.get_map_key()).unwrap();
                        for graph_node in bucket{
                            if cross_chain_asset.borrow().token_data.get_map_key() == graph_node.borrow().asset.borrow().token_data.get_map_key(){
                                current_node.borrow_mut().adjacent_nodes.push((Rc::clone(graph_node), (0,0)));
                            }
                        }
                    }
                },
                None => ()
            }
        }

        TokenGraph{ node_map, asset_registry }
    }

    pub fn get_evm_tokens(&self){

    }

    //Get node through asset key
    pub fn get_node(&self, asset_key: String) -> GraphNodePointer{
        let bucket = self.node_map.get(&asset_key).unwrap();
        for node in bucket{
            if node.borrow().asset_key == asset_key{
                return Rc::clone(node);
            }
        }
        panic!("Could not find node with asset key: {}", asset_key);
    }
    
    pub fn get_swap_pair(&self, asset_key_1: String, asset_key_2: String){
        let asset_1 = self.get_node(asset_key_1);
        let adjacent_node = asset_1.borrow().get_adjacent_node(asset_key_2);


    }

    pub fn display_graph(&self){
        let index = 0;
        let first_node;
        for val in self.node_map.values(){
            first_node = val[0].borrow();
            print!("{}: ", first_node.asset_key);
            for adj_node in &first_node.adjacent_nodes{
                print!("{} -> ", adj_node.0.borrow().asset_key)
            }
            break;
        }
    }

    pub fn random_bfs(&self, asset_registry: &AssetRegistry){
        let starting_node = &self.node_map.values().next().unwrap()[0];
        starting_node.borrow_mut().color = Color::Gray;
        let mut node_queue: VecDeque<GraphNodePointer> = VecDeque::new();
        node_queue.push_back(Rc::clone(&starting_node));
        while !node_queue.is_empty(){
            let current_node = node_queue.pop_front().unwrap_or_else(||panic!("Queue should not be empty"));
            println!("{} {} -> Pop", current_node.borrow().asset.borrow().token_data.get_map_key(), current_node.borrow().asset.borrow().token_data.get_asset_name());
            for adjacent_node in &current_node.borrow().adjacent_nodes{
                if adjacent_node.0.borrow().color == Color::White{
                    adjacent_node.0.borrow_mut().color = Color::Gray;
                    adjacent_node.0.borrow_mut().distance = current_node.borrow().distance + 1;
                    adjacent_node.0.borrow_mut().pred = Some(Rc::clone(&current_node));
                    println!("{} {} -> Gray/In queue", adjacent_node.0.borrow().asset.borrow().token_data.get_map_key(), adjacent_node.0.borrow().asset.borrow().token_data.get_asset_name());
                    node_queue.push_back(Rc::clone(&adjacent_node.0));
                    
                }
            }
            current_node.borrow_mut().color = Color::Black;
            println!("{} {} -> Black", current_node.borrow().get_asset_key(), current_node.borrow().get_asset_name());
        }
    }

    pub fn random_dfs(&self){
        let mut time: u32 = 0;
        for(key, bucket) in &self.node_map{
            for node in bucket{
                if node.borrow().color == Color::White{
                    dfs_visit(Rc::clone(&node), &mut time);
                }
            }
        }
    }

    //Cannot travel to node if it already exists in current path
    pub fn find_best_paths_2(&self, asset_key_1: String, asset_key_2: String, input_amount: f64){
        let starting_node = &self.get_node(asset_key_1);
        
        let formatted_input = &input_amount * f64::powi(10.0, starting_node.borrow().get_asset_decimals() as i32);
        starting_node.borrow_mut().best_path_value = formatted_input as u128;
        starting_node.borrow_mut().path_values.push(input_amount);
        starting_node.borrow_mut().best_path.push(Rc::clone(&starting_node));

        let mut node_queue = VecDeque::new();
        node_queue.push_back(Rc::clone(starting_node));
        println!("***");
        let mut first = true;
        while !node_queue.is_empty(){
            println!("queue length: {}", node_queue.len());
            let current_node = node_queue.pop_front().unwrap();
            print!("-");
            let current_node_display = current_node.borrow().best_path_value_display(&self);
            for adjacent_node in &current_node.borrow().adjacent_nodes{
                //Check if adjacent node shares same location with current node
                if current_node.borrow().get_asset_location() != None && current_node.borrow().get_asset_location() == adjacent_node.0.borrow().get_asset_location(){
                    if current_node.borrow().best_path_value > adjacent_node.0.borrow().best_path_value{
                        
                        //Check if adjacent node already exists within path
                        let mut current_path_contains_adjacent_node= false;
                        for path_node in &current_node.borrow().best_path{
                            if path_node.borrow().get_asset_key() == adjacent_node.0.borrow().get_asset_key(){
                                current_path_contains_adjacent_node = true;
                            }
                        }
                        // println!("Contains adj node already: {}", current_path_contains_adjacent_node);
                        adjacent_node.0.borrow_mut().best_path_value = current_node.borrow().best_path_value;
                        adjacent_node.0.borrow_mut().best_path = current_node.borrow().best_path.clone();
                        adjacent_node.0.borrow_mut().best_path.push(Rc::clone(&adjacent_node.0));
                        adjacent_node.0.borrow_mut().path_values = current_node.borrow().path_values.clone();
                        adjacent_node.0.borrow_mut().path_values.push(current_node.borrow().best_path_value_display(&self));
                        if !current_path_contains_adjacent_node{
                            node_queue.push_back(Rc::clone(&adjacent_node.0));
                            print!("+")
                        }
                        
                    }
                    
                //Kucoin Lps
                } 
                // else if current_node.borrow().is_kucoin_asset() && adjacent_node.0.borrow().is_kucoin_asset(){
                //     //Check if current node is USDT or other asset like RMRK. Are we doing RMRK -> USDT or USDT -> RMRK
                //     let path_value = calculate_kucoin_edge(&current_node, adjacent_node, current_node.borrow().best_path_value);
                //     if current_node.borrow().get_asset_symbol() != "UDST"{

                //     }
                // } 
                else {
                    let path_value;
                    //Check if kucoin edge
                    let mut is_kucoin_edge = false;
                    if current_node.borrow().is_kucoin_asset() && adjacent_node.0.borrow().is_kucoin_asset(){
                        path_value = calculate_kucoin_edge(&self, &current_node, adjacent_node, current_node.borrow().best_path_value);
                        is_kucoin_edge = true;
                    } else {
                        (path_value,_) = calculate_edge(&current_node, adjacent_node, current_node.borrow().best_path_value);
                    }
                    // let (path_value,_) = calculate_edge(&current_node, adjacent_node, current_node.borrow().best_path_value);
                    if path_value > adjacent_node.0.borrow().best_path_value{
                        let mut test= false;
                        for path_node in &current_node.borrow().best_path{
                            if path_node.borrow().get_asset_key() == adjacent_node.0.borrow().get_asset_key(){
                                test = true;
                            }
                        }
                        adjacent_node.0.borrow_mut().best_path_value = path_value;
                        adjacent_node.0.borrow_mut().best_path = current_node.borrow().best_path.clone();
                        adjacent_node.0.borrow_mut().best_path.push(Rc::clone(&adjacent_node.0));
                        adjacent_node.0.borrow_mut().path_values = current_node.borrow().path_values.clone();
                        let formatted_path_value = adjacent_node.0.borrow().best_path_value_display(&self).clone();
                        adjacent_node.0.borrow_mut().path_values.push(formatted_path_value);
                        
                        // println!("Step 1");
                        if !test{
                            // println!("step 2");
                            node_queue.push_back(Rc::clone(&adjacent_node.0));
                            print!("+")
                        }
                        
                    }
                }
            }
        }

        //Print finalized paths for each node
        for node_bucket in self.node_map.values(){
            for node in node_bucket{
                if !node.borrow().best_path.is_empty(){
                    // println!("Node: {} {}", node.borrow().asset_key, node.borrow().get_asset_name());
                    // print!("Path: ");
                    // for path_node in &node.borrow().best_path{
                    //     print!("{} {}-> ", path_node.borrow().get_asset_key(), path_node.borrow().best_path_value_display())
                    // }
                    node.borrow().display_path();
                    println!("");
                    println!("");
                }
            }
        }

        // let starting_node = &self.get_node(asset_key_1);
        println!("START NODE");
        // starting_node.borrow().display_path();
        // let start_node_location = starting_node.borrow().get_asset_location().unwrap();
        // let assets_at_location = &self.asset_registry.get_assets_from_location(start_node_location);
        // for asset in assets_at_location{
        //     let asset_key = asset.borrow().token_data.get_map_key();
        //     let asset_node = &self.get_node(asset_key);
        //     asset_node.borrow().display_path();
        //     println!("");
        //     println!("");
        // }

        starting_node.borrow().display_path();
    }

    pub fn find_best_path(&self, asset_key_1: String, asset_key_2: String, input_amount: f64){
        let starting_node = &self.get_node(asset_key_1);
        let token_1_decimals = starting_node.borrow().asset.borrow().token_data.get_asset_decimals() as u32;

        //Convert input amount to u128
        let converted_input_amount = (input_amount.clone() * f64::powi(10 as f64, token_1_decimals as i32)) as u128;
        let mut current_input_amount = converted_input_amount.clone();

        //Set input
        starting_node.borrow_mut().best_path_value = converted_input_amount;
        starting_node.borrow_mut().best_path.push(Rc::clone(starting_node));

        let mut node_queue: VecDeque<GraphNodePointer> = VecDeque::new();
        // let node_paths: Vec< = Vec::new();
        node_queue.push_front(Rc::clone(starting_node));
        while !node_queue.is_empty(){
            let current_node = node_queue.pop_front().unwrap();
            let current_node_display = current_node.borrow().best_path_value.to_f64().unwrap().clone() / f64::powi(10.0, current_node.borrow().get_asset_decimals() as i32);
            println!("Current node: {} - {} - {}", current_node.borrow().asset_key, current_node.borrow().get_asset_name(), current_node_display);
            // print!("Path: ");
            // for path_node in &current_node.borrow().best_path{
            //     print!("{} {} ->", path_node.borrow().get_asset_key(), path_node.borrow().best_path_value_display);
            // }
            println!("");
            // println!("");
            for adjacent_node in &current_node.borrow().adjacent_nodes{
                if current_node.borrow().asset_key != adjacent_node.0.borrow().asset_key{
                    // println!("Adjacent node: {}, {:?}", adjacent_node.0.borrow().asset_key, adjacent_node.1);

                    //Cross chain node. If current value is greater than cross chain's value, replace its path with new path
                    if current_node.borrow().get_asset_location() != None && current_node.borrow().get_asset_location() == adjacent_node.0.borrow().get_asset_location(){
                        if current_node.borrow().best_path_value > adjacent_node.0.borrow().best_path_value {
                            let mut new_path = Vec::new();
                            for node in &current_node.borrow().best_path{
                                new_path.push(Rc::clone(&node));
                            }
                            new_path.push(Rc::clone(&adjacent_node.0));
                            adjacent_node.0.borrow_mut().best_path_value = current_node.borrow().best_path_value;
                            
                            let adjacent_node_decimals = adjacent_node.0.borrow().asset.borrow().token_data.get_asset_decimals().clone();
                            adjacent_node.0.borrow_mut().best_path_value_display = current_node.borrow().best_path_value as f64/ f64::powi(10.0, adjacent_node_decimals as i32); 
                            adjacent_node.0.borrow_mut().best_path = new_path;
                            node_queue.push_back(Rc::clone(&adjacent_node.0));
                        }
                    } else{
                        
                        //Calculate swap value. If value is greater, replace that nodes path with new path. Add token to queue
                        //check if current pair exists in path nodes
                        //get liquidity for current node - adjacent node
                        let binding = current_node.borrow();
                        let existing_path_edge = binding.get_path_edge(current_node.borrow().get_asset_key(), adjacent_node.0.borrow().get_asset_key());
                        let (liq_current, liq_adjacent) = match existing_path_edge{
                            Some(((current, current_liq), (adjacent, adjacent_liq))) => (current_liq, adjacent_liq),
                            None => (&adjacent_node.1.0, &adjacent_node.1.1)
                        };

                        let adjacent_node_input = (Rc::clone(&adjacent_node.0), (liq_current.clone(), liq_adjacent.clone()));

                        let (edge_value, (new_current_node_liquidity, new_adjacent_node_liquidity)) = calculate_edge(&current_node, &adjacent_node_input, current_node.borrow().best_path_value);

                        //get new liquidity

                        //update liquidity in path nodes
                        if edge_value > adjacent_node.0.borrow().best_path_value{
                            
                            let mut new_path = Vec::new();
                            let mut path_edges = Vec::new();
                            for node in &current_node.borrow().best_path{
                                new_path.push(Rc::clone(&node));
                                
                            }
                            for edge in &current_node.borrow().path_edges{
                                path_edges.push(edge.clone())
                            }
                            new_path.push(Rc::clone(&adjacent_node.0));
                            path_edges.push(((current_node.borrow().asset_key.clone(), new_current_node_liquidity), (adjacent_node.0.borrow().asset_key.clone(), new_adjacent_node_liquidity)));

                            adjacent_node.0.borrow_mut().best_path_value = edge_value;
                            let adjacent_node_decimals = adjacent_node.0.borrow().asset.borrow().token_data.get_asset_decimals().clone();
                            adjacent_node.0.borrow_mut().best_path_value_display = edge_value as f64/ f64::powi(10.0, adjacent_node_decimals as i32); 
                            adjacent_node.0.borrow_mut().best_path = new_path;
                            adjacent_node.0.borrow_mut().path_edges = path_edges;
                            node_queue.push_back(Rc::clone(&adjacent_node.0));
                        }
                    }

                }
                
                
            }

        }
        
    }

    pub fn get_asset_decimals_for_kucoin_asset(&self, kucoin_node: &GraphNodePointer) -> u64 {
        let registry = &self.asset_registry;
        self.asset_registry.get_kucoin_asset_decimals(kucoin_node.borrow().get_asset_location().unwrap())
    }



}

//returns the output amount of adjacent node and resulting liquidity for each asset
pub fn calculate_edge(primary_node: &GraphNodePointer, adjacent_node: &(GraphNodePointer, (u128, u128)), input_amount: u128) -> (u128, (u128,u128)){
    let node_1 = primary_node;
    let (node_2, (node_1_liquidity, node_2_liquidity)) = adjacent_node;

    let node_1_liquidity = node_1_liquidity.to_bigint().unwrap();
    let node_2_liquidity = node_2_liquidity.to_bigint().unwrap();

    let token_1_decimals = node_1.borrow().get_asset_decimals() as u32;
    let token_2_decimals = node_2.borrow().get_asset_decimals() as u32;

    //Convert input amount to u128
    // let converted_input_amount = (input_amount.clone() * f64::powi(10 as f64, token_1_decimals as i32)) as u128;
    let converted_input_amount = input_amount;

    let increments = 5000;
    let token_1_increment:u128 = converted_input_amount / increments;
    let swap_fee = (token_1_increment.clone() as f64 * (0.003)) as u128;
    let token_1_increment_minus_swap = token_1_increment - (swap_fee);
    
    let mut token_1_changing_liquidity = node_1_liquidity.clone();
    let mut token_2_changing_liquidity = node_2_liquidity.clone();
    // / u128::pow(10, token_2_decimals)
    let mut total_slippage = 0.to_bigint().unwrap();
    let mut total_token_2_output = 0.to_bigint().unwrap();
    let mut i = 0;

    while i < increments{
        // let token_2_clone = token_2_changing_liquidity.clone();
        let token_2_out = token_2_changing_liquidity.clone() * token_1_increment_minus_swap / (token_1_changing_liquidity.clone() + token_1_increment_minus_swap);
        let slip = (&token_2_out/token_2_changing_liquidity.clone()) * &token_2_out;
        total_token_2_output += &token_2_out - &slip;
        token_2_changing_liquidity -= &token_2_out - &slip;
        token_1_changing_liquidity += token_1_increment_minus_swap;
        total_slippage += &slip;
        i += 1;
        // println!("{}", token_2_out);
    }

    // println!("Swap fee: {}", swap_fee);
    // println!("Token 1 decimals: {}", token_1_decimals);
    // println!("Input converted: {}", converted_input_amount);
    

    //Convert output to readable decimals
    // let total_token_2_converted = total_token_2_output.clone() / f64::powi(10.0, token_2_decimals as i32);
    let total_token_2_converted = BigRational::from(total_token_2_output.clone() / 10.to_bigint().unwrap().pow(token_2_decimals));
    let total_token_2_float = total_token_2_converted.to_f64();
    // let total_token_2_converted = 10.to_bigint().unwrap().pow(token_2_decimals);
    // println!("Total 2 converted: {}", total_token_2_converted);
    // total_token_2_converted
    
    let total_token_2_output_decimals = total_token_2_output.to_f64().unwrap().clone() / f64::powi(10.0, token_2_decimals as i32);
    // println!("Total out: {}", total_token_2_output_decimals);

    (total_token_2_output.to_u128().unwrap(),(token_1_changing_liquidity.to_u128().unwrap(),token_2_changing_liquidity.to_u128().unwrap()))
    
}

fn calculate_kucoin_edge(token_graph: &TokenGraph, primary_node: &GraphNodePointer, adjacent_node: &(GraphNodePointer, (u128, u128)), input_amount: u128) -> u128{
    
    let usdt_token_decimals = 4;
    //Asset -> USDT
    let (_, (bid, ask)) = adjacent_node;
    if primary_node.borrow().get_asset_symbol() != "USDT"{
        let asset_decimals = token_graph.get_asset_decimals_for_kucoin_asset(primary_node);
        let (bid_decimal, ask_decimal) = primary_node.borrow().asset.borrow().token_data.get_price_decimals();
        //Convert number to normal, display value
        let converted_input = input_amount as f64 / f64::powi(10.0, asset_decimals as i32);
        let converted_bid = bid.clone() as f64 / f64::powi(10.0, bid_decimal as i32);

        let asset_output = converted_input * converted_bid;
        let asset_output_converted = asset_output * f64::powi(10.0, usdt_token_decimals as i32) ;
        asset_output_converted as u128
    }
    //USDT -> Asset
    else {
        let asset_decimals = token_graph.get_asset_decimals_for_kucoin_asset(&adjacent_node.0);
        let (bid_decimal, ask_decimal) = primary_node.borrow().asset.borrow().token_data.get_price_decimals();
        //Convert number to normal, display value
        let converted_input = input_amount as f64 / f64::powi(10.0, usdt_token_decimals as i32);
        let converted_ask = bid.clone() as f64 / f64::powi(10.0, ask_decimal as i32);

        let asset_output = converted_input as f64 / converted_ask;
        let asset_output_converted = asset_output * f64::powi(10.0, asset_decimals as i32) ;
        // asset_output as u128
        asset_output_converted as u128
    }
}

    pub fn dfs_visit(node: GraphNodePointer, time: &mut u32){
        *time = *time + 1;
        node.borrow_mut().time_d = time.clone();
        node.borrow_mut().color = Color::Gray;
        println!("({}) {} {} -> Gray", time, node.borrow().get_asset_key(), node.borrow().get_asset_name());
        for adjacent_node in &node.borrow().adjacent_nodes{
            if adjacent_node.0.borrow().color == Color::White{
                adjacent_node.0.borrow_mut().pred = Some(Rc::clone(&node));
                dfs_visit(Rc::clone(&adjacent_node.0), time);
            }
        }
        node.borrow_mut().color = Color::Black;
        *time = *time + 1;
        node.borrow_mut().time_f = time.clone();
        println!("({}) {} {} -> Black", time, node.borrow().asset.borrow().token_data.get_map_key(), node.borrow().asset.borrow().token_data.get_asset_name());

    }

#[derive(Debug, PartialEq)]
pub struct GraphNode{
    pub asset: AssetPointer,
    pub adjacent_nodes: Vec<(GraphNodePointer, (u128, u128))>,
    pub asset_key: String,
    pub pred: Option<GraphNodePointer>,
    pub distance: u32,
    pub color: Color,
    pub time_d: u32,
    pub time_f: u32,
    pub best_path_value: u128,
    pub best_path_value_display: f64,
    pub path_edges: Vec<((String,u128),(String, u128))>,
    pub best_path: Vec<GraphNodePointer>,
    pub path_values: Vec<f64>,
}

impl GraphNode{
    pub fn get_adjacent_node(&self, asset_key: String) -> &(GraphNodePointer, (u128,u128)){
        for node in &self.adjacent_nodes{
            let adj_node = node.0.borrow();
            if adj_node.asset_key == asset_key{
                return node;
            }
        }
        panic!("Could not get adjacent node with key: {}", asset_key);
    }

    pub fn get_asset_name(&self) -> String{
        self.asset.borrow().token_data.get_asset_name()
    }

    pub fn get_asset_key(&self) -> String{
        self.asset.borrow().token_data.get_map_key()
    }

    pub fn get_asset_decimals(&self) -> u64{
        self.asset.borrow().token_data.get_asset_decimals()
    }

    pub fn get_asset_location(&self) -> Option<AssetLocation>{
        self.asset.borrow().asset_location.clone()
    }

    pub fn get_asset_symbol(&self) -> String{
        self.asset.borrow().token_data.get_symbol()
    }

    //Get the lastest path edge
    pub fn get_path_edge(&self, asset_key_1: String, asset_key_2: String) -> Option<(((&String, &u128), (&String, &u128)))>{
        // let mut v = Vec::new();
        for edge in self.path_edges.iter().rev(){
            // v.push(1);
            let ((node_1, liquidity_1), (node_2, liquidity_2)) = edge;
            if &asset_key_1 == node_1 && &asset_key_2 == node_2{
                println!("found liq 1: {} {} {} {}", node_1, liquidity_1, node_2, liquidity_2);
                return Some(((node_1, liquidity_1), (node_2, liquidity_2)))
            } else if &asset_key_1 == node_2 && &asset_key_2 == node_1{
                println!("found liq 2: {} {} {} {}", node_1, liquidity_1, node_2, liquidity_2);
                return Some(((node_2, liquidity_2), (node_1, liquidity_1)))
            }

        }
        // for x in 
        // for x in v.iter().re
        return None
    }

    pub fn is_kucoin_asset(&self) -> bool {
        self.asset.borrow().token_data.is_exchange_token()
    }

    pub fn best_path_value_display(&self, token_graph: &TokenGraph) -> f64 {
        match self.asset.borrow().token_data{
            TokenData::KucoinToken { .. } => {
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
        // self.best_path_value.to_f64().unwrap().clone() / f64::powi(10.0, self.get_asset_decimals() as i32)
    }

    pub fn path_contains(&self, node: GraphNodePointer) -> bool {
        for path_node in &self.best_path{
            if path_node.borrow().get_asset_key() == node.borrow().get_asset_key(){
                return true
            }
        }
        return false
    }

    pub fn display_path(&self){
        println!("Node: {} {} {}", &self.get_asset_key(), &self.get_asset_name(), &self.get_asset_decimals());
        print!("Path: ");
        for (i, path_node) in self.best_path.iter().enumerate(){
            // let display_value = &self.path_values[i].to_f64().unwrap().clone() 
            print!("{} {} {} ->", path_node.borrow().get_asset_key(), path_node.borrow().get_asset_name(), &self.path_values[i]);
        }
    }

    // pub fn add_path_edge

}

// impl GraphNodePointer{

// }

struct NodePath{
    pub nodes: Vec<GraphNodePointer>,
}

#[derive(PartialEq, Debug)]
pub enum Color{
    White,
    Gray,
    Black
}

// pub fn node_hash_map()

pub fn calculate_swap(token_graph: &TokenGraph, asset_1_key: String, asset_2_key: String, input_amount: f64) -> f64{
    let binding = token_graph.get_node(asset_1_key);
    let node_1 = binding.borrow();
    for adjacent_node in &node_1.adjacent_nodes{
        println!("RMRK - {}", adjacent_node.0.borrow().asset_key);
        println!("{:?}", adjacent_node.1)
    }
    // let node_2 = token_graph.get_node(asset_2_key);
    // token_graph.get_swap_pair(asset_key_1, asset_key_2)
    let (node_2, (node_1_liquidity, node_2_liquidity)) = node_1.get_adjacent_node(asset_2_key);

    

    let token_1_decimals = node_1.asset.borrow().token_data.get_asset_decimals() as u32;
    let token_2_decimals = node_2.borrow().asset.borrow().token_data.get_asset_decimals() as u32;

    //Convert input amount to u128
    let converted_input_amount = (input_amount.clone() * f64::powi(10 as f64, token_1_decimals as i32)) as u128;

    let increments = 5000;
    let token_1_increment:u128 = (converted_input_amount / increments);
    let swap_fee = (token_1_increment.clone() as f64 * (0.003)) as u128;
    let token_1_increment_minus_swap = token_1_increment - (swap_fee);
    
    let mut token_1_changing_liquidity = node_1_liquidity.clone();
    let mut token_2_changing_liquidity = node_2_liquidity.clone();
    // / u128::pow(10, token_2_decimals)
    let mut total_slippage = 0;
    let mut total_token_2_output = 0;
    let mut i = 0;

    println!("Token 1: {} - Token 2: {}", node_1.asset_key, node_2.borrow().asset_key);
    println!("{} - {}", node_1_liquidity, node_2_liquidity);
    println!("{} - {}", token_1_changing_liquidity, token_2_changing_liquidity);
    while i < increments{
        let token_2_out = token_2_changing_liquidity * token_1_increment_minus_swap / (token_1_changing_liquidity + token_1_increment_minus_swap);
        let slip = (token_2_out/token_2_changing_liquidity) * token_2_out;
        total_token_2_output += token_2_out - slip;
        token_2_changing_liquidity -= (token_2_out - slip) as u128;
        token_1_changing_liquidity += token_1_increment_minus_swap as u128;
        total_slippage += slip;
        i += 1;
        // println!("{}", token_2_out);
    }

    println!("Swap fee: {}", swap_fee);
    println!("Token 1 decimals: {}", token_1_decimals);
    println!("Input converted: {}", converted_input_amount);
    println!("Total out: {}", total_token_2_output);

    //Convert output to readable decimals
    let total_token_2_converted = total_token_2_output.clone() as f64 / f64::powi(10.0, token_2_decimals as i32);
    println!("Total 2 converted: {}", total_token_2_converted);
    total_token_2_converted
    
}

// function rmrk_to_kusd(kusdL, rmrkL, rmrkSupply) {
//     // const rmrkSupply = 100;
//     const increments = rmrkSupply / 5000;
//     let i = 0;
//     let kusdChangingLiq = kusdL;
//     let rmrkChangingLiq = rmrkL;
//     let totalSlip = 0;
//     let totalKusd = 0;
//     const rmrkInput = increments - (increments * 0.003)
//     while (i < (rmrkSupply / increments)) {
//         let kusdOut = kusdChangingLiq * rmrkInput / (rmrkChangingLiq + rmrkInput);
//         let slip = (kusdOut / kusdChangingLiq) * kusdOut;
//         totalKusd += kusdOut - slip;
//         kusdChangingLiq -= kusdOut - slip;
//         rmrkChangingLiq += rmrkInput;
//         totalSlip += slip;

//         i++;
//     }
//     console.log("out kusd: ", totalKusd)
//     console.log("slip: ", totalSlip)
//     // console.log("total: ", total);
//     return totalKusd;
// }