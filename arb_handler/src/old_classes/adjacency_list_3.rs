use crate::token;
// use crate::adj_list_node::{AdjListNode, AdjListNodeOption, TokenNodeIterator};
use std::cell::RefCell;
use std::rc::Rc;

struct AdjList3{
    head: Node3,
    length: u32
}

#[derive(Clone, Debug)]
struct AdjListNode3{
    // state: Node3,
    pub content: NodeData3,
    next: Node3
}

#[derive(Clone, Debug)]
struct NodeData3{
    pub symbol: String,
    weight: (u128,u128)
}

#[derive(Clone, Debug)]
enum Node3{
    ListNode(Box<AdjListNode3>),
    None,
    
}

impl AdjListNode3{
    pub fn new(symbol: String) -> AdjListNode3{
        let content = NodeData3{symbol, weight: (0,0)};
        AdjListNode3{
            // state: Node3::ListNode,
            content,
            next: Node3::None
        }
    }

    pub fn push(&mut self, symbol: String){
        match self.next{
            Node3::ListNode(ref mut next) => next.push(symbol),
            Node3::None => {
                let new_node = AdjListNode3::new(symbol);
                self.next = Node3::ListNode(Box::new(new_node));
            }
        }
    }

    pub fn delete(&mut self, symbol: String){
        match self.next {
            Node3::ListNode(ref mut next) => {
                if next.content.symbol == symbol {
                    println!("Delete value {}", next.content.symbol);
                    self.next = next.next.clone();
                }
            },
            Node3::None => {
                if self.content.symbol == symbol {
                    self.content.symbol = "".to_string();
                } else {
                    println!("Could not find {} in list", symbol);
                }
            }
        }
    }
}