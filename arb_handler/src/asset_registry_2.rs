use std::cell::RefCell;
use std::rc::Rc;
use std::hash::{Hasher, Hash};
use std::{path::Path, fs::File, io::Read};
use std::str;
use std::io;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde::de::{Deserializer, Error, Visitor};
type AssetPointer = Rc<RefCell<Asset>>;

#[derive(Debug)]
pub struct AssetRegistry2{
    pub asset_map: HashMap<String, Vec<AssetPointer>>,
    pub asset_location_map: HashMap<AssetLocation, Vec<AssetPointer>>
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Asset{
    pub token_data: TokenData,
    pub asset_location: Option<AssetLocation>,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, Deserialize, Serialize)]
pub struct AssetLocation{
    pub here: bool,
    pub xtype: Option<String>,
    pub properties: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub enum TokenData{
    MyAsset(MyAssetJson),
    CexAsset(CexAssetJson),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct MyAssetJson {
    network: String,
    chain: u64,
    localId: serde_json::Value,
    name: String,
    symbol: String,
    decimals: String,
    minimalBalance: Option<String>,
    isFrozen: Option<bool>,
    deposit: Option<String>,
    contractAddress: Option<String>,
}
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct CexAssetJson{
    exchange: String,
    assetTicker: String,
    name: String,
    chain: String,
    precision: u64,
    contractAddress: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MyAssetRegistryObject {
    tokenData: TokenData,
    hasLocation: bool,
    tokenLocation: Option<serde_json::Value>,
}
// vec!["kar", "bnc", "movr", "hko", "sdn"];
// New asset registry files for statemine, crust, kintsugi...
impl AssetRegistry2{
    pub fn build_asset_registry() -> AssetRegistry2{
        let chains = vec!["kar", "bnc_kusama", "movr", "hko", "mgx", "bsx", "other_kusama"];
        let parsed_files = chains
            .into_iter()
            .map(|chain| {
                // let path_string = format!("../assets/{}/asset_registry.json", chain);
                let path_string = format!("../../../polkadot_assets/assets/asset_registry/{}_assets.json", chain);
                let path = Path::new(&path_string);
                let mut buf = vec![];
                let mut file = File::open(path)?;
                file.read_to_end(&mut buf)?;
                let parsed = serde_json::from_str(str::from_utf8(&buf).unwrap())?;
                Ok(parsed)
            })
            .collect::<Result<Vec<Value>, io::Error>>()
            .unwrap();

        
        let path = Path::new("../assets/ignore_list.json");
        let mut buf = vec![];
        let mut file = File::open(path).unwrap();
        file.read_to_end(&mut buf).unwrap();
        let parsed_ignore_file: Value = serde_json::from_str(str::from_utf8(&buf).unwrap()).unwrap();
        let ignore_list_assets: Vec<MyAssetRegistryObject> = serde_json::from_value(parsed_ignore_file).unwrap();
        let ignore_list_locations: Vec<String> = ignore_list_assets.into_iter().map(|asset| {
            let ignore_asset_location = parse_asset_location(&asset);
            let ignore_asset = Rc::new(RefCell::new(Asset::new(asset.tokenData.clone(), ignore_asset_location)));
            let location_string = ignore_asset.borrow().get_asset_location_string().clone();
            location_string
        }).collect();
        

        let mut asset_map: HashMap<String, Vec<AssetPointer>> = HashMap::new();
        let mut asset_location_map: HashMap<AssetLocation, Vec<AssetPointer>> = HashMap::new();
        for parsed in parsed_files {
            let asset_array: Vec<MyAssetRegistryObject> = serde_json::from_value(parsed).unwrap();
            for asset in asset_array{
                let asset_location = parse_asset_location(&asset);
                let new_asset = Rc::new(RefCell::new(Asset::new(asset.tokenData, asset_location)));
                let map_key = new_asset.borrow().get_map_key();
                // if ignore_list_locations.contains(&new_asset.borrow().get_asset_location_string()){
                //     println!("Ignoring asset: {}", map_key);
                //     println!("asset_location: {:?}", new_asset.borrow().get_asset_location_string());
                //     continue;
                // }
                asset_map.entry(map_key).or_insert(vec![]).push(new_asset.clone());

                if let Some(location) = new_asset.borrow().asset_location.clone() {
                    asset_location_map.entry(location).or_insert(vec![]).push(new_asset.clone());
                };
            }
        }
        AssetRegistry2{
            asset_map: asset_map,
            asset_location_map: asset_location_map
        }
    }

    pub fn build_asset_registry_polkadot() -> AssetRegistry2{
        let chains = vec!["aca", "bnc_polkadot", "glmr", "hdx", "para", "other_polkadot", "glmr_evm", "asset_hub_polkadot"];
        let parsed_files = chains
            .into_iter()
            .map(|chain| {
                let path_string = format!("../../../polkadot_assets/assets/asset_registry/{}_assets.json", chain);
                // println!("path_string: {}", path_string);
                let path = Path::new(&path_string);
                let mut buf = vec![];
                let mut file = File::open(path)?;
                file.read_to_end(&mut buf)?;
                let parsed = serde_json::from_str(str::from_utf8(&buf).unwrap())?;
                Ok(parsed)
            })
            .collect::<Result<Vec<Value>, io::Error>>()
            .unwrap();

        
        let path = Path::new("../../../polkadot_assets/assets/ignore_list.json");
        let mut buf = vec![];
        let mut file = File::open(path).unwrap();
        file.read_to_end(&mut buf).unwrap();
        let parsed_ignore_file: Value = serde_json::from_str(str::from_utf8(&buf).unwrap()).unwrap();
        let ignore_list_assets: Vec<MyAssetRegistryObject> = serde_json::from_value(parsed_ignore_file).unwrap();
        let ignore_list_locations: Vec<String> = ignore_list_assets.iter().map(|asset| {
            let ignore_asset_location = parse_asset_location(&asset);
            let ignore_asset = Rc::new(RefCell::new(Asset::new(asset.tokenData.clone(), ignore_asset_location)));
            let location_string = ignore_asset.borrow().get_asset_location_string().clone();
            location_string
        }).collect();
        let ignore_list_asset_keys: Vec<String> = ignore_list_assets.into_iter().map(|asset| {
            let ignore_asset = Rc::new(RefCell::new(Asset::new(asset.tokenData.clone(), None)));
            let map_key = ignore_asset.borrow().get_map_key();
            map_key
        }).collect();

        let mut asset_map: HashMap<String, Vec<AssetPointer>> = HashMap::new();
        let mut asset_location_map: HashMap<AssetLocation, Vec<AssetPointer>> = HashMap::new();
        for parsed in parsed_files {
            let asset_array: Vec<MyAssetRegistryObject> = serde_json::from_value(parsed).unwrap();
            for asset in asset_array{
                let asset_location = parse_asset_location(&asset);
                // println!("{:?}", asset.tokenData);
                let mut new_asset = Rc::new(RefCell::new(Asset::new(asset.tokenData, asset_location)));
                let map_key = new_asset.borrow().get_map_key();

                // *****************************
                // if ignore_list_locations.contains(&new_asset.borrow().get_asset_location_string()){
                //     println!("Ignoring asset: {}", map_key);
                //     println!("asset_location: {:?}", new_asset.borrow().get_asset_location_string());
                //     continue;
                // }
                if ignore_list_asset_keys.contains(&map_key){
                    println!("Ignoring asset: {}", map_key);

                    // Remove asset location so it wont be added to xcm adjacent nodes
                    new_asset.borrow_mut().asset_location = None;
                    // continue;
                }
                // ******************************

                asset_map.entry(map_key).or_insert(vec![]).push(new_asset.clone());

                if let Some(location) = new_asset.borrow().asset_location.clone() {
                    asset_location_map.entry(location).or_insert(vec![]).push(new_asset.clone());
                };
            }
        }
        AssetRegistry2{
            asset_map: asset_map,
            asset_location_map: asset_location_map
        }
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
    //Get assets with that share the same XCM location of a specific asset
    pub fn get_assets_at_location(&self, asset_location: AssetLocation) -> Vec<AssetPointer>{
        let location_bucket = &self.asset_location_map.get(&asset_location);
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

    // queries first asset in bucket aside from kucoin asset, then gets decimals. Throws if no other asset in kucoin asset bucket
    pub fn get_kucoin_asset_decimals(&self, asset_location: AssetLocation) -> u64 {
        let related_assets = self.get_assets_at_location(asset_location);
        for asset in related_assets{
            if !asset.borrow().is_cex_token(){
                return asset.borrow().get_asset_decimals()
            }
        }
        panic!("Could not find decimals for kucoin asset")
    }
    pub fn display_assets_by_location(&self){
        for (location, assets) in &self.asset_location_map{
            println!("Location: {:?}", location);

            for asset in assets{
                match &asset.borrow().token_data{
                    TokenData::MyAsset(data) => {
                        println!("{} {} {} {}", asset.borrow().get_chain_id().unwrap(), asset.borrow().get_asset_name(), asset.borrow().get_asset_symbol(), asset.borrow().get_map_key());
                    },
                    TokenData::CexAsset(data) => {
                        println!("{} {} {}", asset.borrow().get_exchange().unwrap(), asset.borrow().get_asset_name(), asset.borrow().get_asset_symbol());
                    }
                }
            }
            println!("-----------------")
        }
    }
    pub fn display_all_assets(&self){
        let mut keys = vec![];
        for(key,value) in &self.asset_map{
            let display_string = key.clone() + " " + &value.iter().map(|x| x.borrow().get_asset_name().to_string()).collect::<Vec<String>>().join(", ");
            keys.push(display_string);
        }
        keys.sort();
        for key in keys{
            println!("{}", key);
        }
    }
    pub fn display_all_glmr_assets(&self){
        let mut keys = vec![];
        for(key,value) in &self.asset_map{
            if value[0].borrow().get_chain_id().unwrap() == 2004{
                // println!("TEST");
                println!("{} {} {:?}", key, value[0].borrow().get_asset_name(), value[0].borrow().get_asset_contract_address());
            }
            let display_string = key.clone() + " " + &value.iter().map(|x| x.borrow().get_asset_name().to_string()).collect::<Vec<String>>().join(", ");
            keys.push(display_string);
        }
        keys.sort();
        for key in keys{
            println!("{}", key);
        }
    }

    // 
    pub fn get_asset_by_id(&self, chain_id: u64, local_id: &str) -> Option<Rc<RefCell<Asset>>>{
        let map_key = chain_id.to_string() + local_id;
        // self.asset_map.get(&map_key).map(|x| x[0].clone())
        // println!("Map Key: {}", map_key);
        self.asset_map.get(&map_key).map(|x| x.iter()
            .find(|x| {
                // println!("Asset Map Key: {}", x.borrow().get_map_key());
                x.borrow().get_map_key() == map_key
            }).unwrap().clone())
    }
    pub fn get_asset_by_key(&self, map_key: &str) -> Option<Rc<RefCell<Asset>>>{
        self.asset_map.get(map_key).map(|x| x[0].clone())
    }
}
impl Asset{
    pub fn new(token_data: TokenData, asset_location: Option<AssetLocation>) -> Asset{
        Asset{token_data, asset_location}
    }
    pub fn display_asset(&self){
        match &self.token_data{
            TokenData::MyAsset(data) => print!("{}", data.chain.to_string() + &data.name.to_string()),
            TokenData::CexAsset(data) => print!("{}", data.exchange.to_string() + &data.assetTicker.to_string())
        }
    }
    pub fn get_map_key(&self) -> String{
        match &self.token_data{
            TokenData::MyAsset(data) => data.chain.to_string() + &data.localId.to_string(),
            TokenData::CexAsset(data) => data.exchange.to_string() + &data.assetTicker.to_string()
        }
    }
    pub fn get_asset_name(&self) -> &str {
        match &self.token_data {
            TokenData::MyAsset(data) => &data.name,
            TokenData::CexAsset(data) => &data.name,
        }
    }

    pub fn get_asset_symbol(&self) -> &str {
        match &self.token_data {
            TokenData::MyAsset(data) => &data.symbol,
            TokenData::CexAsset(data) => &data.assetTicker,
        }
    }
    pub fn get_relay_chain(&self) -> String {
        match &self.token_data {
            TokenData::MyAsset(data) => data.network.clone(),
            TokenData::CexAsset(data) => data.chain.clone(),
        }
    }

    

    pub fn get_ticker_symbol(&self) -> &str {
        match &self.token_data {
            TokenData::MyAsset(data) => &data.symbol,
            TokenData::CexAsset(data) => &data.assetTicker,
        }
    }
    pub fn get_chain_id(&self) -> Option<u64> {
        match &self.token_data {
            TokenData::MyAsset(data) => Some(data.chain),
            TokenData::CexAsset(_) => None,
        }
    }
    pub fn get_exchange(&self) -> Option<&str> {
        match &self.token_data {
            TokenData::MyAsset(_) => None,
            TokenData::CexAsset(data) => Some(&data.exchange),
        }
    }
    pub fn get_local_id(&self) -> Option<&serde_json::Value>{
        match &self.token_data {
            TokenData::MyAsset(data) => Some(&data.localId),
            TokenData::CexAsset(data) => None
        }
    }
    pub fn get_asset_decimals(&self) -> u64{
        // self.token_data.decimals.parse::<u64>().unwrap()
        match &self.token_data {
            TokenData::MyAsset(data) => data.decimals.parse::<u64>().unwrap(),
            TokenData::CexAsset(data) => data.precision
        }
    }
    pub fn get_asset_contract_address(&self) -> Option<String>{
        match &self.token_data {
            TokenData::MyAsset(data) => data.contractAddress.clone(),
            TokenData::CexAsset(data) => Some(data.contractAddress.clone())
        }
    }
    pub fn get_asset_location(&self) -> Option<AssetLocation>{
        self.asset_location.clone()
    }
    pub fn get_asset_location_string(&self) -> String{
        match &self.asset_location{
            Some(location) => {
                let mut location_string = String::new();
                if location.here{
                    location_string.push_str("here");
                }
                if let Some(xtype) = &location.xtype{
                    location_string.push_str(&xtype);
                }
                if let Some(properties) = &location.properties{
                    for property in properties{
                        location_string.push_str(&property);
                    }
                }
                location_string
            },
            None => String::new()
        }
    }

    pub fn get_origin_chain_id(&self) -> Option<u64> {
        match &self.asset_location{
            Some(location) => location.get_parachain_id(),
            None => None
        }
        // match &self.token_data {
        //     TokenData::MyAsset(data) => Some(data.chain),
        //     TokenData::CexAsset(_) => None,
        // }
    }
    pub fn is_cex_token(&self) -> bool {
        match &self.token_data {
            TokenData::MyAsset(_) => false,
            TokenData::CexAsset(_) => true,
        }
    }
}

fn parse_asset_registry_object(asset: &Value) -> MyAssetRegistryObject {
    serde_json::from_value(asset.clone()).unwrap()
}

fn parse_asset_location(parsed_asset_registry_object: &MyAssetRegistryObject) -> Option<AssetLocation> {
    match &parsed_asset_registry_object.tokenLocation {
        Some(location) if location.is_string() => Some(AssetLocation::new(true, None, None)),
        Some(location) if location.is_object() => {
            let location_obj = location.as_object().unwrap();
            let xtype = location_obj.keys().next().map(|x| x.to_string());
            let properties = location_obj.get(xtype.as_ref().unwrap()).unwrap();
            // println!("{:?}", properties);
            let properties = match properties{
                x if x.is_array() => x.as_array().unwrap().iter().map(|x| serde_json::to_string(x.as_object().unwrap()).unwrap()).collect(),
                x => vec![serde_json::to_string(x.as_object().unwrap()).unwrap()]
            };
            Some(AssetLocation::new(false, xtype, Some(properties)))
        },
        _ => None,
    }
}
impl AssetLocation{
    pub fn new(here: bool, xtype: Option<String>, properties: Option<Vec<String>>) -> AssetLocation{
        AssetLocation{
            here, xtype, properties
        }
    }

    pub fn get_parachain_id(&self) -> Option<u64> {
        if self.here {
            return Some(0);
        }
        match &self.properties {
            Some(properties) => {
                let parachain_property = properties.iter().find(|x| x.contains("Parachain"));
                let parachain_id = parachain_property.map(|x| x.split(":").last().unwrap()).unwrap();
                let parachain_id = parachain_id.replace(['{','}','\\','"'], "").parse::<u64>().unwrap();
                Some(parachain_id)
            },
            None => None,
        }
    }
}
impl<'de> serde::Deserialize<'de> for TokenData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let value = serde_json::Value::deserialize(deserializer)?;
        let object = value.as_object().ok_or_else(|| Error::custom("Expected a JSON object"))?;

        if object.contains_key("network") {
            Ok(TokenData::MyAsset(MyAssetJson::deserialize(value).unwrap()))
        } else if object.contains_key("exchange") {
            Ok(TokenData::CexAsset(CexAssetJson::deserialize(value).unwrap()))
        } else {
            Err(Error::custom("Unable to determine the type of TokenData from the keys in the JSON"))
        }
    }
}