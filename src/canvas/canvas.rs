use std::io::{stdout, Write};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;

use crossterm::{cursor::{Hide, Show}, execute, style::{Color, SetForegroundColor}, terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}};
use crossterm::event::{self, Event};

use crate::models::donut::Donut;

pub struct Canvas {
    model: Donut,
    frame_rate: u64,
    fill: Vec<char>,
}


impl Canvas {
    pub fn new(model: Donut, frame_rate: u64, fill: Vec<char>, color: Color) -> Canvas {
        execute!(stdout(), EnterAlternateScreen, Clear(ClearType::All), Hide, SetForegroundColor(color)).unwrap();
        Canvas {
            model,
            frame_rate,
            fill,
        }
    }

    pub fn drop(&mut self) {
        execute!(stdout(), LeaveAlternateScreen, Show).unwrap();
    }

    pub fn display(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        thread::spawn(move || {
            while r.load(Ordering::SeqCst) {
                if event::poll(Duration::from_millis(100)).unwrap() {
                    if let Event::Key(_event) = event::read().unwrap() {
                        r.store(false, Ordering::SeqCst);
                    }
                }
            }
        });

        let mut last_screen = vec![vec![' '; self.model.screen_size[1] as usize]; self.model.screen_size[0] as usize];
        while running.load(Ordering::SeqCst) {
            let screen = self.model.render();
            last_screen = self.next_frame(&screen, &last_screen);

            self.model.update();

            if !running.load(Ordering::SeqCst) {
                break;
            }
            thread::sleep(Duration::from_millis(1000 / self.frame_rate));
        }
        Ok(())
    }

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

        let mut frame_str = String::new();
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

        writeln!(stdout(), "{}", frame_str).unwrap();
        screen
    }

}
