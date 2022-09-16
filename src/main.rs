mod args;
mod mosaic;

use args::MosaicArgs;
use clap::Parser;
use mosaic::Mosaic;

fn main() {
    let args = MosaicArgs::parse();
    let mut mosaic = Mosaic::new(args.input, args.output, args.tiles);
    mosaic.create();
}
