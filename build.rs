use image::{ImageFormat, ImageReader};
use sha256::try_digest;
use std::error::Error;
use std::ffi::OsString;
use std::fs::{read_dir, rename, write};
use std::path::{Path, PathBuf};
use webp::Encoder;

#[allow(dead_code)]
fn encode_as_webp(path: &Path) -> Result<(), Box<dyn Error>> {
    let image = {
        let res = ImageReader::open(path)?.with_guessed_format()?;
        let format = res.format().ok_or("Failed to get format of file!")?;

        // skip these files
        if format == ImageFormat::WebP || format == ImageFormat::Ico {
            return Ok(());
        }

        res.decode()?
    };

    let encoded = Encoder::from_image(&image)?.encode(90.0);

    let new_path = {
        let mut res = path.parent().unwrap().to_owned();
        let mut filename = path.file_stem().unwrap().to_owned();
        filename.push(".webp");
        res.push(filename);
        res
    };

    println!("Writing WebP to {new_path:?}");

    write(new_path, &*encoded)?;

    Ok(())
}

#[allow(dead_code)]
fn add_hash_to_filename(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let hash = try_digest(path.clone())?;

    'outer: {
        let Some((_, current_hash)) =
            ({ path.file_stem().unwrap().to_str().unwrap().split_once('-') })
        else {
            break 'outer;
        };
        if current_hash == hash {
            return Ok(());
        }
    }

    let new_path = {
        let mut res = path.parent().unwrap().to_owned();
        let mut p = path.file_stem().unwrap().to_owned();
        p.push(format!("-{hash}."));
        p.push(path.extension().unwrap());
        res.push(p);
        res
    };

    println!("Renaming file to {new_path:?}");

    rename(path, new_path)?;

    Ok(())
}

#[allow(dead_code)]
fn remove_hash(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let Some((original_name, _)) = path.file_stem().unwrap().to_str().unwrap().split_once('-')
    else {
        return Ok(());
        // return Err("Hash not found in name!")?;
    };

    let new_path = {
        let mut res = path.parent().unwrap().to_owned();
        let mut f = OsString::new();
        f.push(format!("{original_name}."));
        f.push(path.extension().unwrap());
        res.push(f);
        res
    };

    println!("Renaming file to {new_path:?}");

    rename(path, new_path)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=migrations");

    read_dir("./assets")
        .expect("Dir should exist!")
        .for_each(|entry| {
            // add_hash_to_filename(&entry.expect("Entry should exist").path())
            //     .expect("Failed to rename file!");

            // use only if files have hash in name
            // remove_hash(&entry.expect("Entry should exist").path()).expect("Failed to rename file!");

            // encode the dir as WebP
            encode_as_webp(&entry.expect("Entry should exist").path())
                .expect("Failed to encode as WebP");
        });

    Ok(())
}
