mod statement;

use std::io;
use std::io::{Error, Write};
use std::process::exit;

fn main() {
    loop {
        print_prompt();
        let input_buffer = read_input().unwrap();
        if input_buffer.buffer.starts_with(".") {
            handle_meta(&input_buffer).unwrap()
        }
        let sql = statement::prepare(input_buffer).unwrap();
        statement::execute(sql).unwrap()
    }
}

#[derive(Default)]
pub struct InputBuffer {
    buffer: String,
}

impl From<&str> for InputBuffer {
    fn from(s: &str) -> Self {
        InputBuffer {
            buffer: s.to_string()
        }
    }
}

fn print_prompt() {
    print!("msql > ");
    io::stdout().flush().unwrap();
}

fn read_input() -> Result<InputBuffer, Error> {
    let mut str = String::from("");
    io::stdin().read_line(&mut str)?;
    str.pop();
    Ok(InputBuffer::from(&str[..]))
}

fn handle_meta(buf: &InputBuffer) -> Result<(), &'static str> {
    if buf.buffer == ".exit" {
        exit(0)
    } else {
        Err("Unrecognized Command")
    }
}