use std::{
    fs::File,
    io::{BufReader, BufWriter, Read},
};

use tar::{Archive, Builder};
use tempfile::tempdir_in;
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

fn second() {
    let some = Decoder::new(BufReader::new(File::open("some.tar.ztd").unwrap())).unwrap();
    let mut arch = Archive::new(some);
    for mut entry in arch.entries().unwrap().flatten() {
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
        let tempdir = tempdir_in(".").unwrap();
        let temp_file = {
            let path = tempdir.path().join(filename);
            entry.unpack(&path).unwrap();
            File::open(path).unwrap()
        };
        let mut content = String::new();
        if let Some(err) = BufReader::new(temp_file).read_to_string(&mut content).err() {
            eprintln!("{err}");
        }
        println!("{content}");

        tempdir.close().unwrap();
    }
}

fn main() {
    first();
    second();
    println!("Hello, world!");
}
