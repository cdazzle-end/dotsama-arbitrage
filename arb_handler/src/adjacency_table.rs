use crate::token::{self, TokenData};
// use crate::adj_list_node::{AdjListNode, AdjListNodeOption, TokenNodeIterator};
use crate::{LiqPoolRegistry};

use std::cell::RefCell;
use std::collections::{VecDeque, HashMap};
use std::rc::Rc;
use crate::{asset_registry::{AssetRegistry, Asset}};

type AssetPointer = Rc<RefCell<Asset>>;

//Table bucket is a list of adjacency lists. Each adjacency list item is a tuple. Each tuple is an (asset, (base liquidity, adjacent liquidity)). The first item in an adjacency list is the base asset
type TableBucket = Vec<Vec<(AssetPointer, (u128, u128))>>;

#[derive(Clone)]
pub struct AdjacencyTable{
    pub table: HashMap<String, TableBucket>
}

impl AdjacencyTable{
    pub fn build_adjacency_table(liq_pool_registry: &LiqPoolRegistry) -> AdjacencyTable{
        // let mut table = HashMap::new()
        let mut adjacency_table = AdjacencyTable { table: HashMap::new() };
        for pool in &liq_pool_registry.liq_pools{
            if pool.exchange == None{
                let (asset_0, asset_1) = (Rc::clone(&pool.assets[0]), Rc::clone(&pool.assets[1]));
                let (liquidity_0, liquidity_1) = (pool.liquidity[0], pool.liquidity[1]);
                // println!("{} - {}", liquidity_0, liquidity_1);

                adjacency_table.add_pair_to_table(Rc::clone(&asset_0), liquidity_0, Rc::clone(&asset_1), liquidity_1);
                adjacency_table.add_pair_to_table(asset_1, liquidity_1, asset_0, liquidity_0);
            } else {
                let (asset_0, asset_1) = (Rc::clone(&pool.assets[0]), Rc::clone(&pool.assets[1]));
                let (bid_price, ask_price) = (pool.prices.unwrap().0, pool.prices.unwrap().1);

                adjacency_table.add_kucoin_pair_to_table(Rc::clone(&asset_0), Rc::clone(&asset_1), bid_price, ask_price);
                adjacency_table.add_kucoin_pair_to_table(asset_1, asset_0, bid_price, ask_price);
            }
        }
        adjacency_table
    }

    //Add asset to adjacency list in table
    pub fn add_pair_to_table(&mut self,  base_asset: AssetPointer, base_liquidity: u128, adjacent_asset: AssetPointer, adjacent_liquidity: u128){
        let table = &mut self.table;
        let base_asset_key = base_asset.borrow().token_data.get_map_key();

        //Get bucket from base asset key. If none exists yet, create new one
        let table_bucket = table.entry(base_asset_key.clone()).or_insert(vec![vec![(Rc::clone(&base_asset), (0,0))]]);
        

        //Find base_asset adjacency list and add new asset
        let mut value_has_been_inserted = false;

        //Loop through lists, find one that corresponds to base asset
        for adjacency_list in table_bucket{

            //The first asset in an adjacency list is the primary asset (the rest are adjacent to the primary)
            let adjacency_list_key = adjacency_list[0].0.borrow().token_data.get_map_key();
            if base_asset_key == adjacency_list_key{
                adjacency_list.push((Rc::clone(&adjacent_asset), (base_liquidity, adjacent_liquidity)));
                value_has_been_inserted = true;
            }
        }

        //If base_asset has no corresponding adjacency list in the table, create one
        //This should only happen if there is a hashmap collision, i.e. 2 different asset keys hash to the same value.
        if value_has_been_inserted == false{
            let new_adjacency_list = vec![(base_asset, (0,0)), (adjacent_asset, (base_liquidity, adjacent_liquidity))];
            table.entry(base_asset_key.clone()).and_modify(|e| e.push(new_adjacency_list));
        }
    }

    pub fn add_kucoin_pair_to_table(&mut self, base_asset: AssetPointer, adjacent_asset: AssetPointer, bid: u64, ask: u64 ){
        let table = &mut self.table;
        let base_asset_key = base_asset.borrow().token_data.get_map_key();

        let table_bucket = table.entry(base_asset_key.clone()).or_insert(vec![vec![(Rc::clone(&base_asset), (0,0))]]);
        let mut value_has_been_inserted = false;

        for adjacency_list in table_bucket{

            //The first asset in an adjacency list is the primary asset (the rest are adjacent to the primary)
            let adjacency_list_key = adjacency_list[0].0.borrow().token_data.get_map_key();
            if base_asset_key == adjacency_list_key{
                adjacency_list.push((Rc::clone(&adjacent_asset), (bid as u128, ask as u128)));
                value_has_been_inserted = true;
            }
        }

         if value_has_been_inserted == false{
            let new_adjacency_list = vec![(base_asset, (0,0)), (adjacent_asset, (bid as u128, ask as u128))];
            table.entry(base_asset_key.clone()).and_modify(|e| e.push(new_adjacency_list));
        }

    }

    pub fn display_table(&self){
        let mut num_buckets = 0;
        let mut num_lists = 0;
        for bucket in self.table.values(){
            num_buckets += 1;
            for list in bucket{
                num_lists += 1;
                print!("{} ", list[0].0.borrow().token_data.get_chain());
                for asset in list{
                    print!("{} ->", asset.0.borrow().token_data.get_asset_name())
                }
                println!("")
            }
        }
        println!("table buckets {}", num_buckets);
        println!("table lists {}", num_lists);
    }

    //Get the adjacent assets and liquidity for a specific asset
    pub fn get_adjacent_assets(&self, input_asset: AssetPointer) -> Vec<(AssetPointer, (u128, u128))>{
        let table_bucket = self.table.get(&input_asset.borrow().token_data.get_map_key());

        //List of adjacent assets and liquidity, without the primary asset at the head of the list
        let mut adjacent_assets: Vec<(AssetPointer, (u128,u128))> = Vec::new();

        //Check the bucket that corresponds to the input asset key
        match table_bucket{
            Some(bucket) => {

                //Loop through lists, find list that corresponds to input asset
                for adjacency_list in bucket{
                    if input_asset.borrow().token_data.get_map_key() == adjacency_list[0].0.borrow().token_data.get_map_key(){
                        let mut index = 1;
                        while index < adjacency_list.len(){
                            adjacent_assets.push((Rc::clone(&adjacency_list[index].0), adjacency_list[index].1));
                            index += 1;
                        }
                    }
                }
            },
            None => ()
        }
        adjacent_assets
    }
}