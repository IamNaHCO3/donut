mod models;
mod canvas;

use std::f64::consts::PI;
use rand::Rng;
use models::donut::Donut;
use canvas::canvas::Canvas;

fn main() {
    let frame_rate = 30;
    let frame_count = 100;

    let filler = vec![' ', '.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];

    // 显示大小主要取决于zoom以及major_radius和minor_radius的比例
    let zoom = 200.0;
    let major_radius = 40.0;
    let minor_radius = 5.0;
    let screen_size = [100, 200];
    let granularity = 100;

    let mut rng = rand::thread_rng();
    // let light_vec = [rng.gen_range(0.0..=1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)];
    let light_vec = [0.1, 0.5, 0.4];
    let rotation_speed = [rng.gen_range(0.0..=0.01), rng.gen_range(0.0..=0.01), rng.gen_range(0.0..=0.01)];
    let rotation_angle = [rng.gen_range(0.0..=2.0 * PI), rng.gen_range(0.0..=2.0 * PI), rng.gen_range(0.0..=2.0 * PI)];


    let donut = Donut::new(major_radius, minor_radius, zoom, rotation_speed, rotation_angle, granularity, screen_size, light_vec);

    match donut {
        Ok(donut) => {
            let mut canvas = Canvas::new(donut, frame_rate, filler);
            canvas.display(frame_count);
        },
        Err(e) => {
            println!("Failed to create a donut: {}", e);
        }
    }
}
