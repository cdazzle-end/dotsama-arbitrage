// use crate::asset_registry::AssetLocation;
// use crate::token::{self, TokenData};
// use crate::{LiqPoolRegistry, asset_registry, liq_pool_registry};
use crate::liq_pool_registry_2::LiqPoolRegistry2;
use num::{BigInt, ToPrimitive, FromPrimitive, CheckedAdd, BigUint, CheckedMul, CheckedDiv, CheckedSub};
use num::bigint::{ToBigInt, ToBigUint};
use num::BigRational;
use num;
// use num::BigRational::
use std::cell::RefCell;
use std::collections::{VecDeque, HashMap};
use std::rc::Rc;
use std::vec;
// use crate::{asset_registry::{AssetRegistry, Asset}};
use crate::AssetRegistry2;
use crate::asset_registry_2::{Asset, AssetLocation, TokenData};
use crate::adjacency_table_2::{AdjacencyTable2, AdjacencyGroup, Liquidity, GroupType, DexLp, StableLp, CexLp};
type AssetPointer = Rc<RefCell<Asset>>;
type GraphNodePointer = Rc<RefCell<GraphNode>>;

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
        println!("asset key: {}", asset_key);
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

    pub fn display_graph_2(&self){
        for (key, buckets) in &self.node_map{
            for node in buckets{
                // print!("{}: ", node.borrow().asset_key);
                node.borrow().asset.borrow().display_asset();
                print!(" -> ");
                for adj_node in &node.borrow().adjacent_pairs{
                    adj_node.adjacent_node.borrow().asset.borrow().display_asset();
                    print!(" | ");
                }
                println!("");
            }

            println!("------------------------------------")
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
    pub fn find_arbitrage_2(&self, asset_key_1: String, input_amount: f64) -> String{
        let starting_node = &self.get_node(asset_key_1).clone();
        
        let formatted_input = &input_amount * f64::powi(10.0, starting_node.borrow().get_asset_decimals() as i32);
        starting_node.borrow_mut().best_path_value = formatted_input as u128;
        starting_node.borrow_mut().path_values.push(input_amount);
        starting_node.borrow_mut().best_path.push(Rc::clone(&starting_node));

        let mut node_queue = VecDeque::new();
        node_queue.push_back(Rc::clone(starting_node));
        println!("***");
        while !node_queue.is_empty(){
            println!("queue length: {}", node_queue.len());
            let current_node = node_queue.pop_front().unwrap();
            for adjacent_pair in &current_node.borrow().adjacent_pairs{
                //Check if adjacent node shares same location with current node
                if current_node.borrow().get_asset_location() != None && current_node.borrow().get_asset_location() == adjacent_pair.adjacent_node.borrow().get_asset_location(){
                    if current_node.borrow().best_path_value > adjacent_pair.adjacent_node.borrow().best_path_value{
                        
                        //Check if adjacent node already exists within path
                        let mut current_path_contains_adjacent_node= false;
                        for path_node in &current_node.borrow().best_path{
                            if path_node.borrow().get_asset_key() == adjacent_pair.adjacent_node.borrow().get_asset_key(){
                                current_path_contains_adjacent_node = true;
                            }
                        }
                        // println!("Contains adj node already: {}", current_path_contains_adjacent_node);
                        adjacent_pair.adjacent_node.borrow_mut().best_path_value = current_node.borrow().best_path_value;
                        adjacent_pair.adjacent_node.borrow_mut().best_path = current_node.borrow().best_path.clone();
                        adjacent_pair.adjacent_node.borrow_mut().best_path.push(Rc::clone(&adjacent_pair.adjacent_node));
                        adjacent_pair.adjacent_node.borrow_mut().path_values = current_node.borrow().path_values.clone();
                        adjacent_pair.adjacent_node.borrow_mut().path_values.push(current_node.borrow().best_path_value_display(&self));
                        if !current_path_contains_adjacent_node{
                            node_queue.push_back(Rc::clone(&adjacent_pair.adjacent_node));
                            // print!("+")
                        }
                        
                    }
                    
                } 
                else {
                    let path_value;
                    if current_node.borrow().is_cex_token() && adjacent_pair.adjacent_node.borrow().is_cex_token(){
                        path_value = calculate_kucoin_edge_2(&self, &current_node, adjacent_pair, current_node.borrow().best_path_value);
                    } else {
                        path_value = calculate_edge_2(&current_node, adjacent_pair, current_node.borrow().best_path_value);
                    }
                    if path_value > adjacent_pair.adjacent_node.borrow().best_path_value{
                        let mut test= false;
                        for path_node in &current_node.borrow().best_path{
                            if path_node.borrow().get_asset_key() == adjacent_pair.adjacent_node.borrow().get_asset_key(){
                                test = true;
                            }
                        }
                        adjacent_pair.adjacent_node.borrow_mut().best_path_value = path_value;
                        adjacent_pair.adjacent_node.borrow_mut().best_path = current_node.borrow().best_path.clone();
                        adjacent_pair.adjacent_node.borrow_mut().best_path.push(Rc::clone(&adjacent_pair.adjacent_node));
                        adjacent_pair.adjacent_node.borrow_mut().path_values = current_node.borrow().path_values.clone();
                        let formatted_path_value = adjacent_pair.adjacent_node.borrow().best_path_value_display(&self).clone();
                        adjacent_pair.adjacent_node.borrow_mut().path_values.push(formatted_path_value);
                        
                        if !test{
                            node_queue.push_back(Rc::clone(&adjacent_pair.adjacent_node));
                        }
                        
                    }
                }
            }
        }

        //Print finalized paths for each node
        for node_bucket in self.node_map.values(){
            for node in node_bucket{
                if !node.borrow().best_path.is_empty(){
                    node.borrow().display_path();
                    println!();
                    println!("---------------------------------------------------------------------------");
                }
            }
        }

        println!("START NODE");
        starting_node.borrow().display_path();
        let return_string = starting_node.borrow().get_display_path().clone();
        // starting_node.borrow().get_display_path().clone()
        return_string
    }

    pub fn find_arbitrage_3(&self, asset_key_1: String, input_amount: f64) -> (String, Vec<Rc<RefCell<GraphNode>>>) {
        let starting_node = &self.get_node(asset_key_1).clone();
        
        let formatted_input = &input_amount * f64::powi(10.0, starting_node.borrow().get_asset_decimals() as i32);
        starting_node.borrow_mut().best_path_value = formatted_input as u128;
        starting_node.borrow_mut().path_values.push(input_amount);
        starting_node.borrow_mut().path_value_types.push(0);
        starting_node.borrow_mut().best_path.push(Rc::clone(&starting_node));

        
        let mut node_queue = VecDeque::new();
        node_queue.push_back(Rc::clone(starting_node));
        println!("***");

        while !node_queue.is_empty() {
            println!("queue length: {}", node_queue.len());
            let current_node = node_queue.pop_front().unwrap();
            for adjacent_pair in &current_node.borrow().adjacent_pairs2{
                // let path_value;
                match adjacent_pair {
                    AdjacentNodePair2::XcmPair(adjacent_pair) => {
                        if current_node.borrow().best_path_value > adjacent_pair.adjacent_node.borrow().best_path_value{
                            let mut current_path_contains_adjacent_node= false;
                            for path_node in &current_node.borrow().best_path{
                                if path_node.borrow().get_asset_key() == adjacent_pair.adjacent_node.borrow().get_asset_key(){
                                    current_path_contains_adjacent_node = true;
                                }
                            }
                            adjacent_pair.adjacent_node.borrow_mut().best_path_value = current_node.borrow().best_path_value;
                            adjacent_pair.adjacent_node.borrow_mut().best_path = current_node.borrow().best_path.clone();
                            adjacent_pair.adjacent_node.borrow_mut().best_path.push(Rc::clone(&adjacent_pair.adjacent_node));
                            adjacent_pair.adjacent_node.borrow_mut().path_values = current_node.borrow().path_values.clone();
                            adjacent_pair.adjacent_node.borrow_mut().path_values.push(current_node.borrow().best_path_value_display(&self));
                            adjacent_pair.adjacent_node.borrow_mut().path_value_types = current_node.borrow().path_value_types.clone();
                            adjacent_pair.adjacent_node.borrow_mut().path_value_types.push(0);
                            if !current_path_contains_adjacent_node{
                                node_queue.push_back(Rc::clone(&adjacent_pair.adjacent_node));
                            }
                            
                        }
                    },
                    AdjacentNodePair2::DexPair(dex_pair) =>  {
                        let path_value = calculate_dex_edge( adjacent_pair, current_node.borrow().best_path_value);
                        if path_value > dex_pair.adjacent_node.borrow().best_path_value{
                            let mut test= false;
                            for path_node in &current_node.borrow().best_path{
                                if path_node.borrow().get_asset_key() == dex_pair.adjacent_node.borrow().get_asset_key(){
                                    test = true;
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
                            
                            if !test{
                                node_queue.push_back(Rc::clone(&dex_pair.adjacent_node));
                            }
                            
                        }
                    },
                    AdjacentNodePair2::CexPair(cex_pair) => {
                        let path_value = calculate_cex_edge( &self, &current_node, adjacent_pair, current_node.borrow().best_path_value);
                        if path_value > cex_pair.adjacent_node.borrow().best_path_value{
                            let mut test= false;
                            for path_node in &current_node.borrow().best_path{
                                if path_node.borrow().get_asset_key() == cex_pair.adjacent_node.borrow().get_asset_key(){
                                    test = true;
                                }
                            }
                            cex_pair.adjacent_node.borrow_mut().best_path_value = path_value;
                            cex_pair.adjacent_node.borrow_mut().best_path = current_node.borrow().best_path.clone();
                            cex_pair.adjacent_node.borrow_mut().best_path.push(Rc::clone(&cex_pair.adjacent_node));
                            cex_pair.adjacent_node.borrow_mut().path_values = current_node.borrow().path_values.clone();
                            let formatted_path_value = cex_pair.adjacent_node.borrow().best_path_value_display(&self).clone();
                            cex_pair.adjacent_node.borrow_mut().path_values.push(formatted_path_value);
                            cex_pair.adjacent_node.borrow_mut().path_value_types = current_node.borrow().path_value_types.clone();
                            cex_pair.adjacent_node.borrow_mut().path_value_types.push(3);
                            
                            if !test{
                                node_queue.push_back(Rc::clone(&cex_pair.adjacent_node));
                            }
                            
                        }
                    },
                    AdjacentNodePair2::StablePair(stable_pair) => {
                        // for (i, adj_node) in stable_pair.adjacent_nodes.iter().enumerate(){
                        //     let path_value = calculate_stable_edge( &self, &current_node, &adjacent_pair, current_node.borrow().best_path_value, i);
                        // }
                        for i in 0..stable_pair.adjacent_nodes.len(){
                            let path_value = calculate_stable_edge( &self, &current_node, &adjacent_pair, current_node.borrow().best_path_value, i);
                            if path_value > stable_pair.adjacent_nodes[i].borrow().best_path_value{
                                let mut test= false;
                                for path_node in &current_node.borrow().best_path{
                                    if path_node.borrow().get_asset_key() == stable_pair.adjacent_nodes[i].borrow().get_asset_key(){
                                        test = true;
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
                                if !test{
                                    node_queue.push_back(Rc::clone(&stable_pair.adjacent_nodes[i]));
                                }
                                
                            }
                        }
                    }
                }
            }
        }

        println!("START NODE");
        starting_node.borrow().display_path();
        let return_string = starting_node.borrow().get_display_path().clone();
        let best_path = starting_node.borrow().best_path.clone();
        // starting_node.borrow().get_display_path().clone()
        let path_and_display: (String, Vec<Rc<RefCell<GraphNode>>>) = (return_string, best_path);
        path_and_display
    }

    pub fn find_best_route(&self, asset_key_1: String, asset_key_2: String, input_amount: f64) -> (String, Vec<Rc<RefCell<GraphNode>>>) {

        let starting_node = &self.get_node(asset_key_1).clone();
        let formatted_input = &input_amount * f64::powi(10.0, starting_node.borrow().get_asset_decimals() as i32);
        starting_node.borrow_mut().best_path_value = formatted_input as u128;
        starting_node.borrow_mut().path_values.push(input_amount);
        starting_node.borrow_mut().path_value_types.push(0);
        starting_node.borrow_mut().best_path.push(Rc::clone(&starting_node));
        // starting_node.borrow_mut()
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
        
        // let formatted_input = &input_amount * f64::powi(10.0, starting_node.borrow().get_asset_decimals() as i32);

        let mut node_queue = VecDeque::new();
        node_queue.push_back(Rc::clone(starting_node));

        while !node_queue.is_empty() {
            println!("queue length: {}", node_queue.len());
            let current_node = node_queue.pop_front().unwrap();
            for adjacent_pair in &current_node.borrow().adjacent_pairs2{
                // let path_value;
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
                            
                            adjacent_pair.adjacent_node.borrow_mut().best_path_value = current_node.borrow().best_path_value;
                            adjacent_pair.adjacent_node.borrow_mut().best_path = current_node.borrow().best_path.clone();
                            adjacent_pair.adjacent_node.borrow_mut().best_path.push(Rc::clone(&adjacent_pair.adjacent_node));
                            adjacent_pair.adjacent_node.borrow_mut().path_values = current_node.borrow().path_values.clone();
                            adjacent_pair.adjacent_node.borrow_mut().path_values.push(current_node.borrow().best_path_value_display(&self));
                            adjacent_pair.adjacent_node.borrow_mut().path_value_types = current_node.borrow().path_value_types.clone();
                            adjacent_pair.adjacent_node.borrow_mut().path_value_types.push(0);
                            if !current_path_contains_adjacent_node && !is_destination_node{
                                node_queue.push_back(Rc::clone(&adjacent_pair.adjacent_node));
                            }
                            
                        }
                    },
                    AdjacentNodePair2::DexPair(dex_pair) =>  {
                        let path_value = calculate_dex_edge( adjacent_pair, current_node.borrow().best_path_value);
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
                            
                            if !test && !is_destination_node{
                                node_queue.push_back(Rc::clone(&dex_pair.adjacent_node));
                            }
                            
                        }
                    },
                    AdjacentNodePair2::CexPair(cex_pair) => {
                        let path_value = calculate_cex_edge( &self, &current_node, adjacent_pair, current_node.borrow().best_path_value);
                        if path_value > cex_pair.adjacent_node.borrow().best_path_value{
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
                            cex_pair.adjacent_node.borrow_mut().best_path_value = path_value;
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
                            let path_value = calculate_stable_edge( &self, &current_node, &adjacent_pair, current_node.borrow().best_path_value, i);
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
        let mut highest_value: Option<u128> = None;
        let mut highest_value_node: Option<Rc<RefCell<GraphNode>>> = None;

        for possible_node in possible_destination_nodes{
            // println!("POSSIBLE NODE PATHS");
            possible_node.borrow().display_path();

            // println!("GETTING HIGHEST VALUE PATH");
            let best_path_value = possible_node.borrow().best_path_value;
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
            adjacent_pairs: Vec::new(),
            adjacent_pairs2: Vec::new(),
            asset_key: asset.borrow().get_map_key(),
            pred: None,
            best_path_value: 0,
            best_path_value_display: 0.0,
            path_edges: Vec::new(),
            best_path: Vec::new(),
            path_values: Vec::new(),
            path_value_types: Vec::new(),
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
                            println!("Found stable pair")

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
                        let adjacent_node = AdjacentNodePair::new(&potential_adjacent_node, adj_group.liquidity.clone(), 1);
                        current_node.borrow_mut().adjacent_pairs.push(adjacent_node);

                        let dex_lp: DexLp = if let Liquidity::Dex(x) = adj_group.liquidity.clone().unwrap(){
                            x
                        } else {
                            panic!("Dex liquidity should be DexLp")
                        };
                        let adjacent_node_2 = AdjacentNodePair2::DexPair(DexPair{adjacent_node: Rc::clone(&potential_adjacent_node), liquidity: adj_group.liquidity.clone().unwrap()});
                        current_node.borrow_mut().adjacent_pairs2.push(adjacent_node_2);
                        println!("found DEx")
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
    // println!("Add cross chain assets");
    let current_node_location = current_node.borrow().get_asset_location();
    // for x in &current_node_location{
    //     println!("{:?}", x);
    // }
    if let Some(asset_location) = current_node_location{
        for cross_chain_asset in asset_registry.get_assets_at_location(asset_location){
            let bucket = node_map.get(&cross_chain_asset.borrow().get_map_key()).unwrap();
            for graph_node in bucket{
                if cross_chain_asset.borrow().get_map_key() == graph_node.borrow().asset.borrow().get_map_key(){
                    // current_node.borrow_mut().adjacent_pairs.push((Rc::clone(graph_node), ((0,0),(0,0))));
                    let adjacent_node = AdjacentNodePair::new(&graph_node, None, 0);
                    current_node.borrow_mut().adjacent_pairs.push(adjacent_node);

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
    pub adjacent_pairs: Vec<AdjacentNodePair>,
    pub adjacent_pairs2: Vec<AdjacentNodePair2>,
    pub asset_key: String,
    pub pred: Option<GraphNodePointer>,
    pub best_path_value: u128,
    pub best_path_value_display: f64,
    pub path_edges: Vec<((String,u128),(String, u128))>,
    pub best_path: Vec<GraphNodePointer>,
    pub path_values: Vec<f64>,
    pub path_value_types: Vec<u64>,
}
#[derive(Debug, PartialEq)]
pub struct AdjacentNodePair{
    pub adjacent_node: GraphNodePointer,
    pub liquidity: Option<Liquidity>,
    pub pair_type: u64,
}
#[derive(Debug, PartialEq)]
pub enum AdjacentNodePair2{
    DexPair(DexPair),
    CexPair(CexPair),
    StablePair(StablePair),
    XcmPair(XcmPair),
}
#[derive(Debug, PartialEq)]
pub struct DexPair{
    pub adjacent_node: GraphNodePointer,
    pub liquidity: Liquidity,
}
#[derive(Debug, PartialEq)]
pub struct CexPair{
    pub adjacent_node: GraphNodePointer,
    pub liquidity: Liquidity,
}
#[derive(Debug, PartialEq)]
pub struct StablePair{
    pub adjacent_nodes: Vec<GraphNodePointer>,
    pub liquidity: Liquidity,
}
#[derive(Debug, PartialEq)]
pub struct XcmPair{
    pub adjacent_node: GraphNodePointer,
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
    // pub fn get_adjacent_node(&self, asset_key: String) -> &(GraphNodePointer, ((u128,u128),(u128,u128))){
    //     for node in &self.adjacent_nodes{
    //         let adj_node = node.0.borrow();
    //         if adj_node.asset_key == asset_key{
    //             return node;
    //         }
    //     }
    //     panic!("Could not get adjacent node with key: {}", asset_key);
    // }

    pub fn get_asset_name(&self) -> String{
        self.asset.borrow().get_asset_name().clone().to_string()
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
        self.asset.borrow().get_asset_symbol().clone().to_string()
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

pub fn calculate_edge_3(token_graph: &TokenGraph2, primary_node: &GraphNodePointer, adjacent_nodes: &AdjacentNodePair2, input: u128) -> u128{
    match adjacent_nodes {
        AdjacentNodePair2::DexPair(adj_pair) => {
            let (base_liquidity, adjacent_liquidity) = match adj_pair.liquidity {
                Liquidity::Dex(dexLp) => (dexLp.base_liquidity, dexLp.adjacent_liquidity),
                _ => panic!("Tried to get dex liquidity from non-dex liquidity"),
            };
            // calculate_dex_edge(adjacent_nodes, base_liquidity, adjacent_liquidity, input)
        0
            
        },
        AdjacentNodePair2::StablePair(adj_pair) => {
            0
        },
        AdjacentNodePair2::CexPair(adj_pair) => {
            calculate_cex_edge(token_graph, primary_node, adjacent_nodes, input)
        },
        _ => 0
    }
}

pub fn calculate_dex_edge(adjacent_node: &AdjacentNodePair2, input_amount: u128) -> u128{
    match adjacent_node{
        AdjacentNodePair2::DexPair(adj_pair) => {
            let (base_liquidity, adjacent_liquidity) = match adj_pair.liquidity {
                Liquidity::Dex(dexLp) => (dexLp.base_liquidity, dexLp.adjacent_liquidity),
                _ => panic!("Tried to get dex liquidity from non-dex liquidity"),
            };
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
        },
        _ => panic!("Tried to get dex liquidity from non-dex liquidity"),
    }
    
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

pub fn calculate_stable_edge(token_graph: &TokenGraph2, primary_node: &GraphNodePointer, adjacent_pair: &AdjacentNodePair2, input_amount: u128, target_index: usize) -> u128{

    match adjacent_pair{
        AdjacentNodePair2::StablePair(adj_pair) => {
            // println!(" BALANCES 3{:?}", adj_pair.liquidity);
            let (base_liquidity, base_token_precision, adjacent_liquidity, a, adjacent_token_precisions) = match &adj_pair.liquidity {
                Liquidity::Stable(stableLp) => (stableLp.base_liquidity, stableLp.base_token_precision, stableLp.adjacent_liquidity.clone(), stableLp.a, stableLp.adjacent_token_precisions.clone()),
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
            let input_formatted = input_amount as f64 / f64::powi(10.0, total_node_decimals as i32);
            println!("{} {} -> {}", primary_node.borrow().asset_key, input_formatted, adj_pair.adjacent_nodes[target_index].borrow().asset_key);
            // println!("Input unformatted: {}", input_amount);
            let swap_fee: BigInt;
            if a == 10000{
                swap_fee = BigInt::from(5000000)
            } else {
                swap_fee = BigInt::from(25000000)
            }
            let fee_precision =  "10000000000".parse::<BigInt>().unwrap();
            let fee_ration = BigRational::from(swap_fee.clone()) / BigRational::from(fee_precision.clone());

            let D = getD(&balances, a).unwrap();
            let token_in_index = balances.len() - 1;
            balances[token_in_index] = balances[token_in_index].clone() + input_amount;
            let y = getY(&balances, target_index as u128, D, a, primary_node_symbol.clone()).unwrap();

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
            total_output_minus_precision.to_u128().unwrap()
            
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
    println!("Start D: {}", d);
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

pub fn getY(balances: &Vec<BigInt>, target_index: u128, D: u128, a: u128, primary_node: String) -> Option<u128>{
    let one: BigInt = BigInt::from(1u128);
    let two: BigInt = BigInt::from(2u128);
    let mut c = BigInt::from(D);
    let mut sum = BigInt::from(0u128);
    let mut ann = BigInt::from(a);
    let mut balance_size = BigInt::from(balances.len());
    let target_d = BigInt::from(D);
    let a_precision = BigInt::from(1 as usize);
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
			ann = ann.checked_mul(&balance_size)?;
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