use crate::token_graph_2::{PathData, PathType};
use crate::{NodePath, PathNode};
use std::fs::File;
use std::io::prelude::*;
use bigdecimal::BigDecimal;
use serde_json::{Value};
use serde::{Deserialize, Serialize};
use std::str;
use std::fs::OpenOptions;
pub struct ResultLogger;



impl ResultLogger {
    pub fn log(info: &str) {
        // Example: prepend log entries with a timestamp
        // println!("[{}] {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), info);
    }

    // You can add more functions for different log levels if needed
    pub fn error(info: &str) {
        // eprintln!("[{}] Error: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), info);
    }

    // pub fn log_results_default_polkadot(result_log: Vec<PathNode>, start_node_name: String){
    //     let json = serde_json::to_string_pretty(&result_log.clone()).unwrap();
    //     // Get the current timestamp
    //     let timestamp = chrono::Local::now().format("%Y-%m-%d___%H-%M-%S").to_string();
    //     let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    //     let time = chrono::Local::now().format("%H-%M-%S").to_string();
    
    //     // Construct the directory path for the current date
    //     let log_folder_path = format!("result_log_data_polkadot/{}", date);
    
    //     // Create a directory for the current date if it doesn't exist
    //     match std::fs::create_dir_all(&log_folder_path) {
    //         Ok(_) => println!("Directory created successfully"),
    //         Err(e) => println!("Error creating directory: {:?}", e),
    //     }
    
    //     // Construct the file path including the directory
    //     let log_data_path = format!("{}/{}_{}.json", log_folder_path, start_node_name, time);
    //     println!("Log data path: {}", log_data_path);
    //     let mut file = File::create(log_data_path).expect("Failed to create file");
    //     file.write_all(json.as_bytes()).expect("Failed to write data");
    
    //     // let log_path = format!("result_log.txt", start_node.get_asset_name(), timestamp);
    //     let best_path_value = result_log[result_log.len()-1].path_value;
    //     let result_log_string = format!("{} {} - {}", timestamp, start_node_name, best_path_value);
    //     let mut file = OpenOptions::new()
    //         .append(true)
    //         .create(true)
    //         .open("result_log.txt")
    //         .expect("Failed to open or create file");
    //     writeln!(file, "{}", result_log_string).expect("Failed to write data");
    // }
    
    pub fn log_results_default_kusama(result_log: Vec<PathNode>, start_node_name: String, input_amount: BigDecimal){
        let json = serde_json::to_string_pretty(&result_log.clone()).unwrap();
        // Get the current timestamp
        let timestamp = chrono::Local::now().format("%Y-%m-%d___%H-%M-%S").to_string();
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        let time = chrono::Local::now().format("%H-%M-%S").to_string();
    
        // Construct the directory path for the current date
        let log_folder_path = format!("default_log_data/kusama/{}/{}", date, input_amount.to_string());
    
        // Create a directory for the current date if it doesn't exist
        match std::fs::create_dir_all(&log_folder_path) {
            Ok(_) => println!("Directory created successfully"),
            Err(e) => println!("Error creating directory: {:?}", e),
        }
    
        // Construct the file path including the directory
        let log_data_path = format!("{}/{}_{}.json", log_folder_path, start_node_name, time);
        println!("Log data path: {}", log_data_path);
       
        let mut file = File::create(log_data_path).expect("Failed to create file");
        file.write_all(json.as_bytes()).expect("Failed to write data");
    
        // let log_path = format!("result_log.txt", start_node.get_asset_name(), timestamp);
        let best_path_value = &result_log[result_log.len()-1].path_value;
        let result_log_string = format!("{} {} - {}", timestamp, start_node_name, best_path_value);
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("result_log.txt")
            .expect("Failed to open or create file");
        writeln!(file, "{}", result_log_string).expect("Failed to write data");
    
        // result_log.clone()
    }
    
    pub fn log_results_default_polkadot(result_log: Vec<PathNode>, start_node_name: String, input_amount: BigDecimal){
        let json = serde_json::to_string_pretty(&result_log.clone()).unwrap();
        // Get the current timestamp
        let timestamp = chrono::Local::now().format("%Y-%m-%d___%H-%M-%S").to_string();
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        let time = chrono::Local::now().format("%H-%M-%S").to_string();
    
        // Construct the directory path for the current date
        let log_folder_path = format!("default_log_data/polkadot/{}/{}", date, input_amount.to_string());
    
        // Create a directory for the current date if it doesn't exist
        match std::fs::create_dir_all(&log_folder_path) {
            Ok(_) => println!("Directory created successfully"),
            Err(e) => println!("Error creating directory: {:?}", e),
        }
    
        // Construct the file path including the directory
        let log_data_path = format!("{}/{}_{}.json", log_folder_path, start_node_name, time);
        println!("Log data path: {}", log_data_path);
       
        let mut file = File::create(log_data_path).expect("Failed to create file");
        file.write_all(json.as_bytes()).expect("Failed to write data");
    
        // let log_path = format!("result_log.txt", start_node.get_asset_name(), timestamp);
        let best_path_value = &result_log[result_log.len()-1].path_value;
        let result_log_string = format!("{} {} - {}", timestamp, start_node_name, best_path_value);
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("result_log.txt")
            .expect("Failed to open or create file");
        writeln!(file, "{}", result_log_string).expect("Failed to write data");
    
        // result_log.clone()
    }
    
    pub fn log_results(path: NodePath) -> Vec<PathNode>{
        let start_node = path[0].borrow();
        let path_values = &start_node.path_values;
        let path_value_types = &start_node.path_value_types;
        let path_datas = &start_node.path_datas;
        let mut result_log: Vec<PathNode> = Vec::new();
        for(i, node) in path.iter().enumerate(){
            let path_node = PathNode{
                node_key: node.borrow().get_asset_key(),
                asset_name: node.borrow().get_asset_name(),
                path_value: path_values[i].to_string(),
                path_type: path_value_types[i].clone(),
                path_data: path_datas[i].clone(),
            };
            result_log.push(path_node);
        }
    
        let json = serde_json::to_string_pretty(&result_log.clone()).unwrap();
        // Get the current timestamp
        let timestamp = chrono::Local::now().format("%Y-%m-%d___%H-%M-%S").to_string();
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        let time = chrono::Local::now().format("%H-%M-%S").to_string();
    
        // Construct the directory path for the current date
        let log_folder_path = format!("result_log_data/{}", date);
    
        // Create a directory for the current date if it doesn't exist
        match std::fs::create_dir_all(&log_folder_path) {
            Ok(_) => println!("Directory created successfully"),
            Err(e) => println!("Error creating directory: {:?}", e),
        }
    
        // Construct the file path including the directory
        let log_data_path = format!("{}/{}_{}.json", log_folder_path, start_node.get_asset_name(), time);
        println!("Log data path: {}", log_data_path);
        // let log_data_path = format!("result_log_data/{}_{}.json", start_node.get_asset_name(), timestamp);
        // println!("Log data path: {}", log_data_path);
        // When creating the file, use the log_data_path which includes the directory
        let mut file = File::create(log_data_path).expect("Failed to create file");
        file.write_all(json.as_bytes()).expect("Failed to write data");
    
        // let log_path = format!("result_log.txt", start_node.get_asset_name(), timestamp);
        let best_path_value = &result_log[result_log.len()-1].path_value;
        let result_log_string = format!("{} {} - {}", timestamp, start_node.get_asset_name(), best_path_value);
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("result_log.txt")
            .expect("Failed to open or create file");
        writeln!(file, "{}", result_log_string).expect("Failed to write data");
    
        result_log.clone()
    }
    
    pub fn log_async_search_target(path: Vec<PathNode>, asset_name: String, relay: String) {
        let json = serde_json::to_string_pretty(&path.clone()).unwrap();
        // Get the current timestamp
        let timestamp = chrono::Local::now().format("%Y-%m-%d___%H-%M-%S").to_string();
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        let time = chrono::Local::now().format("%H-%M-%S").to_string();
    
        // Construct the directory path for the current date
        let log_folder_path = format!("target_log_data/{}/{}", relay, date);
    
        // Create a directory for the current date if it doesn't exist
        match std::fs::create_dir_all(&log_folder_path) {
            Ok(_) => println!("Directory created successfully"),
            Err(e) => println!("Error creating directory: {:?}", e),
        }
    
        // Construct the file path including the directory
        let log_data_path = format!("{}/{}_{}.json", log_folder_path, asset_name.clone(), time);
        println!("Log data path: {}", log_data_path);
        // let log_data_path = format!("result_log_data/{}_{}.json", start_node.get_asset_name(), timestamp);
        // println!("Log data path: {}", log_data_path);
        // When creating the file, use the log_data_path which includes the directory
        let mut file = File::create(log_data_path).expect("Failed to create file");
        file.write_all(json.as_bytes()).expect("Failed to write data");
    
        // let log_path = format!("result_log.txt", start_node.get_asset_name(), timestamp);
        let best_path_value = &path[path.len()-1].path_value;
        let result_log_string = format!("{} {} - {}", timestamp, asset_name, best_path_value);
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("result_log.txt")
            .expect("Failed to open or create file");
        writeln!(file, "{}", result_log_string).expect("Failed to write data");
    }
    
    pub fn log_results_target(path: NodePath) -> Vec<PathNode> {
        let target_node = path[path.len() - 1].borrow();
        let path_values = &target_node.path_values;
        let path_value_types = &target_node.path_value_types;
        let path_datas = &target_node.path_datas;
        let mut result_log: Vec<PathNode> = Vec::new();
        for(i, node) in path.iter().enumerate(){
            let path_node = PathNode{
                node_key: node.borrow().get_asset_key(),
                asset_name: node.borrow().get_asset_name(),
                path_value: path_values[i].to_string(),
                path_type: path_value_types[i].clone(),
                path_data: path_datas[i].clone(),
            };
            println!("{} : {}", node.borrow().get_asset_name(), path_values[i] );
            result_log.push(path_node);
        }
    
        let json = serde_json::to_string_pretty(&result_log.clone()).unwrap();
        // Get the current timestamp
        let timestamp = chrono::Local::now().format("%Y-%m-%d___%H-%M-%S").to_string();
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        let time = chrono::Local::now().format("%H-%M-%S").to_string();
    
        // Construct the directory path for the current date
        let log_folder_path = format!("target_log_data/{}", date);
    
        // Create a directory for the current date if it doesn't exist
        match std::fs::create_dir_all(&log_folder_path) {
            Ok(_) => println!("Directory created successfully"),
            Err(e) => println!("Error creating directory: {:?}", e),
        }
    
        // Construct the file path including the directory
        let log_data_path = format!("{}/{}_{}.json", log_folder_path, target_node.get_asset_name(), time);
        println!("Log data path: {}", log_data_path);
        // let log_data_path = format!("result_log_data/{}_{}.json", start_node.get_asset_name(), timestamp);
        // println!("Log data path: {}", log_data_path);
        // When creating the file, use the log_data_path which includes the directory
        let mut file = File::create(log_data_path).expect("Failed to create file");
        file.write_all(json.as_bytes()).expect("Failed to write data");
    
        // let log_path = format!("result_log.txt", start_node.get_asset_name(), timestamp);
        let best_path_value = &result_log[result_log.len()-1].path_value;
        let result_log_string = format!("{} {} - {}", timestamp, target_node.get_asset_name(), best_path_value);
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("result_log.txt")
            .expect("Failed to open or create file");
        writeln!(file, "{}", result_log_string).expect("Failed to write data");
    
        result_log.clone()
    }
    
    pub fn log_results_fallback(path: NodePath, relay: String) -> Vec<PathNode> {
        let target_node = path[path.len() - 1].borrow();
        let path_values = &target_node.path_values;
        let path_value_types = &target_node.path_value_types;
        let path_datas = &target_node.path_datas;
        let mut result_log: Vec<PathNode> = Vec::new();
        for(i, node) in path.iter().enumerate(){
            let path_node = PathNode{
                node_key: node.borrow().get_asset_key(),
                asset_name: node.borrow().get_asset_name(),
                path_value: path_values[i].to_string(),
                path_type: path_value_types[i].clone(),
                path_data: path_datas[i].clone(),
            };
            println!("{} : {}", node.borrow().get_asset_name(), path_values[i] );
            result_log.push(path_node);
        }
    
        let json = serde_json::to_string_pretty(&result_log.clone()).unwrap();
        // Get the current timestamp
        let timestamp = chrono::Local::now().format("%Y-%m-%d___%H-%M-%S").to_string();
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        let time = chrono::Local::now().format("%H-%M-%S").to_string();
    
        // Construct the directory path for the current date
        let log_folder_path = format!("fallback_log_data/{}/{}", relay.to_ascii_lowercase(), date);
    
        // Create a directory for the current date if it doesn't exist
        match std::fs::create_dir_all(&log_folder_path) {
            Ok(_) => println!("Directory created successfully"),
            Err(e) => println!("Error creating directory: {:?}", e),
        }
    
        // Construct the file path including the directory
        let log_data_path = format!("{}/{}_{}.json", log_folder_path, target_node.get_asset_name(), time);
        println!("Log data path: {}", log_data_path);
        // let log_data_path = format!("result_log_data/{}_{}.json", start_node.get_asset_name(), timestamp);
        // println!("Log data path: {}", log_data_path);
        // When creating the file, use the log_data_path which includes the directory
        let mut file = File::create(log_data_path).expect("Failed to create file");
        file.write_all(json.as_bytes()).expect("Failed to write data");
    
        // let log_path = format!("result_log.txt", start_node.get_asset_name(), timestamp);
        let best_path_value = &result_log[result_log.len()-1].path_value;
        let result_log_string = format!("{} {} - {}", timestamp, target_node.get_asset_name(), best_path_value);
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("result_log.txt")
            .expect("Failed to open or create file");
        writeln!(file, "{}", result_log_string).expect("Failed to write data");
    
        result_log.clone()
    }
    
    pub fn log_results_small(path: NodePath) -> Vec<PathNode>{
        let start_node = path[0].borrow();
        let path_values = &start_node.path_values;
        let path_value_types = &start_node.path_value_types;
        let path_datas = &start_node.path_datas;
        let mut result_log: Vec<PathNode> = Vec::new();
        for(i, node) in path.iter().enumerate(){
            let path_node = PathNode{
                node_key: node.borrow().get_asset_key(),
                asset_name: node.borrow().get_asset_name(),
                path_value: path_values[i].to_string(),
                path_type: path_value_types[i].clone(),
                path_data: path_datas[i].clone(),

            };
            result_log.push(path_node);
        }
    
        let json = serde_json::to_string_pretty(&result_log.clone()).unwrap();
        // Get the current timestamp
        let timestamp = chrono::Local::now().format("%Y-%m-%d___%H-%M-%S").to_string();
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        let time = chrono::Local::now().format("%H-%M-%S").to_string();
    
        // Construct the directory path for the current date
        let log_folder_path = format!("result_log_data/{}_small", date);
    
        // Create a directory for the current date if it doesn't exist
        match std::fs::create_dir_all(&log_folder_path) {
            Ok(_) => println!("Directory created successfully"),
            Err(e) => println!("Error creating directory: {:?}", e),
        }
    
        // Construct the file path including the directory
        let log_data_path = format!("{}/{}_{}.json", log_folder_path, start_node.get_asset_name(), time);
        println!("Log data path: {}", log_data_path);
        // let log_data_path = format!("result_log_data/{}_{}.json", start_node.get_asset_name(), timestamp);
        // println!("Log data path: {}", log_data_path);
        // When creating the file, use the log_data_path which includes the directory
        let mut file = File::create(log_data_path).expect("Failed to create file");
        file.write_all(json.as_bytes()).expect("Failed to write data");
    
        // let log_path = format!("result_log.txt", start_node.get_asset_name(), timestamp);
        let best_path_value = &result_log[result_log.len()-1].path_value;
        let result_log_string = format!("{} {} - {}", timestamp, start_node.get_asset_name(), best_path_value);
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("result_log.txt")
            .expect("Failed to open or create file");
        writeln!(file, "{}", result_log_string).expect("Failed to write data");
    
        result_log.clone()
    }
    
}