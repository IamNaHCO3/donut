use std::f64::consts::PI;
use std::io;

use crossterm::style::Color;
use crossterm::terminal::size;
use rand::Rng;

use canvas::canvas::Canvas;
use models::donut::Donut;

mod models;
mod canvas;

fn main() -> io::Result<()> {
    let frame_rate = 20;
    let filler = vec![' ', '.', ',', '-', '~', ':', ';', '!', '=', '*', '#', '$', '@'];

    // 显示大小主要取决于zoom以及major_radius和minor_radius的比例
    let major_radius = 40.0;
    let minor_radius = 5.0;

    let mut rng = rand::thread_rng();
    let light_vec = [rng.gen_range(0.0..=1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)];
    let rotation_speed = [rng.gen_range(0.0..=0.01), rng.gen_range(0.0..=0.01), rng.gen_range(0.0..=0.01)];
    let rotation_angle = [rng.gen_range(0.0..=2.0 * PI), rng.gen_range(0.0..=2.0 * PI), rng.gen_range(0.0..=2.0 * PI)];

    let (cols, rows) = size()?;
    let screen_size = [rows as i64, cols as i64];
    let zoom = rows.min(cols) as f64;
    let granularity = ((rows + cols) as f64 * 0.35) as i64;

    let donut = Donut::new(major_radius, minor_radius, zoom, rotation_speed, rotation_angle, granularity, screen_size, light_vec);
    match donut {
        Ok(donut) => {
            let mut canvas = Canvas::new(donut, frame_rate, filler, Color::Blue);
            canvas.display().expect("Failed to display donut");
            canvas.drop();
        },
        Err(e) => {
            println!("Failed to create a donut: {}", e);
        }
    }
    Ok(())
}
