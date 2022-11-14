
mod liq_pool;
mod asset_registry;
mod token;
mod adjacency_list;
mod token_node;
use liq_pool::{LiqPool, LiqPoolList};
use token::{Token, AssetKeyType};
use asset_registry::AssetRegistry;
use adjacency_list::AdjacencyList;

// use std::borrow::Borrow;
use std::fs::File;
use std::io::prelude::*;
use serde_json::{Value};
use std::str;
use std::path::Path;

pub fn get_asset_registry() -> AssetRegistry{
    let ar = AssetRegistry::build_asset_registry_from_file();
    for token in &ar.asset_list{
        token.get_asset_key()
    }

    ar
}

pub fn get_liq_pools() -> LiqPoolList{
    LiqPoolList::build_from_json()
}

pub fn calculate_swap(token_a_str: &str, token_b_str: &str, supply_amount: u128){
    let token_a = lookup_token_by_symbol(token_a_str.to_string());
    let token_b = lookup_token_by_symbol(token_b_str.to_string());
}

pub fn find_all_paths(token_a_str: &str, token_b_str: &str){
    let token_a = lookup_token_by_symbol(token_a_str.to_string());
    let token_b = lookup_token_by_symbol(token_b_str.to_string());
    
}

pub fn lookup_token_by_symbol(symbol: String) -> Token {
    let ar = AssetRegistry::build_asset_registry_from_file();
    let token = ar.asset_lookup_by_symbol(symbol).clone();
    println!("{:#?}", token);
    token
}

pub fn test_adj_list(){
    // let mut list = AdjacencyList::new("Token_1".to_string());
    let mut list = AdjacencyList::new_empty();
    list.push_end("Token_1".to_string());
    list.push_end("Token_2".to_string());
    list.push_end("Token_3".to_string());
    list.push_end("Token_4".to_string());
    list.print_items();
    // list.print_test()
    // list.delete("Token_3".to_string());
    // list.print_items();
    list.delete("Token_1".to_string());
    list.print_items();
    list.delete("Token_2".to_string());
    list.print_items();
    list.delete("Token_3".to_string());
    list.delete("Token_4".to_string());
    list.print_items();

    
}