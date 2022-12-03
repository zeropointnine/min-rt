/// Logic for rendering a `Scene` to a `Canvas`
///
/// This is a reasonably faithful translation of the pseudo-code from the book
/// ["Computer Graphics from Scratch"](https://gabrielgambetta.com/computer-graphics-from-scratch/)
/// by Gabriel Gambetta.

use std::sync::{Arc, RwLock};
use std::thread;
use crate::cgmath::{InnerSpace, Vector3, Rotation};
use crate::base::color::Color;
use crate::scene::scene::{Light, Scene, Specs, Sphere};
use crate::canvas::canvas::Canvas;
use crate::canvas::u8_canvas::U8Canvas;
use crate::util::maths;

const EPSILON: f64 = 0.001;
const RECURSION_DEPTH: usize = 3;

pub fn render_to_canvas_ranged(
        scene: &Arc<RwLock<Scene>>,
        sub_canvas: &mut dyn Canvas<Color>,
        full_canvas_row_start: usize,
        full_canvas_row_end: usize,
        full_canvas_height: usize) {

    let scene = scene.read().unwrap();

    let full_canvas_num_cols = sub_canvas.get_width() as f64;
    let full_canvas_num_rows = full_canvas_height as f64;
    let canvas_width = scene.specs.canvas_width;  // rem, scene space (not data grid values)
    let canvas_height = scene.specs.canvas_height;
    let canvas_width_half = scene.specs.canvas_width * 0.5;
    let canvas_height_half = scene.specs.canvas_width * 0.5;

    let mut viewport_width = scene.specs.viewport_width;
    viewport_width *= full_canvas_num_cols / full_canvas_num_rows; // Adjust for canvas grid aspect ratio
    viewport_width *= scene.specs.pixel_ar; // Adjust for pixel aspect ratio (viz., for terminal output)
    let viewport_height = scene.specs.viewport_height;

    for iy in full_canvas_row_start..full_canvas_row_end {

        let full_canvas_iy = iy - full_canvas_row_start;

        let y = maths::map(iy as f64,
           0.0, full_canvas_num_rows, canvas_height_half, -canvas_height_half);
        let y = y * (viewport_height / canvas_height);

        for ix in 0..sub_canvas.get_width() {

            let x = maths::map(ix as f64,
                0.0, full_canvas_num_cols, -canvas_width_half, canvas_width_half);
            let x = x * (viewport_width / canvas_width);

            let d = canvas_to_viewport(x, y, &scene.specs);
            let quat = scene.specs.camera_orientation.clone();
            let d = quat.rotate_vector(d);

            let o = scene.specs.camera_pos;
            let color = trace_ray(o, d, 1.0, f64::INFINITY, &scene, -1, RECURSION_DEPTH);

            sub_canvas.set_value(ix, full_canvas_iy, &color);
        }
    }
}

pub fn render_to_canvas_all(
        scene: &Arc<RwLock<Scene>>,
        canvas: &mut dyn Canvas<Color>) {

    let height = canvas.get_height();
    render_to_canvas_ranged(scene, canvas, 0_usize, height, height);
}

pub fn render_to_canvas_all_mt(
        scene: &Arc<RwLock<Scene>>,
        canvas: &mut dyn Canvas<Color>,
        worker_count: usize) {

    let canvas_full_height = canvas.get_height();
    let mut handles = Vec::new();

    for i in 0..worker_count {

        // make arc clone
        let scene = Arc::clone(scene);

        // make "subcanvas"
        let y_span = canvas.get_height() / worker_count;
        let y_start = i * y_span;
        let mut y_end = (i + 1) * y_span;
        if i == worker_count - 1 {
            y_end = canvas.get_height();
        }
        let sub_canvas_height = y_end - y_start;
        let mut sub_canvas = U8Canvas::new(canvas.get_width(), sub_canvas_height);

        // make thread and do work
        let handle = thread::spawn(move || {
            render_to_canvas_ranged(&scene, &mut sub_canvas, y_start, y_end, canvas_full_height);
            (y_start, sub_canvas)
        });

        handles.push(handle);
    }

    // Blocks until next thread is finished
    // Copy the sub_canvas data into the mut main canvas
    for handle in handles {
        let (y_start, sub_canvas) = handle.join().unwrap();
        for y in y_start..(y_start + &sub_canvas.get_height()) {
            for x in 0..sub_canvas.get_width() {
                let sub_canvas_y = y - y_start;
                let color = sub_canvas.get_value(x, sub_canvas_y);
                canvas.set_value(x, y, &color);
            }
        }
    }
}

/// Don't understand how the canvas width and height properties are useful, but
fn canvas_to_viewport(x: f64, y: f64, specs: &Specs) -> Vector3<f64> {
    let x = x * specs.viewport_width / specs.canvas_width;
    let y = y * specs.viewport_height / specs.canvas_height;
    Vector3::<f64>::from([x, y, specs.viewport_distance])
}

/// Returns the two 'distances' on a ray where it intersects a sphere.
fn intersect_ray_sphere(
        origin: Vector3<f64>,
        direction: Vector3<f64>,
        sphere: &Sphere) -> (f64, f64) {

    let r = sphere.radius;
    let c0 = origin - &sphere.center;
    let a = direction.magnitude2(); // ie, d dot d
    let b = 2.0 * &c0.dot(direction.clone());
    let c = &c0.dot(c0.clone()) - (r * r);

    let discriminant = b * b  -  4.0 * a * c;
    if discriminant < 0.0 {
        return (f64::NEG_INFINITY, f64::NEG_INFINITY);
    }

    let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b - discriminant.sqrt()) / (2.0 * a);
    (t1, t2)
}

/// Returns sphere index and closest_t
fn get_closest_ray_sphere_intersection(
        origin: Vector3<f64>,
        direction: Vector3<f64>,
        t_min:f64,
        t_max: f64,
        spheres: &Vec<Sphere>,
        sphere_ignore_index: i32) -> Option<(f64, usize)> {

    let mut result: Option<(f64, usize)> = None;

    let mut closest_t  = f64::INFINITY;
    for (i, sphere) in spheres.iter().enumerate() {
        if i as i32 == sphere_ignore_index {
            continue;
        }
        let (t1, t2) = intersect_ray_sphere(origin, direction, &sphere);
        if maths::contains(t1, t_min, t_max) && t1 < closest_t {
            closest_t = t1;
            result = Some((t1, i));
        }
        if maths::contains(t2, t_min, t_max) && t2 < closest_t {
            closest_t = t2;
            result = Some((t2, i));
        }
    }
    result
    // todo return borrowed sphere reference rather than index?
}

fn trace_ray(
    origin: Vector3<f64>,
    direction: Vector3<f64>,
    distance_min: f64,
    distance_max: f64,
    scene: &Scene,
    ignore_sphere_index: i32,
    recursion_depth: usize) -> Color {

    let mut color;

    // Compute local color
    let option = get_closest_ray_sphere_intersection(origin, direction, distance_min, distance_max, &scene.spheres, ignore_sphere_index);
    if option.is_none() {
        color = scene.specs.background_color.clone();
        return color;
    }
    let (t1, sphere_index) = option.unwrap();
    let sphere = &scene.spheres[sphere_index];
    let p = origin + (direction * t1);
    let mut n = p - sphere.center;
    n = n / n.magnitude();
    let neg_d = direction * -1.0;
    let intensity = compute_lighting(p, n, neg_d, sphere.specular, &scene);
    color = sphere.color * intensity;

    // Reflected color
    if sphere.reflective > 0.0 && recursion_depth > 0 {
        let neg_d = direction * -1.0;
        let r2 = reflect_ray(neg_d, n);
        // Recursion action
        let reflected_color = trace_ray(p, r2, EPSILON, f64::INFINITY, scene, -1, recursion_depth - 1);
        color = Color::lerp(color, reflected_color, sphere.reflective);
    }

    // Transparency
    if sphere.transparency > 0.0 {
        let trans_color
            = trace_ray(p, direction, EPSILON, distance_max, scene, sphere_index as i32, 3);
        color = Color::lerp(color, trans_color, sphere.transparency);
    }

    return color
}

fn compute_lighting(
        p: Vector3<f64>,
        n: Vector3<f64>,
        v: Vector3<f64>,
        s: f64,
        scene: &Scene) -> f64 {

    let mut final_intensity = 0.0;

    for light in &scene.lights {

        let mut light_intensity:f64 = 0.0;

        let l: Vector3<f64>;
        let t_max;
        let factor: f64;

        match light {
            Light::Ambient { intensity } => {
                final_intensity += intensity;
                continue;
            },
            Light::Point{ intensity, position} => {
                l = position - p;
                t_max = 1.0;
                factor = *intensity;
            },
            Light::Directional { intensity, direction } => {
                l = direction.clone();
                t_max = f64::INFINITY;
                factor = *intensity;
            }
        }

        // Get shadow attenuation factor
        let shadow_attenuation: f64;
        let option = get_closest_ray_sphere_intersection(p, l, EPSILON, t_max, &scene.spheres, -1);
        match option {
            Some((distance, index)) => {
                // Attenuate by the object's amount of opacity
                shadow_attenuation = 1.0 - scene.spheres[index].transparency;
            },
            _ => shadow_attenuation = 0.0
        }
         if shadow_attenuation == 1.0 {
            continue;
        }

        // Diffuse
        let n_dot_l = n.dot(l);
        if n_dot_l > 0.0 {
            let modif = factor * n_dot_l / (n.magnitude() * l.magnitude());
            light_intensity += modif;
        }

        // Specular
        if s > 0.0 {
            let r = n * (2.0 * n.dot(l));
            let r = &r - &l;
            let r_dot_v = r.dot(v);
            if r_dot_v > 0.0 {
                light_intensity += factor * (r_dot_v / (r.magnitude() * v.magnitude())).powf(s);
            }
        }

        // Apply shadow attenuation
        light_intensity *= 1.0 - shadow_attenuation;

        final_intensity += light_intensity;
    }
    final_intensity
}

fn reflect_ray(r: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
    2.0 * n * n.dot(r)  -  r
}
