// use core::num::dec2flt::parse;
// mod token;

use arb_handler::*;
// use asset_registry::AssetRegistry;
use std::fs::File;
use std::io::prelude::*;
use serde_json::{Value};
use std::str;
use std::path::Path;
// mod liq_pool;

// use liq_pool::LiqPool;

fn main() {
    
    // get_liq_pools();
    // get_asset_registry();
    // lookup_token_by_symbol("rMRK".to_string());

    test_adj_list();
    // println!("teest")

}



fn clean_string(s: &str) -> &str{
    //remove brackets
    &s[1..s.len()-1]
}

//Read json from kar_asset_registry file




