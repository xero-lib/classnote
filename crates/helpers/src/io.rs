use std::io::{stdout, stdin, Write, Error};

pub fn stdin_readline() -> Result<String, Error> {
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;

    return Ok(buf);
}

pub fn print(input: &str) {
    print!("{input}");
    stdout().flush().expect("Unable to flush stdout buffer.");
}
