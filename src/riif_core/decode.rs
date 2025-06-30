use crate::riif_core::read::read;
use std::path::Path;

pub fn decode(input: &str) -> std::io::Result<()> {
    let output = Path::new(input)
        .with_extension("png")
        .to_string_lossy()
        .into_owned();
    let img = read(input)?;
    img.save(&output).expect("Failed to save PNG");
    println!("Saved RIIF '{}' as PNG '{}'", input, output);
    Ok(())
}
