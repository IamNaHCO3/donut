

pub struct Canvas {
    fill: Vec<char>,
}


impl Canvas {
    pub fn new(fill: Vec<char>) -> Canvas {
        println!("\x1b[2J\x1b[3J\x1b[1;1H");
        Canvas {
            fill,
        }
    }

    pub fn display(&self, screen: Vec<Vec<f64>>) {
        print!("\x1b[H");
        for i in 0..screen.len() {
            for j in 0..screen[0].len() {
                print!("{}", self.fill[(screen[i][j] * (self.fill.len() - 1) as f64) as usize]);
            }
            println!();
        }
    }

}
