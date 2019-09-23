mod convert;

use std::env;
use std::io;
use std::io::prelude::*;
use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "\nPress any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}\n", args);
    if args.len() < 2 {
        println!("No file selected.");
    } else {
        let options = collect_options(&args);
        let sp = Spinner::new(Spinners::Line, "Converting image into bricks... ".into());
        convert::convert(options.0, options.1, options.2, options.3, options.4);
        sp.message("Work Complete.                          ".into());
        sleep(Duration::from_millis(150));
        sp.stop();
    }
    pause();
}

fn collect_options(args: &Vec<String>) -> (&String, (u32, u32, u32), u32, bool, u32) {
    let file = &args[args.len()-1];
    let mut size = String::from("5 5 2");
    let mut brick_type = String::from("0");
    let mut vertical = String::from("false");
    let mut material = String::from("0");


    let mut i = 0;
    while i < args.len()-1 {
        match &(args[i])[..] {
            "--size" | "-s" => {
                size = String::from(&args[i+1]);
                i += 2;
            }
            "--brick_type" | "-b" => {
                brick_type = String::from(&args[i+1]);
                i += 2;
            }
            "--vertical" | "-v" => {
                vertical = String::from(&args[i+1]);
                i += 2;
            }
            "--material" | "-m" => {
                material = String::from(&args[i+1]);
                i += 2;
            }
            _ => {
                i += 1;
            }
        }
    }

    let size_split: Vec<&str> = size.split(" ").collect();
    let tuple = (
        file,
        (
            size_split[0].parse::<u32>().unwrap(),
            size_split[1].parse::<u32>().unwrap(),
            size_split[2].parse::<u32>().unwrap()
        ),
        brick_type.parse::<u32>().unwrap(),
        if vertical == "true" { true } else { false },
        material.parse::<u32>().unwrap()
    );

    return tuple
}