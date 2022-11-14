// use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

use crate::token::Token;

//use rc/refcell as pointer to nodes for linked list
type TokenNodeRef = Rc<RefCell<TokenNode>>;
pub type TokenNodeOption = Option<TokenNodeRef>;

#[derive(PartialEq, Debug)]
pub struct TokenNode{
    pub token_symbol: String,
    pub next: TokenNodeOption,
    pub prev: TokenNodeOption,
}

impl TokenNode{
    //Create a new node using simply token symbol
    pub fn new(token_symbol: String) -> TokenNodeRef{
        Rc::new(RefCell::new(TokenNode{
            token_symbol,
            next: None,
            prev: None
        }))
    }

    // pub fn get_next(&mut self) -> Option<Box<TokenNode>>{
    //     let next_node = self.next;
    //     let mut x = match next_node{
    //         Some(inner) => inner,
    //         None => return None,
    //     };
    //     Some(x)
    // }

    // pub fn get_prev(&self) -> Option<Box<TokenNode>>{
    //     self.prev
    // }

    // pub fn get_token_symbol(&self) -> String{
    //     self.token_symbol
    // }

    // pub fn set_next(&self, token: TokenNode){
    //     self.next = Some(Box::new(token));
    // }

    // pub fn set_prev(&self, token: TokenNode){
    //     self.prev = Some(Box::new(token));
    // }

    // pub fn has_prev(&self) -> bool{
    //     let prev_node = self.prev;
    //     match prev_node{
    //         Some(x) => true,
    //         None => false
    //     }
    // }

    // pub fn has_next(&self) -> bool{
    //     let next_node = self.next;
    //     match next_node{
    //         Some(x) => true,
    //         None => false
    //     }
    // }
}

impl TokenNode{
    pub fn test(){
        println!("TEST")
    }
}

impl Drop for TokenNode {
    fn drop(&mut self) {
        println!("Node with this data -> '{}' just dropped", self.token_symbol);
    }
}

//Use Iterator to move through the linked list
pub struct TokenNodeIterator{
    current: TokenNodeOption
}

impl TokenNodeIterator{
    pub fn new(start_at: TokenNodeOption) -> Self{
        TokenNodeIterator { current: start_at }
    }
}

impl Iterator for TokenNodeIterator{
    type Item = TokenNodeRef;

    fn next(&mut self) -> TokenNodeOption {
        let current = &self.current;
        let mut result = None;

        self.current = match current {
            //Unwrap current token, copy pointer to result
            Some(ref current) => {
                result = Some(Rc::clone(current));
                //Get next node, copy pointer, wrap in option and assign to self.current
                match &current.borrow().next {
                    Some(next_node) => {
                        Some(Rc::clone(next_node))
                    },
                    _ => None
                }
            },
            _ => None
        };

        result
    }
}

mod tests {

    use super::*;
    
    #[test]
    fn test_new_node() {
        let node = TokenNode::new("node_1".to_string());

        assert_eq!(node, Rc::new(RefCell::new(TokenNode { token_symbol: "node_1".to_string(), next: None, prev: None})));
    }
}