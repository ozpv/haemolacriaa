use image::{ImageFormat, open};
use std::fs::{File, read_dir};
use std::path::Path;

/// Doesn't actually optimize yet
/// I'm going to have to figure out
/// Encoding settings
fn encode_as_webp<'a>(assets_dir: &'a str) {
    let assets_dir = Path::new(assets_dir);

    read_dir(assets_dir)
        .expect("Failed to read assets directory")
        .filter_map(|file| file.ok())
        .map(|entry| entry.path())
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
            let new_file = &mut File::create(Path::new(&format!("{}.webp", file.display()))).unwrap();
            println!("Optimized the following image: {file:?}");
            image.write_to(new_file, ImageFormat::WebP).unwrap();
        });
}

fn main() {
    encode_as_webp("assets");
}
