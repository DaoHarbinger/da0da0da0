use colored::*;

fn main() {
    const WIDTH: u32 = 35;
    const HEIGHT: u32 = 17;
    let ratio: f32 = WIDTH as f32 / HEIGHT as f32;

    for y in 0..=HEIGHT {
        for x in 0..=WIDTH {
            let symbol: String = match (x, y) {
                (0, 0) | (WIDTH, 0) | (0, HEIGHT) | (WIDTH, HEIGHT) => "O".yellow().bold().to_string(),
                (_, 0) | (_, HEIGHT) => "=".blue().to_string(),
                (0, _) | (WIDTH, _) => "|".blue().to_string(),
                _ if x == (y as f32 * ratio) as u32 => "/".green().to_string(),
                _ if x == WIDTH - (y as f32 * ratio) as u32 => "\\".green().to_string(),
                _ => " ".to_string(),
            };
            print!("{}", symbol);
        }
        println!();
    }
}
