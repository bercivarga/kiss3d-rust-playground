extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::window::Window;
use na::Translation3;
use rand::Rng;

fn main() {
    const FPS: f32 = 60.0;
    let mut window = Window::new("Random stuff");
    let mut circle_count = 500;
    let mut circles = vec![];

    let mut rng = rand::thread_rng();

    while circle_count > 0 {
        let mut c = window.add_sphere(0.0);
        c.append_translation(&Translation3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        ));
        c.set_color(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        );

        circles.push(c);
        circle_count -= 1;
    }

    window.set_light(Light::StickToCamera);

    while window.render() {
        for circle in &mut circles {
            circle.set_local_scale(
                rng.gen_range(0.0..1.0) / FPS,
                rng.gen_range(0.0..1.0) / FPS,
                rng.gen_range(0.0..1.0) / FPS,
            );
        }
    }
}
