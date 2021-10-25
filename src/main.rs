use std::path::{Path, PathBuf};
use std::{env, process};

use gif2json::ImageData;
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
    let path = Path::new(args.get(1).expect("error getting path"));

    let image = match ImageData::new_from_gif(path) {
        Ok(i) => i,
        Err(err) => {
            println!("\nError: {}\n\nSyntax: gif2json [image_file].gif\n", err);
            process::exit(1);
        }
    };

    let mut output = PathBuf::from(path);
    output.set_extension("json");
    match image.save_as_json(&output) {
        Ok(_) => {}
        Err(err) => {
            println!("\nError: {}\n\nSyntax: gif2json [image_file].gif\n", err);
            process::exit(1);
        }
    }
}
