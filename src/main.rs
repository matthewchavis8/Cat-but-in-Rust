use cat_rust::{Args, Cat};
use clap::Parser;
 
fn main() {
    let args = Args::parse();
    let mut cat = Cat::new();

    cat.parse_file(args.files[0].clone());
    cat.run();
}