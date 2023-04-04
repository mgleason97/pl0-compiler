mod compiler;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

use compiler::lexer;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        panic!("Improper args");
    }

    let mut buf = [0u8; 1024];
    let mut reader = BufReader::new(File::open(&args[1])?);
    let nbytes = reader.read(&mut buf[..])?;
    
    let tokens = lexer::lex(&mut buf[..nbytes]);


    println!("Tokens:");
    for (idx, token) in tokens.iter().enumerate() {
        println!("{}: {:?}", idx, token);
    }
    
    Ok(())
}