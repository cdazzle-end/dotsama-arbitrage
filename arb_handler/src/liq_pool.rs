use std::{str, path::Path, fs::File, io::Read};

use serde_json::Value;

use crate::{asset_registry::AssetRegistry, token::Token};

// use crate::asset_registry::AssetRegistry;

// use crate::asset_registry::AssetRegistry;


pub struct LiqPoolList{
    pub liq_pools: Vec<LiqPool>
}

#[derive(Debug)]
pub struct LiqPool{
    pub tokens: Vec<Token>,
    pub liquidity: Vec<u128>
}

impl LiqPool{
    pub fn new(tokens: Vec<Token>, liquidity: Vec<u128>) -> LiqPool{
        LiqPool {
            tokens,
            liquidity
        }
    }

    //convert liq balance to actual value with correct decimals
    pub fn display_liq_pool(&self){
        let actual_vals = self.get_actual_values();
        println!("TOKEN 1: {:#?}", &self.tokens[0]);
        println!("Actual Value: {}", actual_vals[0]);
        println!("TOKEN 2: {:#?}", &self.tokens[1]);
        println!("Actual Value: {}", actual_vals[1]);
    }

    pub fn get_pool_tokens(&self){
        // let token_1 = &self.tokens[0];
        println!("Token 1: {}", &self.tokens[0].symbol);
        println!("Token 2: {}", &self.tokens[1].symbol)
    }

    pub fn get_actual_values(&self) -> Vec<u128>{
        let mut actual_values: Vec<u128> = Vec::new();

        let token_1 = &self.tokens[0];
        let liq_1 = self.liquidity[0];

        let decimals_1 = token_1.decimals;
        let exponent_1:u128 = 10_u128.pow(decimals_1).try_into().unwrap();
        let actual_value_1 = liq_1 / exponent_1;
        actual_values.push(actual_value_1);

        // println!("ACTUAL LIQ 1 {}", actual_value_1);

        let token_2 = &self.tokens[1];
        let liq_2 = self.liquidity[1];

        let decimals_2 = token_2.decimals;
        let exponent_2:u128 = 10_u128.pow(decimals_2).try_into().unwrap();
        let actual_value_2 = liq_2 / exponent_2;
        actual_values.push(actual_value_2);

        // println!("ACTUAL LIQ 2 {}", actual_value_2);
        actual_values
    }
}

impl LiqPoolList{
    pub fn build_from_json() -> LiqPoolList{
        let parsed_json = get_file_json();

        //Get AssetRegistry so we can access token objects
        let asset_registry = AssetRegistry::build_asset_registry_from_file();

        println!("{:?}", parsed_json);
        //deconstruct pools object(token keys)
        // println!("{:?}", parsed_json["pools"].as_array().unwrap());

        let mut liq_pools: Vec<LiqPool> = Vec::new();
        let mut i = 0;
        while i < parsed_json.as_array().unwrap().len() {
            // println!("{} : {:?}", i, parsed_json[i]["poolAssets"]);
            let pool_assets = &parsed_json[i]["poolAssets"];
            let liquidity_stats = &parsed_json[i]["liquidityStats"];

            
            let clean_token_strings = get_clean_token_strings_from_json(pool_assets);
            // println!("Clean Token String: {:#?}", clean_token_strings);

            let mut tokens: Vec<Token> = Vec::new();
            for clean_token in clean_token_strings{
                let matched_token = token_lookup(clean_token, &asset_registry);
                tokens.push(matched_token);
                
            }
            //Reverse the order of the Tokens. Json serializing gets read end to beginning. Need to reverse that cuz order matters
            let mut reversed_tokens: Vec<Token> = Vec::new();
            reversed_tokens.push(tokens.pop().unwrap());
            reversed_tokens.push(tokens.pop().unwrap());

            let liquidity: Vec<u128> = get_liquidity_values_from_json(liquidity_stats);
            

            let new_liq_pool = LiqPool::new(reversed_tokens, liquidity);
            liq_pools.push(new_liq_pool);
            // new_liq_pool.display_liq_pool();

            i += 1;
        }

        //RETURN LiqPoolList
        LiqPoolList { liq_pools }

    }


}

//read json and return relevant info: token type and token id
//Vector of tokens, the tokens are vectors of type and id
fn get_clean_token_strings_from_json(value: &Value) -> Vec<Vec<String>>{
    let tokens_strings: Vec<&str> = value.as_str().unwrap().rsplit(',').collect();

    // println!("{:#?}", tokens_strings);

    let mut clean_token_strings: Vec<Vec<String>> = Vec::new();
    
    //break token string down into type and id, remove extra chars
    for token in tokens_strings {
        // clean_token_strings.push(token.replace(['[',']'], ""));
        let token_clean = token.replace(['[',']'], "");
        let string_parts: Vec<&str> = token_clean.rsplit(":").collect();
        let token_id = string_parts[0].replace(['}','\\','"'], "");
        let token_type = string_parts[1].replace(['{','\\','"'], "");

        let mut clean_token_parts: Vec<String> = Vec::new();
        clean_token_parts.push(token_type);
        clean_token_parts.push(token_id);

        clean_token_strings.push(clean_token_parts);
        // token_lookup(token_type, token_id);
    }

    clean_token_strings
    // println!("{:#?}", clean_token_strings);

    // let asset_registry = AssetRegistry::build_asset_registry_from_file();

}

pub fn get_liquidity_values_from_json(value: &Value) -> Vec<u128>{
    let mut liq_values: Vec<u128> = Vec::new();
    let vals = value.as_array().unwrap();
    
    // println!("LIQ VALS: {:?}", vals);
    for val in vals{
        // println!("VAL: {}", val);
        let parsed_value: u128 = val.as_str().unwrap().replace(",", "").parse().unwrap();
        // println!("PARSED VAL: {}", parsed_value);
        liq_values.push(parsed_value);
    }
    liq_values
}

//Lookup token from asset registry with token type and id
fn token_lookup(clean_token: Vec<String>, registry: &AssetRegistry) -> Token{
    let token_type = clean_token[0].as_str();
    let token_id = clean_token[1].as_str();
    
    //Convert token_type to asset_registry compatible format
    let token_type_key = match token_type {
            "token" => "NativeAssetId",
            "foreignAsset" => "ForeignAssetId",
            "stableAssetPoolToken" => "StableAssetId",
            _ => panic!("Failed token lookup: err identifying token type")
        };
    
    //Find and return matching token
    let matched_token = registry.asset_lookup(token_type_key.to_string(), token_id.to_string());
    // println!("MATCHED TOKEN {:#?}", matched_token);
    matched_token.clone()
}

fn get_file_json() -> Value{
    let path = Path::new(r"..\kar\liqpools.txt");
    let mut buf: Vec<u8> = Vec::new();
    let mut file = File::open(path).unwrap_or_else(|err| panic!("problem opening file: Error {:?}", err));
    file.read_to_end(&mut buf).unwrap_or_else(|err| panic!("problem reading activity map into buffer {:?}", err));

    //return parsed json value
    serde_json::from_str(str::from_utf8(&buf).unwrap()).unwrap()
}