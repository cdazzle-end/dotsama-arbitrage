use serde_json::Value;
use std::fs;
use num::{BigInt, BigUint, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, FromPrimitive, Num, One, Signed, ToPrimitive, Zero};
use num::bigint::{ToBigInt, ToBigUint};
use num::BigRational;
use num;
use bigdecimal::{BigDecimal};
// use std::borrow::{Borrow, BorrowMut};
// use num::BigRational::
use std::cell::RefCell;
use std::collections::{VecDeque, HashMap};
use std::{path::Path, fs::File, io::Read};
use serde::{Deserialize, Deserializer, Serialize};
use std::ops::{Add, Mul};
use std::rc::Rc;
use std::str::FromStr;
use std::vec;
use crate::liq_pool_registry_2::{LiquidityPool, TickData, TokenRate, BncStableData, CexData, DexData, DexV3Data, StableData};
use crate::token_graph_2::GraphNode;
// use crate::{asset_registry::{AssetRegistry, Asset}};
use crate::AssetRegistry2;
use crate::asset_registry_2::{Asset, AssetLocation, TokenData};
use crate::adjacency_table_2::{AdjacencyGroup, AdjacencyTable2,  GroupType, };

use std::hash::{Hasher, Hash};
use std::str;
use std::io;
use serde::de::{Error, Visitor};
type AssetPointer = Rc<RefCell<Asset>>;
type GraphNodePointer = Rc<RefCell<GraphNode>>;

#[derive(Debug, Clone, Deserialize)]
pub struct XcmFeeData {
    pub fee: String,
    pub decimals: String,
}

// #[derive(Debug, Deserialize)]
// pub struct TransferDepositFeeBook{
//     #[serde(rename = "polkadot-transfer")]
//     pub polkadot_transfer: HashMap<String, ChainTransferData>,
//     #[serde(rename = "polkadot-deposit")]
//     pub polkadot_deposit: HashMap<String, ChainDepositData>,
// }
#[derive(Debug, Deserialize)]
pub struct TransferDepositFeeBook{
    #[serde(rename = "polkadot-transfer")]
    pub polkadot_transfer: HashMap<String, ChainTransferDepositData>,
    #[serde(rename = "polkadot-deposit")]
    pub polkadot_deposit: HashMap<String, ChainTransferDepositData>,
}
// #[derive(Debug, Serialize, Deserialize)]
// pub struct ChainTransferData {
//     #[serde(flatten)]
//     pub assets: HashMap<String, XcmTransferData>,
// }
// #[derive(Debug, Serialize, Deserialize)]
// pub struct ChainDepositData {
//     #[serde(flatten)]
//     pub assets: HashMap<String, XcmTransferData>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ChainTransferData {
//     #[serde(flatten)]
//     pub assets: HashMap<String, TransferData>,
// }
// #[derive(Debug, Serialize, Deserialize)]
// pub struct ChainDepositData {
//     #[serde(flatten)]
//     pub assets: HashMap<String, DepositData>,
// }
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct TransferData {
//     pub transferAmount: Option<String>,
//     pub transferDecimals: Option<String>,
//     pub transferAssetSymbol: Option<String>,
//     transferAssetId: serde_json::Value,
//     pub feeAmount: Option<String>,
//     pub feeDecimals: Option<String>,
//     pub feeAssetSymbol: Option<String>,
//     feeAssetId: serde_json::Value,
// }
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct DepositData {
//     pub depositAmount: Option<String>,
//     pub feeAmount: Option<String>,
//     pub feeDecimals: Option<String>,
//     pub feeAssetSymbol: Option<String>,
//     feeAssetId: serde_json::Value,
// }
#[derive(Debug, Serialize, Deserialize)]
pub struct ChainTransferDepositData {
    #[serde(flatten)]
    pub assets: HashMap<String, XcmTransferData>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XcmTransferData {
    pub xcmAmount: Option<String>,
    pub xcmDecimals: Option<String>,
    pub xcmAssetSymbol: Option<String>,
    xcmAssetId: serde_json::Value,
    pub feeAmount: Option<String>,
    pub feeDecimals: Option<String>,
    pub feeAssetSymbol: Option<String>,
    feeAssetId: serde_json::Value,
    pub node: Option<String>
}
// impl TransferData{
//     pub fn get_fee_asset_id(&self) -> String {
//         self.feeAssetId.clone().to_string().clone()
//     }
// }
// impl DepositData{
//     pub fn get_fee_asset_id(&self) -> String {
//         self.feeAssetId.clone().to_string().clone()
//     }
// }
impl XcmTransferData{
    pub fn get_fee_asset_id(&self) -> String {
        self.feeAssetId.clone().to_string().clone()
    }
}
fn deserialize_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;
    match value {
        Value::String(s) => Ok(s),
        Value::Object(_) => Ok(value.to_string()),
        _ => Err(serde::de::Error::custom("Unexpected type for asset ID")),
    }
}

impl TransferDepositFeeBook {
    pub fn new() -> Self {
        TransferDepositFeeBook {
            polkadot_transfer: HashMap::new(),
            polkadot_deposit: HashMap::new(),
        }
    }
    pub fn from_json_file(file_path: &str) -> Self {
        let mut file = File::open(file_path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let fee_book: TransferDepositFeeBook = serde_json::from_str(&data).unwrap();
        fee_book
    }
    pub fn get_transfer_fee_data(&self, node: GraphNodePointer) -> Option<XcmTransferData> {

        let location = node.borrow().get_asset_location();

        match location {
            Some(x) => {
                let chain_id = node.borrow().get_chain_id().to_string();
                let asset_symbol = node.borrow().get_asset_symbol();
                let asset_id = node.borrow().get_local_id();
                let chain_data = self.polkadot_transfer.get(&chain_id);
                match chain_data {
                    Some(data) => {
                        // println!("Asset Data: {:?}", data);
                        // println!("Asset Fee Key: {:?}", asset_fee_key.as_str());
                        let asset_data = data.assets.get(asset_id.as_str());
                        match asset_data {
                            Some(data) => {
                                let amount = data.feeAmount.clone();
                                let fee_asset_symbol = data.feeAssetSymbol.clone();
                                // print!("Transfer Data: Found asset data *** Chain id: {} | Asset fee key: {:?} | ", chain_id, asset_id.as_str());
                                // println!("Fee asset symbol: {:?} | amount: {:?}", fee_asset_symbol, amount);
                                Some(data.clone())
                            }
                            None => {
                                None
                            }
                        }
                    }
                    None => {
                        None
                    }
                }
            }
            _ => {
                None
            }
        }
    }

    // pub fn get_transfer_fee_data_2(&self, node: GraphNodePointer) -> Option<TransferData> {

    //     let location = node.borrow().get_asset_location();

    //     match location {
    //         Some(x) => {
    //             let chain_id = node.borrow().get_chain_id().to_string();
    //             let asset_symbol = node.borrow().get_asset_symbol();
    //             let asset_id = node.borrow().get_local_id();
    //             let chain_data = self.polkadot_transfer.get(&chain_id);
    //             match chain_data {
    //                 Some(data) => {
    //                     // println!("Asset Data: {:?}", data);
    //                     // println!("Asset Fee Key: {:?}", asset_fee_key.as_str());
    //                     let asset_data = data.assets.get(asset_id.as_str());
    //                     match asset_data {
    //                         Some(data) => {
    //                             let amount = data.transferAmount.clone();
    //                             let fee_asset_symbol = data.feeAssetSymbol.clone();
    //                             print!("Transfer Data: Found asset data *** Chain id: {} | Asset fee key: {:?} | ", chain_id, asset_id.as_str());
    //                             println!("Fee asset symbol: {:?} | amount: {:?}", fee_asset_symbol, amount);
    //                             Some(data.clone())
    //                         }
    //                         None => {
    //                             None
    //                         }
    //                     }
    //                 }
    //                 None => {
    //                     None
    //                 }
    //             }
    //         }
    //         _ => {
    //             None
    //         }
    //     }
    // }

    pub fn get_deposit_fee_data(&self, node: GraphNodePointer) -> Option<XcmTransferData> {
        let location = node.borrow().get_asset_location();
        match location {
            Some(x) => {
                let chain_id = node.borrow().get_chain_id().to_string();
                let asset_symbol = node.borrow().get_asset_symbol();
                let asset_id = node.borrow().get_local_id();
                let chain_data = self.polkadot_deposit.get(&chain_id);
                match chain_data {
                    Some(data) => {
                        // println!("Asset Data: {:?}", data);
                        // println!("Asset Fee Key: {:?}", asset_fee_key.as_str());
                        let asset_data = data.assets.get(asset_id.as_str());
                        match asset_data {
                            Some(data) => {
                                let amount = data.xcmAmount.clone();
                                let fee_asset_symbol = data.feeAssetSymbol.clone();
                                // print!("Deposit Data: Found asset data *** Chain id: {} | Asset fee key: {:?} | ", chain_id, asset_id.as_str());
                                // println!("Fee asset symbol: {:?} | amount: {:?}", fee_asset_symbol, amount);
                                Some(data.clone())
                            }
                            None => {
                                None
                            }
                        }
                    }
                    None => {
                        None
                    }
                }
            }
            _ => {
                None
            }
        }
    }
    
    pub fn get_all_transfer_fee_data(&self) -> Vec<(String, &ChainTransferDepositData)> {
        let mut data: Vec<(String, &ChainTransferDepositData)> = Vec::new();
        for (chain_id, chain_data) in self.polkadot_transfer.iter() {
            // for (chain, asset) in chain_data.assets.iter() {
            //     data.push((chain.clone(), asset.clone()));
            // }
            data.push((chain_id.clone(), chain_data));
        }
        data
    }

    pub fn get_all_deposit_fee_data(&self) -> Vec<(String, &ChainTransferDepositData)> {
        let mut data: Vec<(String, &ChainTransferDepositData)> = Vec::new();
        for (chain_id, chain_data) in self.polkadot_deposit.iter() {
            data.push((chain_id.clone(), chain_data));
        }
        data
    }


    
    // pub fn get_deposit_data(&self, asset_id: &str) -> Option<&DepositData> {
    //     self.polkadot_deposit.get(asset_id)
    // }
    // pub fn get_transfer_fee(&self, asset_id: &str) -> Option<&TransferData> {
    //     self.polkadot_transfer.get(asset_id)
    // }
    // pub fn get_deposit_fee(&self, asset_id: &str) -> Option<&DepositData> {
    //     self.polkadot_deposit.get(asset_id)
    // }
}