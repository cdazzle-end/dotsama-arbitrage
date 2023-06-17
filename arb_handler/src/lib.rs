
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PathNode{
    pub node_key: String,
    pub asset_name: String,
    pub path_value: f64,
}

pub fn build_sub_assets(){
    AssetRegistry::build_sub_asset_registry_from_file();
}

pub async fn async_search(){
    let future1 = task::spawn(search_rmrk());
    let future2 = task::spawn(search_ksm());
    let future3 = task::spawn(search_movr());
    let (result1, result2, result3) = join!(future1, future2, future3);
    // future1
    let (ksm_display, ksm_log) = result2.unwrap();
    println!();
    println!("------------------------------------");
    println!("RESULTS");
    println!("------------------------------------");
    println!("Result of function 1: {:?}", result1);
    println!("------------------------------------");
    println!("Result of function 2: {:?}", ksm_display);
    println!("------------------------------------");
    println!("Result of function 3: {:?}", result3);

    for node in ksm_log{
        println!("{}: {} {}", node.node_key, node.asset_name, node.path_value);
    }
}

pub fn log_results(path: NodePath) -> Vec<PathNode>{
    let start_node = path[0].borrow();
    let path_values = &start_node.path_values;
    let mut result_log: Vec<PathNode> = Vec::new();
    for(i, node) in path.iter().enumerate(){
        let path_node = PathNode{
            node_key: node.borrow().get_asset_key(),
            asset_name: node.borrow().get_asset_name(),
            path_value: path_values[i].clone(),
        };
        result_log.push(path_node);
    }

    let json = serde_json::to_string_pretty(&result_log.clone()).unwrap();
    // Get the current timestamp
    let timestamp = chrono::Local::now().format("%Y-%m-%d___%H-%M-%S").to_string();
    let log_data_path = format!("result_log_data/{}_{}.json", start_node.get_asset_name(), timestamp);
    println!("Log data path: {}", log_data_path);
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
    // for node in result_log{
    //     println!("{}: {} {}", node.node_key, node.asset_name, node.path_value);
    // }
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

