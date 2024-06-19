use std::{fs::File, io::{BufReader, BufWriter}};

use tar::{Archive, Builder};
use zstd::{Decoder, Encoder};

fn first() {
    let mut some = File::open("some.txt").unwrap();
    let mut arch_builder = Builder::new(Encoder::new(BufWriter::new(File::create("some.tar.ztd").unwrap()), 19).unwrap().auto_finish());
    arch_builder
        .append_file("texts/some.txt", &mut some)
        .unwrap();
    arch_builder.finish().unwrap();
}

fn second() {
    let some = Decoder::new(BufReader::new(File::open("some.tar.ztd").unwrap())).unwrap();
    let mut arch = Archive::new(some);
    for entry in arch.entries().unwrap().flatten() {
        println!("{}", entry.path().unwrap().to_path_buf().to_str().unwrap());
    }
}

fn main() {
    first();
    second();
    println!("Hello, world!");
}
