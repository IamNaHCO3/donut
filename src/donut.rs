use std::f64::consts::PI;
use ndarray::{Array1, Array2};

pub struct Donut {
    major_radius: f64,
    minor_radius: f64,
    zoom: f64,
    rotation_speed: [f64; 3],
    rotation_angle: [f64; 3],
    granularity: i64,
    screen_size: [i64; 2],
}

impl Donut {
    pub fn new(major_radius: f64, minor_radius: f64, zoom: f64, rotation_speed: [f64; 3], rotation_angle: [f64; 3], granularity: i64, screen_size: [i64; 2]) -> Result<Donut, &'static str> {
        if major_radius <= 0.0 {
            return Err("Major radius must be greater than 0");
        }
        if minor_radius <= 0.0 {
            return Err("Minor radius must be greater than 0");
        }
        if zoom <= 0.0 {
            return Err("Zoom must be greater than 0");
        }
        if granularity <= 0 {
            return Err("Granularity must be greater than 0");
        }
        if screen_size[0] <= 0 || screen_size[1] <= 0 {
            return Err("Screen size must be greater than 0");
        }
        // 0.0 <= rotation_speed <= 1.0
        if rotation_speed[0] < 0.0 || rotation_speed[0] > 1.0 || rotation_speed[1] < 0.0 || rotation_speed[1] > 1.0 || rotation_speed[2] < 0.0 || rotation_speed[2] > 1.0 {
            return Err("Rotation speed must be between 0.0 and 1.0");
        }
        // -2 * PI <= rotation_angle <= 2 * PI
        if rotation_angle[0] < -2.0 * PI || rotation_angle[0] > 2.0 * PI || rotation_angle[1] < -2.0 * PI || rotation_angle[1] > 2.0 * PI || rotation_angle[2] < -2.0 * PI || rotation_angle[2] > 2.0 * PI {
            return Err("Rotation angle must be between -2 * PI and 2 * PI");
        }

        Ok(Donut {
            major_radius,
            minor_radius,
            zoom,
            rotation_speed,
            rotation_angle,
            granularity,
            screen_size,
        })
    }


    pub fn render(&self) -> Vec<Vec<f64>> {
        let mut z_buffer: Vec<Vec<f64>> = vec![vec![0.0; self.screen_size[1] as usize]; self.screen_size[0] as usize];
        let mut screen: Vec<Vec<f64>> = vec![vec![0.0; self.screen_size[1] as usize]; self.screen_size[0] as usize];

        let object_distance = 2.0 * self.major_radius + self.minor_radius;

        let rotation_x = Array2::from(vec![
            [1.0, 0.0, 0.0],
            [0.0, self.rotation_angle[0].cos(), -self.rotation_angle[0].sin()],
            [0.0, self.rotation_angle[0].sin(), self.rotation_angle[0].cos()],
        ]);

        let rotation_y = Array2::from(vec![
            [self.rotation_angle[1].cos(), 0.0, self.rotation_angle[1].sin()],
            [0.0, 1.0, 0.0],
            [-self.rotation_angle[1].sin(), 0.0, self.rotation_angle[1].cos()],
        ]);

        let rotation_z = Array2::from(vec![
            [self.rotation_angle[2].cos(), -self.rotation_angle[2].sin(), 0.0],
            [self.rotation_angle[2].sin(), self.rotation_angle[2].cos(), 0.0],
            [0.0, 0.0, 1.0],
        ]);

        let rotation = rotation_x.dot(&rotation_y).dot(&rotation_z);

        for i in 0..self.granularity {
            let phi = i as f64 * 2.0 * PI / self.granularity as f64;
            let rotation_torus = Array2::from(vec![
                [phi.cos(), -phi.sin(), 0.0],
                [phi.sin(), phi.cos(), 0.0],
                [0.0, 0.0, 1.0],
            ]);
            for j in 0..=self.granularity {
                let theta = j as f64 * 2.0 * PI / self.granularity as f64;
                let vec = Array1::from(vec![self.major_radius + self.minor_radius * theta.cos(), 0.0, self.minor_radius * theta.sin()]);

                let rotated_vec = rotation_torus.dot(&vec).dot(&rotation);
                let x = rotated_vec[0];
                let y = rotated_vec[1];
                let z = rotated_vec[2] + object_distance;

                let x_screen = (self.zoom * x) / (self.zoom + z);
                let y_screen = (self.zoom * y) / (self.zoom + z) * 2.0;

                let x_screen = &x_screen + (self.screen_size[0] as f64 / 2.0);
                let y_screen = &y_screen + (self.screen_size[1] as f64 / 2.0);

                if x_screen >= 0.0 && x_screen < self.screen_size[0] as f64 && y_screen >= 0.0 && y_screen < self.screen_size[1] as f64 {
                    if z_buffer[x_screen as usize][y_screen as usize] < z {
                        z_buffer[x_screen as usize][y_screen as usize] = z;

                        screen[x_screen as usize][y_screen as usize] += 0.3;
                        if screen[x_screen as usize][y_screen as usize] >= 1.0 {
                            screen[x_screen as usize][y_screen as usize] = 1.0;
                        }
                    }
                }
            }
        }

        return screen;
    }


    pub fn update(&mut self) {
        self.rotation_angle[0] += self.rotation_speed[0] * 2.0 * PI;
        self.rotation_angle[1] += self.rotation_speed[1] * 2.0 * PI;
        self.rotation_angle[2] += self.rotation_speed[2] * 2.0 * PI;
    }

}
