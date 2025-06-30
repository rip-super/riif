#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod riif_core;

use riif_core::{read::read, view::view};
use std::{env, error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .ok_or("Usage: image_viewer <path-to-image>")?;

    let path_ref = Path::new(&path);
    if !path_ref.is_file() {
        return Err(format!("Error: '{}' is not a valid file path.", path).into());
    }

    let img = read(&path)?;
    view(img, path)?;

    Ok(())
}
