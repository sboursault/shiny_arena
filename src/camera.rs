
use bevy::{prelude::OrthographicCameraBundle, render::camera::{ScalingMode}, math::Vec3};


pub fn new_camera_2d() -> OrthographicCameraBundle {
    let mut camera = OrthographicCameraBundle::new_2d(); // Orthographic for 2d games, use Perspective when you don't need a concept of depth
    
    // let's keep proportions on window resized
    camera.orthographic_projection.scaling_mode = ScalingMode::FixedHorizontal; 
    camera.transform.scale = Vec3::new(10., 10., 1.);
    camera
}