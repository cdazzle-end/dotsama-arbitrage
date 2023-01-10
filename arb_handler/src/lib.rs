
mod liq_pool_registry;
mod asset_registry;
mod token;
// mod adjacency_list;
// mod adj_list_node;
mod token_graph;
// mod adjacency_list_2;
// mod adjacency_list_3;
mod adjacency_table;
// mod hash_table;
// mod evm_reader;
// use adjacency_list_2::{ListNode2, NodeData};
use liq_pool_registry::LiqPoolRegistry;
// use evm_reader::EvmReader;
// use liq_pool::{LiqPool, LiqPoolList};
use token::{Token, AssetKeyType};
use asset_registry::AssetRegistry;
// use adjacency_list::AdjacencyList;
use adjacency_table::{AdjacencyTable};
use token_graph::TokenGraph;
use token_graph::calculate_swap;

// use std::borrow::Borrow;
use std::fs::File;
use std::io::prelude::*;
use serde_json::{Value};
use std::str;
use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;

pub fn build_sub_assets(){
    AssetRegistry::build_sub_asset_registry_from_file();
}

pub fn cross_chain(){
    let mut registry = AssetRegistry::build_sub_asset_registry_from_file();
    registry.add_evm_tokens();
    registry.display_registry();
    // registry.cross_chain_assets();
    
    let mut liq_pool_registry = LiqPoolRegistry::build_sub_liqpool_registry(&registry);
    let adj_table = AdjacencyTable::build_adjacency_table(&liq_pool_registry);
    let token_graph = TokenGraph::build_graph(&registry, adj_table);
    let input_amount = 15 as f64;
    let key_1 = r"2000{ForeignAssetId:0}".to_string();
    let key_2 = r#"2000{NativeAssetId:{"Token":"KUSD"}"#.to_string();
    // calculate_swap(&token_graph, key_1, key_2, input_amount);
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

