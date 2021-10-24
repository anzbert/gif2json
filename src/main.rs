use std::fs::File;
use std::io::prelude::*;
use std::{env, process};

use image::gif::GifDecoder;
use image::{AnimationDecoder, Pixel};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Output {
    frames: Vec<DecodedFrame>,
}
impl Output {
    fn new() -> Self {
        Self { frames: Vec::new() }
    }
    fn add(&mut self, next_frame: DecodedFrame) {
        self.frames.push(next_frame);
    }
}

#[derive(Serialize, Deserialize)]
struct DecodedFrame {
    pixels: Vec<Vec<u8>>,
    size_x: u32,
    size_y: u32,
}

impl DecodedFrame {
    fn new(pixels: Vec<Vec<u8>>, size_x: u32, size_y: u32) -> Self {
        Self {
            pixels,
            size_x,
            size_y,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {}
        _ => {
            println!(
                "\nError: Invalid Parameters\n\n
            Syntax: img2json [image_file].gif\n"
            );
            process::exit(1);
        }
    }

    let filename = args.get(1).expect("Error reading first Argument");

    if !filename.ends_with(".gif") {
        println!(
            "\nError: Invalid Filetype. Not a 'gif'\n\n
    Syntax: img2json [image_file].gif\n"
        );
        process::exit(1);
    }

    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            println!("Error loading specified Image: {}", err);
            process::exit(1);
        }
    };

    let decoder = match GifDecoder::new(file) {
        Ok(decoded) => decoded,
        Err(err) => {
            println!("Error decoding Image: {}", err);
            process::exit(1);
        }
    };

    let frames = decoder.into_frames();
    let frames = frames
        .collect_frames()
        .expect("Error splitting gif into frames");

    let mut output = Output::new();

    for frame in frames.iter() {
        let image_buffer = frame.buffer();

        let (size_x, size_y) = image_buffer.dimensions();

        let pixels_as_rgb_vec: Vec<Vec<u8>> = image_buffer
            .pixels()
            .map(|p| {
                let (r, g, b, _a) = p.channels4();
                vec![r, g, b]
            })
            .collect();

        let decoded_frame = DecodedFrame::new(pixels_as_rgb_vec, size_x, size_y);

        output.add(decoded_frame);
    }

    let output_json = serde_json::to_string(&output).expect("Error creating JSON string");

    let mut json_file = File::create("output.json").expect("Error creating output file");
    json_file
        .write_all(output_json.as_bytes())
        .expect("Error writing to output file");
    println!("\nWritten to: 'output.json'\n");
}
