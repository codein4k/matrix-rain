use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;
use std::process::Command;

const REFRESH_RATE: u64 = 40; // ms

#[derive(Clone)]
struct Drop {
    x: usize,
    y: f32,
    speed: f32,
    trail_len: usize,
}

fn get_terminal_size() -> (usize, usize) {
    let output = Command::new("sh")
        .arg("-c")
        .arg("stty size < /dev/tty")
        .output()
        .expect("failed to get terminal size");

    let s = String::from_utf8_lossy(&output.stdout);
    let mut parts = s.split_whitespace();

    let rows: usize = parts.next().unwrap_or("24").parse().unwrap_or(24);
    let cols: usize = parts.next().unwrap_or("80").parse().unwrap_or(80);

    (cols, rows)
}

fn init_drops(cols: usize) -> Vec<Drop> {
    let mut drops = Vec::with_capacity(cols);

    for i in 0..cols {
        drops.push(Drop {
            x: i + 1,
            y: 1.0,
            speed: 0.1 + (rand::random::<u8>() % 100) as f32 / 100.0,
            trail_len: 10 + (rand::random::<u8>() % 10) as usize,
        });
    }

    drops
}

fn print_pixel(y: i32, x: usize, r: u8, g: u8, b: u8, c: char) {
    if y < 1 {
        return;
    }

    print!("\x1b[{};{}H\x1b[38;2;{};{};{}m{}", y, x, r, g, b, c);
}

fn get_char() -> char {
    if rand::random::<bool>() { '0' } else { '1' }
}

fn main() {
    let mut stdout = io::stdout();

    let (mut width, mut height) = get_terminal_size();
    let mut drops = init_drops(width);

    // hide cursor + clear
    print!("\x1b[?25l\x1b[2J");
    stdout.flush().unwrap();

    loop {
        let (new_width, new_height) = get_terminal_size();

        if new_width != width || new_height != height {
            width = new_width;
            height = new_height;
            drops = init_drops(width);
            print!("\x1b[2J");
        }

        for i in 0..width {
            let d = &mut drops[i];
            let head_y = d.y as i32;

            if head_y >= 1 && head_y <= height as i32 {
                print_pixel(head_y, d.x, 150, 255, 150, get_char());
            }

            for j in 1..=d.trail_len {
                let ty = head_y - j as i32;
                if ty < 1 || ty > height as i32 {
                    continue;
                }

                let ratio = 1.0 - (j as f32 / d.trail_len as f32);
                let green = (255.0 * ratio) as u8;

                print_pixel(ty, d.x, 0, green, 0, get_char());
            }

            d.y += d.speed;

            if d.y - d.trail_len as f32 > height as f32 {
                d.y = 0.0;
                d.speed = 0.1 + (rand::random::<u8>() % 100) as f32 / 100.0;
            }
        }

        stdout.flush().unwrap();
        sleep(Duration::from_millis(REFRESH_RATE));
    }
}
