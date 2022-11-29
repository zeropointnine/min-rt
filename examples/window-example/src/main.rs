use std::f32;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder};
use winit_input_helper::WinitInputHelper;
use min_rt::base::color::Color;
use min_rt::canvas::u8_canvas::U8Canvas;
use min_rt::{scene, util};
use min_rt::scene::renderer;
use min_rt::scene::scene::{Light, Scene};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const TIME_INCREMENT: f32 = 5.0;

/// Window setup and event loop based on pixels sample program
/// https://github.com/parasyte/pixels/tree/main/examples/minimal-winit
///
fn main() -> Result<(), Error> {

    println!("\r\nPress spacebar to update scene");

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("window-example")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture
            = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };

    let path = util::file::find_file_starting_from_cwd("scene1.yaml").unwrap();
    let mut scene = scene::loader::load(&path).unwrap();

    let mut canvas = U8Canvas::new(WIDTH, HEIGHT);

    let mut is_scene_dirty = true;
    let mut time = 0_f32;

    event_loop.run(move |event, _, control_flow| {

        if let Event::RedrawRequested(_) = event {
            // Copy from the canvas to `pixels`
            pixels.get_frame_mut().copy_from_slice(&canvas.data);
            if pixels
                .render()
                .map_err(|_e| println!("Error") )
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {

            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                // Quit
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                // Resize the window
                pixels.resize_surface(size.width, size.height);
            }

            if input.key_pressed(VirtualKeyCode::Space) {
                is_scene_dirty = true;
            }

            if is_scene_dirty {
                update_scene(&mut scene, time);
                time += TIME_INCREMENT;
                renderer::render_scene_to_canvas(&scene, &mut canvas);
                is_scene_dirty = false;
                window.request_redraw();
            }
        }
    });
}

fn update_scene(scene: &mut Scene, time: f32) {
    // light [2.0, 1.0, 0.0]
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
