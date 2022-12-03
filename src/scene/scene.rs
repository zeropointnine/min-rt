use crate::cgmath::{Quaternion, Vector3};
use crate::base::color::Color;

/// Simple data structure of the objects for a 3d scene, including lights.
/// Plus the `specs` needed to render the scene.
/// This is the scene's data model.
#[derive(Debug)]
pub struct Scene {
    pub spheres: Vec::<Sphere>,
    pub lights: Vec::<Light>,
    pub specs: Specs
}

// ---

#[derive(Debug)]
pub struct Specs {
    /// Canvas dimensions in world units (unclear about how this is useful tbh)
    pub canvas_width: f64,
    pub canvas_height: f64,

    /// Viewport dimensions in world units
    pub viewport_width: f64,
    pub viewport_height: f64,

    /// Distance from camera to viewport
    pub viewport_distance: f64,

    /// Aspect ratio of a pixel. This would conventionally be 1.0.
    /// If outputting text to the terminal, would be font-dependent,
    /// in the neighborhood of ~0.6 to ~0.8.
    pub pixel_ar: f64,

    /// Camera
    pub camera_pos: Vector3<f64>,
    pub camera_orientation: Quaternion<f64>,

    /// Background color
    pub background_color: Color,
}

impl Specs {
    pub fn new_with_defaults() -> Specs {
        Specs {
            canvas_width: 1.0,
            canvas_height: 1.0,
            viewport_width: 1.0,
            viewport_height: 1.0,
            viewport_distance: 1.0,
            pixel_ar: 1.0,
            camera_pos: Vector3::<f64>::new(0.0, 0.0, 0.0),
            camera_orientation: Quaternion::<f64>::new(1.0, 0.0, 0.0, 0.0),
            background_color: Color::new_black(),
        }
    }
}

// ---

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub color: Color,
    pub specular: f64,
    pub reflective: f64,
    pub transparency: f64,
}

#[derive(Clone, Copy, Debug)]
pub enum Light {
    Ambient { intensity: f64 },
    Point { intensity: f64, position: Vector3<f64> },
    Directional { intensity: f64, direction: Vector3<f64> }
}
