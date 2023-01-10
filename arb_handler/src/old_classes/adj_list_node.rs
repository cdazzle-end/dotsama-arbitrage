// use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

use crate::token::Token;

//use rc/refcell as pointer to nodes for linked list
pub type AdjListNodeRef = Rc<RefCell<AdjListNode>>;
pub type AdjListNodeOption = Option<AdjListNodeRef>;

#[derive(PartialEq, Debug)]
pub struct AdjListNode{
    pub token_symbol: String,
    pub token: Token,
    pub weight: (u128, u128),
    pub next: AdjListNodeOption,
    pub prev: AdjListNodeOption,
}

impl AdjListNode{
    //Create a new node using simply token symbol
    pub fn new(token_symbol: String, token: Token, weight: (u128,u128)) -> AdjListNodeRef{
        Rc::new(RefCell::new(AdjListNode{
            token_symbol,
            token,
            weight,
            next: None,
            prev: None
        }))
    }

    pub fn new_head(token_symbol: String, token: Token) -> AdjListNodeRef{
        Rc::new(RefCell::new(AdjListNode{
            token_symbol,
            token,
            weight: (0,0),
            next: None,
            prev: None
        }))
    }

}

impl AdjListNode{
    pub fn test(){
        println!("TEST")
    }
}

impl Drop for AdjListNode {
    fn drop(&mut self) {
        println!("Node with this data -> '{}' just dropped", self.token_symbol);
    }
}

//Use Iterator to move through the linked list
pub struct TokenNodeIterator{
    current: AdjListNodeOption
}

impl TokenNodeIterator{
    pub fn new(start_at: AdjListNodeOption) -> Self{
        TokenNodeIterator { current: start_at }
    }
}

impl Iterator for TokenNodeIterator{
    type Item = AdjListNodeRef;

    //return current node, set self to next
    fn next(&mut self) -> AdjListNodeOption {
        let current = self.current.clone();

        self.current = match &self.current {
            Some(ref current) => {
                match &current.borrow().next {
                    Some(next_node) => {
                        Some(Rc::clone(next_node))
                    },
                    _ => None
                }
            },
            _ => None
        };
       current
    }
}

mod tests {

    use super::*;
    
    // #[test]
    // fn test_new_node() {
    //     let node = AdjListNode::new("node_1".to_string());

    //     assert_eq!(node, Rc::new(RefCell::new(AdjListNode { token_symbol: "node_1".to_string(), next: None, prev: None})));
    // }
}