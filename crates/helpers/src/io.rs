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

pub fn demand_stdin(thing: &str) -> String {
    loop {
        let input = prompt!("{thing}: ");

        if input.len() == 0 {
            println!("Please enter {thing}.");
            continue;
        }

        return input.to_string();
    }
}

#[macro_export]
macro_rules! prompt {
    (required, $($arg:tt)*) => {{
        loop {
            let input = $crate::prompt!($($arg)*);
            if input.is_empty() {
                println!("This is a required field.");
                continue;
            }

            break input;
        }
    }};

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