use std::{fs::File, io::{BufReader, BufWriter}};

use tar::{Archive, Builder};

fn first() {
    let mut some = File::open("some.txt").unwrap();
    let mut arch_builder = Builder::new(BufWriter::new(File::create("some.tar").unwrap()));
    arch_builder
        .append_file("texts/some.txt", &mut some)
        .unwrap();
    arch_builder.finish().unwrap();
}

fn second() {
    let some = BufReader::new(File::open("some.tar").unwrap());
    let mut arch = Archive::new(some);
    for entry in arch.entries().unwrap().flatten() {
        println!("{}", entry.path().unwrap().to_path_buf().to_str().unwrap());
    }
}

fn main() {
    second();
    println!("Hello, world!");
}
