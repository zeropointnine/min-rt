use crate::base::vec3::Vec3;
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
    pub canvas_width: f32,
    pub canvas_height: f32,

    /// Viewport dimensions in world units
    pub viewport_width: f32,
    pub viewport_height: f32,

    /// Distance from camera to viewport
    pub viewport_distance: f32,

    /// Aspect ratio of a pixel. This would conventionally be 1.0.
    /// If outputting text to the terminal, would be font-dependent,
    /// in the neighborhood of ~0.6 to ~0.8.
    pub pixel_ar: f32,

    /// Camera position
    pub camera_pos: Vec3,

    /// Background color
    pub background_color: Color
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
            camera_pos: Vec3::new_origin(),
            background_color: Color::new(255, 255, 255)
        }
    }
}

// ---

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub color: Color,
    pub specular: f32,
    pub reflective: f32
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, color: Color, specular: f32, reflective: f32) -> Sphere {
        Sphere { center, radius, color, specular, reflective }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Light {
    Ambient { intensity: f32 },
    Point { intensity: f32, position: Vec3 },
    Directional { intensity: f32, direction: Vec3 }
}