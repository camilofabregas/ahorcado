use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

mod partida;
use partida::nueva_partida;

fn main()-> std::io::Result<()> {
    let file = File::open("archivo.txt")
        .expect("file not found!");
    let  buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        nueva_partida(&line?);
    }
    Ok(())
}