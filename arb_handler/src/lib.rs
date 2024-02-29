
mod asset_registry;
mod liq_pool_registry;
mod adjacency_table;
mod token_graph;
mod token;
use token::{Token, AssetKeyType};
use adjacency_table::{AdjacencyTable};
use asset_registry::AssetRegistry;
use liq_pool_registry::LiqPoolRegistry;
use token_graph::TokenGraph;
use token_graph::calculate_swap;

mod asset_registry_2;
mod liq_pool_registry_2;
mod adjacency_table_2;
mod token_graph_2;
use adjacency_table_2::{AdjacencyTable2};
use asset_registry_2::AssetRegistry2;
use liq_pool_registry_2::LiqPoolRegistry2;
use token_graph_2::TokenGraph2;
use token_graph_2::GraphNode;

use std::fs::File;
use std::io::prelude::*;
use serde_json::{Value};
use serde::{Deserialize, Serialize};
use std::str;
use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;
use tokio::{join, task};
use std::fs::OpenOptions;
// use std::io::prelude::*;
type NodePath = Vec<Rc<RefCell<GraphNode>>>;

// This is the object that we log at the end
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PathNode{
    pub node_key: String,
    pub asset_name: String,
    pub path_value: f64,
    pub path_identifier: u64, // 0 - 3 for transfer code
    // pub path_id: String, // Any extra info like pool ID
}

pub fn build_sub_assets(){
    AssetRegistry::build_sub_asset_registry_from_file();
}

pub async fn async_search(){
    // let future1 = task::spawn(search_rmrk());
    let future2 = task::spawn(search_ksm());
    let future3 =  task::spawn(search_ksm_small());
    let (result2, result3) = join!(future2, future3);
    // future1
    let (ksm_display, ksm_log) = result2.unwrap();
    println!();
    // println!("------------------------------------");
    // println!("RESULTS");
    // println!("------------------------------------");
    // println!("Result of function 1: {:?}", result1);
    // println!("------------------------------------");
    // println!("Result of function 2: {:?}", ksm_display);
    // println!("------------------------------------");
    // println!("Result of function 3: {:?}", result3);

    // for node in ksm_log{
        // println!("{}: {} {}", node.node_key, node.asset_name, node.path_value);
    // }
}
pub async fn async_search_default(){
    let start_key = "2000{\"NativeAssetId\":{\"Token\":\"KSM\"}}".to_string();
    let destination_key = "2000{\"NativeAssetId\":{\"Token\":\"KSM\"}}".to_string();
    let input_amount = 1 as f64;
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

        // let future = task::spawn(search_best_path_a_to_b_async(key, destination_key.clone(), input_amount));
        let handle = task::spawn(async move {
            search_best_path_a_to_b_async(key, dest_key, input_amount).await
        });
        handles.push(handle);
    }

    let mut path_nodes = vec![];
    // Await all the spawned tasks
    for handle in handles {
        let result = handle.await;
        match result {
            Ok(ok) => {
                println!("Task completed with result: {:?}", ok);
                let (display_string, path) = ok;
                path_nodes.push((display_string, path));
            },
            Err(e) => println!("Task failed with error: {:?}", e),
        }
    }

    let mut highest_value: f64 = 0.0;
    let mut highest_value_path: Vec<PathNode> = vec![];

    for (display, path) in path_nodes.iter(){
        println!("*****************************************");
        let path_value = path[path.len()-1].path_value;
        if path_value > highest_value{
            highest_value_path = path.clone();
            highest_value = path_value;
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
    log_results_default(highest_value_path, start_node_asset_name);
}
pub async fn async_search_best_path_a_to_b(start_key: String, destination_key: String, input_amount: f64){
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

        // let future = task::spawn(search_best_path_a_to_b_async(key, destination_key.clone(), input_amount));
        let handle = task::spawn(async move {
            search_best_path_a_to_b_async(key, dest_key, input_amount).await
        });
        handles.push(handle);
    }

    let mut path_nodes = vec![];
    // Await all the spawned tasks
    for handle in handles {
        let result = handle.await;
        match result {
            Ok(ok) => {
                println!("Task completed with result: {:?}", ok);
                let (display_string, path) = ok;
                path_nodes.push((display_string, path));
            },
            Err(e) => println!("Task failed with error: {:?}", e),
        }
    }

    let mut highest_value: f64 = 0.0;
    let mut highest_value_path: Vec<PathNode> = vec![];

    for (display, path) in path_nodes.iter(){
        println!("*****************************************");
        let path_value = path[path.len()-1].path_value;
        if path_value > highest_value{
            highest_value_path = path.clone();
            highest_value = path_value;
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

    log_async_search_target(highest_value_path, start_node_asset_name);

}

pub async fn async_search_default_polkadot(){
    let start_key = "2000{\"NativeAssetId\":{\"Token\":\"DOT\"}}".to_string();
    let destination_key = "2000{\"NativeAssetId\":{\"Token\":\"DOT\"}}".to_string();
    let input_amount = 1 as f64;
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

        // let future = task::spawn(search_best_path_a_to_b_async(key, destination_key.clone(), input_amount));
        let handle = task::spawn(async move {
            search_best_path_a_to_b_async_polkadot(key, dest_key, input_amount).await
        });
        handles.push(handle);
    }

    let mut path_nodes = vec![];
    // Await all the spawned tasks
    for handle in handles {
        let result = handle.await;
        match result {
            Ok(ok) => {
                println!("Task completed with result: {:?}", ok);
                let (display_string, path) = ok;
                path_nodes.push((display_string, path));
            },
            Err(e) => println!("Task failed with error: {:?}", e),
        }
    }

    let mut highest_value: f64 = 0.0;
    let mut highest_value_path: Vec<PathNode> = vec![];

    for (display, path) in path_nodes.iter(){
        println!("*****************************************");
        let path_value = path[path.len()-1].path_value;
        if path_value > highest_value{
            highest_value_path = path.clone();
            highest_value = path_value;
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
    log_results_default_polkadot(highest_value_path, start_node_asset_name);
}

pub fn log_results_default_polkadot(result_log: Vec<PathNode>, start_node_name: String){
    let json = serde_json::to_string_pretty(&result_log.clone()).unwrap();
    // Get the current timestamp
    let timestamp = chrono::Local::now().format("%Y-%m-%d___%H-%M-%S").to_string();
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let time = chrono::Local::now().format("%H-%M-%S").to_string();

    // Construct the directory path for the current date
    let log_folder_path = format!("result_log_data_polkadot/{}", date);

    // Create a directory for the current date if it doesn't exist
    match std::fs::create_dir_all(&log_folder_path) {
        Ok(_) => println!("Directory created successfully"),
        Err(e) => println!("Error creating directory: {:?}", e),
    }

    // Construct the file path including the directory
    let log_data_path = format!("{}/{}_{}.json", log_folder_path, start_node_name, time);
    println!("Log data path: {}", log_data_path);
    let mut file = File::create(log_data_path).expect("Failed to create file");
    file.write_all(json.as_bytes()).expect("Failed to write data");

    // let log_path = format!("result_log.txt", start_node.get_asset_name(), timestamp);
    let best_path_value = result_log[result_log.len()-1].path_value;
    let result_log_string = format!("{} {} - {}", timestamp, start_node_name, best_path_value);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("result_log.txt")
        .expect("Failed to open or create file");
    writeln!(file, "{}", result_log_string).expect("Failed to write data");
}
pub async fn search_best_path_a_to_b_async(start_key: String, destination_key: String, input_amount: f64) -> (String, Vec<PathNode>){
    let mut asset_registry = AssetRegistry2::build_asset_registry();
    let lp_registry = LiqPoolRegistry2::build_liqpool_registry(&mut asset_registry);
    lp_registry.display_stable_pools();
    let list = AdjacencyTable2::build_table_2(&lp_registry);
    let graph = TokenGraph2::build_graph_2(asset_registry, list);
    // let key_1 = start_key;
    let (display_string, path) = graph.find_best_route(start_key, destination_key, input_amount);

    let return_path = return_path_nodes(path);

    (display_string, return_path)
}
pub async fn search_best_path_a_to_b_async_polkadot(start_key: String, destination_key: String, input_amount: f64) -> (String, Vec<PathNode>){
    let mut asset_registry = AssetRegistry2::build_asset_registry_polkadot();
    let lp_registry = LiqPoolRegistry2::build_liqpool_registry_polkadot(&mut asset_registry);
    let list = AdjacencyTable2::build_table_2(&lp_registry);
    let graph = TokenGraph2::build_graph_2(asset_registry, list);
    // let key_1 = start_key;
    let (display_string, path) = graph.find_best_route(start_key, destination_key, input_amount);

    let return_path = return_path_nodes(path);

    (display_string, return_path)
}
pub fn log_results_default(result_log: Vec<PathNode>, start_node_name: String){
    let json = serde_json::to_string_pretty(&result_log.clone()).unwrap();
    // Get the current timestamp
    let timestamp = chrono::Local::now().format("%Y-%m-%d___%H-%M-%S").to_string();
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let time = chrono::Local::now().format("%H-%M-%S").to_string();

    // Construct the directory path for the current date
    let log_folder_path = format!("result_log_data/{}", date);

    // Create a directory for the current date if it doesn't exist
    match std::fs::create_dir_all(&log_folder_path) {
        Ok(_) => println!("Directory created successfully"),
        Err(e) => println!("Error creating directory: {:?}", e),
    }

    // Construct the file path including the directory
    let log_data_path = format!("{}/{}_{}.json", log_folder_path, start_node_name, time);
    println!("Log data path: {}", log_data_path);
    // let log_data_path = format!("result_log_data/{}_{}.json", start_node.get_asset_name(), timestamp);
    // println!("Log data path: {}", log_data_path);
    // When creating the file, use the log_data_path which includes the directory
    let mut file = File::create(log_data_path).expect("Failed to create file");
    file.write_all(json.as_bytes()).expect("Failed to write data");

    // let log_path = format!("result_log.txt", start_node.get_asset_name(), timestamp);
    let best_path_value = result_log[result_log.len()-1].path_value;
    let result_log_string = format!("{} {} - {}", timestamp, start_node_name, best_path_value);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("result_log.txt")
        .expect("Failed to open or create file");
    writeln!(file, "{}", result_log_string).expect("Failed to write data");

    // result_log.clone()
}
pub fn log_results(path: NodePath) -> Vec<PathNode>{
    let start_node = path[0].borrow();
    let path_values = &start_node.path_values;
    let path_value_types = &start_node.path_value_types;
    let mut result_log: Vec<PathNode> = Vec::new();
    for(i, node) in path.iter().enumerate(){
        let path_node = PathNode{
            node_key: node.borrow().get_asset_key(),
            asset_name: node.borrow().get_asset_name(),
            path_value: path_values[i].clone(),
            path_identifier: path_value_types[i].clone(),
        };
        result_log.push(path_node);
    }

    let json = serde_json::to_string_pretty(&result_log.clone()).unwrap();
    // Get the current timestamp
    let timestamp = chrono::Local::now().format("%Y-%m-%d___%H-%M-%S").to_string();
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let time = chrono::Local::now().format("%H-%M-%S").to_string();

    // Construct the directory path for the current date
    let log_folder_path = format!("result_log_data/{}", date);

    // Create a directory for the current date if it doesn't exist
    match std::fs::create_dir_all(&log_folder_path) {
        Ok(_) => println!("Directory created successfully"),
        Err(e) => println!("Error creating directory: {:?}", e),
    }

    // Construct the file path including the directory
    let log_data_path = format!("{}/{}_{}.json", log_folder_path, start_node.get_asset_name(), time);
    println!("Log data path: {}", log_data_path);
    // let log_data_path = format!("result_log_data/{}_{}.json", start_node.get_asset_name(), timestamp);
    // println!("Log data path: {}", log_data_path);
    // When creating the file, use the log_data_path which includes the directory
    let mut file = File::create(log_data_path).expect("Failed to create file");
    file.write_all(json.as_bytes()).expect("Failed to write data");

    // let log_path = format!("result_log.txt", start_node.get_asset_name(), timestamp);
    let best_path_value = result_log[result_log.len()-1].path_value;
    let result_log_string = format!("{} {} - {}", timestamp, start_node.get_asset_name(), best_path_value);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("result_log.txt")
        .expect("Failed to open or create file");
    writeln!(file, "{}", result_log_string).expect("Failed to write data");

    result_log.clone()
}

pub fn return_path_nodes(path: NodePath) -> Vec<PathNode> {
    let target_node = path[path.len() - 1].borrow();
    let path_values = &target_node.path_values;
    let path_value_types = &target_node.path_value_types;
    let mut result_log: Vec<PathNode> = Vec::new();
    for(i, node) in path.iter().enumerate(){
        let path_node = PathNode{
            node_key: node.borrow().get_asset_key(),
            asset_name: node.borrow().get_asset_name(),
            path_value: path_values[i].clone(),
            path_identifier: path_value_types[i].clone(),
        };
        result_log.push(path_node);
    }
    result_log.clone()

}

pub fn log_async_search_target(path: Vec<PathNode>, asset_name: String) {
    let json = serde_json::to_string_pretty(&path.clone()).unwrap();
    // Get the current timestamp
    let timestamp = chrono::Local::now().format("%Y-%m-%d___%H-%M-%S").to_string();
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let time = chrono::Local::now().format("%H-%M-%S").to_string();

    // Construct the directory path for the current date
    let log_folder_path = format!("target_log_data/{}", date);

    // Create a directory for the current date if it doesn't exist
    match std::fs::create_dir_all(&log_folder_path) {
        Ok(_) => println!("Directory created successfully"),
        Err(e) => println!("Error creating directory: {:?}", e),
    }

    // Construct the file path including the directory
    let log_data_path = format!("{}/{}_{}.json", log_folder_path, asset_name.clone(), time);
    println!("Log data path: {}", log_data_path);
    // let log_data_path = format!("result_log_data/{}_{}.json", start_node.get_asset_name(), timestamp);
    // println!("Log data path: {}", log_data_path);
    // When creating the file, use the log_data_path which includes the directory
    let mut file = File::create(log_data_path).expect("Failed to create file");
    file.write_all(json.as_bytes()).expect("Failed to write data");

    // let log_path = format!("result_log.txt", start_node.get_asset_name(), timestamp);
    let best_path_value = path[path.len()-1].path_value;
    let result_log_string = format!("{} {} - {}", timestamp, asset_name, best_path_value);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("result_log.txt")
        .expect("Failed to open or create file");
    writeln!(file, "{}", result_log_string).expect("Failed to write data");
}

pub fn log_results_target(path: NodePath) -> Vec<PathNode> {
    let target_node = path[path.len() - 1].borrow();
    let path_values = &target_node.path_values;
    let path_value_types = &target_node.path_value_types;
    let mut result_log: Vec<PathNode> = Vec::new();
    for(i, node) in path.iter().enumerate(){
        let path_node = PathNode{
            node_key: node.borrow().get_asset_key(),
            asset_name: node.borrow().get_asset_name(),
            path_value: path_values[i].clone(),
            path_identifier: path_value_types[i].clone(),
        };
        println!("{} : {}", node.borrow().get_asset_name(), path_values[i] );
        result_log.push(path_node);
    }

    let json = serde_json::to_string_pretty(&result_log.clone()).unwrap();
    // Get the current timestamp
    let timestamp = chrono::Local::now().format("%Y-%m-%d___%H-%M-%S").to_string();
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let time = chrono::Local::now().format("%H-%M-%S").to_string();

    // Construct the directory path for the current date
    let log_folder_path = format!("target_log_data/{}", date);

    // Create a directory for the current date if it doesn't exist
    match std::fs::create_dir_all(&log_folder_path) {
        Ok(_) => println!("Directory created successfully"),
        Err(e) => println!("Error creating directory: {:?}", e),
    }

    // Construct the file path including the directory
    let log_data_path = format!("{}/{}_{}.json", log_folder_path, target_node.get_asset_name(), time);
    println!("Log data path: {}", log_data_path);
    // let log_data_path = format!("result_log_data/{}_{}.json", start_node.get_asset_name(), timestamp);
    // println!("Log data path: {}", log_data_path);
    // When creating the file, use the log_data_path which includes the directory
    let mut file = File::create(log_data_path).expect("Failed to create file");
    file.write_all(json.as_bytes()).expect("Failed to write data");

    // let log_path = format!("result_log.txt", start_node.get_asset_name(), timestamp);
    let best_path_value = result_log[result_log.len()-1].path_value;
    let result_log_string = format!("{} {} - {}", timestamp, target_node.get_asset_name(), best_path_value);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("result_log.txt")
        .expect("Failed to open or create file");
    writeln!(file, "{}", result_log_string).expect("Failed to write data");

    result_log.clone()
}

pub fn log_results_fallback(path: NodePath) -> Vec<PathNode> {
    let target_node = path[path.len() - 1].borrow();
    let path_values = &target_node.path_values;
    let path_value_types = &target_node.path_value_types;
    let mut result_log: Vec<PathNode> = Vec::new();
    for(i, node) in path.iter().enumerate(){
        let path_node = PathNode{
            node_key: node.borrow().get_asset_key(),
            asset_name: node.borrow().get_asset_name(),
            path_value: path_values[i].clone(),
            path_identifier: path_value_types[i].clone(),
        };
        println!("{} : {}", node.borrow().get_asset_name(), path_values[i] );
        result_log.push(path_node);
    }

    let json = serde_json::to_string_pretty(&result_log.clone()).unwrap();
    // Get the current timestamp
    let timestamp = chrono::Local::now().format("%Y-%m-%d___%H-%M-%S").to_string();
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let time = chrono::Local::now().format("%H-%M-%S").to_string();

    // Construct the directory path for the current date
    let log_folder_path = format!("fallback_log_data/{}", date);

    // Create a directory for the current date if it doesn't exist
    match std::fs::create_dir_all(&log_folder_path) {
        Ok(_) => println!("Directory created successfully"),
        Err(e) => println!("Error creating directory: {:?}", e),
    }

    // Construct the file path including the directory
    let log_data_path = format!("{}/{}_{}.json", log_folder_path, target_node.get_asset_name(), time);
    println!("Log data path: {}", log_data_path);
    // let log_data_path = format!("result_log_data/{}_{}.json", start_node.get_asset_name(), timestamp);
    // println!("Log data path: {}", log_data_path);
    // When creating the file, use the log_data_path which includes the directory
    let mut file = File::create(log_data_path).expect("Failed to create file");
    file.write_all(json.as_bytes()).expect("Failed to write data");

    // let log_path = format!("result_log.txt", start_node.get_asset_name(), timestamp);
    let best_path_value = result_log[result_log.len()-1].path_value;
    let result_log_string = format!("{} {} - {}", timestamp, target_node.get_asset_name(), best_path_value);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("result_log.txt")
        .expect("Failed to open or create file");
    writeln!(file, "{}", result_log_string).expect("Failed to write data");

    result_log.clone()
}

pub fn log_results_small(path: NodePath) -> Vec<PathNode>{
    let start_node = path[0].borrow();
    let path_values = &start_node.path_values;
    let path_value_types = &start_node.path_value_types;
    let mut result_log: Vec<PathNode> = Vec::new();
    for(i, node) in path.iter().enumerate(){
        let path_node = PathNode{
            node_key: node.borrow().get_asset_key(),
            asset_name: node.borrow().get_asset_name(),
            path_value: path_values[i].clone(),
            path_identifier: path_value_types[i].clone(),
        };
        result_log.push(path_node);
    }

    let json = serde_json::to_string_pretty(&result_log.clone()).unwrap();
    // Get the current timestamp
    let timestamp = chrono::Local::now().format("%Y-%m-%d___%H-%M-%S").to_string();
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let time = chrono::Local::now().format("%H-%M-%S").to_string();

    // Construct the directory path for the current date
    let log_folder_path = format!("result_log_data/{}_small", date);

    // Create a directory for the current date if it doesn't exist
    match std::fs::create_dir_all(&log_folder_path) {
        Ok(_) => println!("Directory created successfully"),
        Err(e) => println!("Error creating directory: {:?}", e),
    }

    // Construct the file path including the directory
    let log_data_path = format!("{}/{}_{}.json", log_folder_path, start_node.get_asset_name(), time);
    println!("Log data path: {}", log_data_path);
    // let log_data_path = format!("result_log_data/{}_{}.json", start_node.get_asset_name(), timestamp);
    // println!("Log data path: {}", log_data_path);
    // When creating the file, use the log_data_path which includes the directory
    let mut file = File::create(log_data_path).expect("Failed to create file");
    file.write_all(json.as_bytes()).expect("Failed to write data");

    // let log_path = format!("result_log.txt", start_node.get_asset_name(), timestamp);
    let best_path_value = result_log[result_log.len()-1].path_value;
    let result_log_string = format!("{} {} - {}", timestamp, start_node.get_asset_name(), best_path_value);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("result_log.txt")
        .expect("Failed to open or create file");
    writeln!(file, "{}", result_log_string).expect("Failed to write data");

    result_log.clone()
}
pub fn test_polkadot_assets(){
    
    let asset_registry = AssetRegistry2::build_asset_registry_polkadot();
    asset_registry.display_assets_by_location();

    let liq_pool_registry = LiqPoolRegistry2::build_liqpool_registry_polkadot(&asset_registry);
    liq_pool_registry.display_stable_pools();

    let graph: TokenGraph2 = TokenGraph2::build_graph_2(asset_registry, AdjacencyTable2::build_table_2(&liq_pool_registry));
    let key_1 = "2000{\"NativeAssetId\":{\"Token\":\"DOT\"}}".to_string();
    let input_amount = 1 as f64;
    let (display_string, path) = graph.find_arbitrage_3(key_1, input_amount);
    // display_string
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
pub async fn search_movr() -> String{
    let mut asset_registry = AssetRegistry2::build_asset_registry();
    let lp_registry = LiqPoolRegistry2::build_liqpool_registry(&mut asset_registry);
    let list = AdjacencyTable2::build_table_2(&lp_registry);
    // list.display_table_2();
    let graph = TokenGraph2::build_graph_2(asset_registry, list);
    // graph.display_graph_2();
    let key_1 = "2023\"MOVR\"".to_string();
    let input_amount = 4 as f64;
    let (display_string, path) = graph.find_arbitrage_3(key_1, input_amount);
    display_string
}

pub async fn search_rmrk() -> String{
    let mut asset_registry = AssetRegistry2::build_asset_registry();
    let lp_registry = LiqPoolRegistry2::build_liqpool_registry(&mut asset_registry);
    let list = AdjacencyTable2::build_table_2(&lp_registry);
    // list.display_table_2();
    let graph = TokenGraph2::build_graph_2(asset_registry, list);
    // graph.display_graph_2();
    let key_1 = "2000{\"ForeignAssetId\":\"0\"}".to_string();
    let input_amount = 15 as f64;
    let (display_string, path) = graph.find_arbitrage_3(key_1, input_amount);
    display_string
}

pub fn test_arb_3() {
    let mut asset_registry = AssetRegistry2::build_asset_registry();
    let lp_registry = LiqPoolRegistry2::build_liqpool_registry(&mut asset_registry);
    let list = AdjacencyTable2::build_table_2(&lp_registry);
    // list.display_table_2();
    let graph = TokenGraph2::build_graph_2(asset_registry, list);
    // graph.display_graph_2();
    let key_1 = "2000{\"ForeignAssetId\":\"0\"}".to_string();
    let input_amount = 15 as f64;
    let result = graph.find_arbitrage_3(key_1, input_amount);
    println!("{:?}", result);
}

pub async fn search_ksm() -> (String, Vec<PathNode>){
    let mut asset_registry = AssetRegistry2::build_asset_registry();
    let lp_registry = LiqPoolRegistry2::build_liqpool_registry(&mut asset_registry);
    lp_registry.display_stable_pools();
    let list = AdjacencyTable2::build_table_2(&lp_registry);
    let graph = TokenGraph2::build_graph_2(asset_registry, list);
    let key_1 = "2000{\"NativeAssetId\":{\"Token\":\"KSM\"}}".to_string();
    let input_amount = 1 as f64;
    let (display_string, path) = graph.find_arbitrage_3(key_1, input_amount);
    let loggable_results = log_results(path);
    (display_string, loggable_results)
}

pub async fn test_assets(){
    let mut asset_registry = AssetRegistry2::build_asset_registry();
    asset_registry.display_all_assets();
}
pub async fn search_best_path_a_to_b(start_key: String, destination_key: String, input_amount: f64){
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

    let loggable_results = log_results_target(path);
    println!("{}", display_string);

    for node in loggable_results.iter(){
        println!("{}: {} {}", node.node_key, node.asset_name, node.path_value);
    }
}


pub async fn fallback_search_a_to_b(start_key: String, destination_key: String, input_amount: f64){
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

    let loggable_results = log_results_fallback(path);
    println!("{}", display_string);

    for node in loggable_results.iter(){
        println!("{}: {} {}", node.node_key, node.asset_name, node.path_value);
    }
    // (display_string, loggable_results)
    // "lol".to_string()
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


pub async fn search_ksm_small() -> (String, Vec<PathNode>){
    let mut asset_registry = AssetRegistry2::build_asset_registry();
    let lp_registry = LiqPoolRegistry2::build_liqpool_registry(&mut asset_registry);
    lp_registry.display_stable_pools();
    let list = AdjacencyTable2::build_table_2(&lp_registry);
    let graph = TokenGraph2::build_graph_2(asset_registry, list);
    let key_1 = "2000{\"NativeAssetId\":{\"Token\":\"KSM\"}}".to_string();
    let input_amount = 0.05 as f64;
    let (display_string, path) = graph.find_arbitrage_3(key_1, input_amount);
    let loggable_results = log_results_small(path);
    (display_string, loggable_results)  

    // "lol".to_string()
}

// pub fn test_asset_registry(){
//     let mut asset_registry = AssetRegistry2::build_asset_registry();
//     let lp_registry = LiqPoolRegistry2::build_liqpool_registry(&mut asset_registry);
//     let list = AdjacencyTable2::build_table(&lp_registry);
//     let graph = TokenGraph2::build_graph(asset_registry, list);
//     let input_amount = 15 as f64;
//     let key_0 = "2000{\"NativeAssetId\":{\"Token\":\"KSM\"}}".to_string();
//     let key_1 = "2000{\"ForeignAssetId\":\"0\"}".to_string();
//     let key_2 = r#"2000{NativeAssetId:{"Token":"KUSD"}"#.to_string();
//     graph.find_arbitrage(key_1, input_amount);

// }

pub fn cross_chain(){
    let mut registry = AssetRegistry::build_sub_asset_registry_from_file();
    registry.add_evm_tokens();
    registry.display_registry();
    registry.add_exchange_tokens();
    let mut liq_pool_registry = LiqPoolRegistry::build_sub_liqpool_registry(&registry);
    let adj_table = AdjacencyTable::build_adjacency_table(&liq_pool_registry);
    let token_graph = TokenGraph::build_graph(registry, adj_table);
    let input_amount = 15 as f64;
    let key_1 = r"2000{ForeignAssetId:0}".to_string();
    let key_2 = r#"2000{NativeAssetId:{"Token":"KUSD"}"#.to_string();
    token_graph.find_best_paths_2(key_1, key_2, input_amount);

}

pub fn assets_with_no_pairs(asset_registry: &AssetRegistry, liq_pool_registry: &LiqPoolRegistry){
    let mut all_liq_pool_assets = Vec::new();
    for pool in &liq_pool_registry.liq_pools{
        for asset in &pool.assets{
            let asset_key = asset.borrow().token_data.get_map_key();
            if !all_liq_pool_assets.contains(&asset_key){
                all_liq_pool_assets.push(asset_key)
            }
        }
    }
    let mut lone_assets = Vec::new();
    // let asset_keys = asset_registry.asset_map.keys();
    for asset_bucket in asset_registry.asset_map.values(){
        for asset in asset_bucket{
            let asset_key = asset.borrow().token_data.get_map_key();
            if !all_liq_pool_assets.contains(&asset_key){
                lone_assets.push(asset_key);
            }
        }
    }

    all_liq_pool_assets.sort();
    for asset in &all_liq_pool_assets{
        println!("{}", asset)
    }
    // println!("{}", liq_pool_registry.liq_pools.len());
    println!("{}", all_liq_pool_assets.len());
    // println!("{}", lone_assets.len());

    lone_assets.sort();
    for asset in &lone_assets{
        println!("{}", asset)
    }
    println!("{}", lone_assets.len())

}

