#[macro_use]
pub mod utils;
pub mod node;
pub mod sphere_node;
pub mod camera_node;
pub mod node_3d;
pub mod scene;
pub mod scene_items;
pub mod render_system;


//use self::utils::Utils;
extern crate cgmath;
use self::scene::Scene;
use self::camera_node::CameraNode; 
use self::node_3d::Node3D;
use self::sphere_node::SphereNode;
use self::scene_items::{Color, Ray};
use self::node::Node; 
use self::render_system::RenderSystem;

use cgmath::{Point3, Vector3, Matrix4};
use std::rc::{Weak, Rc};
use std::cell::RefCell;

fn main() {
    //-------------- Create empty Scene ---------------
    let camera = Rc::new(RefCell::new(CameraNode::new(
        "main_camera".to_string(),
        Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)),
        Vector3::new(0.0, 0.0, -1.0),
        800,
        600
    )));

    let root = Rc::new(RefCell::new(Node3D::new(
        "root".to_string(),
        Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)),
    )));

    let root_move = Rc::clone(&root);
    let cam_move = Rc::clone(&camera);
    let scene = Scene::new("main_scene".to_string(), root_move, cam_move);

    //-------------- Add Sphere's to Scene ------------

    let sphere1 = Rc::new(RefCell::new(SphereNode::new(
        "Sphere 1".to_string(),
        Matrix4::from_translation(Vector3::new(0.0, 0.0, -5.0)),
        1.0,
        Color::new(0.0, 255.0, 0.0)
    )));

    let sphere2 = Rc::new(RefCell::new(SphereNode::new(
        "Sphere 2".to_string(),
        Matrix4::from_translation(Vector3::new(-3.0, 1.0, -6.0)),
        1.0,
        Color::new(0.0, 255.0, 0.0)
    )));

    let sphere3 = Rc::new(RefCell::new(SphereNode::new(
        "Sphere 3".to_string(),
        Matrix4::from_translation(Vector3::new(2.0, 1.0, -4.0)),
        1.0,
        Color::new(0.0, 255.0, 0.0)
    )));

    let sphere_root = Rc::new(RefCell::new(Node3D::new(
        "Sphere root".to_string(),
        Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)),
    )));    

    value!(sphere_root).add_child(sphere1);
    value!(sphere_root).add_child(sphere2);
    value!(sphere_root).add_child(sphere3);
    value!(root).add_child(sphere_root);
    //------------ Render Scene ---------------
    
    let renderer = RenderSystem {
        output_path: "output.png".to_string()
    };

    println!("Render!");
    renderer.render(&scene, camera);
}   
