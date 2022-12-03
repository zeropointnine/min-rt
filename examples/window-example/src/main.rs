use std::{f64};
use std::sync::{Arc, RwLock};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder};
use winit_input_helper::WinitInputHelper;
use min_rt::canvas::u8_canvas::U8Canvas;
use min_rt::{quaternion_from_euler, scene, util};
use min_rt::cgmath::{Euler, InnerSpace};
use min_rt::scene::renderer;
use min_rt::scene::scene::{Light, Scene};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const TIME_INCREMENT: f64 = 2.0;
const ADVANCE_ON_SPACEBAR_ONLY: bool = false;

fn main() -> Result<(), Error> {

    if ADVANCE_ON_SPACEBAR_ONLY {
        println!("\r\nPress spacebar to update scene");
    }

    let path = util::file::find_file_starting_from_cwd("scene1.yaml").unwrap();
    let scene = scene::loader::load(&path).expect("Error in scene file, aborting");

    // Note the extra necessary step of wrapping the scene with Arc<RwLock>>
    // for multi-threading purposes
    let mut scene = Arc::new(RwLock::new(scene));

    let mut canvas = U8Canvas::new(WIDTH, HEIGHT);

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

    let mut is_scene_dirty = true;
    let mut time = 0_f64;
    let mut cursor_position = (0, 0);
    let mut should_quit = false;

    event_loop.run(move |event, _, control_flow| {

        // Handle various events
        match &event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested, window_id
            } if window_id == &window.id() => {
            },

            Event::RedrawRequested(_) => {
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
            Event::WindowEvent {
                event: WindowEvent::CursorMoved {device_id,position, .. }, ..
            } => {
                let logical_position = position.to_logical::<i32>(window.scale_factor());
                // println!("Scale Factor: {}    Physical {:?}    Logical: {:?}", window.scale_factor(), position, logical_position);
                cursor_position = (logical_position.x, logical_position.y);
            },
            _ => (),
        }

        // Handle input events
        if input.update(&event) {

            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                should_quit = true;
                return;
            }
            if input.mouse_pressed(0) {
                println!("mou ${:?}", cursor_position);
            }

            if let Some(size) = input.window_resized() {
                // Resize the window
                pixels.resize_surface(size.width, size.height);
            }

            if input.key_pressed(VirtualKeyCode::Space) {
                is_scene_dirty = true;
            }

            if is_scene_dirty || !ADVANCE_ON_SPACEBAR_ONLY {

                update_scene(&mut scene, time);

                // Note, using the multi-threaded version of the render fuction here:
                renderer::render_to_canvas_all_mt(&scene, &mut canvas, num_cpus::get());
                time += TIME_INCREMENT;
                is_scene_dirty = false;
                window.request_redraw();
            }
        }

        if should_quit {
            *control_flow = ControlFlow::Exit;
        }
    });
}

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
