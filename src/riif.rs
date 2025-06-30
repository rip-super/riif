mod riif_core;

use riif_core::{cli::parse_args, decode::decode, encode::encode, read::read, view::view};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args();

    if args.encode {
        encode(&args.input)?;
    } else if args.decode {
        decode(&args.input)?;
    } else if args.view {
        let img = read(&args.input)?;
        view(img, args.input)?;
    } else {
        eprintln!("Please specify one of --encode, --decode or --view");
    }

    Ok(())
}
