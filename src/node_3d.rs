use crate::{Node, Ray, Color};
use std::rc::{Weak, Rc};
use cgmath::{Matrix4, Vector3};
use std::cell::RefCell;

pub struct Node3D {
    childs: Vec<Rc<RefCell<Node>>>,
    parent: Weak<Rc<RefCell<Node>>>,
    size: usize,
    pub name: String,
    pub frame_transform: Matrix4<f64>,
    pub world_transform: Matrix4<f64>,
}

impl Node3D {
    pub fn new(name: String, transform: Matrix4<f64>) -> Node3D {
        Node3D {
            childs: vec![],
            parent: Weak::new(),
            size: 0,
            name: name,
            frame_transform: transform,
            world_transform: transform,
        }
    }

    pub fn empty() -> Node3D {
        Node3D {
            childs: vec![],
            parent: Weak::new(),
            size: 0,
            name: "".to_string(),
            frame_transform: Matrix4::from_scale(0.0),
            world_transform: Matrix4::from_scale(0.0),
        }
    }
}

impl Node for Node3D {
    fn get_parent(&self) -> Option<Rc<RefCell<Node>>> {
        let strong = &self.parent.upgrade();
        let strong = match strong {
            Some(x) => x,
            None => return None,
        };
        return Some(Rc::clone(&(*strong))); //Some(Rc::clone(&(*(&self.parent))));
    }

    fn get_child(&self, index: usize) -> Option<Rc<RefCell<Node>>> {
        if index >= self.size {
            return None;
        }
        return Some(Rc::clone(&(self.childs[index])));
    }

    fn add_child(&mut self, node: Rc<RefCell<Node>>) {
        self.size += 1;
        self.childs.push(Rc::clone(&node));
    }

    fn get_size(&self) -> usize {
        return self.size;
    }

    fn intersect(&self, ray: &Ray) -> Option<(f64, Vector3<f64>)> {
        return None;
    }

    fn get_color(&self) -> Color {
        return Color::new(0.0, 0.0, 0.0);
    }

    fn get_world_transform(&self) -> Matrix4<f64> {
        return self.world_transform;
    }

    fn set_world_transform(&mut self, transform: Matrix4<f64>) -> () {
        self.world_transform = transform;
    }
}