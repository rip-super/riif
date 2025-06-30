use crate::riif_core::filters::apply_filters;
use flate2::{Compression, write::ZlibEncoder};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[repr(C)]
struct Header {
    magic: [u8; 4], // b"RIIF"
    width: u32,     // little-endian
    height: u32,    // little-endian
}

pub fn encode(input: &str) -> std::io::Result<()> {
    let output = Path::new(input)
        .with_extension("riif")
        .to_string_lossy()
        .into_owned();
    let img = image::open(input)
        .expect("Failed to open input image")
        .into_rgba8();
    let (width, height) = img.dimensions();
    let file = File::create(&output)?;
    let mut writer = BufWriter::new(file);

    // --- Header (12 bytes) ---
    let header = Header {
        magic: *b"RIIF",
        width: width.to_le(),
        height: height.to_le(),
    };

    writer.write_all(&header.magic)?;
    writer.write_all(&header.width.to_le_bytes())?;
    writer.write_all(&header.height.to_le_bytes())?;

    // --- Filter Data (height bytes) ---
    let (filters, filtered_bytes) = apply_filters(&img);
    writer.write_all(&filters)?;

    // --- Pixel Data (Uncompressed: width * height * 4 bytes) ---
    let mut encoder = ZlibEncoder::new(writer, Compression::default());
    encoder.write_all(&filtered_bytes)?;
    encoder.finish()?;

    println!("Encoded '{}' to '{}'", input, output);

    Ok(())
}
