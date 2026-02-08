use std::io::{stdin, Error};

pub fn stdin_readline() -> Result<String, Error> {
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;

    return Ok(buf);
}

pub fn get_trimmed_stdin() -> String {
    let raw = stdin_readline().expect("Encountered error when reading from stdin.");
    return raw.trim().to_string();
}

#[macro_export]
macro_rules! prompt {
    ($($arg:tt)*) => {{
        use std::io::Write;
        print!($($arg)*);
        std::io::stdout().flush().expect("Unable to flush stdout.");
        $crate::io::get_trimmed_stdin()
    }};
}

#[macro_export]
macro_rules! print_flush {
    ($($t:tt)*) => {
        print!($($t)*);
        std::io::Write::flush(&mut std::io::stdout()).unwrap()
    };
}

pub use { prompt, print_flush };