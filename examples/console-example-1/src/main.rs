use min_rt::base::color::Color;
use min_rt::util::ansi;
use min_rt::scene::renderer;
use min_rt::scene::scene::{Light, Scene, Specs, Sphere};
use min_rt::canvas::console_composite_canvas::ConsoleCompositeCanvas;
use min_rt::base::vec3::Vec3;

fn main() {

    // Make the canvas onto which the scene will be rendered
    let mut canvas
        = ConsoleCompositeCanvas::new(80, 40, Color::from_u8(255, 64, 64));

    // Construct a minimal scene programmatically
    let scene = {
        let mut specs = Specs::new_with_defaults();
        specs.pixel_ar = 0.40; // tall pixel aspect ratio because terminal
        specs.background_color.set(0.0, 0.0, 0.0);

        let light = Light::Ambient { intensity: 1.0 };
        let mut lights = Vec::<Light>::new();
        lights.push(light);

        let sphere = Sphere::new(
            Vec3::new(0.0, 0.0, 3.0),
            1.0,
            Color::from_u8(255, 0, 0),
            500.0,
            -1.0);
        let mut spheres = Vec::<Sphere>::new();
        spheres.push(sphere);

        Scene { specs, lights, spheres }
    };

    // Render the scene to the canvas
    renderer::render_scene_to_canvas(&scene, &mut canvas.colors);

    // Write some token text to the canvas
    canvas.clear_chars(' ');
    canvas.set_text(2, 2, "hello min-rt");

    // Print the canvas data to the console
    // Terminal program must support ANSI TrueColor
    canvas.print_to_console();

    // Restore terminal state, somewhat
    print!("{}{}{}",
           ansi::CODE_SHOW_CURSOR,
           ansi::background_color(0, 0, 0),
           ansi::foreground_color(192, 192, 192));
}
