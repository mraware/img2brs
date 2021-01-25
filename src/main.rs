mod convert;

use std::env;
use std::io;
use std::io::prelude::*;

use convert::structs::*;

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
        println!("Converting please wait...");
        convert::convert_path(options.0, options.1);
        println!("Work Complete");
    }
    pause();
}

fn collect_options(args: &Vec<String>) -> (&String, ImgOptions) {
    let file = &args[args.len()-1];
    let mut size = String::from("5 5 2");
    let mut offset = String::from("0 0 0");
    let mut brick_type = String::from("0");
    let mut vertical = String::from("false");
    let mut remove_alpha = String::from("false");
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
            "--offset" | "-o" => {
                offset = String::from(&args[i+1]);
                i += 2;
            }
            "--remove_alpha" | "-ra" => {
                remove_alpha = String::from(&args[i+1]);
                i += 2;
            }
            _ => {
                i += 1;
            }
        }
    }

    let size_split: Vec<&str> = size.split(" ").collect();
    let offset_split: Vec<&str> = offset.split(" ").collect();
    let tuple = (
        file,
        ImgOptions { 
          size_x: size_split[0].parse::<u32>().unwrap(),
          size_y: size_split[1].parse::<u32>().unwrap(),
          size_z: size_split[2].parse::<u32>().unwrap(),
          asset_name_index: brick_type.parse::<u32>().unwrap(),
          vertical: if vertical == "true" { true } else { false },
          material_index: material.parse::<u32>().unwrap(),
          remove_alpha: if remove_alpha == "true" { true } else { false },
          offset_x: offset_split[0].parse::<i32>().unwrap(),
          offset_y: offset_split[1].parse::<i32>().unwrap(),
          offset_z: offset_split[2].parse::<i32>().unwrap(),
          raw: false
        }
    );

    return tuple
}