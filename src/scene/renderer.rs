use crate::base::color::Color;
use crate::scene::scene::{Light, Scene, Specs, Sphere};
use crate::base::vec3::Vec3;
use crate::canvas::canvas::Canvas;
use crate::util::maths;

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

            let color: Color;
            let o = &specs.camera_pos;
            let opt = trace_ray(o, &d, 1.0, f32::INFINITY, &scene.spheres);
            match opt {
                None => {
                    color = scene.specs.background_color;
                }
                Some((sphere_index, closest_t)) => {
                    let p = o + &(&d * closest_t); // point on surface
                    let sphere = &scene.spheres[sphere_index];
                    let mut n = &p - &sphere.center; // surface normal
                    n = &n * (1.0 / n.length());
                    let v = &d * -1.0;
                    let intensity = calculate_lighting(&p, &n, &v, sphere.specular, &scene.lights);
                    color = sphere.color * intensity;
                }
            }
            canvas.set_value(ix, iy, &color);
        }
    }
}

/// Don't really understand how this is useful, but
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

/// Finds the nearest intersection point between ray and sphere within the given range t,
/// and returns the sphere's index and the point.
fn trace_ray(o: &Vec3, d: &Vec3, t_min: f32, t_max: f32, spheres: &Vec<Sphere>) -> Option<(usize, f32)> {

    let mut closest_t = f32::INFINITY;
    let mut closest_sphere_index: Option<usize> = None;

    for i in 0..spheres.len() {
        let sphere = &spheres[i];
        let (t1, t2) = intersect_ray_sphere(&o, &d, sphere);
        if t1 >=- t_min && t1 <= t_max && t1 < closest_t {
            closest_t = t1;
            closest_sphere_index = Some(i);
        }
        if t2 >= t_min && t2 <= t_max && t2 < closest_t {
            closest_t = t2;
            closest_sphere_index = Some(i);
        }
    }

    if closest_sphere_index.is_none() {
        return None;
    }

    let closest_sphere_index = closest_sphere_index.unwrap();
    Some((closest_sphere_index, closest_t))
}

/// p - point on surface
/// n - normal
/// v - something something specularity
/// spec - specularity factor
/// lights - the lights to consider
///
fn calculate_lighting(p: &Vec3, n: &Vec3, v: &Vec3, spec: f32, lights: &Vec::<Light>) -> f32 {

    let mut sum_intensity = 0_f32;

    for light in lights {

        if let Light::Ambient { intensity } = light {
            sum_intensity  += intensity;
            continue;
        }

        // find `l` (which is somethingsomething) for either point light or directional light
        let l;
        let light_intensity: f32;
        if let Light::Point { intensity, position } = light {
            l = position - p;
            light_intensity = *intensity;
        } else if let Light::Directional { intensity, direction} = light {
            l = direction.clone();
            light_intensity = *intensity;
        } else { // unreachable actually
            continue;
        }

        let n_dot_l = n.dot(&l);
        if n_dot_l > 0.0 {
            let modif = light_intensity * n_dot_l / (n.length() * l.length());
            sum_intensity += modif;
        }

        // plus specular highlight
        if spec != -1.0 {
            let r = n * (2.0 * n.dot(&l));
            let r = &r - &l;
            let r_dot_v = r.dot(&v);
            if r_dot_v > 0.0 {
                sum_intensity += light_intensity * (r_dot_v / (r.length() * v.length())).powf(spec);
            }
        }
    }
    sum_intensity
}

// fn calculate_point_light_intensity(point_light: Light::Point) -> f32 {
//
// }
