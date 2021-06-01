use ferris_says::*;
use std::io::{ stdout, BufWriter };

fn main() {
    let stdout = stdout();
    let out = String::from("Creating my first program in the Rust language!");
    let width = 24;

    let mut writer = BufWriter::new(stdout.lock());

    say(out.as_bytes(), width, &mut writer).unwrap();
}
