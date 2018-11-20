mod scene;
extern crate cgmath;
use self::scene::{Scene, CameraNode, Node3D, SphereNode, Color, Node};
use cgmath::{Point3, Vector3, Matrix4};
use std::rc::{Weak, Rc};
use std::cell::RefCell;

fn main() {
    //-------------- Create empty Scene ---------------
    let camera = Rc::new(RefCell::new(CameraNode::new(
        "main_camera".to_string(),
        Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)),
        Vector3::new(0.0, 0.0, -1.0),
        1080,
        720
    )));

    let root = Rc::new(RefCell::new(Node3D::new(
        "root".to_string(),
        Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)),
    )));

    let root_move = Rc::clone(&root);
    let scene = Scene::new("main_scene".to_string(), root_move, camera);

    //-------------- Add Sphere to Scene ------------

    let sphere = Rc::new(RefCell::new(SphereNode::new(
        "Sphere 1".to_string(),
        Matrix4::from_translation(Vector3::new(0.0, 0.0, -2.0)),
        1.0,
        Color::new(0.0, 255.0, 0.0)
    )));

    (*(*root).borrow_mut()).add_child(sphere);

    //------------ Render Scene (TODO) ---------------
    
}   
