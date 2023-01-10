use crate::token::Token;
use crate::adj_list_node::{AdjListNode, AdjListNodeOption, AdjListNodeRef, TokenNodeIterator};

// use std::borrow::Borrow;
// use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
// struct TokenGraph{
//     token_nodes: Vec<TokenNode>,
//     adj_list: Vec<Vec<TokenNode>>
// }

//Implementing a Linked List from scratch
#[derive(PartialEq, Debug)]
pub struct AdjacencyList{
    pub list_head: AdjListNodeOption,
    pub length: usize,
}

impl AdjacencyList{
    pub fn new_empty() -> Self{
        AdjacencyList { list_head: None, length: 0 }
    }

    pub fn new(token_symbol: String, token: Token) -> Self{
        let new_head = AdjListNode::new_head(token_symbol, token);

        AdjacencyList { 
            list_head: Some(new_head), 
            length: 1 
        }
    }

    pub fn push_end(&mut self, token_symbol: String, token: Token, weight: (u128,u128)){
        let new_token = AdjListNode::new(token_symbol, token, weight);
        let tail_node = self.get_tail();
        match tail_node {
            //link tail node to new node
            Some(node) =>{
                node.borrow_mut().next = Some(Rc::clone(&new_token));
                new_token.borrow_mut().prev = Some(Rc::clone(&node));
                self.length += 1;
                // println!("Length + 1 = {}", self.length)
            },
            //Empty list, assign list head
            None => {
                self.list_head = Some(Rc::clone(&new_token));
                self.length += 1;
            },
        }
    }

    pub fn pop_end(&mut self) -> AdjListNodeOption {
        let current_tail = self.get_tail();
        match current_tail{
            //Get previous node and remove its link to current tail
            Some(ref node) => {
                let new_tail = &node.borrow().prev;
                match new_tail{
                    Some(node) => {
                        node.borrow_mut().next = None;
                        self.length -= 1;
                        // println!("Length - 1 = {}", self.length);
                        
                    },
                    _ => {
                        println!("Removing head");
                        self.list_head = None;
                        self.length -= 1;
                    } 
                }
                
            },
            _ => println!("List already empty")
        }
        current_tail
    }

    pub fn pop_end_2(&mut self) -> AdjListNodeOption{
        let old_tail = self.get_tail().take().map(|old_tail| {
            // let old_tail_display = old_tail.
            println!("Self.get_tail.take() = {:?}", old_tail.borrow().token_symbol);
            // let mut iter = self.node_iterator();
            // let mut temp = iter.next();
            let new_tail = &old_tail.borrow().prev;
            match new_tail{
                Some(new_tail) => {println!("Pop Node"); new_tail.borrow_mut().next = None; self.length -= 1;},
                None => {
                    println!("Removing head");
                        self.list_head = None;
                        self.length -= 1;
                }
            }
            old_tail.clone()
        });
        println!("old_tail = {}", old_tail.clone().unwrap().borrow().token_symbol);
        old_tail
    }

    //Search list for token symbol
    pub fn search(&self, token_symbol: String) -> AdjListNodeOption{
        for node in self.node_iterator(){
            if token_symbol == node.borrow().token_symbol{
                return Some(Rc::clone(&node));
            }
        }
        return None
    }

    //select node with search() and splice it out
    pub fn delete(&mut self, token_symbol: String){
        let target_node = self.search(token_symbol);
        match target_node{
            //Get pointer to target node, link its .prev <---> .next
            Some(target_node) => {
                let target = target_node.borrow();
                let prev = &target.prev;
                let next = &target.next;

                //Set prev.next = .next
                match &prev{
                    Some(prev) => {
                        let mut prev_token = prev.borrow_mut();
                        prev_token.next = match next{
                            Some(ref next) => {
                                Some(Rc::clone(next))
                            },
                            None => {
                                None
                            }
                        };
                    },
                    //IF prev matches none, we're removing the head. So push list head up 1
                    None => {
                        self.push_list_head();
                    }
                };
                //set next.prev = .prev
                match &next{
                    Some(next) => {
                        let mut next_token = next.borrow_mut();
                        next_token.prev = match prev{
                            Some(ref prev) => {
                                Some(Rc::clone(prev))
                            },
                            None => None
                        };
                        
                    }
                    None => ()
                }
                self.length -= 1;
                // println!("Token removed")
            }
            None => println!("No token found")
        }
    }

    //Get tail using .fold(), could also use .last()
    pub fn get_tail(&self) -> AdjListNodeOption {
        let mut iter = self.node_iterator();
        let tail_node = iter.fold(None, |acc, x|Some(x));
        tail_node
    }

    //use when removing first token, push the list head to next token
    pub fn push_list_head(&mut self){
        let mut iter = self.node_iterator();
        let first = iter.next();
        match first{
            Some(x) => {
                let next = iter.next();
                self.list_head = next;
            },
            None => self.list_head = None
        }
    }

    pub fn print_test(&self){
        println!("{:?}", self.list_head)
    }

    //Create a token node iterator starting at the list head
    pub fn node_iterator(&self) -> TokenNodeIterator{
        match &self.list_head{
            Some(head) => {
                TokenNodeIterator::new(Some(Rc::clone(head)))
            },
            _ => TokenNodeIterator::new(None),
        }
    }

    pub fn print_items(&self) {
        println!("Length: {}", self.length);
        let head = &self.list_head;
        match head {
            Some(head) =>{
                // let symbol: String = &head.borrow().token_symbol;
                // println!("Token head: {}", head.borrow_mut().token_symbol);
            }
            _ => println!("NO LIST HEAD"),
        }
        // println!("Token head: {}", self.list_head.borrow_mut().token_symbol)
        for node in self.node_iterator() {
            println!("Edge to {}. [{},{}]", node.borrow().token_symbol, node.borrow().weight.0, node.borrow().weight.1);
        }
    }

    pub fn get_list_head_symbol(&self) -> String{
        let head = &self.list_head;
        match head{
            Some(head) => head.borrow().token_symbol.to_string(),
            None => panic!("No list head symbol"),
        }
    }

    pub fn get_list_as_vec(&self) -> Vec<AdjListNodeRef>{
        let mut tokens: Vec<AdjListNodeRef> = Vec::new();
        for node in self.node_iterator(){
            tokens.push(node);
        }
        tokens
    }


}

//print current token symbol and symbol of prev and next
pub fn print_token(token: AdjListNodeOption){
    match token{
        Some(token) => {
            let token = token.borrow();
            print!("Current: {}, ", token.token_symbol);
            let token_next = &token.next;
            match token_next{
                Some(token_next) => {
                    let token_next = token_next.borrow();
                    print!("next: {}, ", token_next.token_symbol);
                },
                None => print!("next: None, ")
            }
            let token_prev = &token.prev;
            match token_prev{
                Some(token_prev) => {
                    let token_prev = token_prev.borrow();
                    print!("prev: {}", token_prev.token_symbol);
                },
                None => print!("prev: None")
            }
        },
        None => println!("None")
    }
}