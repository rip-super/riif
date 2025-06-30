use crate::riif_core::filters::remove_filters;
use flate2::read::ZlibDecoder;
use image::RgbaImage;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Read, Result as IoResult};

pub fn read(path: &str) -> IoResult<RgbaImage> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let (width, height) = read_header(&mut reader)?;
    let filters = read_filters(&mut reader, height)?;
    let filtered_bytes = read_filtered_bytes(&mut reader, height, width)?;

    let img = remove_filters(width, height, &filters, &filtered_bytes);
    Ok(img)
}

fn read_header(reader: &mut BufReader<File>) -> IoResult<(u32, u32)> {
    let mut magic = [0u8; 4];
    reader.read_exact(&mut magic)?;
    if &magic != b"RIIF" {
        return Err(Error::new(ErrorKind::InvalidData, "Invalid RIIF magic"));
    }

    let mut width_bytes = [0u8; 4];
    let mut height_bytes = [0u8; 4];
    reader.read_exact(&mut width_bytes)?;
    reader.read_exact(&mut height_bytes)?;

    let width = u32::from_le_bytes(width_bytes);
    let height = u32::from_le_bytes(height_bytes);

    Ok((width, height))
}

fn read_filters(reader: &mut BufReader<File>, height: u32) -> IoResult<Vec<u8>> {
    let mut filters = vec![0u8; height as usize];
    reader.read_exact(&mut filters)?;
    Ok(filters)
}

fn read_filtered_bytes(reader: &mut BufReader<File>, height: u32, width: u32) -> IoResult<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(reader);
    let pixel_count = (width * height) as usize;
    let mut rgba_bytes = vec![0u8; pixel_count * 4];
    decoder.read_exact(&mut rgba_bytes)?;
    Ok(rgba_bytes)
}
