use crate::base::color::Color;
use crate::scene::scene::{Light, Scene, Specs, Sphere};
use crate::base::vec3::Vec3;
use crate::canvas::canvas::Canvas;
use crate::util::maths;

static RENDERER_EPSILON: f32 = 0.001;

/// Logic for rendering a `Scene`` onto a `Canvas`
///
/// This is a fairly literal translation of the pseudo-code from the book
/// ["Computer Graphics from Scratch"](https://gabrielgambetta.com/computer-graphics-from-scratch/)
/// by Gabriel Gambetta into Rust. The names of functions and parameters have been preserved for
/// easy cross-referencing.
///

// todo update function description comments

///
///
pub fn render_scene_to_canvas(scene: &Scene, canvas: &mut dyn Canvas<Color>) {

    let specs = &scene.specs;

    let canvas_grid_ar = canvas.get_width() as f32 / canvas.get_height() as f32;

    // Account for pixel aspect ratio (viz., for terminal text output)
    let adjusted_viewport_width= specs.viewport_width * specs.pixel_ar;

    let steps_x = canvas.get_width();
    let steps_y = canvas.get_height();

    let half_canvas_width = specs.canvas_width / 2.0;
    let half_canvas_height = specs.canvas_height / 2.0;

    for iy in 0..steps_y {

        let mut y = maths::map(iy as f32, steps_y as f32, 0.0, -half_canvas_height, half_canvas_height);

        for ix in 0..steps_x {

            let mut x = maths::map(ix as f32, 0.0, steps_x as f32, -half_canvas_width, half_canvas_width);

            x *= adjusted_viewport_width / specs.canvas_width;
            x *= canvas_grid_ar; // Account for canvas grid size
            y *= specs.viewport_height / specs.canvas_height;
            let d = canvas_to_viewport(x, y, &scene.specs);

            let o = &specs.camera_pos;
            let color = trace_ray(o, &d, 1.0, f32::INFINITY, &scene);
            canvas.set_value(ix, iy, &color);
        }
    }
}

/// Don't understand how the canvas width and height properties are useful, but
fn canvas_to_viewport(x: f32, y: f32, specs: &Specs) -> Vec3 {
    let x = x * specs.viewport_width / specs.canvas_width;
    let y = y * specs.viewport_height / specs.canvas_height;
    Vec3::new(x, y, specs.viewport_distance)
}

/// Returns the two 'distances' on a ray where it intersects a sphere.
fn intersect_ray_sphere(o: &Vec3, d: &Vec3, sphere: &Sphere) -> (f32, f32) {
    let r = sphere.radius;
    let c0 = o - &sphere.center;
    let a = d.dot(d);
    let b = 2.0 * &c0.dot(d);
    let c = &c0.dot(&c0) - (r * r);

    let discriminant = b * b  -  4.0 * a * c;
    if discriminant < 0.0 {
        return (f32::NEG_INFINITY, f32::NEG_INFINITY);
    }

    let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b - discriminant.sqrt()) / (2.0 * a);
    (t1, t2)
}

/// Returns sphere index and closest_t // return borrowed sphere reference rather than index?
fn closest_intersection(o: &Vec3, d: &Vec3, t_min:f32 , t_max: f32, spheres: &Vec<Sphere>) -> Option<(usize, f32)> {

    let mut closest_t = f32::INFINITY;
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

fn trace_ray(o: &Vec3, d: &Vec3, t_min: f32, t_max: f32, scene: &Scene) -> Color {

    let option = closest_intersection(o, d, t_min, t_max, &scene.spheres);
    if option.is_none() {
        return scene.specs.background_color.clone();
    }
    let (closest_sphere_index, closest_t) = option.unwrap();
    let closest_sphere = &scene.spheres[closest_sphere_index];
    let p = o + (d * closest_t);
    let mut n = p - closest_sphere.center;
    n = n / n.length();
    let intensity = compute_lighting(&p, &n, &-d, closest_sphere.specular, &scene);
    return closest_sphere.color * intensity
}

fn compute_lighting(p: &Vec3, n: &Vec3, v: &Vec3, s: f32, scene: &Scene) -> f32 {

    let mut i = 0.0;

    for light in &scene.lights {

        let l: Vec3;
        let t_max;
        let intens: f32;

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
                t_max = f32::INFINITY;
                intens = *intensity;
            }
        }

        // Shadow check
        let option = closest_intersection(p, &l, RENDERER_EPSILON, t_max, &scene.spheres);
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
    }i
}
