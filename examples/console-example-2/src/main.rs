use std::f32;
use std::{thread, time};
use std::time::Instant;
use device_query::{DeviceQuery, DeviceState, Keycode};
use min_rt::base::color::Color;
use min_rt::{scene, util};
use min_rt::canvas::console_composite_canvas::ConsoleCompositeCanvas;
use min_rt::scene::renderer;
use min_rt::scene::scene::{Light, Scene};
use min_rt::util::ansi;

const MS_PER_FRAME: i64 = 33;
const TIME_INCREMENT: f32 = 1.0;

fn main() {

    let device_state = DeviceState::new();

    // Load scene using yaml config file
    let path = util::file::find_file_starting_from_cwd("scene1.yaml").unwrap();
    let mut scene = scene::loader::load(&path).unwrap();

    // Adjust pixel aspect ratio because terminal
    scene.specs.pixel_ar = 0.40;

    // Make the canvas using the terminal's character dimensions
    let mut canvas = make_canvas_using_term_size();

    // Start the render loop
    let mut time = 0_f32;
    loop {
        let start = Instant::now();

        // If terminal has resized, resize the canvas to match
        let (width, height) = get_terminal_size();
        let has_changed = (width != canvas.get_width()) || (height != canvas.get_height());
        if has_changed {
            canvas = make_canvas_using_term_size();
        }

        // Render scene to the canvas
        renderer::render_scene_to_canvas(&scene, &mut canvas.colors);

        // Add some token info text for good measure
        canvas.clear_chars(' ');
        canvas.set_text(1, 1, &format!("time {}", time.floor()));

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

        // Update the scene
        update_scene(&mut scene, time);
        time += TIME_INCREMENT;

        // Sleep until time for next frame
        let ms = start.elapsed().as_millis() as i64;
        let delay = MS_PER_FRAME - ms;
        if delay > 0 {
            thread::sleep(time::Duration::from_millis(delay as u64));
        }
    }
}

/// Adds some rudimentary movement for fun
fn update_scene(scene: &mut Scene, time: f32) { return;
    // light
    let light: &mut Light = &mut scene.lights[1];
    if let Light::Point { intensity: _, position } = light {
        let radians = (f32::consts::PI / 180.0) * (time * 2.5);
        position.x = 2.0 + (radians.sin() * 3.0);
        position.z = -3.0 + (radians.cos() * 3.0);
    }
    // sphere pos
    let mut pos = &mut scene.spheres[0].center;
    let radians = (f32::consts::PI / 180.0) * (time * 1.25);
    pos.y = -1.0 + (radians.sin() * 1.0);
}

fn make_canvas_using_term_size() -> ConsoleCompositeCanvas {
    let (width, height) = get_terminal_size();
    ConsoleCompositeCanvas::new(width, height, Color::new(255, 128, 128))
}

fn get_terminal_size() -> (usize, usize) {
    match term_size::dimensions() {
        Some(size) => size,
        None => (80, 60) // fallback
    }
}
