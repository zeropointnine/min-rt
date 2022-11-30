use crate::base::color::Color;
use crate::scene::scene::{Light, Scene, Specs, Sphere};
use crate::base::vec3::Vec3;
use crate::canvas::canvas::Canvas;
use crate::util::maths;

const EPSILON: f64 = 0.001;
const RECURSION_DEPTH: usize = 3;

/// Logic for rendering a `Scene`` onto a `Canvas`
///
/// This is a fairly literal translation of the pseudo-code from the book
/// ["Computer Graphics from Scratch"](https://gabrielgambetta.com/computer-graphics-from-scratch/)
/// by Gabriel Gambetta into Rust. The names of functions and parameters have been preserved for
/// easy cross-referencing.
///

// todo update function description comments
// todo consider making this a trait on Scene. or encapsulated in some other construct.

///
///
pub fn render_scene_to_canvas(scene: &Scene, canvas: &mut dyn Canvas<Color>) {

    let specs = &scene.specs;

    let canvas_grid_ar = canvas.get_width() as f64 / canvas.get_height() as f64;

    // Account for pixel aspect ratio (viz., for terminal text output)
    let adjusted_viewport_width= specs.viewport_width * specs.pixel_ar;

    let steps_x = canvas.get_width();
    let steps_y = canvas.get_height();

    let half_canvas_width = specs.canvas_width / 2.0;
    let half_canvas_height = specs.canvas_height / 2.0;

    for iy in 0..steps_y {

        let mut y = maths::map(iy as f64, steps_y as f64, 0.0, -half_canvas_height, half_canvas_height);

        for ix in 0..steps_x {

            let mut x = maths::map(ix as f64, 0.0, steps_x as f64, -half_canvas_width, half_canvas_width);

            x *= adjusted_viewport_width / specs.canvas_width;
            x *= canvas_grid_ar; // Account for canvas grid size
            y *= specs.viewport_height / specs.canvas_height;
            let d = canvas_to_viewport(x, y, &scene.specs);

            let o = &specs.camera_pos;
            let color = trace_ray(o, &d, 1.0, f64::INFINITY, &scene, RECURSION_DEPTH);
            canvas.set_value(ix, iy, &color);
        }
    }
}

/// Don't understand how the canvas width and height properties are useful, but
fn canvas_to_viewport(x: f64, y: f64, specs: &Specs) -> Vec3 {
    let x = x * specs.viewport_width / specs.canvas_width;
    let y = y * specs.viewport_height / specs.canvas_height;
    Vec3::new(x, y, specs.viewport_distance)
}

/// Returns the two 'distances' on a ray where it intersects a sphere.
fn intersect_ray_sphere(o: &Vec3, d: &Vec3, sphere: &Sphere) -> (f64, f64) {
    let r = sphere.radius;
    let c0 = o - &sphere.center;
    let a = d.dot(d);
    let b = 2.0 * &c0.dot(d);
    let c = &c0.dot(&c0) - (r * r);

    let discriminant = b * b  -  4.0 * a * c;
    if discriminant < 0.0 {
        return (f64::NEG_INFINITY, f64::NEG_INFINITY);
    }

    let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b - discriminant.sqrt()) / (2.0 * a);
    (t1, t2)
}

/// Returns sphere index and closest_t // return borrowed sphere reference rather than index?
fn closest_intersection(o: &Vec3, d: &Vec3, t_min:f64 , t_max: f64, spheres: &Vec<Sphere>) -> Option<(usize, f64)> {

    let mut closest_t = f64::INFINITY;
    let mut closest_sphere_index: Option<usize> = None;
    for (i, sphere) in spheres.iter().enumerate() {
        let (t1, t2) = intersect_ray_sphere(o, d, &sphere);
        if maths::within(t1, t_min, t_max) && t1 < closest_t {
            closest_t = t1;
            closest_sphere_index = Some(i);
        }
        if maths::within(t2, t_min, t_max) && t2 < closest_t {
            closest_t = t2;
            closest_sphere_index = Some(i);
        }
    }
    match closest_sphere_index {
        None => None,
        Some(closest_sphere_index) => Some((closest_sphere_index, closest_t))
    }
}

fn trace_ray(o: &Vec3, d: &Vec3, t_min: f64, t_max: f64, scene: &Scene, recursion_depth: usize) -> Color {

    let option = closest_intersection(o, d, t_min, t_max, &scene.spheres);
    if option.is_none() {
        return scene.specs.background_color.clone();
    }

    // Compute local color
    let (closest_sphere_index, closest_t) = option.unwrap();
    let closest_sphere = &scene.spheres[closest_sphere_index];
    let p = o + (d * closest_t);
    let mut n = p - closest_sphere.center;
    n = n / n.length();
    let intensity = compute_lighting(&p, &n, &-d, closest_sphere.specular, &scene);
    let local_color = closest_sphere.color * intensity;

    // If we hit the recursion limit or the object is not reflective, we're done
    let r = closest_sphere.reflective;
    if recursion_depth == 0 || r <= 0.0 {
        return local_color;
    }

    // Compute the reflected color
    let R = reflect_ray(&-d, &n);
    let reflected_color = trace_ray(&p, &R, EPSILON, f64::INFINITY, scene, recursion_depth - 1);

    local_color * (1.0 - r)  +  reflected_color * r
}

fn compute_lighting(p: &Vec3, n: &Vec3, v: &Vec3, s: f64, scene: &Scene) -> f64 {

    let mut i = 0.0;

    for light in &scene.lights {

        let l: Vec3;
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
        let option = closest_intersection(p, &l, EPSILON, t_max, &scene.spheres);
        if option.is_some() {
            continue;
        }

        // Diffuse
        let n_dot_l = n.dot(&l);
        if n_dot_l > 0.0 {
            let modif = intens * n_dot_l / (n.length() * l.length());
            i += modif;
        }

        // Specular
        if s > 0.0 {
            let r = n * (2.0 * n.dot(&l));
            let r = &r - &l;
            let r_dot_v = r.dot(&v);
            if r_dot_v > 0.0 {
                i += intens * (r_dot_v / (r.length() * v.length())).powf(s);
            }
        }
    }
    i
}

fn reflect_ray(r: &Vec3, n: &Vec3) -> Vec3 {
    2.0 * n * n.dot(r)  -  r
}
