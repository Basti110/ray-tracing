mod scene;
extern crate cgmath;
use self::scene::{Scene, CameraNode, Node3D, SphereNode, Color, Node, RenderSystem};
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

    //-------------- Add Sphere to Scene ------------

    let sphere = Rc::new(RefCell::new(SphereNode::new(
        "Sphere 1".to_string(),
        Matrix4::from_translation(Vector3::new(0.0, 0.0, -5.0)),
        1.0,
        Color::new(0.0, 255.0, 0.0)
    )));

    //------------ Render Scene ---------------
    
    let renderer = RenderSystem {
        output_path: "output.png".to_string()
    };
    
    renderer.render(&scene, camera);
}   
