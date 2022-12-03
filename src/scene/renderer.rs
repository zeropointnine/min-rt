/// Logic for rendering a `Scene` to a `Canvas`
///
/// This is a reasonably faithful translation of the pseudo-code from the book
/// ["Computer Graphics from Scratch"](https://gabrielgambetta.com/computer-graphics-from-scratch/)
/// by Gabriel Gambetta.
///
/// Function and parameter names have been preserved for easy cross-referencing.

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
        scene: &Arc<RwLock<Scene>>, canvas: &mut dyn Canvas<Color>,
        canvas_y_start: usize, canvas_y_end: usize, canvas_full_height: usize) {

    let scene = scene.read().unwrap();
    let specs = &scene.specs;
    let canvas_grid_ar = canvas.get_width() as f64 / canvas_full_height as f64;
    // Account for pixel aspect ratio (viz., for ANSI color output in the terminal)
    let adjusted_viewport_width= specs.viewport_width * specs.pixel_ar;

    for iy in canvas_y_start..canvas_y_end {

        let canvas_y = iy - canvas_y_start;

        let mut y = maths::map(iy as f64, 0.0, canvas_full_height as f64,
            specs.canvas_height * 0.5, specs.canvas_height * -0.5);

        for ix in 0..canvas.get_width() {

            let mut x = maths::map(ix as f64, 0.0,canvas.get_width() as f64,
                specs.canvas_width * -0.5, specs.canvas_width * 0.5);

            x *= adjusted_viewport_width / specs.canvas_width;
            x *= canvas_grid_ar; // Account for canvas grid size
            y *= specs.viewport_height / specs.canvas_height;
            let d = canvas_to_viewport(x, y, &scene.specs);

            let quat = scene.specs.camera_orientation.clone();
            let d = quat.rotate_vector(d);

            let o = specs.camera_pos;
            let color = trace_ray(o, d, 1.0, f64::INFINITY, &scene, -1, RECURSION_DEPTH);

            canvas.set_value(ix, canvas_y, &color);
        }
    }
}

pub fn render_to_canvas_all(
    scene: &Arc<RwLock<Scene>>, canvas: &mut dyn Canvas<Color>) {
    let height = canvas.get_height();
    render_to_canvas_ranged(scene, canvas, 0_usize, height, height);
}

pub fn render_to_canvas_all_mt(scene: &Arc<RwLock<Scene>>, canvas: &mut dyn Canvas<Color>, worker_count: usize) {

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
fn intersect_ray_sphere(o: Vector3<f64>, d: Vector3<f64>, sphere: &Sphere) -> (f64, f64) {
    let r = sphere.radius;
    let c0 = o - &sphere.center;
    let a = d.magnitude2(); // ie, d dot d
    let b = 2.0 * &c0.dot(d.clone());
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
    sphere_ignore_index: i32)
    -> Option<(f64, usize)> {

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
    recursion_depth: usize)
    -> Color {

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
            = trace_ray(p, direction, EPSILON, distance_max, scene, sphere_index as i32, 0);
        color = Color::lerp(color, trans_color, sphere.transparency);
    }

    return color
}

fn compute_lighting(p: Vector3<f64>, n: Vector3<f64>, v: Vector3<f64>, s: f64, scene: &Scene) -> f64 {

    let mut i = 0.0;

    for light in &scene.lights {

        let l: Vector3<f64>;
        let t_max;
        let intens: f64;

        match light {
            Light::Ambient { intensity } => {
                i += intensity;
                continue;
            },
            Light::Point{ intensity, position} => {
                l = position - p;
                t_max = 1.0;
                intens = *intensity;
            },
            Light::Directional { intensity, direction } => {
                l = direction.clone();
                t_max = f64::INFINITY;
                intens = *intensity;
            }
        }

        // Shadow check
        let option = get_closest_ray_sphere_intersection(p, l, EPSILON, t_max, &scene.spheres, -1);
        if option.is_some() {
            continue; // todo make shadow honor the transparency of the object which is casting it
        }

        // Diffuse
        let n_dot_l = n.dot(l);
        if n_dot_l > 0.0 {
            let modif = intens * n_dot_l / (n.magnitude() * l.magnitude());
            i += modif;
        }

        // Specular
        if s > 0.0 {
            let r = n * (2.0 * n.dot(l));
            let r = &r - &l;
            let r_dot_v = r.dot(v);
            if r_dot_v > 0.0 {
                i += intens * (r_dot_v / (r.magnitude() * v.magnitude())).powf(s);
            }
        }
    }
    i
}

fn reflect_ray(r: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
    2.0 * n * n.dot(r)  -  r
}
