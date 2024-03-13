use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::liq_pool_registry_2::{LiqPoolRegistry2, TickData};
use crate::asset_registry_2::{Asset, TokenData};

//The Adjacency Table contains the connections between nodes in the token graph

type AssetPointer = Rc<RefCell<Asset>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Liquidity{
    Dex(DexLp),
    DexV3(DexV3),
    Cex(CexLp),
    Stable(StableLp),
}
#[derive(Debug, Clone, PartialEq)]
pub struct DexLp{
    pub lp_id: Option<String>,
    pub base_liquidity: u128,
    pub adjacent_liquidity: u128,
}
#[derive(Debug, Clone, PartialEq)]
pub struct DexV3{
    pub lp_id: Option<String>,
    pub token_0: String,
    pub token_1: String,
    pub active_liquidity: u128,
    pub current_tick: i64,
    pub fee_rate: u128,
    pub lower_ticks: Vec<TickData>,
    pub upper_ticks: Vec<TickData>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CexLp{
    pub bid_price: u128,
    pub bid_decimals: u128,
    pub ask_price: u128,
    pub ask_decimals: u128,
}
#[derive(Debug, Clone, PartialEq)]
pub struct StableLp{
    pub lp_id: Option<String>, 
    pub base_liquidity: u128,
    pub adjacent_liquidity: Vec<u128>,
    pub a: u128,
    pub total_supply: u128,
    pub base_token_precision: u128,
    pub adjacent_token_precisions: Vec<u128>,
}

pub struct AdjacencyTable2{
    pub table_2: HashMap<String, Vec<AdjacencyList>>
}

pub struct AdjacencyList{
    pub primary_asset: AssetPointer,
    pub list: Vec<AdjacencyGroup>
}
#[derive(Debug, Clone)]
pub struct AdjacencyGroup{
    pub group_type: GroupType,
    pub adjacent_asset: Vec<AssetPointer>,
    pub liquidity: Option<Liquidity>
}
#[derive(Debug, Clone)]
pub enum GroupType{
    Stable,
    Cex,
    Dex,
    DexV3,
    Xcm
}
impl AdjacencyTable2{
    pub fn build_table_2(lp_registry: &LiqPoolRegistry2) -> AdjacencyTable2{
        let mut adjacency_table = AdjacencyTable2 { table_2: HashMap::new() };
        for lp in &lp_registry.liq_pools{   
            // println!("lp: {:?}", lp);
            let lp_id = lp.lp_id.clone();
            let (asset_0, asset_1) = (Rc::clone(&lp.assets[0]), Rc::clone(&lp.assets[1]));
            // This is for cex pools (NOT IN OPERATION ATM)
            if let Some(x) = &lp.exchange{
                let (bid_price, ask_price) = (lp.prices.unwrap().0 as u128, lp.prices.unwrap().0 as u128);
                let (bid_decimals, ask_decimals) = (lp.price_decimals.unwrap().0 as u128, lp.price_decimals.unwrap().0 as u128);
                adjacency_table.add_cex_pair_to_table(
                    Rc::clone(&asset_0),
                    Rc::clone(&asset_1),
                    bid_price, bid_decimals,
                    ask_price, ask_decimals
                );
                adjacency_table.add_cex_pair_to_table(
                    Rc::clone(&asset_1),
                    Rc::clone(&asset_0),
                    bid_price, bid_decimals,
                    ask_price, ask_decimals
                );

            // This is for stable pools
            } else if let Some(x) = &lp.a {
                for asset in &lp.assets{
                    // println!("LIQUIDITY: {:?}", lp.liquidity);
                    let base_asset = Rc::clone(&asset);
                    let base_asset_index = lp.assets.iter().position(|x| Rc::ptr_eq(x, &base_asset)).unwrap();
                    let base_asset_liquidity = lp.liquidity[base_asset_index];
                    let token_precisions = lp.token_precisions.as_ref().unwrap().iter().map(|x| x.parse::<u128>().unwrap()).collect::<Vec<u128>>();
                    let total_supply = lp.total_supply.unwrap(); 
                    let base_token_precision = token_precisions[base_asset_index];
                    let mut adjacent_assets = vec![];
                    let mut adjacent_liquidity = vec![];
                    let mut adjacent_token_precisions = vec![];
                    for(i, asset) in lp.assets.iter().enumerate(){
                        if i != base_asset_index{
                            adjacent_assets.push(Rc::clone(asset));
                            // adjacent_liquidity.push(lp.liquidity[i]);
                            adjacent_token_precisions.push(token_precisions[i]);
                        }
                    }
                    for(i, liq) in lp.liquidity.iter().enumerate(){
                        if i != base_asset_index{
                            adjacent_liquidity.push(*liq);
                        }
                    }
                    adjacency_table.add_stable_pair_to_table(lp_id.clone(), base_asset, base_asset_liquidity, base_token_precision, adjacent_assets, adjacent_liquidity, lp.a.unwrap().into(), total_supply, adjacent_token_precisions);
                }
            // This is for regular dex pools, which is most of what were working with
            } else if let Some(x) = &lp.current_tick{
                // println!("Active Liquidity: {:?}", lp.active_liquidity);
                let active_liquidity = &lp.active_liquidity.clone().unwrap().as_str().parse::<u128>().unwrap();
                let contract_address = lp.contract_address.clone();
                let token_0 = asset_0.borrow().get_asset_contract_address().unwrap();
                let token_1 = asset_1.borrow().get_asset_contract_address().unwrap();
                adjacency_table.add_dex_3_to_table(
                    lp_id.clone(),
                    Rc::clone(&asset_0),
                    Rc::clone(&asset_1),
                    token_0.clone(),
                    token_1.clone(),
                    *active_liquidity,
                    lp.fee_rate.clone().unwrap().parse::<u128>().unwrap(),
                    lp.current_tick.clone().unwrap().into(),
                    lp.lower_ticks.clone().unwrap(),
                    lp.upper_ticks.clone().unwrap()
                );
                adjacency_table.add_dex_3_to_table(
                    lp_id.clone(),
                    Rc::clone(&asset_1),
                    Rc::clone(&asset_0),
                    token_0,
                    token_1,
                    *active_liquidity,
                    lp.fee_rate.clone().unwrap().parse::<u128>().unwrap(),
                    lp.current_tick.clone().unwrap().into(),
                    lp.lower_ticks.clone().unwrap(),
                    lp.upper_ticks.clone().unwrap()
                );
            } else {
                let (liquidity_0, liquidity_1) = (lp.liquidity[0], lp.liquidity[1]);
                adjacency_table.add_dex_pair_to_table(
                    lp_id.clone(),
                    Rc::clone(&asset_0),
                    liquidity_0,
                    Rc::clone(&asset_1),
                    liquidity_1,
                );
                adjacency_table.add_dex_pair_to_table(lp_id, asset_1, liquidity_1, asset_0, liquidity_0);
            }
        }
        adjacency_table
    }

    fn add_dex_3_to_table(
        &mut self,
        lp_id: Option<String>,
        base_asset: AssetPointer,
        adjacent_asset: AssetPointer,
        token_0: String,
        token_1: String,
        active_liquidity: u128,
        fee_rate: u128,
        current_tick: i64,
        lower_ticks: Vec<TickData>,
        upper_ticks: Vec<TickData>,
    ){
        let base_asset_key = base_asset.borrow().get_map_key();

        let table_bucket = self
            .table_2
            .entry(base_asset_key.clone())
            .or_insert(vec![AdjacencyList::new(&base_asset)]);

        //Find base_asset adjacency list and add new asset
        let mut inserted = false;

        //Loop through lists, find one that corresponds to base asset
        for adjacency_list in table_bucket{

            //The first asset in an adjacency list is the primary asset (the rest are adjacent to the primary)
            let adjacency_list_key = adjacency_list.primary_asset.borrow().get_map_key();
            if base_asset_key == adjacency_list_key{
                adjacency_list.add_dex_3_pair(adjacent_asset, lp_id, token_0, token_1, active_liquidity, fee_rate, current_tick, lower_ticks, upper_ticks);
                inserted = true;
                break;
            }
        }

        //If base_asset has no corresponding adjacency list in the table, that means there is a list at that index which isn't the one we're looking for
        //This should only happen if there is a hashmap collision, i.e. 2 different asset keys hash to the same value.
        if !inserted{
            let new_adjacency_list = AdjacencyList::new(&base_asset);
            self.table_2.entry(base_asset_key.clone()).and_modify(|e| e.push(new_adjacency_list));
        }

    }

    fn add_dex_pair_to_table(
        &mut self,
        lp_id: Option<String>,
        base_asset: AssetPointer,
        base_liquidity: u128,
        adjacent_asset: AssetPointer,
        adjacent_liquidity: u128,
    ){
        let base_asset_key = base_asset.borrow().get_map_key();

        let table_bucket = self
            .table_2
            .entry(base_asset_key.clone())
            .or_insert(vec![AdjacencyList::new(&base_asset)]);

        //Find base_asset adjacency list and add new asset
        let mut inserted = false;

        //Loop through lists, find one that corresponds to base asset
        for adjacency_list in table_bucket{

            //The first asset in an adjacency list is the primary asset (the rest are adjacent to the primary)
            let adjacency_list_key = adjacency_list.primary_asset.borrow().get_map_key();
            if base_asset_key == adjacency_list_key{
                adjacency_list.add_dex_pair(adjacent_asset, lp_id, base_liquidity, adjacent_liquidity);
                inserted = true;
                break;
            }
        }

        //If base_asset has no corresponding adjacency list in the table, that means there is a list at that index which isn't the one we're looking for
        //This should only happen if there is a hashmap collision, i.e. 2 different asset keys hash to the same value.
        if !inserted{
            let new_adjacency_list = AdjacencyList::new(&base_asset);
            self.table_2.entry(base_asset_key.clone()).and_modify(|e| e.push(new_adjacency_list));
        }
    }

    pub fn add_cex_pair_to_table(
        &mut self,
        base_asset: AssetPointer,
        adjacent_asset: AssetPointer,
        bid_price: u128, bid_decimals: u128,
        ask_price: u128, ask_decimals: u128
    ){
        // let table = &mut self.table;
        let base_asset_key = base_asset.borrow().get_map_key();

        //Get bucket from base asset key. If none exists yet, create new one
        let table_bucket = self
            .table_2
            .entry(base_asset_key.clone())
            .or_insert(vec![AdjacencyList::new(&base_asset)]);
        

        //Find base_asset adjacency list and add new asset
        let mut inserted = false;

        //Loop through lists, find one that corresponds to base asset
        for adjacency_list in table_bucket{

            //The first asset in an adjacency list is the primary asset (the rest are adjacent to the primary)
            let adjacency_list_key = adjacency_list.primary_asset.borrow().get_map_key();
            if base_asset_key == adjacency_list_key{
                adjacency_list.add_cex_pair(adjacent_asset, bid_price, bid_decimals, ask_price, ask_decimals);
                inserted = true;
                break;
            }
        }

        //If base_asset has no corresponding adjacency list in the table, create one
        //This should only happen if there is a hashmap collision, i.e. 2 different asset keys hash to the same value.
        if !inserted{
            let new_adjacency_list = AdjacencyList::new(&base_asset);
            self.table_2.entry(base_asset_key.clone()).and_modify(|e| e.push(new_adjacency_list));
        }
    }

    pub fn add_stable_pair_to_table(
        &mut self,
        lp_id: Option<String>,
        base_asset: AssetPointer,
        base_liquidity: u128,
        base_token_precision: u128,
        adjacent_assets: Vec<AssetPointer>,
        adjacent_liquidity: Vec<u128>,
        a: u128,
        total_supply: u128,
        adjacent_token_precisions: Vec<u128>,
    ){
        // println!("BASE LIQ {:?}", base_liquidity);
        // println!("ADJ LiQ: {:?}", adjacent_liquidity);

        // let table = &mut self.table;
        let base_asset_key = base_asset.borrow().get_map_key();

        //Get bucket from base asset key. If none exists yet, create new one
        let table_bucket = self
            .table_2
            .entry(base_asset_key.clone())
            .or_insert(vec![AdjacencyList::new(&base_asset)]);
        

        //Find base_asset adjacency list and add new asset
        let mut inserted = false;

        //Loop through lists, find one that corresponds to base asset
        for adjacency_list in table_bucket{
            let adjacency_list_key = adjacency_list.primary_asset.borrow().get_map_key();
            if base_asset_key == adjacency_list_key{
                adjacency_list.add_stable_pair(lp_id, adjacent_assets, base_liquidity, base_token_precision, adjacent_liquidity, a, total_supply, adjacent_token_precisions);
                inserted = true;
                break;
            }
        }

        //If base_asset has no corresponding adjacency list in the table, create one
        //This should only happen if there is a hashmap collision, i.e. 2 different asset keys hash to the same value.
        if !inserted{
            let new_adjacency_list = AdjacencyList::new(&base_asset);
            self.table_2.entry(base_asset_key.clone()).and_modify(|e| e.push(new_adjacency_list));
        }
    }

    pub fn get_adjacent_assets_2(&self, input_asset: AssetPointer) -> Vec<AdjacencyGroup>{
        let list_key = input_asset.borrow().get_map_key();
        let table_bucket = self.table_2.get(&list_key);
        let mut adjacent_assets: Vec<AdjacencyGroup> = Vec::new();
        if let Some(bucket) = table_bucket{
            for list in bucket{
                if list.primary_asset.borrow().get_map_key() == list_key{
                    adjacent_assets = list.get_adjacent_assets();
                }
            }
        }
        adjacent_assets
    }

    pub fn display_table(&self){
        for (key, bucket) in &self.table_2{
            for list in bucket{
                // list.primary_asset.borrow().display_asset();
                print!("Base asset: ( "); list.primary_asset.borrow().display_asset(); print!(" ) - Adjacent assets: ");
                for adjacency_group in &list.list{
                    
                    match adjacency_group.group_type{
                        GroupType::Dex => {
                            print!("(D)"); adjacency_group.adjacent_asset[0].borrow().display_asset(); print!(" | "); 
                        }
                        GroupType::Cex => {
                            print!("(C)");adjacency_group.adjacent_asset[0].borrow().display_asset(); print!(" | "); 
                        },
                        // GroupType::Stable => adjacency_group.adjacent_asset[0].borrow().display_asset(),
                        GroupType::Stable => {
                            // print!("Base asset: ( "); list.primary_asset.borrow().display_asset(); print!(" ) ");
                            adjacency_group.display_stable_adjacenct(); print!(" | ");
                        },
                        // GroupType::Xcm => adjacency_group.adjacent_asset.[0]borrow().display_asset(),
                        _ => {}
                    }
                }
                println!("");
                println!("--------------------------------");
            }
            // println!("Primary Asset: {}", list.primary_asset.borrow().get_map_key());
            // for adjacency_group in &list.list{
            //     println!("Adjacent Asset: {}", adjacency_group.adjacent_asset.borrow().get_map_key());
            // }
        }
    }

}

impl AdjacencyList{
    pub fn new(primary_asset: &AssetPointer) -> AdjacencyList{
        AdjacencyList{
            primary_asset: Rc::clone(primary_asset),
            list: Vec::new()
        }
    }

    pub fn add_dex_pair(&mut self, adjacent_assets: AssetPointer, lp_id: Option<String>, base_liquidity: u128, adjacent_liquidity: u128){
        let pair = AdjacencyGroup::new_dex_pair(Rc::clone(&adjacent_assets), lp_id, base_liquidity, adjacent_liquidity);
        self.list.push(pair);
    }

    pub fn add_dex_3_pair(&mut self, adjacent_assets: AssetPointer, lp_id: Option<String>, token_0: String, token_1: String, active_liquidity: u128, fee_rate: u128, current_tick: i64, lower_ticks: Vec<TickData>, upper_ticks: Vec<TickData>){
        let pair = AdjacencyGroup::new_dex_3_pair(Rc::clone(&adjacent_assets), lp_id, token_0, token_1, active_liquidity, fee_rate, current_tick, lower_ticks, upper_ticks);
        self.list.push(pair);
    }

    pub fn add_cex_pair(&mut self, adjacent_asset: AssetPointer, bid_price: u128, bid_decimals: u128, ask_price: u128, ask_decimals: u128){
        let pair = AdjacencyGroup::new_cex_pair(Rc::clone(&adjacent_asset), bid_price, bid_decimals, ask_price, ask_decimals);
        self.list.push(pair);
    }

    pub fn  add_stable_pair(&mut self, lp_id: Option<String>, adjacent_assets: Vec<AssetPointer>, base_liquidity: u128, base_token_precision: u128, adjacent_liquidity: Vec<u128>, a: u128, total_supply: u128, adjacent_token_precisions: Vec<u128>){
        let pair = AdjacencyGroup::new_stable_pair(lp_id, adjacent_assets, base_liquidity, base_token_precision, adjacent_liquidity, a, total_supply, adjacent_token_precisions);
        self.list.push(pair);
    }

    pub fn get_adjacent_assets(&self) -> Vec<AdjacencyGroup>{
        let mut adjacent_assets: Vec<AdjacencyGroup> = Vec::new();
        for pair in &self.list{
            // let pair_copy = AdjacencyGroup{
            //     adjacent_assets: Rc::clone(&pair.adjacent_asset),
            //     liquidity: pair.liquidity.clone()
            // };
            adjacent_assets.push(pair.clone());
        }
        adjacent_assets
    }
}

impl AdjacencyGroup{
    pub fn new_dex_pair(adjacent_asset: AssetPointer, lp_id: Option<String>, base_liquidity: u128, adjacent_liquidity: u128) -> AdjacencyGroup{
        AdjacencyGroup{
            group_type: GroupType::Dex,
            adjacent_asset: vec![Rc::clone(&adjacent_asset)],
            liquidity: Some(Liquidity::Dex(DexLp{
                lp_id: lp_id,
                base_liquidity: base_liquidity,
                adjacent_liquidity: adjacent_liquidity
            }))
        }
    }

    pub fn new_dex_3_pair(adjacent_asset: AssetPointer, lp_id: Option<String>, token_0: String, token_1: String, active_liquidity: u128, fee_rate: u128, current_tick: i64, lower_ticks: Vec<TickData>, upper_ticks: Vec<TickData>) -> AdjacencyGroup{
        AdjacencyGroup{
            group_type: GroupType::DexV3,
            adjacent_asset: vec![Rc::clone(&adjacent_asset)],
            liquidity: Some(Liquidity::DexV3(DexV3{
                lp_id,
                token_0: token_0,
                token_1: token_1,
                current_tick: current_tick,
                active_liquidity: active_liquidity,
                fee_rate: fee_rate,
                lower_ticks: lower_ticks,
                upper_ticks: upper_ticks
            }))
        }
    }

    pub fn new_cex_pair(adjacent_asset: AssetPointer, bid_price: u128, bid_decimals: u128, ask_price: u128, ask_decimals: u128) -> AdjacencyGroup{
        AdjacencyGroup{
            group_type: GroupType::Cex,
            adjacent_asset: vec![Rc::clone(&adjacent_asset)],
            liquidity: Some(Liquidity::Cex(CexLp{
                bid_price: bid_price, bid_decimals: bid_decimals,
                ask_price: ask_price, ask_decimals: ask_decimals
            }))
        }
    }

    pub fn new_stable_pair(lp_id: Option<String>, adjacent_assets: Vec<AssetPointer>, base_liquidity: u128, base_token_precision: u128, adjacent_liquidity: Vec<u128>, a: u128, total_supply: u128, adjacent_token_precisions: Vec<u128>) -> AdjacencyGroup{
        AdjacencyGroup{
            group_type: GroupType::Stable,
            adjacent_asset: adjacent_assets,
            liquidity: Some(Liquidity::Stable(
                StableLp{
                    lp_id,
                    base_liquidity: base_liquidity,
                    base_token_precision: base_token_precision,
                    adjacent_liquidity: adjacent_liquidity,
                    a: a,
                    total_supply: total_supply,
                    adjacent_token_precisions: adjacent_token_precisions
                }
            ))
        }
    }


    pub fn new_xcm_pair(adjacent_asset: AssetPointer) -> AdjacencyGroup{
        AdjacencyGroup{
            group_type: GroupType::Xcm,
            adjacent_asset: vec![Rc::clone(&adjacent_asset)],
            liquidity: None
        }
    }

    pub fn display_dex_pair(&self){
        print!("DexPair: ( ");
        for asset in &self.adjacent_asset{
            asset.borrow().display_asset();
            print!(" | ");
        }
        println!(" )");
        println!("");

    }

    pub fn display_stable_adjacenct(&self) {
        print!("StablePool: ( ");
        for asset in &self.adjacent_asset{
            asset.borrow().display_asset();
            print!(" | ");
        }
        print!(" )");
        // println!("");
    }
}