use yaml_rust::{YamlLoader, Yaml};
use std::fs;
use crate::cgmath::{Quaternion, Vector3};
use yaml_rust::yaml::Array;
use crate::base::color::Color;
use crate::scene::scene::{Light, Scene, Specs, Sphere};

// Constructs a scene by loading its data from a yaml file.
pub fn load(filepath: &str) -> Option<Scene> {

    // Load file as string
    let string = match fs::read_to_string(filepath) {
        Err(_) => {
            println!("could not make scene (file error)");
            return None;
        },
        Ok(string) => string
    };

    // Make yaml document
    let docs = match YamlLoader::load_from_str(&string) {
        Err(_) => {
            println!("could not make scene (scan error)");
            return None;
        },
        Ok(docs) => docs
    };
    let doc = &docs[0];

    // Make specs object
    let specs = match make_specs(doc) {
        None => {
            println!("could not make scene (specs object)");
            return None;
        },
        Some(specs) => specs
    };

    // Make lights collection
    let lights = match make_lights(doc) {
        None => {
            println!("could not make scene (lights array)");
            return None;
        },
        Some(lights) => lights
    };

    // Make objects (spheres) collection
    let spheres = match make_objects(doc) {
        None => {
            println!("could not make scene (objects array)");
            return None;
        },
        Some(spheres) => spheres
    };

    // Return scene
    let scene = Scene { specs, lights, spheres };
    Some(scene)
}

fn make_specs(doc: &Yaml) -> Option<Specs> {
    let specs = &doc["specs"];

    let viewport_width = specs["viewport_width"].as_f64()?;
    let viewport_height = specs["viewport_height"].as_f64()?;
    let viewport_distance = specs["viewport_distance"].as_f64()?;
    let canvas_width = specs["canvas_width"].as_f64()?;
    let canvas_height = specs["canvas_height"].as_f64()?;
    let pixel_ar = specs["pixel_ar"].as_f64()?;
    let camera_pos = specs["camera_pos"].as_vec()?;
    let camera_pos = make_vec3(camera_pos)?;
    let camera_orientation = specs["camera_orientation"].as_vec()?;
    let camera_orientation: Quaternion<f64> = make_quat(&camera_orientation)?;
    let background_color = specs["background_color"].as_vec()?;
    let background_color = make_color(background_color)?;

    let specs = Specs {
        viewport_width,
        viewport_height,
        canvas_width,
        canvas_height,
        viewport_distance,
        pixel_ar,
        camera_pos,
        camera_orientation,
        background_color,
    };
    Some(specs)
}

fn make_lights(doc: &Yaml) -> Option<Vec<Light>> {
    let lights = &doc["lights"];
    if !lights.is_array() {
        return None;
    }
    let lights = lights.as_vec().unwrap();

    let mut result = Vec::<Light>::new();
    for light in  lights {
        let light = make_light(light);
        if let None = light {
            println!("bad light object, skipping");
            continue;
        }
        result.push(light.unwrap());
    }
    Some(result)
}

fn make_light(light: &Yaml) -> Option<Light> {

    let ambient_light = &light["ambient"];
    if let Some(_) = ambient_light.as_hash() {
        let intensity = ambient_light["intensity"].as_f64()?;
        let result = Some(Light::Ambient { intensity });
        return result;
    }
    let point_light = &light["point"];
    if let Some(_) = point_light.as_hash() {
        let intensity = point_light["intensity"].as_f64()?;
        let position = point_light["position"].as_vec()?;
        let position = make_vec3(position)?;
        let result = Some(Light::Point { intensity, position });
        return result;
    }
    let directional_light = &light["directional"];
    if let Some(_) = directional_light.as_hash() {
        let intensity = directional_light["intensity"].as_f64()?;
        let direction = directional_light["direction"].as_vec()?;
        let direction = make_vec3(direction)?;
        let result = Some(Light::Directional { intensity, direction });
        return result;
    }
    None
}

fn make_objects(doc: &Yaml) -> Option<Vec<Sphere>> {
    let objects = &doc["objects"];
    if !objects.is_array() {
        return None;
    }
    let objects = objects.as_vec().unwrap();

    let mut result = Vec::<Sphere>::new();
    for object in objects {
        let sphere = make_object(object); // only spheres, for now
        if let None = sphere {
            println!("bad object, skipping");
            continue;
        }
        result.push(sphere.unwrap());
    }
    Some(result)
}

fn make_object(object: &Yaml) -> Option<Sphere> {
    // For now, sphere is the only type of object supported
    let sphere = &object["sphere"];
    if let Some(_) = sphere.as_hash() {
        let center = sphere["center"].as_vec()?;
        let center = make_vec3(center)?;
        let radius = sphere["radius"].as_f64()?;
        let color = sphere["color"].as_vec()?;
        let color = make_color(color)?;
        let specular = sphere["specular"].as_f64()?;
        let reflective = sphere["reflective"].as_f64()?;
        let transparency = sphere["transparency"].as_f64()?;
        let result = Some(Sphere { center, radius, color, specular, reflective, transparency });
        return result;
    }
    None
}

fn make_vec3(array: &Array) -> Option<Vector3<f64>> {
    if array.len() != 3 {
        return None;
    }
    let x = array[0].as_f64()?;
    let y = array[1].as_f64()?;
    let z = array[2].as_f64()?;
    Some(Vector3::<f64>::new(x, y, z))
}

fn make_quat(array: &Array) -> Option<Quaternion<f64>> {
    if array.len() != 4 {
        return None;
    }
    let w = array[0].as_f64()?;
    let x = array[1].as_f64()?;
    let y = array[2].as_f64()?;
    let z = array[3].as_f64()?;
    Some(Quaternion::<f64>::new(w, x, y, z))
}

fn make_color(array: &Array) -> Option<Color> {
    if array.len() != 3 {
        return None;
    }
    let r = array[0].as_i64()? as u8;
    let g = array[1].as_i64()? as u8;
    let b = array[2].as_i64()? as u8;
    Some(Color::from_u8(r, g, b))
}
