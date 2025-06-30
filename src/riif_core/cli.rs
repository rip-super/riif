use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Encode an image to RIIF format
    #[arg(short, long, conflicts_with_all = &["decode", "view"])]
    pub encode: bool,

    /// Save RIIF file to PNG image
    #[arg(short, long, conflicts_with_all = &["encode", "view"])]
    pub decode: bool,

    /// Display a RIIF file in the image viewer
    #[arg(short, long, conflicts_with_all = &["encode", "decode"])]
    pub view: bool,

    /// Input image or RIIF file path
    pub input: String,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
