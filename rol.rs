// This software is in the public domain.

use std::io::stdout;
use std::io::Read;
use std::io::Write;

fn rainbow(i: i32) -> (i32, i32, i32) {
    let red = (f64::sin(0.1 * (i as f64) + 0.0) * 127.0 + 128.0) as i32;
    let green = (f64::sin(0.1 * (i as f64) + 2.0 * 3.1415 / 3.0) * 127.0 + 128.0) as i32;
    let blue = (f64::sin(0.1 * (i as f64) + 4.0 * 3.1415 / 3.0) * 127.0 + 128.0) as i32;

    return (red, green, blue);
}

fn main() {
    let mut i = 0;
    let stdin = std::io::stdin();
    let mut reader = stdin.lock();
    loop {
        let mut input = [0u8; 1];
        match reader.read(&mut input) {
            Ok(0) => break,
            _ => {
                let (r, g, b) = rainbow(i);
                write!(stdout(), "\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, input[0] as char).unwrap();
                i += 1;
            }
        }
    }
}
