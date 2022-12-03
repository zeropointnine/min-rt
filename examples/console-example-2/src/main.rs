use std::f64;
use std::{thread, time};
use std::sync::{Arc, RwLock};
use std::time::Instant;
use device_query::{DeviceQuery, DeviceState, Keycode};
use min_rt::base::color::Color;
use min_rt::{scene, util};
use min_rt::quaternion_from_euler;
use min_rt::canvas::console_canvas_multi::ConsoleCanvasMulti;
use min_rt::cgmath::{Euler, InnerSpace};
use min_rt::scene::renderer;
use min_rt::scene::scene::{Light, Scene};
use min_rt::util::ansi;

const MS_PER_FRAME: i64 = 33;
const TIME_INCREMENT: f64 = 1.0;
const SHOULD_ANIMATE: bool = true;

fn main() {

    let device_state = DeviceState::new();

    // Load scene using yaml config file
    let path = util::file::find_file_starting_from_cwd("scene1.yaml").unwrap();
    let mut scene = scene::loader::load(&path).expect("Error in scene file, aborting");
    // Adjust pixel aspect ratio because terminal
    scene.specs.pixel_ar = 0.40;

    // Note the extra necessary step of wrapping the scene with Arc<RwLock>>
    // for multi-threading purposes
    let mut scene = Arc::new(RwLock::new(scene));

    // Make the canvas using the terminal's character dimensions
    let mut canvas = make_canvas_using_term_size();

    if !SHOULD_ANIMATE {
        renderer::render_to_canvas_all(&scene, &mut canvas.colors_canvas);
        canvas.print_to_console();
        return;
    }

    // Start the render loop
    let mut time = 0_f64;
    loop {
        let start = Instant::now();

        // If terminal has resized, resize the canvas to match
        let (width, height) = get_terminal_size();
        let has_changed = (width != canvas.get_width()) || (height != canvas.get_height());
        if has_changed {
            canvas = make_canvas_using_term_size();
        }

        // Update the scene
        update_scene(&mut scene, time);
        time += TIME_INCREMENT;

        // Render scene to the canvas
        renderer::render_to_canvas_all(&scene, &mut canvas.colors_canvas);

        // Write some token text to the canvas
        canvas.clear_chars(' ');
        canvas.set_text(2, 2, "hello min-rt");
        canvas.set_text(2, 3, &format!("time {}", time.floor()));

        // Print to the console
        canvas.print_to_console();

        if device_state.get_keys().contains(&Keycode::Escape) {
            // Restore terminal state somewhat, and quit
            print!("{}{}{}",
                   ansi::CODE_SHOW_CURSOR,
                   ansi::background_color(0, 0, 0),
                   ansi::foreground_color(192, 192, 192));
            break;
        }

        // Sleep until time for next frame
        let ms = start.elapsed().as_millis() as i64;
        let delay = MS_PER_FRAME - ms;
        if delay > 0 {
            thread::sleep(time::Duration::from_millis(delay as u64));
        }
    }
}

/// Adds some rudimentary movement for fun
fn update_scene(scene: &mut Arc<RwLock<Scene>>, time: f64) {

    // Write-lock and unwrap to gain access to the (mutatable) scene data.
    let mut scene = scene.write().unwrap();

    // sphere position
    let mut pos = &mut scene.spheres[0].center;
    pos.y = (time * 1.25).to_radians().sin() * 1.5;

    // sphere transparency
    scene.spheres[1].transparency = (time * 3.0).to_radians().sin() * 0.3 + 0.7;

    // camera position and orientation
    let radians = (time * 0.5).to_radians();
    let x = 0.0 + (radians.sin() * 5.0);
    let z = 3.0 + (radians.cos() * -4.0);
    scene.specs.camera_pos.x = x;
    scene.specs.camera_pos.z = z;
    let euler = Euler::<f64>::new(0.0, radians * -0.5, 0.0);
    scene.specs.camera_orientation = quaternion_from_euler(euler).normalize();

    // light
    let light: &mut Light = &mut scene.lights[1];
    if let Light::Point { intensity: _, position } = light {
        position.x = x;
        position.z = z;
    }
}

fn make_canvas_using_term_size() -> ConsoleCanvasMulti {
    let (width, height) = get_terminal_size();
    ConsoleCanvasMulti::new(width, height, Color::from_u8(255, 128, 128))
}

fn get_terminal_size() -> (usize, usize) {
    match term_size::dimensions() {
        Some(size) => size,
        None => (80, 60) // fallback
    }
}
