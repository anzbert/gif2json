use std::path::{Path, PathBuf};
use std::{env, process};

use gif2json::RgbImageData;
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

    if !path.exists() {
        println!("\nError: Path does not exist\n");
        process::exit(1);
    }

    let image = match RgbImageData::new_from_gif(path) {
        Ok(data) => data,
        Err(err) => {
            println!("\nError: {}\n\nSyntax: gif2json [image_file].gif\n", err);
            process::exit(1);
        }
    };

    let mut output_path = PathBuf::from(path);
    output_path.set_extension("json");
    match image.save_as_json(&output_path) {
        Ok(_) => {}
        Err(err) => {
            println!("\nError: {}\n\nSyntax: gif2json [image_file].gif\n", err);
            process::exit(1);
        }
    }
}
