use crate::token;
use crate::adj_list_node::{AdjListNode, AdjListNodeOption, TokenNodeIterator};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub enum ListNode2<T>{
    None,
    Tail {item: T},
    Link {item: T, next: Box<ListNode2<T>>}
}

#[derive(Clone, Debug)]
pub struct NodeData{
    symbol: String,
    weight: (u128,u128)
}

#[derive(Clone)]
pub struct Cursor<T>{
    current: ListNode2<T>
}

impl<T> ListNode2<T> where T: Clone{
    pub fn new() -> ListNode2<T>{
        ListNode2::None
    }

    fn to_tail(&mut self, n: T){
        *self = match self{
            Self::None => Self::Tail{item: n},
            Self::Link {item:_, next:_} => Self::Tail{item: n},
            _ => panic!("Couldn't convert to tail")
        }
    }

    //Only converting a tail to link when pushing a new node 
    fn to_link(&mut self, n: T){
        *self = match self{
            Self::Tail{item} => {
                Self::Link {
                    item: item.clone(),
                    next: Box::new(Self::Tail {item: n})
                }
            },
            _ => {panic!("Couldn't convert to LINK")}
        }
    }

    fn to_none(&mut self) {
        *self = std::mem::replace(self, ListNode2::None);
    }

    fn to_next(&mut self, n: ListNode2<T>){
        *self = n;
    }

    pub fn push(&mut self, n: T){
        match self {
            Self::None => self.to_tail(n),
            Self::Tail {..}=> self.to_link(n),
            Self::Link {next, ..} => next.push(n)
        }
    }

    //pop off front of list
    pub fn pop(&mut self) -> Option<T>{
        match self {
            Self::None => None,
            Self::Tail { item } => {
                let item = item.clone();
                self.to_none();
                Some(item)
            },
            Self::Link { item, next } => {
                let mut n = Box::new(Self::None);
                let item = item.clone();
                std::mem::swap(next, &mut n);
                self.to_next(*n);
                // self = n;
                Some(item)
            }
        }
    }
}

impl NodeData{
    pub fn new(symbol: String, weight: (u128,u128)) -> NodeData{
        NodeData{symbol, weight}
    }
}

impl<T> IntoIterator for ListNode2<T> where T: Clone {
    type Item = T;
    type IntoIter = Cursor<T>;

    fn into_iter(self) -> Self::IntoIter {
        Cursor{
            current: self
        }
    }
}

impl <T> Iterator for Cursor<T> where T: Clone{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let nxt = match self.current.clone() {
            ListNode2::None => None,
            ListNode2::Tail {item} => {
                self.current = ListNode2::None;
                Some(item)
            },
            ListNode2::Link { item, next } => {
                // let mut n = Box::new(ListNode2::None);
                // std::mem::swap(next, &mut n);
                // self.current = *n;
                self.current = *next;

                Some(item)
            }
        };
        nxt
    }
}