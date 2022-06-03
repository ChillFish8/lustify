use std::path::Path;
use anyhow::{anyhow, Result};
use image::io::Reader as ImageReader;
use webp::{Encoder, WebPMemory};

fn main() -> Result<()> {
    let mut args = std::env::args();
    let _ = args.next();
    let glob_arg = args.next().unwrap_or_else(|| "**/*.png".to_string());

    for entry in glob::glob(&glob_arg)? {
        match entry {
            Ok(entry) => optimise_image(&entry)?,
            Err(e) => {
                eprintln!("Skipping file due to error {:?}", e);
            }
        }
    }

    Ok(())
}


fn optimise_image(path: &Path) -> Result<()> {
    let img = ImageReader::open(path)?.decode()?;

    let encoder: Encoder = Encoder::from_image(&img)
        .map_err(|e| anyhow!("{}", e))?;

    let webp: WebPMemory = encoder.encode(90f32);
    let output_path = path.with_extension("webp");
    std::fs::write(&output_path, &*webp)?;

    Ok(())
}