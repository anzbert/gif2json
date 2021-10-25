use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::result::Result;

use image::gif::GifDecoder;
use image::{AnimationDecoder, Pixel};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RgbaImageData {
    dimensions: (u32, u32),
    length: u32,
    frames: Vec<DecodedFrame>,
}
impl RgbaImageData {
    pub fn new_from_bytes(bytes: &[u8]) -> Result<RgbaImageData, Box<dyn Error>> {
        let decoder = GifDecoder::new(bytes)?;

        let frames = decoder.into_frames().collect_frames()?;

        let dimensions = frames.get(0).unwrap().buffer().dimensions();
        let mut output = Self::new(frames.len(), dimensions);

        for frame in frames.iter() {
            let image_buffer = frame.buffer();

            let pixels_as_rgba_vec: Vec<(u8, u8, u8, u8)> = image_buffer
                .pixels()
                .map(|p| {
                    let (r, g, b, a) = p.channels4();
                    (r, g, b, a)
                })
                .collect();

            let decoded_frame =
                DecodedFrame::new(frame.delay().numer_denom_ms(), pixels_as_rgba_vec);

            output.add(decoded_frame);
        }
        Ok(output)
    }

    pub fn new_from_gif(path: &Path) -> Result<RgbaImageData, Box<dyn Error>> {
        if path.extension().unwrap() != "gif" {
            Err(format!(
                "Invalid Path : {:?}\n
            Only use .gif files\n",
                path
            ))?;
        };

        let file = File::open(path)?;

        let decoder = GifDecoder::new(file)?;

        let frames = decoder.into_frames().collect_frames()?;

        let dimensions = frames.get(0).unwrap().buffer().dimensions();
        let mut output = Self::new(frames.len(), dimensions);

        for frame in frames.iter() {
            let image_buffer = frame.buffer();

            let pixels_as_rgba_vec: Vec<(u8, u8, u8, u8)> = image_buffer
                .pixels()
                .map(|p| {
                    let (r, g, b, a) = p.channels4();
                    (r, g, b, a)
                })
                .collect();

            let decoded_frame =
                DecodedFrame::new(frame.delay().numer_denom_ms(), pixels_as_rgba_vec);

            output.add(decoded_frame);
        }
        Ok(output)
    }

    fn new(length: usize, dimensions: (u32, u32)) -> Self {
        Self {
            dimensions,
            length: length as u32,
            frames: Vec::new(),
        }
    }
    fn add(&mut self, next_frame: DecodedFrame) {
        self.frames.push(next_frame);
    }
    fn to_json_string(&self) -> String {
        serde_json::to_string(self).expect("Error creating JSON string")
    }
    pub fn save_as_json(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        if path.extension().unwrap() != "json" {
            Err(format!(
                "Invalid Path : {:?}\n
            Only use .json suffix\n",
                path
            ))?;
        };

        let mut json_file = File::create(path)?;

        json_file.write_all(self.to_json_string().as_bytes())?;

        println!("\nSuccessfully written to: '{:?}'\n", path);
        Ok(())
    }

    pub fn get_frame_vec_ref(&self, frame: usize) -> Option<&Vec<(u8, u8, u8, u8)>> {
        match self.frames.get(frame) {
            Some(f) => Some(&f.pixels),
            None => None,
        }
    }
    pub fn get_frame_delay(&self, frame: usize) -> Option<(u32, u32)> {
        match self.frames.get(frame) {
            Some(f) => Some(f.delay_ratio),
            None => None,
        }
    }
    pub fn get_dimensions(&self) -> (u32, u32) {
        self.dimensions
    }
}

#[derive(Serialize, Deserialize)]
struct DecodedFrame {
    delay_ratio: (u32, u32),
    pixels: Vec<(u8, u8, u8, u8)>,
}

impl DecodedFrame {
    fn new(delay_ratio: (u32, u32), pixels: Vec<(u8, u8, u8, u8)>) -> Self {
        Self {
            delay_ratio,
            pixels,
        }
    }
}
