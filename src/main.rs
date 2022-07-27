use crc32fast::Hasher;
use std::env;
use std::io::Read;
use std::fs::File;
use std::io::BufReader;


fn main() {
    if env::args().len() < 2 {
        println!("usage: crc32sum <file>");
        std::process::exit(0);
    }

    let file = env::args().nth(1).unwrap();
    let file = File::open(file).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    
    // Read file into vector.
    reader.read_to_end(&mut buffer).unwrap();

    let mut hasher = Hasher::new();
    hasher.update(&buffer);
    let checksum = hasher.finalize();
    println!("{:#x}", checksum);
}

