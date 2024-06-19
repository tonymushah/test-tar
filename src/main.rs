use std::{fs::File, io::BufWriter};

use tar::Builder;

fn main() {
    let mut some = File::open("some.txt").unwrap();
    let mut arch_builder = Builder::new(BufWriter::new(File::create("some.tar").unwrap()));
    arch_builder
        .append_file("texts/some.txt", &mut some)
        .unwrap();
    arch_builder.finish().unwrap();
    println!("Hello, world!");
}
