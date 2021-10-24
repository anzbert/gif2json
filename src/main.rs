use std::fs::File;
use std::io::prelude::*;
use std::{env, process};

use image::gif::GifDecoder;
use image::{AnimationDecoder, Pixel};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct OutputObject {
    frames: Vec<DecodedFrame>,
}
impl OutputObject {
    fn new() -> Self {
        Self { frames: Vec::new() }
    }
    fn add(&mut self, next_frame: DecodedFrame) {
        self.frames.push(next_frame);
    }
    fn to_json_string(&self) -> String {
        serde_json::to_string(self).expect("Error creating JSON string")
    }
}

#[derive(Serialize, Deserialize)]
struct DecodedFrame {
    dimensions: (u32, u32),
    pixels: Vec<(u8, u8, u8)>,
}

impl DecodedFrame {
    fn new(pixels: Vec<(u8, u8, u8)>, dimensions: (u32, u32)) -> Self {
        Self { dimensions, pixels }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {}
        _ => {
            println!(
                "\nError: Invalid parameters\n\n
            Syntax: gif2json [image_file].gif\n"
            );
            process::exit(1);
        }
    }

    let input_filename = args.get(1).expect("Error reading first argument");

    // let data = match std::fs::metadata(filename) {
    //     Ok(metadata) => metadata,
    //     Err(err) => {
    //         println!("Error Reading File's Metadata: {}", err);
    //         process::exit(1);
    //     }
    // };

    if !input_filename.ends_with(".gif") {
        println!(
            "\nError: Invalid filetype. Only files with '.gif' suffix\n\n
    Syntax: gif2json [image_file].gif\n"
        );
        process::exit(1);
    }

    let file = match File::open(input_filename) {
        Ok(file) => file,
        Err(err) => {
            println!("Error loading specified Image File: {}", err);
            process::exit(1);
        }
    };

    let decoder = match GifDecoder::new(file) {
        Ok(gif) => gif,
        Err(err) => {
            println!("Error decoding gif: {}", err);
            process::exit(1);
        }
    };

    let frames = decoder
        .into_frames()
        .collect_frames()
        .expect("Error splitting gif into individual frames");

    let mut output = OutputObject::new();

    for frame in frames.iter() {
        let image_buffer = frame.buffer();

        let pixels_as_rgb_vec: Vec<(u8, u8, u8)> = image_buffer
            .pixels()
            .map(|p| {
                let (r, g, b, _) = p.channels4(); // ditch alpha, pass on RGB
                (r, g, b)
            })
            .collect();

        let decoded_frame = DecodedFrame::new(pixels_as_rgb_vec, image_buffer.dimensions());

        output.add(decoded_frame);
    }

    let output_filename = format!("{}.json", &input_filename[0..input_filename.len() - 4]);

    let mut json_file = File::create(&output_filename).expect("Error creating output file");

    json_file
        .write_all(output.to_json_string().as_bytes())
        .expect("Error writing to output file");

    println!("\nSuccessfully written to: '{}'\n", output_filename);
}
