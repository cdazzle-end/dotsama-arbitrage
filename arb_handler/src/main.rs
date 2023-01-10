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
// mod liq_pool;

// use liq_pool::LiqPool;

fn main() {

    cross_chain();

}



fn clean_string(s: &str) -> &str{
    //remove brackets
    &s[1..s.len()-1]
}

//Read json from kar_asset_registry file




