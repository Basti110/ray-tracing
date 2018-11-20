mod scene;
use std::rc::{Weak, Rc};
use std::cell::RefCell;

pub struct Node {
    childs: Vec<Rc<RefCell<Node>>>,
    pub value: usize,
}

impl Node {
    pub fn new(value: usize) -> Node {
        Node {
            childs: vec![],
            value: value,
        }
    }

    pub fn get_child(&self, index: usize) -> Rc<RefCell<Node>> {
        return Rc::clone(&(self.childs[index]));
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Node>>) {
        self.childs.push(Rc::clone(&child));
    }
}

fn main() {
    let a = Node::new(5);
    let b = Node::new(10);
    
    let node_1 = Rc::new(RefCell::new(a));
    let node_2 = Rc::new(RefCell::new(b));

    let mut root = Node::new(0);
    root.add_child(Rc::clone(&node_1));
    root.add_child(Rc::clone(&node_2));

    println!("node 1 = {}", (*(root.get_child(0))).borrow_mut().value);
    println!("node 2 = {}", (*(root.get_child(1))).borrow_mut().value);

    (*node_1).borrow_mut().value = 500;
    (*node_2).borrow_mut().value = 1000;

    println!("node 1 = {}", (*(root.get_child(0))).borrow_mut().value);
    println!("node 2 = {}", (*(root.get_child(1))).borrow_mut().value);
}   
