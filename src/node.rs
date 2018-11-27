use std::rc::Rc;
use std::cell::RefCell;
use crate::{Ray, Color};
use cgmath::Vector3;

pub trait Node {
    fn get_parent(&self) -> Option<Rc<RefCell<Node>>>;
    fn get_child(&self, index: usize) -> Option<Rc<RefCell<Node>>>;
    fn add_child(&mut self, node: Rc<RefCell<Node>>);
    fn get_color(&self) -> Color;
    fn get_size(&self) -> usize;
    fn intersect(&self, ray: &Ray) -> Option<(f64, Vector3<f64>)>;
}