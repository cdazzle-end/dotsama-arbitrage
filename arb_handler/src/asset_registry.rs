use std::cell::RefCell;
use std::rc::Rc;
// use core::num::dec2flt::parse;
use std::hash::{Hasher, Hash};
use std::{path::Path, fs::File, io::Read};
use std::str;
use std::collections::HashMap;
use serde_json::Value;
// use std::iter::Map;

// use crate::evm_reader::EvmReader;
use crate::token::{Token, AssetKeyType, TokenData};
// use crate::liq_pool::LiqPool;
type AssetPointer = Rc<RefCell<Asset>>;

#[derive(Debug)]
pub struct AssetRegistry{
    // pub asset_list: Vec<Asset>, //SHOULDNT NEED ASSET LIST, USE ASSET MAP
    pub asset_map: HashMap<String, Vec<AssetPointer>>, //ASSET KEY: CHAIN_ID + LOCAL_ID
    pub location_map: HashMap<AssetLocation, Vec<AssetPointer>>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asset{
    pub token_data: TokenData,
    pub asset_location: Option<AssetLocation>,
}
impl Asset{
    pub fn new(token_data: TokenData, asset_location: Option<AssetLocation>) -> Asset{
        Asset{token_data, asset_location}
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct AssetLocation{
    pub here: bool,
    pub xtype: String,
    pub properties: Option<Vec<String>>,
}
impl AssetLocation{
    pub fn new(here: bool, xtype: String, properties: Option<Vec<String>>) -> AssetLocation{
        AssetLocation{
            here, xtype, properties
        }
    }
}


impl AssetRegistry{
// "kar", "bnc", "movr",
    pub fn build_sub_asset_registry_from_file() -> AssetRegistry{
        let chains = vec![ "kar", "bnc", "movr", "heiko"];
        let mut parsed_files = Vec::new();
        for chain in chains{
            let path_string = r"..\".to_owned() + chain + r"\asset_registry";
            let mut buf: Vec<u8> = Vec::new();
            let mut file = File::open(Path::new(&path_string)).unwrap_or_else(|err| panic!("problem opening file: Error {:?}", err));
            file.read_to_end(&mut buf).unwrap_or_else(|err| panic!("problem reading activity map into buffer {:?}", err));
            let parsed: Value = serde_json::from_str(str::from_utf8(&buf).unwrap()).unwrap();
            parsed_files.push(parsed);
        }
        // let mut token_list: Vec<Token> = Vec::new();
        let mut asset_list: Vec<Asset> = Vec::new();
        let mut asset_map: HashMap<String, Vec<AssetPointer>> = HashMap::new();
        let mut asset_location_map = HashMap::new();
        for parsed in parsed_files{
            // println!("{:?}", parsed);
            let asset_reg_objects = parsed.as_object().unwrap()["assetRegistryObjects"].as_array().unwrap();
            let chain_id = parsed.as_object().unwrap()["parachainId"].as_str();
            let mut x = 0;
            while x < asset_reg_objects.len() {
                // println!("{}", asset_reg_objects[x]);
                let asset = asset_reg_objects[x]["asset"].as_object().unwrap();
                let asset_location = asset_reg_objects[x]["assetLocation"].as_object().unwrap();
                // println!("{:?}", chain_id);
                let token;
                if chain_id.unwrap().to_string() == "2,023".to_string(){
                    token = TokenData::new_evm(
                    asset["evmAddress"].as_str().unwrap().to_string().to_ascii_lowercase(),
                    asset["name"].as_str().unwrap().to_string(),
                    asset["symbol"].as_str().unwrap().to_string(),
                    asset["decimals"].as_str().unwrap().parse().unwrap(),
                    chain_id.unwrap().to_string().replace(",", ""),
                    true,
                    Some(asset["localId"].to_string())
                );
                } else if chain_id.unwrap().to_string() == "2,085" {
                    token = TokenData::HeikoToken { 
                        local_id: asset["localId"].as_str().unwrap().to_string(),
                        name: asset["name"].as_str().unwrap().to_string(),
                        symbol: asset["symbol"].as_str().unwrap().to_string(),
                        decimals: asset["decimals"].as_str().unwrap().parse().unwrap(),
                        deposit: asset["deposit"].as_str().unwrap().parse().unwrap(),
                        isFrozen: false,
                        chain: chain_id.unwrap().to_string().replace(",", ""),
                     }

                }else{
                    token = TokenData::new_sub(
                    parse_asset_key_type(&asset["localId"]),
                    asset["name"].as_str().unwrap().to_string(),
                    asset["symbol"].as_str().unwrap().to_string(),
                    asset["decimals"].as_str().unwrap().parse().unwrap(),
                    asset["minimalBalance"].as_str().unwrap().replace(",", "").parse().unwrap(),
                    chain_id.unwrap().to_string().replace(",", ""),
                    None
                    // asset["evmAddress"].as_str()
                );
                
                }
                println!("{:?}", token);
                

                let location = &asset_location["location"];
                let mut here = false;
                // let mut xtype;
                if location.is_object(){
                    let location = asset_location["location"].as_object().unwrap();
                    let keys = location.keys();
                    let mut properties: Vec<String> = Vec::new();
                    let mut xtype = location["xtype"].as_str().unwrap();
                    for k in keys.rev(){
                        if k != "xtype"{
                            let property = k.as_str().to_string() + ":" + &location[k].as_str().unwrap().replace(',', "");
                            properties.push(property);
                        }
                    }
                    properties.sort();
                    let map_key = token.get_map_key();
                    let asset_location = AssetLocation::new(here, xtype.to_string(), Some(properties));
                    let current_asset = Asset::new(token, Some(asset_location.clone()));
                    let asset_pointer = Rc::new(RefCell::new(current_asset));

                    asset_location_map.entry(asset_location.clone()).or_insert(Vec::new()).push(Rc::clone(&asset_pointer));
                    asset_map.entry(map_key).or_insert(Vec::new()).push(asset_pointer);
                    // asset_list.push(current_asset);
                } else {
                    //LOCATION IS HERE
                    here = true;
                    // let xtype = "X0";
                    let asset_location = AssetLocation::new(here, "X0".to_string(), None);
                    let map_key = token.clone().get_map_key();
                    let current_asset = Asset::new(token, Some(asset_location.clone()));
                    let asset_pointer = Rc::new(RefCell::new(current_asset));

                    asset_location_map.entry(asset_location.clone()).or_insert(Vec::new()).push(Rc::clone(&asset_pointer));
                    asset_map.entry(map_key).or_insert(Vec::new()).push(asset_pointer);
                    // asset_list.push(current_asset);
                }
                
                x+=1;
            }
        }
        
        // asset_location_map.insert(k, v)
        
        println!("map: {:?}", asset_map.values().len());
        // println!("{:?}", chain_id);
        
        AssetRegistry {asset_map: asset_map, location_map: asset_location_map}
    }

    //Get assets with that share the same XCM location of a specific asset
    pub fn get_assets_at_location(&self, asset_location: AssetLocation) -> Vec<AssetPointer>{
        let location_bucket = &self.location_map.get(&asset_location);
        let mut location_assets = Vec::new();
        match location_bucket{
            Some(bucket) => {
                for asset in bucket.iter(){
                    if Some(&asset_location) == asset.borrow().asset_location.as_ref(){
                        location_assets.push(Rc::clone(&asset));
                    }
                }
            },
            None => (),
        }
        location_assets
    }

    //Lookup asset in Hashmap 
    pub fn asset_map_lookup(&self, chain_input: String, id: String) -> Rc<RefCell<Asset>>{
        let map_key = chain_input.clone() + &id;
        let asset_bucket = self.asset_map.get(&map_key).unwrap();
        for asset in asset_bucket{
            match asset.borrow().token_data.clone(){
                TokenData::SubToken { local_id, chain,.. } => {
                    if chain_input == chain && id == local_id.get_key_string(){
                        // kl
                        return Rc::clone(asset)
                        // return asset
                    }
                },
                TokenData::EvmToken { contract_address , chain, ..} => {
                    if chain_input == chain && id == contract_address {
                        return Rc::clone(asset)
                    }
                },
                TokenData::KucoinToken { exchange, symbol, .. } => {
                    panic!("trying to lookup kucoin asset incorrectly")
                },
                TokenData::HeikoToken{local_id, chain, .. } => {
                    if chain_input == chain && id == local_id{
                        // kl
                        return Rc::clone(asset)
                        // return asset
                    }
                }
            };
            // let asset_id = asset.token_data.get_local_id_evm()
        }
        panic!("Couldnt find asset in map");
    }

    pub fn add_exchange_tokens(&mut self){
        let path_string = r"..\kucoin\exchange_data";
        // let mut parsed_files = Vec::new();
        let mut buf: Vec<u8> = Vec::new();
        let mut file = File::open(Path::new(&path_string)).unwrap_or_else(|err| panic!("problem opening file: Error {:?}", err));
        file.read_to_end(&mut buf).unwrap_or_else(|err| panic!("problem reading activity map into buffer {:?}", err));
        let parsed: Value = serde_json::from_str(str::from_utf8(&buf).unwrap()).unwrap();
        // parsed_files.push(parsed);

        let assets = parsed.as_array().unwrap();
            for asset in assets{
                // println!("FILE: {:?}", asset);
                let asset_value = asset.as_object().unwrap();
                let price_data = asset_value["price"].as_array().unwrap();
                let price_data = (price_data[0].as_u64().unwrap(), price_data[1].as_u64().unwrap());
                let price_decimals = asset_value["price_decimals"].as_array().unwrap();
                let price_decimals = (price_decimals[0].as_u64().unwrap(), price_decimals[1].as_u64().unwrap());
                let chain = asset_value["chain"].as_str().unwrap().to_string();
                // let evm_address = asset_value["address"].as_str().unwrap().to_string().to_ascii_lowercase();
                // let mut is_cross_chain = false;
                // let mut local_id = None;
                
                let token = TokenData::new_kucoin(
                    "kucoin".to_string(), 
                    asset_value["name"].as_str().unwrap().to_string(), 
                    asset_value["assetTicker"].as_str().unwrap().to_string(), 
                    asset_value["chain"].as_str().unwrap().to_string(), 
                    asset_value["precision"].as_u64().unwrap() as u32, 
                    asset_value["contractAddress"].as_str().unwrap().to_string(), 
                    price_data,
                    price_decimals
                );
                println!("{:?}", token);

                // self.asset_list.push(Asset::new(token.clone(), None));

                let map_key = token.clone().get_map_key();
                // let asset_pointer = Rc::new(RefCell::new(Asset::new(token.clone(), None)));
                let asset_location_map = &mut self.location_map;
                let asset_map = &mut self.asset_map;

                //Handle case for USDT token
                if chain == "None"{
                    let map_key = token.get_map_key();
                    // let asset_location = AssetLocation::new(here, xtype.to_string(), Some(properties));
                    let new_asset = Asset::new(token, None);
                    let asset_pointer = Rc::new(RefCell::new(new_asset));
                    asset_map.entry(map_key).or_insert(Vec::new()).push(asset_pointer);
                } else{
                    let asset_location_data = asset_value["assetLocation"].as_object().unwrap();
                    let mut property_data = Vec::new();
                    let property_data_option: Option<Vec<String>>;
                    for property in asset_location_data["properties"].as_array().unwrap(){
                        property_data.push(property.as_str().unwrap().to_string());
                    }
                    if property_data[0] == "None"{
                        property_data_option = None;
                    } else {
                        property_data_option = Some(property_data);
                    }
                    let asset_location = AssetLocation::new(
                        asset_location_data["here"].as_bool().unwrap(),
                        asset_location_data["xtype"].as_str().unwrap().to_string(),
                        property_data_option
                    );
                    

                    let map_key = token.get_map_key();
                    // let asset_location = AssetLocation::new(here, xtype.to_string(), Some(properties));
                    let new_asset = Asset::new(token, Some(asset_location.clone()));
                    let asset_pointer = Rc::new(RefCell::new(new_asset));

                    

                    asset_location_map.entry(asset_location.clone()).or_insert(Vec::new()).push(Rc::clone(&asset_pointer));
                    asset_map.entry(map_key).or_insert(Vec::new()).push(asset_pointer);
                }
            
            }

            // let usdt_token = TokenData::new_kucoin(
            //     "kucoin",
            //     "USDT", symbol, chain, precision, contract_address, price_data, price_decimals)
    }

    pub fn display_exchange_tokens(&self){
        for asset_bucket in self.asset_map.values(){
            for asset in asset_bucket{
                match asset.borrow().token_data{
                    TokenData::KucoinToken{..} => {
                        println!("{:?}", asset.borrow());
                        if asset.borrow().asset_location != None{
                            let related_assets = self.get_assets_at_location(asset.borrow().asset_location.clone().unwrap());
                            println!("Related assets:");
                            for relative in related_assets{
                                println!("{:?}", relative.borrow().token_data)
                            }
                        }
                    },
                    _ => ()
                }
            }

        }
    }

    pub fn get_kucoin_tokens(&self) -> Vec<AssetPointer>{
        let mut kucoin_assets = Vec::new();
        for asset_bucket in self.asset_map.values(){
            for asset in asset_bucket{
                match asset.borrow().token_data{
                    TokenData::KucoinToken{..} => {
                        kucoin_assets.push(Rc::clone(asset));
                    },
                    _ => ()
                }
            }

        }
        kucoin_assets
    }

    pub fn get_kucoin_usdt(&self) -> AssetPointer{
        for asset_bucket in self.asset_map.values(){
            for asset in asset_bucket{
                match &asset.borrow().token_data{
                    TokenData::KucoinToken{symbol, ..} => {
                        if symbol == "USDT"{
                            return Rc::clone(&asset)
                        }
                    },
                    _ => ()
                }
            }

        }
        panic!("Could not find kucoin usdt")
    }

    pub fn get_kucoin_asset_decimals(&self, asset_location: AssetLocation) -> u64 {
        let related_assets = self.get_assets_at_location(asset_location);
        for asset in related_assets{
            if !asset.borrow().token_data.is_exchange_token(){
                return asset.borrow().token_data.get_asset_decimals()
            }
        }
        panic!("Could not find decimals for kucoin asset")
    }

    pub fn add_evm_tokens(&mut self){
        let chains = vec!["movr"];
        let evm_chain_ids = vec!["2023"];
        let mut parsed_files = Vec::new();
        for chain in chains{
            let path_string = r"..\".to_owned() + chain + r"\token_registry";
            let mut buf: Vec<u8> = Vec::new();
            let mut file = File::open(Path::new(&path_string)).unwrap_or_else(|err| panic!("problem opening file: Error {:?}", err));
            file.read_to_end(&mut buf).unwrap_or_else(|err| panic!("problem reading activity map into buffer {:?}", err));
            let parsed: Value = serde_json::from_str(str::from_utf8(&buf).unwrap()).unwrap();
            parsed_files.push(parsed);
        }

        let mut asset_list: Vec<Asset> = Vec::new();
        let chain_id = "2023".to_string();

        //List of substrate tokens that have EVM counterpart
        let mut cross_chain_assets = Vec::new();
        for bucket in self.asset_map.values(){
            for asset in bucket{
                if evm_chain_ids.contains(&asset.borrow().token_data.get_chain().as_str()){
                    cross_chain_assets.push(asset.borrow().token_data.get_contract_address().unwrap())
                }
            }
        }
        // let mut asset_location_map = HashMap::new();
        for parsed in parsed_files{
            // println!("{:?}", parsed);
            let assets = parsed.as_array().unwrap();
            for asset in assets{
                let asset_value = asset.as_object().unwrap();
                let evm_address = asset_value["address"].as_str().unwrap().to_string().to_ascii_lowercase();
                // let mut is_cross_chain = false;
                // let mut local_id = None;
                if !cross_chain_assets.contains(&evm_address) {
                    let token = TokenData::new_evm(
                    evm_address, 
                    asset_value["name"].as_str().unwrap().to_string(), 
                    asset_value["symbol"].as_str().unwrap().to_string(), 
                    asset_value["decimals"].as_u64().unwrap(), 
                    chain_id.clone(), 
                    false, 
                    None
                    );

                    // self.asset_list.push(Asset::new(token.clone(), None));

                    let map_key = token.clone().get_map_key();
                    let asset_pointer = Rc::new(RefCell::new(Asset::new(token, None)));
                    let asset_vec = self.asset_map.entry(map_key).or_insert(Vec::new());
                    asset_vec.push(asset_pointer);
                }
            }
            
        }
    }
    pub fn display_registry(&self){
        let mut vcount = 0;
        let mut assets = Vec::new();
        for(key,value) in &self.asset_map{
            // print!("Key: {:?} --", key);
            for val in value{
                // print!(" Value: {}", val.borrow().token_data.get_map_key());
                let display_string = val.borrow().token_data.get_map_key() + " " + &val.borrow().token_data.get_asset_name();
                assets.push(display_string);
                vcount += value.len();
            }
        }
        assets.sort();
        for key in assets{
            println!("Registry value: {}", key)
        }
        println!("{}", &self.asset_map.keys().len());
        println!("{}", vcount);
    }

    pub fn cross_chain_assets(&self){
        let map = &self.location_map;

        let mut vcount = 0;
        for(key,values) in map{
            println!("Key: {:?}", key);
            for val in values{
                println!("Value: {:?}", val);
                vcount += 1;
            }
            println!("")
        }
        println!("{:?}", self.location_map.keys().len());
        println!("{:?}", vcount)
    }

    pub fn get_all_assets(&self) -> Vec<AssetPointer>{
        let mut all_assets = Vec::new();
        for bucket in self.asset_map.values(){
            for asset in bucket{
                all_assets.push(Rc::clone(&asset));
            }
        }
        all_assets
    }

    //Get list of all evm sub tokens addresses
    pub fn get_substrate_evm_tokens(&self) -> Vec<String> {
        let mut evm_token_addresses = Vec::new();
        for bucket in self.asset_map.values(){
            for asset in bucket{
                if asset.borrow().token_data.is_evm_sub_token(){
                    evm_token_addresses.push(asset.borrow().token_data.get_contract_address().unwrap());
                }
            }
        }
        evm_token_addresses
    }

    pub fn save_asset_registry(&self){
        let path_string = r"..\transactions\multi_asset_data";
        // let mut parsed_files = Vec::new();
        let mut buf: Vec<u8> = Vec::new();
        let mut file = File::create(path_string);

        let map = &self.location_map;

        let mut vcount = 0;
        for(key,values) in map{
            // println!("Key: {:?}", key);
            println!("Asset Location: {}: {:?}", key.xtype, key.properties.clone().unwrap_or(vec![key.here.to_string()]));
            for val in values{
                println!("{:?} {:?}", val.borrow().token_data.get_chain(), val.borrow().token_data.get_asset_name());
                vcount += 1;
            }
            println!("")
        }

    }

    // pub fn get_assets_from_location(&self,  location: AssetLocation) -> Vec<AssetPointer> {

    // }
}

pub fn clean_tokens(token_string: Vec<&str>) -> Vec<String>{
    let mut c_tokens: Vec<String> = Vec::new();
    let token_type = token_string[0].replace(['{','\\','"'], "");
    let token_id = token_string[1].replace(['}','\\','"'], "");
    c_tokens.push(token_type);
    c_tokens.push(token_id);
    c_tokens
}

pub fn remove_comma(string: String) -> String{
    string.replace(',', "")
}

fn parse_asset_key_type(value: &Value) -> AssetKeyType{
    let mut keys = value.as_object().unwrap().keys();
    
    match keys.next() {
        Some(x) if x == "NativeAssetId" => {
            let native_id = value["NativeAssetId"].as_object().unwrap();
            let native_key = native_id.keys().next().unwrap();
            let native_id_value = native_id.get_key_value(native_key).unwrap().1;
            let native_string = "\"".to_string() + &native_key.to_string() + "\":" + &native_id_value.to_string();
            // println!("{}", native_string);
            AssetKeyType::NativeAssetId(native_string)
        },
        Some(x) if x == "ForeignAssetId" => AssetKeyType::ForeignAssetId(value["ForeignAssetId"].as_str().unwrap().to_string()),
        Some(x) if x == "StableAssetId" => AssetKeyType::StableAssetId(value["StableAssetId"].as_str().unwrap().to_string()),
        Some(x) if x == "Erc20" => AssetKeyType::Erc20(value["Erc20"].as_str().unwrap().to_string()),
        _ => panic!("Error matching asset key type")
    }
}

