mod canvas;
mod donut;
use std::f64::consts::PI;
use std::thread;
use std::time::Duration;
use rand::Rng;

fn main() {
    let frame_rate = 30;

    // 显示大小主要取决于zoom以及major_radius和minor_radius的比例
    let zoom = 15.0;
    let major_radius = 40.0;
    let minor_radius = 5.0;
    let screen_size = [30, 40];
    let granularity = (screen_size[0] as f64 + screen_size[1] as f64 * 0.8) as i64;

    let mut rng = rand::thread_rng();
    let rotation_speed = [rng.gen_range(0.00..=0.01), rng.gen_range(0.0..=0.01), rng.gen_range(0.0..=0.01)];
    let rotation_angle = [rng.gen_range(0.0..=2.0 * PI), rng.gen_range(0.0..=2.0 * PI), rng.gen_range(0.0..=2.0 * PI)];

    let donut = donut::Donut::new(major_radius, minor_radius, zoom, rotation_speed, rotation_angle, granularity, screen_size);
    match donut {
        Ok(mut donut) => {
            let canva = canvas::Canvas::new(vec!['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@']);
            print!("\x1b[?25l"); // 隐藏光标

            for _i in 0..100 {
                let screen = donut.render();
                canva.display(screen);
                donut.update();

                // 暂停
                thread::sleep(Duration::from_millis(1000 / frame_rate));
            }

            println!("{:?}", rotation_speed);

        },
        Err(e) => {
            println!("Failed to create a donut: {}", e);
        }
    }
}
