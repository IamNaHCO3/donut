use std::thread;
use std::time::Duration;
use crate::models::donut::Donut;

pub struct Canvas {
    model: Donut,
    frame_rate: u64,
    fill: Vec<char>,
}


impl Canvas {
    pub fn new(model: Donut, frame_rate: u64, fill: Vec<char>) -> Canvas {
        Canvas {
            model,
            frame_rate,
            fill,
        }
    }

    pub fn display(&mut self, frame_count: u64) {
        print!("\x1b[2J"); // 清屏
        print!("\x1b[?25l"); // 隐藏光标
        print!("\x1b[33m"); // 设置颜色
        let mut last_screen = vec![vec![' '; self.model.screen_size[1] as usize]; self.model.screen_size[0] as usize];
        for _i in 0..frame_count {
            let screen = self.model.render();
            last_screen = self.next_frame(&screen, &last_screen);

            self.model.update();
            thread::sleep(Duration::from_millis(1000 / self.frame_rate));
        }
        print!("\x1b[?25h"); // 显示光标
    }

    // fn parse_color(&self, color: f64) -> String {
    //     let color = (color * 255.0) as u8;
    //     format!("\x1b[38;2;{};{};{}m", color, color, color)
    // }

    fn brightness2char(&self, screen: &Vec<Vec<f64>>) -> Vec<Vec<char>> {
        let mut result = vec![vec![' '; screen[0].len()]; screen.len()];
        for i in 0..screen.len() {
            for j in 0..screen[0].len() {
                result[i][j] = self.fill[(screen[i][j] * (self.fill.len() - 1) as f64) as usize];
            }
        }
        result
    }

    fn next_frame(&self, screen: &Vec<Vec<f64>>, last_screen_char: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let screen = self.brightness2char(screen);

        let mut frame_str = String::from("\x1b[H");
        for i in 0..screen.len() {
            let mut space = 0;
            let mut buffer = String::new();
            for j in 0..screen[0].len() {
                if screen[i][j] != ' ' || last_screen_char[i][j] != ' ' {
                    buffer.push(screen[i][j]);
                    if space > 0 {
                        frame_str.push_str(&format!("\x1b[{}C", space));
                        space = 0;
                    }
                } else {
                    space += 1;
                    if !buffer.is_empty() {
                        frame_str.push_str(&buffer);
                        buffer.clear();
                    }
                }
            }
            frame_str.push('\n');
        }

        print!("{}", frame_str);
        screen
    }

}
