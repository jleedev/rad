#[macro_use] extern crate matches;
#[macro_use] extern crate nom;
extern crate rusqlite;
extern crate tempdir;

mod buffer;
mod command;

use std::io::{BufReader, BufRead, stdin};

use buffer::Buffer;

type Error = Box<std::error::Error>;

fn load_file(filename: &str) -> Result<Buffer, Error> {
    let mut b = Buffer::new()?;
    let f = std::fs::File::open(filename)?;
    let reader = BufReader::new(f);
    b.extend(reader.lines())?;
    Ok(b)
}

fn editor_main() -> Result<(), Error> {
    let args = std::env::args();
    let filename = args.skip(1).next().ok_or("no filename")?;
    let buffer = load_file(&filename)?;
    println!("{}", buffer.line_count()?);
    loop {
        let mut s = String::new();
        if stdin().read_line(&mut s)? == 0 {
            break;
        }
        let cmd = match command::handle_command(&s) {
            Ok(c) => c,
            Err(e) => {
                println!("?");
                continue
            }
        };
        match cmd {
            command::Command::No(addr) => {
                let line = buffer.line(addr)?;
                println!("{:?}", line);
            },
            _ => {},
        }
    }
    Ok(())
}

fn main() {
    editor_main().unwrap();
}
