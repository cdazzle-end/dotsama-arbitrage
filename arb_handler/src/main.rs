// use core::num::dec2flt::parse;
// mod token;

use arb_handler::*;
use std::collections::HashMap;
// use asset_registry::AssetRegistry;
use std::fs::File;
use std::io::prelude::*;
use serde_json::{Value};
use std::{str, process};
use std::path::Path;
// use tokio::{join, task};
// mod liq_pool;

// use liq_pool::LiqPool;
#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            // "async_search" => async_search().await,
            "search_best_path_a_to_b" if args.len() == 5 => {
                let key_1 = &args[2];
                let key_2 = &args[3];
                let input_amount: f64 = args[4].parse().expect("Input amount must be a float");
                search_best_path_a_to_b(key_1.to_string(), key_2.to_string(), input_amount).await;
            },
            _ => {
                eprintln!("Error: search_best_path_a_to_b incorrect parameters"); // Write an error message to stderr
                process::exit(1); // Exit with a non-zero status code to indicate failure
            }
        }
    } else {
        println!("No arguments provided. Running default function.");
        async_search().await;
    }
    // async_search().await;
    // cross_chain();
    // test_arb_2()
    // test_asset_registry();


    // async_search().await;
    // let key_2 = "2000{\"NativeAssetId\":{\"Token\":\"KSM\"}}".to_string();
    // let key_1 = "2001{\"Native\":\"BNC\"}".to_string();
    // let input_amount = 1 as f64;
    // search_best_path_a_to_b(key_1, key_2, input_amount).await;
    // test_arb_3()
    // test_table_2();
    // search_rmrk().await;
    // search_movr().await;
    // async_search_2().await; 
    // test_table_2();

}



fn clean_string(s: &str) -> &str{
    //remove brackets
    &s[1..s.len()-1]
}

//Read json from kar_asset_registry file




