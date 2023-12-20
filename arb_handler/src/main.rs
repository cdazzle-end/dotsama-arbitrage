// use core::num::dec2flt::parse;
// mod token;

use arb_handler::*;
use std::collections::HashMap;
// use asset_registry::AssetRegistry;
use std::fs::File;
use std::io::prelude::*;
use serde_json::{Value};
use std::str;
use std::path::Path;
// use tokio::{join, task};
// mod liq_pool;

// use liq_pool::LiqPool;
#[tokio::main]
async fn main() {

    // cross_chain();
    // test_arb_2()
    // test_asset_registry();
    // async_search().await

    search_ksm().await;
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




