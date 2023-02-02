use bzip3::read::Bz3Decoder;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::{env, io};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut reader = BufReader::new(File::open("data.bz3").unwrap());

    let mut writer = BufWriter::new(
        File::options()
            .truncate(true)
            .create(true)
            .write(true)
            .read(true)
            .open(PathBuf::from(out_dir).join("include"))
            .unwrap(),
    );
    let mut decoder = Bz3Decoder::new(&mut reader).unwrap();
    io::copy(&mut decoder, &mut writer).unwrap();
}
