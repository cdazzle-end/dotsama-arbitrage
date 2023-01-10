use crate::token::{self, Token};
use crate::adj_list_node::{AdjListNode, AdjListNodeOption, TokenNodeIterator};
use crate::{LiqPoolRegistry, AdjacencyList};

// use std::cell::RefCell;
// use std::rc::Rc;
// use std::borrow::Borrow;
// use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::Index;
use std::rc::Rc;

type GraphNodeRef = Rc<RefCell<GraphNode>>;
pub type GraphNodeOption = Option<GraphNodeRef>;

pub struct TokenGraph{
    pub token_nodes: Vec<GraphNodeRef>,
    pub adj_list_table: AdjacencyTable
}

pub struct AdjacencyTable{
    table: Vec<AdjacencyList>,
    tokens: Vec<String>,
    token_objs: Vec<Token>
    // weight: Vec<Vec<(u128, u128)>>,
}

#[derive(Debug)]
pub struct GraphNode{
    pub symbol: String,
    pub pred: GraphNodeOption,
    pub distance: u32,
    pub color: Color,
    pub adj_list: Rc<RefCell<AdjacencyList>>,
}

#[derive(PartialEq, Debug)]
pub enum Color{
    White,
    Gray,
    Black
}

impl TokenGraph {
    pub fn new_kar() -> Self{
        let liq_pools = LiqPoolRegistry::build_kar_liqpool_list();
        let table = AdjacencyTable::build_from_liqpool_list(liq_pools);
        TokenGraph { token_nodes: Vec::new(), adj_list_table: table }
    }

    pub fn new_all() -> Self{
        let liq_pools = LiqPoolRegistry::build_all_liqpool_list();
        let table = AdjacencyTable::build_from_liqpool_list(liq_pools);
        TokenGraph { token_nodes: Vec::new(), adj_list_table: table }
    }

    pub fn build_kar_graph(&mut self){
        let liq_pools = LiqPoolRegistry::build_kar_liqpool_list();
        let table = AdjacencyTable::build_from_liqpool_list(liq_pools);
        let mut graph_nodes: Vec<GraphNodeRef> = Vec::new();
        for list in table.table{
            let token = list.get_list_head_symbol();
            let graph_node = Rc::new(RefCell::new(GraphNode{
                symbol: token,
                pred: None,
                distance: 0,
                color: Color::White,
                adj_list: Rc::new(RefCell::new(list))
            }));
            graph_nodes.push(graph_node);
        }
        self.token_nodes = graph_nodes;
        // println!("GRAPH NODES: {:?}", self.token_nodes);
        
        // self.adj_list_table = table;
        // TokenGraph { token_nodes: graph_nodes, adj_list_table: table }
    }
    pub fn get_graph_node(&self, symbol: String) -> GraphNodeRef{
        for node in &self.token_nodes{
            if node.borrow().symbol == symbol{
                return Rc::clone(node)
            }
        }
        panic!("Cant find graph node")
    }

    pub fn print_graph_tokens(&self){
        println!("GRAPH NODES");
        for node in &self.token_nodes{
            println!("NODE {}, Adj List: ", node.borrow().symbol);
            node.borrow().adj_list.borrow().print_items();
            println!("")
        }
    }

    pub fn BFS(&mut self, symbol: String){
        let source_node = self.get_graph_node(symbol);
        source_node.borrow_mut().color = Color::Gray;
        let mut node_queue: VecDeque<GraphNodeRef> = VecDeque::new();
        node_queue.push_back(source_node);
        while !node_queue.is_empty(){
            let node = node_queue.pop_front().unwrap_or_else(||panic!("Queue should not be empty"));
            println!("{} -> Pop", node.borrow().symbol);
            for adj in node.borrow().adj_list.borrow().node_iterator(){
                let adj_node = self.get_graph_node(adj.borrow().token_symbol.clone());
                if adj_node.borrow().color == Color::White{
                    adj_node.borrow_mut().color = Color::Gray;
                    adj_node.borrow_mut().distance = node.borrow().distance + 1;
                    adj_node.borrow_mut().pred = Some(Rc::clone(&node));
                    println!("{} -> Gray/In queue", adj_node.borrow().symbol);
                    node_queue.push_back(adj_node)
                }
            }
            node.borrow_mut().color = Color::Black;
            println!("{} -> Black", node.borrow().symbol);
        }

    }

    //BFS path
    pub fn print_path(&self, token_1: String, token_2: String) {
        println!("1 {} 2 {}", &token_1, &token_2);
        let token_1 = self.get_graph_node(token_1);
        let token_2 = self.get_graph_node(token_2);
        if token_2.borrow().symbol == token_1.borrow().symbol{
            println!("{}", token_1.borrow().symbol);
        } else {
            match &token_2.borrow().pred{
                None => println!("NO TOKEN PATH"),
                Some(token_2_pred) => {
                    self.print_path(token_1.borrow().symbol.clone(), token_2_pred.borrow().symbol.clone());
                    println!("{}", token_2.borrow().symbol)
                }
            }
        }

}
}

// impl GraphNode{

// }

impl AdjacencyTable{
    pub fn new() -> Self{
        let table: Vec<AdjacencyList> = Vec::new();
        let tokens: Vec<String> = Vec::new();
        let token_objs: Vec<Token> = Vec::new();
        // let weight: Vec<Vec<(u128, u128)>> = Vec::new();
        AdjacencyTable { table, tokens, token_objs }
    }

    //Go through each liq_pool, add token pairs to graph
    // pub fn build_from_liqpool_list(liqpools: LiqPoolList) -> AdjacencyTable{
    //     let adj_lists: Vec<AdjacencyList> = Vec::new();
    //     let mut table = AdjacencyTable::new();
    //     let already_added: Vec<String> = Vec::new();
    //     for pool in liqpools.liq_pools{
    //         // pool.display_liq_pool();
    //         pool.get_pool_tokens();
    //         let token_1 = pool.tokens[0].clone();
    //         let token_2 = pool.tokens[1].clone();
    //         let liq_1 = &pool.liquidity[0];
    //         let liq_2 = &pool.liquidity[0];
    //         let weight = (liq_1.clone(), liq_2.clone());
    //         // if already_added.contains(token_1){

    //         // }

    //         table.add_token_pair_2(token_1, token_2, weight);
    //     }
    //     // AdjacencyTable { table: self.table, tokens: self.tokens }
    //     table
    // }

    pub fn add_token_pair_2(&mut self, token_1: Token, token_2: Token, weight: (u128, u128)){
        if self.token_objs.contains(&token_1){
            let token_1_list = self.get_list_by_token_obj(token_1.clone());
            token_1_list.push_end(token_2.symbol.clone(), token_2.clone(), weight);
        } else {
            let mut new_list = AdjacencyList::new(token_1.symbol.clone(), token_1.clone());
            new_list.push_end(token_2.symbol.clone(), token_2.clone(), weight);
            self.table.push(new_list);
            self.token_objs.push(token_1.clone());
        }
        //Add token_2 Adj List

        //Get list that matches token_2 from table
        let reverse_weight: (u128, u128) = (weight.1, weight.0);
        if self.token_objs.contains(&token_2){
            let token_2_list = self.get_list_by_token_obj(token_2.clone());
            token_2_list.push_end(token_1.symbol.clone(), token_1.clone(), reverse_weight);
        } else {
            //Create new list with Token 2, push new list to table, add Token 2 to self.tokens
            let mut new_list = AdjacencyList::new(token_2.symbol.clone(), token_2.clone());
            new_list.push_end(token_1.symbol.clone(), token_1.clone(), reverse_weight);
            self.table.push(new_list);
            self.token_objs.push(token_2.clone());
        }
    }

    //Take a token pair, add to their respective adjacency lists
    pub fn add_token_pair(&mut self, token_1: Token, token_2: Token, weight: (u128, u128)){
        // Add token_1 Adj List
        println!("Adding tokens {}, {}", token_1.symbol, token_2.symbol);

        //Get list that matches token_1 from table
        if self.tokens.contains(&token_1.symbol){
            let token_1_list = self.get_list_by_token(token_1.symbol.clone());
            token_1_list.push_end(token_2.symbol.clone(), token_2.clone(), weight);
        } else {
            let mut new_list = AdjacencyList::new(token_1.symbol.clone(), token_1.clone());
            new_list.push_end(token_2.symbol.clone(), token_2.clone(), weight);
            self.table.push(new_list);
            self.tokens.push(token_1.symbol.clone());
        }
    

        //Add token_2 Adj List

        //Get list that matches token_2 from table
        let reverse_weight: (u128, u128) = (weight.1, weight.0);
        if self.tokens.contains(&token_2.symbol){
            
            let token_2_list = self.get_list_by_token(token_2.symbol);
            token_2_list.push_end(token_1.symbol.clone(), token_1.clone(), reverse_weight);
            // println!("Adding to token 1: {} to token 2 {} list", token_1, token_2);
            // let mut token_2_list = self.get_list_by_token(token_2);
            // token_2_list.push_end(token_1);
        } else {
            //Create new list with Token 2, push new list to table, add Token 2 to self.tokens
            
            let mut new_list = AdjacencyList::new(token_2.symbol.clone(), token_2.clone());
            new_list.push_end(token_1.symbol.clone(), token_1.clone(), reverse_weight);
            self.table.push(new_list);
            self.tokens.push(token_2.symbol.clone());
            // println!("Create new list with token 2: {}", token_2);
            // self.table.push(new_list);
        }
    }

    //only if confirmed the specific list exists, get list by token symbol
    pub fn get_list_by_token(&mut self, token: String) -> &mut AdjacencyList{
        let len = self.table.len();
        let mut index = 0;
        while index < len{
        // for mut list in &self.table{
            let list = &self.table[index];
            let list_head = list.get_list_head_symbol();
            if list_head == token{
                let list = &mut self.table[index];
                return list
            }
            index += 1;
        }
        panic!("Could not retrieve adj list by token")
    }

    pub fn get_list_by_token_obj(&mut self, token: Token) -> &mut AdjacencyList{
        // for mut list in &self.table{
        //     let list_head = Rc::clone(&list.list_head.clone().unwrap()).borrow().token.clone();
        //     if list_head == token{
        //         // let r2_list = &mut self.table[1];
        //         // let r_list = &mut list;
        //         let mut list = list;
        //         return list;
        //     }
        // }
        let len = self.table.len();
        let mut index = 0;
        while index < len{
        // for mut list in &self.table{
            let list = &self.table[index];
            let list_head = list.list_head.as_ref().unwrap().borrow().token.clone();
            if list_head == token{
                let list = &mut self.table[index];
                return list
            }
            index += 1;
        }
        panic!("Could not retrieve adj list by token")
    }

    // pub fn get_list_head_from_token(&self, symbol: String) -> Rc<RefCell<AdjacencyList>>{
    //     for list in self.table{
    //         if symbol == list.get_list_head_symbol(){
    //             return Rc::new(RefCell::new(list))
    //         }
    //     }
    //     panic!("Could not match symbol to list in table")
    // }
    
    pub fn display_table(&self){
        println!("TOKEN GRAPH");
        println!("TOKENS: {:?}", self.tokens);
        // for token in &self.tokens{
        //     println!("token {}", token);
        // }
        for list in &self.table{
            println!("NEW LIST: {}", list.get_list_head_symbol());
            list.print_items()
        }
    }

    pub fn display_table_2(&self){
        for list in &self.table{
            let head = list.list_head.clone().unwrap();
            let symbol = &head.borrow().token_symbol;
            let chain = &head.borrow().token.chain;

            print!("[{}] {} -> ", chain, symbol);
            let tokens = list.get_list_as_vec();
            for token in tokens{
                print!("{}, ", token.borrow().token_symbol);
                
            }
            println!("");
            println!("");
        }
    }
}