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
use num::{BigInt, BigUint, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, FromPrimitive, Num, One, Signed, ToPrimitive, Zero};
use bigdecimal::{BigDecimal};
use std::str::FromStr;
// use tokio::{join, task};
// mod liq_pool;

// use liq_pool::LiqPool;
// cargo run search_best_path_a_to_b "2001{\`"Native\`":\`"BNC\`"}" "2000{\`"NativeAssetId\`":{\`"Token\`":\`"KSM\`"}}" 10
// cargo run search_best_path_a_to_b "2000{\`"NativeAssetId\`":{\`"Token\`":\`"KSM\`"}}" "2000{\`"NativeAssetId\`":{\`"Token\`":\`"KSM\`"}}" 1
// cargo run search_best_path_a_to_b_polkadot "2000{\`"NativeAssetId\`":{\`"Token\`":\`"DOT\`"}}" "2000{\`"NativeAssetId\`":{\`"Token\`":\`"DOT\`"}}" 1
// cargo run fallback_search_a_to_b_polkadot "2034\`"102\`"" "2000{\`"NativeAssetId\`":{\`"Token\`":\`"DOT\`"}}" 2.404927102023512903
//     let key_1 = "2000{\"ForeignAssetId\":\"0\"}".to_string();
//     let key_1 = "2023\"MOVR\"".to_string();
//     let key_1 = "2000{\"NativeAssetId\":{\"Token\":\"KSM\"}}".to_string();

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            // "async_search" => async_search().await,
            "search_best_path_a_to_b_kusama" if args.len() == 5 => {
                let key_1 = &args[2];
                let key_2 = &args[3];
                let input_amount_str = &args[4];
                let input_amount_bd = BigDecimal::from_str(input_amount_str)
                    .expect("Input amount must be a valid number");
                async_search_best_path_a_to_b(key_1.to_string(), key_2.to_string(), input_amount_bd, "kusama".to_string()).await;
            },
            "search_best_path_a_to_b_polkadot" if args.len() == 5 => {
                let key_1 = &args[2];
                let key_2 = &args[3];
                let input_amount_str = &args[4];
                let input_amount_bd = BigDecimal::from_str(input_amount_str)
                    .expect("Input amount must be a valid number");
                async_search_best_path_a_to_b(key_1.to_string(), key_2.to_string(), input_amount_bd, "polkadot".to_string()).await;
            },
            "fallback_search_a_to_b_kusama" if args.len() == 5 => {
                let key_1 = &args[2];
                let key_2 = &args[3];
                let input_amount_str = &args[4];
                let input_amount_bd = BigDecimal::from_str(input_amount_str)
                    .expect("Input amount must be a valid number");
                fallback_search_a_to_b(key_1.to_string(), key_2.to_string(), input_amount_bd, "kusama".to_string()).await;
            },
            "fallback_search_a_to_b_polkadot" if args.len() == 5 => {
                let key_1 = &args[2];
                let key_2 = &args[3];
                // let input_amount = &args[4];
                let input_amount_str = &args[4];
                let input_amount_bd = BigDecimal::from_str(input_amount_str)
                    .expect("Input amount must be a valid number");

                // // TEMP FIX *** Until adjust all functions to take BigDecimal and log path values as strings, just round down the float to ensure that the values don't exceed our token balance
                // let mut input_amount_float: f64 = input_amount_str.parse().expect("Input amount must be a float");
                // let input_amount_float_bd = BigDecimal::from_f64(input_amount_float).unwrap();

                // // Parse the input amount as BigDecimal
                // let input_amount_bd = BigDecimal::from_str(input_amount_str)
                //     .expect("Input amount must be a valid number");

                // if input_amount_float_bd.gt(&input_amount_bd){
                //     let input_amount_float_str = input_amount_float.to_string();
                //     println!("Rounded up, need to subtract one");
                //     let decimal_places = if let Some(pos) = input_amount_float_str.find('.') {
                //         input_amount_float_str.len() - pos - 1
                //     } else {
                //         0
                //     };

                //     println!("Float decimal places: {:?}", decimal_places);
                //     let adjustment = BigDecimal::from_str(&format!("1e-{}", decimal_places)).unwrap();

                //     let adjusted_float_value = input_amount_float_bd - adjustment;
                //     println!("Adjusted float value: {:?}", adjusted_float_value);

                //     input_amount_float = adjusted_float_value.to_f64().unwrap();

                // } else {
                //     println!("Rounded down, no need to subtract one");
                // }
                // println!("Final float value: {:?}", input_amount_float);
                // *************************************************************

                fallback_search_a_to_b(key_1.to_string(), key_2.to_string(), input_amount_bd, "polkadot".to_string()).await;
            },
            "search_kusama" => {
                let asset_key = "2000{\"NativeAssetId\":{\"Token\":\"DOT\"}}".to_string();
                async_search_default_kusama().await;
            },
            "search_polkadot" => {
                let asset_key = "2000{\"NativeAssetId\":{\"Token\":\"DOT\"}}".to_string();
                async_search_default_polkadot().await;
            },
            "search_polkadot_sync" => {
                println!("Running polkadot search SYNC. One by one");
                let asset_key = "2000{\"NativeAssetId\":{\"Token\":\"DOT\"}}".to_string();
                sync_search_default_polkadot();
            },
            "p_1" => {
                let asset_key = "2000{\"NativeAssetId\":{\"Token\":\"DOT\"}}".to_string();
                search_best_path_a_to_b_async_polkadot(asset_key.clone(), asset_key, BigDecimal::from(1)).await;
            },
            "test" => {
                let asset_key = "2000{\"NativeAssetId\":{\"Token\":\"DOT\"}}".to_string();
                test_stable_swap().await;
            },
            _ => {
                eprintln!("Error: search_best_path_a_to_b incorrect parameters"); // Write an error message to stderr
                process::exit(1); // Exit with a non-zero status code to indicate failure
            }
        }
    } else {
        println!("No arguments provided. Running default function.");
        async_search_default_kusama().await;
    }

}
// #[tokio::main]
// async fn main(){
//     let asset_key = "2000{\"NativeAssetId\":{\"Token\":\"KSM\"}}".to_string();
//     let polkadot_assets = test_polkadot_assets();
// }


fn clean_string(s: &str) -> &str{
    //remove brackets
    &s[1..s.len()-1]
}

//Read json from kar_asset_registry file




