
mod liq_pool;
mod asset_registry;
mod token;
mod adjacency_list;
mod adj_list_node;
mod token_graph;
mod adjacency_list_2;
mod adjacency_list_3;
mod hash_table;
use adjacency_list_2::{ListNode2, NodeData};
use liq_pool::{LiqPool, LiqPoolList};
use token::{Token, AssetKeyType};
use asset_registry::AssetRegistry;
use adjacency_list::AdjacencyList;
use token_graph::{TokenGraph, AdjacencyTable};

// use std::borrow::Borrow;
use std::fs::File;
use std::io::prelude::*;
use serde_json::{Value};
use std::str;
use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;

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
    list.push_end("Token_1".to_string(),(0,0));
    list.push_end("Token_2".to_string(),(0,0));
    list.print_items();
    list.pop_end_2();
    list.pop_end_2();
    list.pop_end_2();
    list.print_items();
    
}

pub fn test_list_2(){
    let mut list: ListNode2<NodeData> = ListNode2::new();
    let mut list2: ListNode2<NodeData> = ListNode2::new();
    let n1 = NodeData::new("one".to_string(), (1,1));
    let n2 = NodeData::new("two".to_string(), (1,1));
    let n3 = NodeData::new("3".to_string(), (1,1));

    let n4 = NodeData::new("4".to_string(), (1,1));
    let n5 = NodeData::new("5".to_string(), (1,1));
    let n6 = NodeData::new("6".to_string(), (1,1));

    list.push(n1);
    list.push(n2);
    list.push(n3);

    list2.push(n4);
    list2.push(n5);
    list2.push(n6);

    println!("{:?}", list2.pop().unwrap());
    println!("{:?}", list2.pop().unwrap());
    println!("{:?}", list2.pop().unwrap());
    // println!("{:?}", list2.pop().unwrap());
    for i in list.clone(){
        println!("{:?}", i)
    }

    for (i,x) in list.into_iter().enumerate(){
        println!("iter2: {}, {:?}", i, x )
    }
}

pub fn build_adj_lists(){
    let pools = get_liq_pools();
    // for pool in pools.liq_pools {
    //     pool.get_pool_tokens();
    // }
    // pools.
    let tablee = AdjacencyTable::new();
    // let adj_table = tablee.build_from_liqpool_list(pools);
    // adj_table.display_table();
    // let adj_lists: Vec<AdjacencyList> = Vec::new();
    // let aleady_added = Vec<
    // for pool in pools.liq_pools{
    //     // pool.display_liq_pool();
    //     pool.get_pool_tokens();
    //     let token_1 = &pool.tokens[0].symbol;
    //     let token_2 = &pool.tokens[1].symbol;
    // }

    // println!("{:?}", pools);
}

pub fn build_graph(){
    let mut graph = TokenGraph::new();
    graph.build_graph();
    graph.print_graph_tokens();
    graph.BFS("ARIS".to_string());
    graph.print_path("ARIS".to_string(), "KINT".to_string())
}

pub fn test_pointer(){
    let mut list = AdjacencyList::new_empty();
    list.push_end("Token_1".to_string(),(0,0));

    let item = list.pop_end();
    let item2 = item.clone();

    let inner = item.unwrap();
    let inner2 = inner.clone();
    // let inn = Rc::clone(&inner);
    let count = Rc::strong_count(&inner);
    println!("RC = {}", count);
    // match item {
    //     Some(inner) => inner.new
    // }
}