use clap::Parser;

/// Simple photomosaic creator
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct MosaicArgs {
    /// Input image
    #[clap(short, long, value_parser)]
    pub input: String,

    /// Output image
    #[clap(short, long, value_parser, default_value = "output.png")]
    pub output: String,

    /// Tiles directory
    #[clap(short, long, value_parser)]
    pub tiles: String,
}
