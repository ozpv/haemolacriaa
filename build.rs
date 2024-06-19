use image::{open, ImageFormat};
use std::fs::{read_dir, File};
use std::path::Path;

/// Doesn't actually optimize yet
/// I'm going to have to figure out
/// Encoding settings
fn encode_as_webp(assets_dir: &str) {
    let assets_dir = Path::new(assets_dir);

    read_dir(assets_dir)
        .expect("Failed to read assets directory")
        .filter_map(|file| file.ok())
        .map(|file| file.path())
        .filter_map(|file| {
            if file
                .extension()
                .map_or(false, |extention| extention != "webp" && extention != "ico")
            {
                Some(file)
            } else {
                None
            }
        })
        .for_each(|file| {
            let image = open(file.clone()).unwrap();
            let new_file =
                &mut File::create(Path::new(&format!("{}.webp", file.display()))).unwrap();
            println!("Optimized the following image: {file:?}");
            image.write_to(new_file, ImageFormat::WebP).unwrap();
        });
}

fn main() {
    println!("cargo:rerun-if-changed=migrations");
    encode_as_webp("assets");
}
