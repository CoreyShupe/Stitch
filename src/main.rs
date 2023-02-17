use clap::Parser;
use std::process::exit;

pub(crate) mod arg_parse;

fn main() {
    let args = arg_parse::CommandLineArgs::parse();
    if !args.entry_point.exists() {
        println!(
            "Entry point could not be found: {:#?}",
            args.entry_point.canonicalize()
        );
        exit(1);
    }
}
