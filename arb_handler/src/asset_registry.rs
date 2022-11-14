use std::{path::Path, fs::File, io::Read};
use std::str;
use std::collections::HashMap;
use serde_json::Value;

use crate::token::{Token, AssetKeyType};



pub struct AssetRegistry{
    pub asset_list: Vec<Token>
}

impl AssetRegistry{
    pub fn build_asset_registry_from_file() -> AssetRegistry{
        let path_string = r"..\kar\kar_asset_registry_2";
        let path = Path::new(path_string);
        let mut buf: Vec<u8> = Vec::new();
        let mut file = File::open(path).unwrap_or_else(|err| panic!("problem opening file: Error {:?}", err));
        file.read_to_end(&mut buf).unwrap_or_else(|err| panic!("problem reading activity map into buffer {:?}", err));
        let parsed: Value = serde_json::from_str(str::from_utf8(&buf).unwrap()).unwrap();

        let mut token_list: Vec<Token> = Vec::new();
        // let mut token_map = HashMap::new();
        

        let mut x = 0;
        while x < parsed.as_array().unwrap().len() {
            // println!("{}", parsed[x]);

            let token = Token::new(
                parse_asset_key_type(&parsed[x]["asset_key"][0]),
                parsed[x]["asset_data"]["name"].as_str().unwrap().to_string(),
                parsed[x]["asset_data"]["symbol"].as_str().unwrap().to_string(),
                parsed[x]["asset_data"]["decimals"].as_str().unwrap().parse().unwrap(),
                parsed[x]["asset_data"]["minimalBalance"].as_str().unwrap().replace(",", "").parse().unwrap()
            );
            token_list.push(token);
            // token_map.insert(token.asset_key, token);
            x+=1;
        }
        
        AssetRegistry {asset_list: token_list}
    }

    // pub fn get_token_from_key(&self, key: String) -> {
    //     self.asset_list.
    // }

    // pub fn liq_pool_token_lookup(&self, token_string: String){
    //     //split the string
    //     let string_parts: Vec<&str> = token_string.rsplit(":").collect();
    //     let clean_string_parts = clean_tokens(string_parts);
    //     match clean_string_parts[0].as_str() {
    //         "foreignAsset" => println!("FA"),
    //         "token" => println!("NA"),
    //         "stableAssetPoolToken" => println!("SA"),
    //         _ => println!("No token found")
    //     }
    // }

    //lookup a token using the token type and token id
    pub fn asset_lookup(&self, token_type: String, token_id: String) -> &Token{
        //Convert token type into AssetTypeKey
        let key_to_lookup = match token_type.as_str() {
            "NativeAssetId" => AssetKeyType::NativeAssetId{token: token_id},
            "ForeignAssetId" => AssetKeyType::ForeignAssetId(token_id),
            "StableAssetId" => AssetKeyType::StableAssetId(token_id),
            "Erc20" => AssetKeyType::Erc20(token_id),
            _ => panic!("Error matching asset key type")
        };
        // println!("KEY LOOKUP: {:#?}", key_to_lookup);
        for token in &self.asset_list{
            if key_to_lookup == token.asset_key{
                let matched_token = token;
                return matched_token;
            }
        }
        panic!("Couldnt match token")
    }

    pub fn asset_lookup_by_symbol(&self, symbol: String) -> &Token {
        for token in &self.asset_list{
            if symbol.to_ascii_lowercase() == token.symbol.to_ascii_lowercase(){
                let matched_token = token;
                return matched_token;
            }
        }
        panic!("Couldnt match symbol to token")
    }
}

pub fn clean_tokens(token_string: Vec<&str>) -> Vec<String>{
    let mut c_tokens: Vec<String> = Vec::new();
    let token_type = token_string[0].replace(['{','\\','"'], "");
    let token_id = token_string[1].replace(['}','\\','"'], "");
    c_tokens.push(token_type);
    c_tokens.push(token_id);
    c_tokens
}

fn parse_asset_key_type(value: &Value) -> AssetKeyType{
    let mut keys = value.as_object().unwrap().keys();
    
    match keys.next() {
        Some(x) if x == "NativeAssetId" => AssetKeyType::NativeAssetId{token:value["NativeAssetId"]["Token"].as_str().unwrap().to_string()},
        Some(x) if x == "ForeignAssetId" => AssetKeyType::ForeignAssetId(value["ForeignAssetId"].as_str().unwrap().to_string()),
        Some(x) if x == "StableAssetId" => AssetKeyType::StableAssetId(value["StableAssetId"].as_str().unwrap().to_string()),
        Some(x) if x == "Erc20" => AssetKeyType::Erc20(value["Erc20"].as_str().unwrap().to_string()),
        _ => panic!("Error matching asset key type")
    }
}

