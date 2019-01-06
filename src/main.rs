#[macro_use]
pub mod utils;
pub mod node;
pub mod sphere_node;
pub mod camera_node;
pub mod node_3d;
pub mod scene;
pub mod scene_items;
pub mod render_system;
pub mod light;
pub mod plane;

//use self::utils::Utils;
extern crate cgmath;
use self::plane::Plane;
use self::scene::Scene;
use self::camera_node::CameraNode; 
use self::node_3d::Node3D;
use self::sphere_node::SphereNode;
use self::scene_items::{Color, Ray};
use self::node::Node; 
use self::render_system::RenderSystem;
use self::light::{DirectionalLight, SphericalLight, Light};

use cgmath::{Point3, Vector3, Matrix4, Deg};
use std::rc::{Weak, Rc};
use std::cell::RefCell;

fn main() {
    //-------------- Create empty Scene ---------------
    let camera = Rc::new(RefCell::new(CameraNode::new(
        "main_camera".to_string(),
        Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)),
        Vector3::new(0.0, 0.0, -1.0),
        600,
        600
    )));

    let root = Rc::new(RefCell::new(Node3D::new(
        "root".to_string(),
        Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)),
    )));

    let root_move = Rc::clone(&root);
    let cam_move = Rc::clone(&camera);
    let mut scene = Scene::new("main_scene".to_string(), root_move, cam_move);

    //-------------- Add Sphere's to Scene ------------

    let sphere1 = Rc::new(RefCell::new(SphereNode::new(
        "Sphere Green".to_string(),
        Matrix4::from_translation(Vector3::new(1.0, 0.0, -4.0)),
        0.5,
        Color::new_rgb(51, 255, 51)
    )));

    let sphere2 = Rc::new(RefCell::new(SphereNode::new(
        "Sphere Red".to_string(),
        Matrix4::from_translation(Vector3::new(2.0, -3.0, -5.0)),
        2.0,
        Color::new_rgb(255, 51, 51)
    )));

    let sphere3 = Rc::new(RefCell::new(SphereNode::new(
        "Sphere Blue".to_string(),
        Matrix4::from_translation(Vector3::new(-2.0, -3.0, -5.0)),
        2.0,
        Color::new_rgb(51, 51, 255)
    )));

    
    let sphere_root = Rc::new(RefCell::new(Node3D::new(
        "Sphere root".to_string(),
        Matrix4::from_translation(Vector3::new(0.0, 0.5, 0.0)),
    )));    

    value!(sphere_root).add_child(sphere1);
    //value!(sphere_root).add_child(sphere2);
    //value!(sphere_root).add_child(sphere3);
    

    //----------- Add Planes -------------------
    //let rad = Deg(180.0);
    //let s = Matrix4::from_scale(1.0);
    let r = Matrix4::from_angle_x(Deg(-90.0));
    let t = Matrix4::from_translation(Vector3::new(0.0, 0.0, -8.0));
    let plane1 = Rc::new(RefCell::new(Plane::new(
        "Plane back grey".to_string(),
        t*r,
        Color::new_rgb(200, 200, 200)
    )));

    let r = Matrix4::from_angle_z(Deg(-90.0));
    let t = Matrix4::from_translation(Vector3::new(4.0, 0.0, -6.0));
    let plane2 = Rc::new(RefCell::new(Plane::new(
        "Plane right green".to_string(),
        t*r,
        Color::new_rgb(128, 255, 128)
    )));

    let r = Matrix4::from_angle_z(Deg(90.0));
    let t = Matrix4::from_translation(Vector3::new(-4.0, 0.0, -6.0));
    let plane3 = Rc::new(RefCell::new(Plane::new(
        "Plane left red".to_string(),
        t*r,
        Color::new_rgb(255, 128, 128)
    )));

    let r = Matrix4::from_angle_z(Deg(0.0));
    let t = Matrix4::from_translation(Vector3::new(0.0, -3.99, -6.0));
    let plane4 = Rc::new(RefCell::new(Plane::new(
        "Plane down grey".to_string(),
        t*r,
        Color::new_rgb(200, 200, 200)
    )));

    let r = Matrix4::from_angle_z(Deg(180.0));
    let t = Matrix4::from_translation(Vector3::new(0.0, 3.99, -6.0));
    let plane5 = Rc::new(RefCell::new(Plane::new(
        "Plane up grey".to_string(),
        t*r,
        Color::new_rgb(200, 200, 200)
    )));

    //value!(sphere_root).add_child(plane1);
    //value!(sphere_root).add_child(plane2);
    //value!(sphere_root).add_child(plane3);
    //value!(sphere_root).add_child(plane4);
    //value!(sphere_root).add_child(plane5);
    
    //----------- Add Light to Scene ----------
    let light = Rc::new(RefCell::new(Light::Spherical(SphericalLight::new(
        "Light".to_string(),
        Matrix4::from_translation(Vector3::new(0.0, 3.0, -1.0)),
        Color::new_rgb(255, 255, 255),
        80.0
    ))));

    scene.lights = Rc::clone(&light);
    //value!(sphere_root).add_child(light);
    value!(root).add_child(sphere_root);

    //------------ Render Scene ---------------
    
    let renderer = RenderSystem {
        output_path: "output.png".to_string()
    };

    println!("Render!");
    renderer.render(&scene, camera);
}   
