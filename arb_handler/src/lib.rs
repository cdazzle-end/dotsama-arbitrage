
use bigdecimal::BigDecimal;
// mod asset_registry;
// mod liq_pool_registry;
// mod adjacency_table;
// mod token_graph;
// mod token;
// use token::{Token, AssetKeyType};
// use adjacency_table::{AdjacencyTable};
// use asset_registry::AssetRegistry;
// use liq_pool_registry::LiqPoolRegistry;
// use token_graph::TokenGraph;
// use token_graph::calculate_swap;
use futures::future::join_all;
mod fee_book;
mod asset_registry_2;
mod liq_pool_registry_2;
mod adjacency_table_2;
mod token_graph_2;
mod result_logger;
use adjacency_table_2::{AdjacencyTable2};
use asset_registry_2::AssetRegistry2;
use liq_pool_registry_2::LiqPoolRegistry2;
use num::BigInt;
use num::FromPrimitive;
use token_graph_2::get_sqrt_ratio_at_tick;
use token_graph_2::PathData;
use token_graph_2::TokenGraph2;
use token_graph_2::GraphNode;
use result_logger::ResultLogger;

use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use serde_json::{Value};
use serde::{Deserialize, Serialize};
use std::str;
use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;
use tokio::{join, task};
use std::fs::OpenOptions;

use crate::asset_registry_2::Asset;
use crate::token_graph_2::PathNode;
// use std::io::prelude::*;
type NodePath = Vec<Rc<RefCell<GraphNode>>>;



// pub fn build_sub_assets(){
//     AssetRegistry::build_sub_asset_registry_from_file();
// }

// pub async fn async_search(){
//     // let future1 = task::spawn(search_rmrk());
//     let future2 = task::spawn(search_ksm());
//     let future3 =  task::spawn(search_ksm_small());
//     let (result2, result3) = join!(future2, future3);
//     // future1
//     let (ksm_display, ksm_log) = result2.unwrap();
//     println!();

// }

pub async fn async_search_default_kusama(){
    let start_key = "2000{\"NativeAssetId\":{\"Token\":\"KSM\"}}".to_string();
    let destination_key = "2000{\"NativeAssetId\":{\"Token\":\"KSM\"}}".to_string();
    let big_amount = BigDecimal::from(1);
    let mut asset_registry = AssetRegistry2::build_asset_registry();
    let lp_registry = LiqPoolRegistry2::build_liqpool_registry(&mut asset_registry);
    lp_registry.display_stable_pools();
    let list = AdjacencyTable2::build_table_2(&lp_registry);
    let graph = TokenGraph2::build_graph_2(asset_registry, list);

    let start_node = &graph.get_node(start_key).clone();
    
    let start_node_asset_name = start_node.borrow().get_asset_name();

    let start_asset_location = start_node.borrow().get_asset_location().unwrap();
    let all_start_assets = &graph.asset_registry.get_assets_at_location(start_asset_location);
    let mut start_nodes = vec![];
    for start_asset in all_start_assets{
        if !start_asset.borrow().is_cex_token() {
            let new_start_node = &graph.get_node(start_asset.borrow().get_map_key()).clone();
            start_nodes.push(Rc::clone(&new_start_node));
        }
    }

    let mut big_handles = Vec::new();
    let small_input = BigDecimal::from_f64(0.1).unwrap();
    let mut small_handles = Vec::new();
    let medium_input = BigDecimal::from_f64(0.5).unwrap();
    let mut medium_handles = Vec::new();

    for node in start_nodes.clone(){
        let key = node.borrow().get_asset_key();
        println!("Searching for {}", key);
        let dest_key = destination_key.clone();
        let big_amount_clone = big_amount.clone();
        // let future = task::spawn(search_best_path_a_to_b_async(key, destination_key.clone(), input_amount));
        let handle = task::spawn(async move {
            search_best_path_a_to_b_async("kusama".to_string(), key, dest_key, big_amount_clone).await
        });
        big_handles.push(handle);
    }

    for node in start_nodes.clone(){
        let key = node.borrow().get_asset_key();
        println!("Searching for {}", key);
        let dest_key = destination_key.clone();
        let small_amount_clone = small_input.clone();
        // let future = task::spawn(search_best_path_a_to_b_async(key, destination_key.clone(), input_amount));
        let handle = task::spawn(async move {
            search_best_path_a_to_b_async("kusama".to_string(), key, dest_key, small_amount_clone).await
        });
        small_handles.push(handle);
    }

    for node in start_nodes{
        let key = node.borrow().get_asset_key();
        println!("Searching for {}", key);
        let dest_key = destination_key.clone();

        let medium_amount = medium_input.clone();
        // let future = task::spawn(search_best_path_a_to_b_async(key, destination_key.clone(), input_amount));
        let handle = task::spawn(async move {
            search_best_path_a_to_b_async("kusama".to_string(), key, dest_key, medium_amount).await
        });
        medium_handles.push(handle);
    }


   
    let mut merged_handles = Vec::new();
    merged_handles.extend(big_handles);
    merged_handles.extend(small_handles);
    merged_handles.extend(medium_handles);

    // Now, use join_all to await all the tasks at once
    let results = join_all(merged_handles).await;
    let mut big_amount_path_nodes: Vec<(String, Vec<PathNode>)> = vec![];
    let mut small_amount_path_nodes: Vec<(String, Vec<PathNode>)> = vec![];
    let mut medium_amount_path_nodes: Vec<(String, Vec<PathNode>)> = vec![];

    for result in results {
        match result {
            Ok(ok) => {
                // Depending on your logic, you might need a way to distinguish between results of `input_handles` and `small_handles`
                // For example, you could use a tuple or struct to include metadata with each task to identify its type
                // println!("Task completed with result: {:?}", ok);
                let (path_amount, display_string, path) = ok;
                if path_amount == big_amount {
                    big_amount_path_nodes.push((display_string, path));
                } else if path_amount == small_input {
                    small_amount_path_nodes.push((display_string, path));
                } else if path_amount == medium_input {
                    medium_amount_path_nodes.push((display_string, path));
                } else {
                    println!("Unknown path amount: {}", path_amount);
                }
            },
            Err(e) => println!("Task failed with error: {:?}", e),
        }
    }

    let mut highest_big_value: BigDecimal = BigDecimal::from_f64(0.0).unwrap();
    let mut highest_big_value_path: Vec<PathNode> = vec![];

    let mut highest_small_value: BigDecimal = BigDecimal::from_f64(0.0).unwrap();
    let mut highest_small_value_path: Vec<PathNode> = vec![];

    let mut highest_medium_value: BigDecimal = BigDecimal::from_f64(0.0).unwrap();
    let mut highest_medium_value_path: Vec<PathNode> = vec![];

    for (display, path) in big_amount_path_nodes.iter(){
        // println!("*****************************************");
        let path_value = &path[path.len()-1].path_value;
        if BigDecimal::from_str(&path_value).unwrap().gt(&highest_big_value){
            highest_big_value_path = path.clone();
            highest_big_value = BigDecimal::from_str(&path_value).unwrap();
        }
    }

    for (display, path) in small_amount_path_nodes.iter(){
        // println!("*****************************************");
        let path_value = &path[path.len()-1].path_value;
        if BigDecimal::from_str(&path_value).unwrap().gt(&highest_small_value){
            highest_small_value_path = path.clone();
            highest_small_value = BigDecimal::from_str(&path_value).unwrap();
        }
    }

    for (display, path) in medium_amount_path_nodes.iter(){
        // println!("*****************************************");
        let path_value = &path[path.len()-1].path_value;
        if BigDecimal::from_str(&path_value).unwrap().gt(&highest_medium_value){
            highest_medium_value_path = path.clone();
            highest_medium_value = BigDecimal::from_str(&path_value).unwrap();
        }
        println!("Final path value: {}", path_value);
    }

    
    println!("Highest big value: {}", highest_big_value);
    for node in highest_big_value_path.clone(){
        println!("{}: {} {}", node.node_key, node.asset_name, node.path_value);
    }
    println!("*****************************************");
    println!("Highest small value: {}", highest_small_value);
    for node in highest_small_value_path.clone(){
        println!("{}: {} {}", node.node_key, node.asset_name, node.path_value);
    }
    println!("*****************************************");
    println!("Highest medium value: {}", highest_medium_value);
    for node in highest_medium_value_path.clone(){
        println!("{}: {} {}", node.node_key, node.asset_name, node.path_value);
    }
    println!("*****************************************");
    ResultLogger::log_results_default_kusama(highest_big_value_path, start_node_asset_name.clone(), big_amount);
    ResultLogger::log_results_default_kusama(highest_small_value_path, start_node_asset_name.clone(), small_input);
    ResultLogger::log_results_default_kusama(highest_medium_value_path, start_node_asset_name, medium_input);
}

// ****** PROBLEM FUNCTION ***********
pub async fn async_search_best_path_a_to_b(start_key: String, destination_key: String, input_amount: BigDecimal, relay: String){
    let mut asset_registry: AssetRegistry2;
    let lp_registry: LiqPoolRegistry2;

    if relay.clone() == "kusama".to_string(){
        println!("RUNNING KUSAMA SEARCH");
        asset_registry = AssetRegistry2::build_asset_registry();
        lp_registry = LiqPoolRegistry2::build_liqpool_registry(&mut asset_registry);

    } else if relay.clone() == "polkadot".to_string() {
        println!("Building polkadot registry");
        asset_registry = AssetRegistry2::build_asset_registry_polkadot();
        // asset_registry.display_all_assets();
        lp_registry = LiqPoolRegistry2::build_liqpool_registry_polkadot(&mut asset_registry);
    } else{
        panic!("Unknown relay: {}", relay);
    }
    // lp_registry.display_stable_pools();
    let list = AdjacencyTable2::build_table_2(&lp_registry);
    let graph = TokenGraph2::build_graph_2(asset_registry, list);
    // graph.display_graph_3();
    let start_node = &graph.get_node(start_key).clone();
    let start_asset_key = start_node.borrow().asset_key.clone();
    // println!("Start asset key: {}", start_asset_key);
    let start_node_asset_name = start_node.borrow().get_asset_name();

    let start_asset_location = start_node.borrow().get_asset_location().unwrap();
    let all_start_assets = &graph.asset_registry.get_assets_at_location(start_asset_location);
    let mut start_nodes: Vec<Rc<RefCell<GraphNode>>> = vec![];

    // For testing, only one node ***
    // for start_asset in all_start_assets{
    //     if(!start_asset.borrow().is_cex_token()){
    //         let new_start_node = &graph.get_node(start_asset.borrow().get_map_key()).clone();
    //         start_nodes.push(Rc::clone(&new_start_node));
    //     }
    // }
    start_nodes.push(Rc::clone(&start_node));
    // *****************************************
    let mut handles = Vec::new();

    for node in start_nodes{
        let relay_parameter = relay.clone();
        let key = node.borrow().get_asset_key();
        // println!("Searching for {}", key);
        let dest_key = destination_key.clone();
        let amount = input_amount.clone();

        // let future = task::spawn(search_best_path_a_to_b_async(key, destination_key.clone(), input_amount));
        let handle = task::spawn(async move {
            search_best_path_a_to_b_async(relay_parameter, key, dest_key, amount).await
        });
        handles.push(handle);
    }

    let mut path_nodes = vec![];
    // Await all the spawned tasks
    for handle in handles {
        let result = handle.await;
        match result {
            Ok(ok) => {
                // println!("Task completed with result: {:?}", ok);
                let (path_amount, display_string, path) = ok;
                path_nodes.push((display_string, path));
            },
            Err(e) => println!("Task failed with error: {:?}", e),
        }
    }

    let mut highest_value: BigDecimal = BigDecimal::from_f64(0.0).unwrap();
    let mut highest_value_path: Vec<PathNode> = vec![];

    for (display, path) in path_nodes.iter(){
        println!("*****************************************");
        let path_value = &path[path.len()-1].path_value;
        if BigDecimal::from_str(&path_value).unwrap().gt(&highest_value){
            highest_value_path = path.clone();
            highest_value = BigDecimal::from_str(&path_value).unwrap();
        }
        println!("Final path value: {}", path_value);
        println!("Display: {}", display);
        for node in path{
            println!("{}: {} {}", node.node_key, node.asset_name, node.path_value);
        }
        println!("*****************************************");
    }

    println!("Highest value: {}", highest_value);
    for node in highest_value_path.clone(){
        println!("{}: {} {}", node.node_key, node.asset_name, node.path_value);
    }

    ResultLogger::log_async_search_target(highest_value_path, start_node_asset_name, relay);

}


pub async fn async_search_default_polkadot(){
    let start_key = "2000{\"NativeAssetId\":{\"Token\":\"DOT\"}}".to_string();
    let destination_key = "2000{\"NativeAssetId\":{\"Token\":\"DOT\"}}".to_string();
    // let input_amount = 1 as f64;
    let mut asset_registry = AssetRegistry2::build_asset_registry_polkadot();
    let lp_registry = LiqPoolRegistry2::build_liqpool_registry_polkadot(&mut asset_registry);
    let list = AdjacencyTable2::build_table_2(&lp_registry);
    let graph = TokenGraph2::build_graph_2(asset_registry, list);

    let start_node = &graph.get_node(start_key).clone();
    
    let start_node_asset_name = start_node.borrow().get_asset_name();

    let start_asset_location = start_node.borrow().get_asset_location().unwrap();
    let all_start_assets = &graph.asset_registry.get_assets_at_location(start_asset_location);
    let mut start_nodes = vec![];
    for start_asset in all_start_assets{
        if !start_asset.borrow().is_cex_token() {
            let new_start_node = &graph.get_node(start_asset.borrow().get_map_key()).clone();
            start_nodes.push(Rc::clone(&new_start_node));
        }
    }

    
    let small_input = BigDecimal::from_f64(0.5 as f64).unwrap();
    let mut small_handles = Vec::new();
    let medium_input = BigDecimal::from_f64(2 as f64).unwrap();
    let mut medium_handles = Vec::new();
    let big_input = BigDecimal::from_f64(5 as f64).unwrap();
    let mut big_handles = Vec::new();

    for node in start_nodes.clone(){
        let key = node.borrow().get_asset_key();
        // println!("Searching for {}", key);
        let dest_key = destination_key.clone();
        let amount = big_input.clone();
        // let future = task::spawn(search_best_path_a_to_b_async(key, destination_key.clone(), input_amount));
        let handle = task::spawn(async move {
            search_best_path_a_to_b_async_polkadot(key, dest_key, amount).await
        });
        big_handles.push(handle);
    }

    for node in start_nodes.clone(){
        let key = node.borrow().get_asset_key();
        // println!("Searching for {}", key);
        let dest_key = destination_key.clone();

        let amount = medium_input.clone();
        // let future = task::spawn(search_best_path_a_to_b_async(key, destination_key.clone(), input_amount));
        let handle = task::spawn(async move {
            search_best_path_a_to_b_async_polkadot(key, dest_key, amount).await
        });
        medium_handles.push(handle);
    }

    for node in start_nodes.clone(){
        let key = node.borrow().get_asset_key();
        // println!("Searching for {}", key);
        let dest_key = destination_key.clone();

        let amount = small_input.clone();
        // let future = task::spawn(search_best_path_a_to_b_async(key, destination_key.clone(), input_amount));
        let handle = task::spawn(async move {
            search_best_path_a_to_b_async_polkadot(key, dest_key, amount).await
        });
        small_handles.push(handle);
    }

    let mut merged_handles = Vec::new();
    merged_handles.extend(big_handles);
    merged_handles.extend(small_handles);
    merged_handles.extend(medium_handles);

    // Now, use join_all to await all the tasks at once
    let results = join_all(merged_handles).await;

    let mut small_amount_path_nodes: Vec<(String, Vec<PathNode>)> = vec![];
    let mut medium_amount_path_nodes: Vec<(String, Vec<PathNode>)> = vec![];
    let mut big_amount_path_nodes: Vec<(String, Vec<PathNode>)> = vec![];

    for result in results {
        match result {
            Ok(ok) => {
                let (path_amount, display_string, path) = ok;
                if path_amount == big_input {
                    big_amount_path_nodes.push((display_string, path));
                } else if path_amount == small_input {
                    small_amount_path_nodes.push((display_string, path));
                } else if path_amount == medium_input {
                    medium_amount_path_nodes.push((display_string, path));
                } else {
                    println!("Unknown path amount: {}", path_amount);
                }
            },
            Err(e) => println!("Task failed with error: {:?}", e),
        }
    }

    let mut highest_big_value: BigDecimal = BigDecimal::from(0);
    let mut highest_big_value_path: Vec<PathNode> = vec![];

    let mut highest_small_value:BigDecimal = BigDecimal::from(0);
    let mut highest_small_value_path: Vec<PathNode> = vec![];

    let mut highest_medium_value: BigDecimal = BigDecimal::from(0);
    let mut highest_medium_value_path: Vec<PathNode> = vec![];


    for (display, path) in big_amount_path_nodes.iter(){
        // println!("*****************************************");
        let path_value = &path[path.len()-1].path_value;
        if BigDecimal::from_str(&path_value).unwrap().gt(&highest_big_value){
            highest_big_value_path = path.clone();
            highest_big_value = BigDecimal::from_str(&path_value).unwrap();
        }
    }

    for (display, path) in small_amount_path_nodes.iter(){
        // println!("*****************************************");
        let path_value = &path[path.len()-1].path_value;
        if BigDecimal::from_str(&path_value).unwrap().gt(&highest_small_value){
            highest_small_value_path = path.clone();
            highest_small_value = BigDecimal::from_str(&path_value).unwrap();
        }
    }

    for (display, path) in medium_amount_path_nodes.iter(){
        // println!("*****************************************");
        let path_value = &path[path.len()-1].path_value;
        if BigDecimal::from_str(&path_value).unwrap().gt(&highest_medium_value){
            highest_medium_value_path = path.clone();
            highest_medium_value = BigDecimal::from_str(&path_value).unwrap();
        }
        println!("Final path value: {}", path_value);
    }

    println!("Highest input value: {}", highest_big_value);
    for node in highest_big_value_path.clone(){
        println!("{}: {} {} || {:?}", node.node_key, node.asset_name, node.path_value, node.path_data);
    }
    println!("*****************************************");
    println!("Highest small value: {}", highest_small_value);
    for node in highest_small_value_path.clone(){
        println!("{}: {} {} || {:?}", node.node_key, node.asset_name, node.path_value, node.path_data);
    }
    println!("*****************************************");
    println!("Highest medium value: {}", highest_medium_value);
    for node in highest_medium_value_path.clone(){
        println!("{}: {} {} || {:?}", node.node_key, node.asset_name, node.path_value, node.path_data);
    }
    println!("*****************************************");
    ResultLogger::log_results_default_polkadot(highest_big_value_path, start_node_asset_name.clone(), big_input);
    ResultLogger::log_results_default_polkadot(highest_small_value_path, start_node_asset_name.clone(), small_input);
    ResultLogger::log_results_default_polkadot(highest_medium_value_path, start_node_asset_name, medium_input);




}
pub async fn async_search_best_path_a_to_b_polkadot(start_key: String, destination_key: String, input_amount: BigDecimal){
    let mut asset_registry = AssetRegistry2::build_asset_registry_polkadot();
    let lp_registry = LiqPoolRegistry2::build_liqpool_registry_polkadot(&mut asset_registry);
    lp_registry.display_stable_pools();
    let list = AdjacencyTable2::build_table_2(&lp_registry);
    let graph = TokenGraph2::build_graph_2(asset_registry, list);

    let start_node = &graph.get_node(start_key).clone();
    
    let start_node_asset_name = start_node.borrow().get_asset_name();

    let start_asset_location = start_node.borrow().get_asset_location().unwrap();
    let all_start_assets = &graph.asset_registry.get_assets_at_location(start_asset_location);
    let mut start_nodes = vec![];
    for start_asset in all_start_assets{
        if(!start_asset.borrow().is_cex_token()){
            let new_start_node = &graph.get_node(start_asset.borrow().get_map_key()).clone();
            start_nodes.push(Rc::clone(&new_start_node));
        }
    }

    let mut handles = Vec::new();

    for node in start_nodes{
        let key = node.borrow().get_asset_key();
        println!("Searching for {}", key);
        let dest_key = destination_key.clone();
        let amount = input_amount.clone();
        // let future = task::spawn(search_best_path_a_to_b_async(key, destination_key.clone(), input_amount));
        let handle = task::spawn(async move {
            search_best_path_a_to_b_async("polkadot".to_string(), key, dest_key, amount).await
        });
        handles.push(handle);
    }

    let mut path_nodes = vec![];
    // Await all the spawned tasks
    for handle in handles {
        let result = handle.await;
        match result {
            Ok(ok) => {
                // println!("Task completed with result: {:?}", ok);
                let (path_amount, display_string, path) = ok;
                path_nodes.push((display_string, path));
            },
            Err(e) => println!("Task failed with error: {:?}", e),
        }
    }

    let mut highest_value: BigDecimal = BigDecimal::from_f64(0.0).unwrap();
    let mut highest_value_path: Vec<PathNode> = vec![];

    for (display, path) in path_nodes.iter(){
        println!("*****************************************");
        let path_value = &path[path.len()-1].path_value;
        if BigDecimal::from_str(&path_value).unwrap() > highest_value{
            highest_value_path = path.clone();
            highest_value = BigDecimal::from_str(&path_value).unwrap();
        }
        println!("Final path value: {}", path_value);
        println!("Display: {}", display);
        for node in path{
            println!("{}: {} {}", node.node_key, node.asset_name, node.path_value);
        }
        println!("*****************************************");
    }

    println!("Highest value: {}", highest_value);
    for node in highest_value_path.clone(){
        println!("{}: {} {}", node.node_key, node.asset_name, node.path_value);
    }

    ResultLogger::log_async_search_target(highest_value_path, start_node_asset_name, "polkadot".to_string());

}

pub fn sync_search_default_polkadot(){
    let start_key = "2030{\"Token2\":\"0\"}".to_string();
    let destination_key = "2000{\"NativeAssetId\":{\"Token\":\"DOT\"}}".to_string();
    let mut asset_registry = AssetRegistry2::build_asset_registry_polkadot();
    let lp_registry = LiqPoolRegistry2::build_liqpool_registry_polkadot(&mut asset_registry);
    let list = AdjacencyTable2::build_table_2(&lp_registry);
    let graph = TokenGraph2::build_graph_2(asset_registry, list);

    let start_node = &graph.get_node(start_key.clone()).clone();
    
    let start_node_asset_name = start_node.borrow().get_asset_name();

    let start_asset_location = start_node.borrow().get_asset_location().unwrap();
    let all_start_assets = &graph.asset_registry.get_assets_at_location(start_asset_location);

    // search_best_path_a_to_b_sync_polkadot(start_key.clone(), start_key, 1 as f64);

    //***************************************** */
    let mut start_nodes = vec![];
    // let mut inputAmounts = vec![];
    for start_asset in all_start_assets{
        if !start_asset.borrow().is_cex_token() {
            let new_start_node = &graph.get_node(start_asset.borrow().get_map_key()).clone();
            start_nodes.push(Rc::clone(&new_start_node));
        }
    }

    let input_amount = BigDecimal::from(1);

    for node in start_nodes.clone(){
        let key = node.borrow().get_asset_key();
        println!("Searching for {}", key);
        let dest_key = destination_key.clone();
        let amount = input_amount.clone();
        let (value, display, path) = search_best_path_a_to_b_sync_polkadot(key, dest_key, amount);
    }
    // *********************************************


}

pub fn one_search_default_polkadot(){
    let start_key = "2030{\"Token2\":\"0\"}".to_string();
    let destination_key = "2000{\"NativeAssetId\":{\"Token\":\"DOT\"}}".to_string();
    // let mut asset_registry = AssetRegistry2::build_asset_registry_polkadot();
    // let lp_registry = LiqPoolRegistry2::build_liqpool_registry_polkadot(&mut asset_registry);
    // let list = AdjacencyTable2::build_table_2(&lp_registry);
    // let graph = TokenGraph2::build_graph_2(asset_registry, list);

    // let start_node: &Rc<RefCell<GraphNode>> = &graph.get_node(start_key.clone()).clone();
    // let start_node_asset_name = start_node.borrow().get_asset_name();

    let input_amount = BigDecimal::from(1);
    let (value, display, path) = search_best_path_a_to_b_sync_polkadot(start_key, destination_key, input_amount);
    println!("Display: {}", display);
    println!("***** Path Nodes *****");
    for node in path{
        let path_data: PathData = node.path_data;
        print!("{}: {} {}", node.node_key, node.asset_name, node.path_value);
        println!(" || {} {:?} {:?}", path_data.path_type, path_data.lp_id, path_data.xcm_transfer_reserve_amounts);
    }
}


// ******************* PROBLEM FUNCTION CALLS THIS FUNCTION *************
pub async fn search_best_path_a_to_b_async(relay: String, start_key: String, destination_key: String, input_amount: BigDecimal) -> (BigDecimal, String, Vec<PathNode>){
    let mut asset_registry;
    let lp_registry;
    if relay == "kusama".to_string(){
        asset_registry = AssetRegistry2::build_asset_registry();
        lp_registry = LiqPoolRegistry2::build_liqpool_registry(&mut asset_registry);
    } else if relay == "polkadot".to_string() {
        asset_registry = AssetRegistry2::build_asset_registry_polkadot();
        lp_registry = LiqPoolRegistry2::build_liqpool_registry_polkadot(&mut asset_registry);
    } else{
        panic!("Unknown relay: {}", relay);
    }

    let list = AdjacencyTable2::build_table_2(&lp_registry);
    let graph = TokenGraph2::build_graph_2(asset_registry, list);
    // let key_1 = start_key;
    let (display_string, path) = graph.find_best_route(start_key, destination_key, input_amount.clone());

    let return_path = return_path_nodes(path);

    (input_amount, display_string, return_path)
}

// All searches at once. MAIN default search calls this one
pub async fn search_best_path_a_to_b_async_polkadot(start_key: String, destination_key: String, input_amount: BigDecimal) -> (BigDecimal, String, Vec<PathNode>){
    let mut asset_registry = AssetRegistry2::build_asset_registry_polkadot();
    let lp_registry = LiqPoolRegistry2::build_liqpool_registry_polkadot(&mut asset_registry);
    let list = AdjacencyTable2::build_table_2(&lp_registry);
    let graph = TokenGraph2::build_graph_2(asset_registry, list);
    // let key_1 = start_key;
    let (display_string, path) = graph.find_best_route(start_key, destination_key, input_amount.clone());

    println!("Display string: {}", display_string);

    let return_path = return_path_nodes(path);

    (input_amount, display_string, return_path)
}

// Search one by one
pub fn search_best_path_a_to_b_sync_polkadot(start_key: String, destination_key: String, input_amount: BigDecimal) -> (BigDecimal, String, Vec<PathNode>){
    let mut asset_registry = AssetRegistry2::build_asset_registry_polkadot();
    let lp_registry = LiqPoolRegistry2::build_liqpool_registry_polkadot(&mut asset_registry);
    let list = AdjacencyTable2::build_table_2(&lp_registry);
    let graph = TokenGraph2::build_graph_2(asset_registry, list);
    // let key_1 = start_key;
    let (display_string, path) = graph.find_best_route(start_key, destination_key, input_amount.clone());

    println!("Display string: {}", display_string);

    let return_path = return_path_nodes(path);

    (input_amount, display_string, return_path)
}

pub fn return_path_nodes(path: NodePath) -> Vec<PathNode> {
    let target_node = path[path.len() - 1].borrow();
    let path_values = &target_node.path_values;
    let path_value_types = &target_node.path_value_types;
    let path_datas = &target_node.path_datas;
    let mut result_log: Vec<PathNode> = Vec::new();
    for(i, node) in path.iter().enumerate(){
        let path_node = PathNode{
            node_key: node.borrow().get_asset_key(),
            asset_name: node.borrow().get_asset_name(),
            path_value: path_values[i].to_string(),
            path_type: path_value_types[i].clone(),
            path_data: path_datas[i].clone(),
        };
        result_log.push(path_node);
    }
    result_log.clone()

}

pub fn test_table_2(){
    let mut asset_registry = AssetRegistry2::build_asset_registry();
    // asset_registry.display_all_assets();
    let lp_registry = LiqPoolRegistry2::build_liqpool_registry(&mut asset_registry);
    // lp_registry.display_stable_pools()
    let list = AdjacencyTable2::build_table_2(&lp_registry);
    // list.display_table();
    let graph = TokenGraph2::build_graph_2(asset_registry, list);
    graph.display_graph_3();
}

pub async fn test_assets(){
    let mut asset_registry = AssetRegistry2::build_asset_registry();
    asset_registry.display_all_assets();
}

pub async fn test_polkadot_lps(){
    let start_key = "2000{\"NativeAssetId\":{\"Token\":\"DOT\"}}".to_string();
    let destination_key = "2000{\"NativeAssetId\":{\"Token\":\"DOT\"}}".to_string();
    // // let input_amount = 1 as f64;
    // let mut asset_registry = AssetRegistry2::build_asset_registry_polkadot();

    // // asset_registry.display_all_glmr_assets();

    // let lp_registry = LiqPoolRegistry2::build_liqpool_registry_polkadot(&mut asset_registry);
    // // lp_registry.display_liq_pools()
    // let list = AdjacencyTable2::build_table_2(&lp_registry);
    // let graph = TokenGraph2::build_graph_2(asset_registry, list);
    search_best_path_a_to_b_async_polkadot(start_key, destination_key, BigDecimal::from_f64(1.0).unwrap()).await;
}

pub async fn test_v3_swap(){
    let start_key = "2000{\"NativeAssetId\":{\"Token\":\"DOT\"}}".to_string();
    let destination_key = "2000{\"NativeAssetId\":{\"Token\":\"DOT\"}}".to_string();
    let glmr_dot_v3_pool = "0xB13B281503F6eC8A837Ae1a21e86a9caE368fCc5".to_string();
    let glmr_aca_v3_pool = "0x7c0b3bf935b457738d87926110300b3c5d76c77b".to_string();
    let mut asset_registry = AssetRegistry2::build_asset_registry_polkadot();
    let lp_registry = LiqPoolRegistry2::build_liqpool_registry_polkadot(&mut asset_registry);

    lp_registry.display_stable_pools();
    // let list = AdjacencyTable2::build_table_2(&lp_registry);
    // let graph = TokenGraph2::build_graph_2(asset_registry, list);
    // graph.calculate_v3_swap("asset_key_1".to_string(), "`asset_key_2`".to_string(), glmr_aca_v3_pool, 80.0)
    // let asset_node = graph.get_asset_by_chain_and_symbol(2004, "XCDOT".to_string()).unwrap();
    // let asset_key = asset_node.borrow().get_asset_key();
    // println!("Asset key: {}", asset_key);
    
    // let adj_pairs = asset_node.borrow().adjacent_pairs2.clone();
    
    // let key_1 = start_key;
    // let (display_string, path) = graph.find_best_route(start_key, destination_key, input_amount);

    // println!("Display string: {}", display_string);

}

pub async fn test_stable_swap(){
    // let start_key = "2000{\"NativeAssetId\":{\"Token\":\"DOT\"}}".to_string();
    // let destination_key = "2000{\"NativeAssetId\":{\"Token\":\"DOT\"}}".to_string();
    let chain_id = 2034;
    let stable_4_pool_id = "100".to_string();
    let dot_usdt = "10";
    let glmr_dai = "18";
    let glmr_usdc = "21";
    let dot_usdc = "22";
    let glmr_usdt = "23";

    let stable_2_pool_id = "102".to_string(); // dot usdt and dot usdc
    let kar_usd_stable_pool_id = "1".to_string();
    let ausd = "ASEED".to_string();
    let usdc = "USDCet".to_string();
    // let glmr_aca_v3_pool = "0x7c0b3bf935b457738d87926110300b3c5d76c77b".to_string();
    // let mut asset_registry = AssetRegistry2::build_asset_registry_polkadot();
    // let lp_registry = LiqPoolRegistry2::build_liqpool_registry_polkadot(&mut asset_registry);

    let mut asset_registry = AssetRegistry2::build_asset_registry();
    let lp_registry = LiqPoolRegistry2::build_liqpool_registry(&mut asset_registry);

    let input_amount = 1 as f64;
    // lp_registry.display_stable_pools();

    let list = AdjacencyTable2::build_table_2(&lp_registry);
    let graph = TokenGraph2::build_graph_2(asset_registry, list);
    graph.calculate_stable_swap(ausd, usdc.clone(), 2000, kar_usd_stable_pool_id, input_amount);
    // graph.calculate_stable_swap(stable_2_pool_id, chain_id, stable_4_pool_id, input_amount);
    // graph.display_stable_share_pairs();



}

pub async fn test_asset_location(){
    let mut asset_registry = AssetRegistry2::build_asset_registry();
    let lp_registry = LiqPoolRegistry2::build_liqpool_registry(&mut asset_registry);
    let list = AdjacencyTable2::build_table_2(&lp_registry);
    // let graph = TokenGraph2::build_graph_2(&asset_registry, list);

    for(key,value) in &asset_registry.asset_map{
        // let display_string = key.clone() + " " + &value.iter().map(|x| x.borrow().get_asset_name().to_string()).collect::<Vec<String>>().join(", ");
        // keys.push(display_string);
        println!("----------------------");
        println!("Asset key: {}", key);
        let asset = value[0].borrow();
        let asset_location = asset.get_asset_location();
        // println!("Asset location: {:?}", asset_location);
        if let Some(location) = asset_location{
            println!("Asset location: {:?}", location);
            let chain_id = location.get_parachain_id();
            println!("Chain id: {:?}", chain_id);
        }
    }


}

pub async fn test_graph(){
    let mut asset_registry = AssetRegistry2::build_asset_registry_polkadot();
    let lp_registry = LiqPoolRegistry2::build_liqpool_registry_polkadot(&mut asset_registry);
    let list = AdjacencyTable2::build_table_2(&lp_registry);
    let graph = TokenGraph2::build_graph_2(asset_registry, list);

    // graph.display_bnc_stable_pairs();

    // let asset_key_1 = "{\"Token2\":\"0\"}".to_string();
    // let asset_key_2 = "{\"VToken2\":\"0\"}".to_string();
    let asset_key_2 = "INTR".to_string();
    let asset_key_1 = "IBTC".to_string();
    let chain_id = 2000;
    let pool_id = "0".to_string();
    let input_amount = 1 as f64;

    // graph.calculate_bnc_stable_swap(asset_key_1, asset_key_2, chain_id, pool_id, input_amount);
    // graph.calculate_aca_stable_swap(asset_key_1, asset_key_2, chain_id, pool_id, input_amount);
    // println!("Get test asset");
    // let asset_id = "UNQ";
    // let asset_id_string = serde_json::to_string(asset_id).unwrap();
    // let asset_key = "2037".to_string() + asset_id;
    // let test_asset = graph.asset_registry.get_asset_by_id(2037, asset_key.as_str()).unwrap();
    let test_node_one = graph.get_asset_by_chain_and_symbol(2032, asset_key_1).unwrap();
    let test_node_two = graph.get_asset_by_chain_and_symbol(2032, asset_key_2).unwrap();

    let fee_amount = BigInt::from_str("112586400").unwrap();

    println!("Finding path amount from {} {} -> {}", test_node_two.borrow().get_asset_key_and_symbol(), fee_amount, test_node_one.borrow().get_asset_key_and_symbol());

    // let path_amount_between_nodes = graph.get_path_bfs(test_node_two, test_node_one, fee_amount).unwrap();
    let path_amount = graph.find_path_between_nodes_on_chain(test_node_two, test_node_one, fee_amount).unwrap();
    println!("Path amount between nodes: {}", path_amount);
    // test_asset.borrow().display_asset();

    // let asset_location = test_asset.borrow().get_asset_location().unwrap();
    // println!("Asset location: {:?}", asset_location);

    // let assets_at_location = graph.asset_registry.get_assets_at_location(asset_location);
    // println!("Assets at location: {}", assets_at_location.len());
    // for asset in assets_at_location{
    //     asset.borrow().display_asset();
    // }

    // graph.asset_registry.display_all_assets();

    

    // graph.display_graph_3();
    // graph.test_all_fees();

} 

pub async fn test_ticks(){
    let test_tick = BigInt::from(-214375);
    let test_tick_i32 = -214375;
    let sqrt_price = get_sqrt_ratio_at_tick(test_tick_i32);
    println!("Sqrt price: {}", sqrt_price);
}

pub async fn search_best_path_a_to_b(start_key: String, destination_key: String, input_amount: BigDecimal){
    let mut asset_registry = AssetRegistry2::build_asset_registry();
    let lp_registry = LiqPoolRegistry2::build_liqpool_registry(&mut asset_registry);
    lp_registry.display_stable_pools();
    let list = AdjacencyTable2::build_table_2(&lp_registry);
    let graph = TokenGraph2::build_graph_2(asset_registry, list);
    // let key_1 = start_key;
    let (display_string, path) = graph.find_best_route(start_key, destination_key, input_amount);

    let target_node = path[path.len() - 1].clone();
    let path_values = target_node.borrow().path_values.clone();
    for (i, node) in path.clone().iter().enumerate(){
        println!("SEARCH RESULT {}: {} {}", node.borrow().get_asset_key(), node.borrow().get_asset_name(), path_values[i]);
    }

    let loggable_results = ResultLogger::log_results_target(path);
    println!("{}", display_string);

    for node in loggable_results.iter(){
        println!("{}: {} {}", node.node_key, node.asset_name, node.path_value);
    }
}



pub async fn fallback_search_a_to_b(start_key: String, destination_key: String, input_amount: BigDecimal, relay: String){

    let mut asset_registry: AssetRegistry2;
    let lp_registry: LiqPoolRegistry2;

    if(relay.to_lowercase() == "kusama"){
        asset_registry = AssetRegistry2::build_asset_registry();
        lp_registry = LiqPoolRegistry2::build_liqpool_registry(&mut asset_registry);
    } else if(relay.to_lowercase() == "polkadot"){
        asset_registry = AssetRegistry2::build_asset_registry_polkadot();
        lp_registry = LiqPoolRegistry2::build_liqpool_registry_polkadot(&mut asset_registry);
    } else {
        // asset_registry = AssetRegistry2::build_asset_registry();
        // lp_registry = LiqPoolRegistry2::build_liqpool_registry(&mut asset_registry);
        panic!("Unknown relay: {}", relay);
    }

    // let mut asset_registry = AssetRegistry2::build_asset_registry();
    // let lp_registry = LiqPoolRegistry2::build_liqpool_registry(&mut asset_registry);
    lp_registry.display_stable_pools();
    let list = AdjacencyTable2::build_table_2(&lp_registry);
    let graph = TokenGraph2::build_graph_2(asset_registry, list);
    // let key_1 = start_key;
    let (display_string, path) = graph.find_best_route(start_key, destination_key, input_amount);

    let target_node = path[path.len() - 1].clone();
    let path_values = target_node.borrow().path_values.clone();
    for (i, node) in path.clone().iter().enumerate(){
        println!("SEARCH RESULT {}: {} {}", node.borrow().get_asset_key(), node.borrow().get_asset_name(), path_values[i]);
    }

    let loggable_results = ResultLogger::log_results_fallback(path, relay.to_lowercase());
    println!("{}", display_string);

    for node in loggable_results.iter(){
        println!("{}: {} {}", node.node_key, node.asset_name, node.path_value);
    }

}

pub async fn print_asset_keys(start_key: String){
    let mut asset_registry = AssetRegistry2::build_asset_registry();
    let lp_registry = LiqPoolRegistry2::build_liqpool_registry(&mut asset_registry);
    lp_registry.display_stable_pools();
    let list = AdjacencyTable2::build_table_2(&lp_registry);
    let graph = TokenGraph2::build_graph_2(asset_registry, list);
    // let key_1 = start_key;
    graph.get_asset_keys(start_key);
    // (display_string, loggable_results)
    // "lol".to_string()
}


// pub async fn search_ksm_small() -> (String, Vec<PathNode>){
//     let mut asset_registry = AssetRegistry2::build_asset_registry();
//     let lp_registry = LiqPoolRegistry2::build_liqpool_registry(&mut asset_registry);
//     lp_registry.display_stable_pools();
//     let list = AdjacencyTable2::build_table_2(&lp_registry);
//     let graph = TokenGraph2::build_graph_2(asset_registry, list);
//     let key_1 = "2000{\"NativeAssetId\":{\"Token\":\"KSM\"}}".to_string();
//     let input_amount = 0.05 as f64;
//     let (display_string, path) = graph.find_arbitrage_3(key_1, input_amount);
//     let loggable_results = ResultLogger::log_results_small(path);
//     (display_string, loggable_results)  

//     // "lol".to_string()
// }