use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Seek, SeekFrom},
};

use tar::{Archive, Builder};
use zstd::{Decoder, Encoder};

fn first() {
    let mut some = File::open("some.txt").unwrap();
    let mut arch_builder = Builder::new(
        Encoder::new(BufWriter::new(File::create("some.tar.ztd").unwrap()), 19)
            .unwrap()
            .auto_finish(),
    );
    arch_builder
        .append_file("texts/some.txt", &mut some)
        .unwrap();
    some = File::open("some.txt").unwrap();
    arch_builder
        .append_file("texts/some2.txt", &mut some)
        .unwrap();
    arch_builder.finish().unwrap();
}

fn seek_file<R: Read>(file: R) {
    let some = Decoder::new(file).unwrap();
    let mut arch = Archive::new(some);
    for entry in arch.entries().unwrap().flatten() {
        let filename: String = entry
            .path()
            .unwrap()
            .to_path_buf()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .into();
        println!("{}, {}", filename, entry.header().size().unwrap());
        let mut content = String::new();
        if let Some(err) = BufReader::new(entry).read_to_string(&mut content).err() {
            eprintln!("{err}");
        }
        println!("{content}");

    }
}

fn second() {
    let mut file = BufReader::new(File::open("some.tar.ztd").unwrap());
    println!("first seek");
    seek_file(&mut file);
    println!("second seek");
    file.seek(SeekFrom::Start(0)).unwrap();
    seek_file(&mut file);
}

fn main() {
    first();
    second();
    println!("Hello, world!");
}
