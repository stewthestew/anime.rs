use crossterm::{
    execute,
    style::{self},
};
use std::{
    io::{self, Write},
    process::exit,
    thread::sleep,
    time::Duration,
};

#[allow(dead_code)]
pub fn flint(str: &str, dur_ms: u64) {
    if let Err(e) = execute!(io::stdout(), style::Print(str),) {
        eprintln!("{e}");
        exit(1);
    };
    print!("\x1b[2K\r");
    sleep(Duration::from_millis(dur_ms));
}
pub fn show() {
    if let Err(e) = execute!(io::stdout(), crossterm::cursor::Show) {
        eprintln!("{e}");
        exit(1);
    }
}

pub fn hide() {
    if let Err(e) = execute!(io::stdout(), crossterm::cursor::Hide) {
        eprintln!("{e}");
        exit(1);
    }
}

#[allow(dead_code)]
pub fn spinner(start: &str, end: &str, times: u32, delay: u64) {
    hide();
    for _ in 0..times {
        flint(&format!("\r{}/{}", start, end), delay);
        flint(&format!("\r{}-{}", start, end), delay);
        flint(&format!("\r{}\\{}", start, end), delay);
        flint(&format!("\r{}|{}", start, end), delay);
    }
    show();
}

#[allow(dead_code)]
pub fn arrow(start: &str, end: &str, shaft_num: u32, delay: u64) {
    hide();
    let mut arrow: Vec<String> = vec![];
    for _ in 0..shaft_num {
        sleep(Duration::from_millis(delay));
        arrow.push(String::from("="));

        let formatted_arrow = format!("{}{}>{}", start, arrow.join(""), end);
        let trimmed_arrow = formatted_arrow.trim_matches('"');

        print!("\r{}", trimmed_arrow);
        match io::stdout().flush() {
            Ok(value) => value,
            Err(e) => {
                eprintln!("{e}");
                exit(1);
            }
        };
    }
    show();
}

#[allow(dead_code)]
pub fn arrow_brackets(start: &str, end: &str, num_shaft: u32, delay: u64) {
    hide();
    let mut arrow: Vec<String> = vec![];
    for _ in 0..num_shaft {
        sleep(Duration::from_millis(delay));
        arrow.push(String::from("="));

        let formatted_arrow = format!("{}[{}>{}]", start, arrow.join(""), end);
        let trimmed_arrow = formatted_arrow.trim_matches('"');

        print!("\r{}", trimmed_arrow);
        match io::stdout().flush() {
            Ok(value) => value,
            Err(e) => {
                eprintln!("{e}");
                exit(1);
            }
        };
    }
    show();
}

#[allow(dead_code)]
pub fn dots(start: &str, end: &str, empty_at_start: bool, times: u32, delay: u64) {
    hide();
    if empty_at_start == true {
        for _ in 0..times {
            flint(&format!("\r{}{}", start, end), delay);
            flint(&format!("\r{}.{}", start, end), delay);
            flint(&format!("\r{}..{}", start, end), delay);
            flint(&format!("\r{}...{}", start, end), delay);
        }
    } else {
        for _ in 0..times {
            flint(&format!("\r{}.{}", start, end), delay);
            flint(&format!("\r{}..{}", start, end), delay);
            flint(&format!("\r{}...{}", start, end), delay);
        }
    }
    show();
}

#[allow(dead_code)]
pub fn dots_spinner(start: &str, end: &str, times: u32, delay: u64) {
    hide();
    for _ in 0..times {
        flint(&format!("\r{}⣾{}", start, end), delay);
        flint(&format!("\r{}⣽{}", start, end), delay);
        flint(&format!("\r{}⣻{}", start, end), delay);
        flint(&format!("\r{}⢿{}", start, end), delay);
        flint(&format!("\r{}⡿{}", start, end), delay);
        flint(&format!("\r{}⣟{}", start, end), delay);
        flint(&format!("\r{}⣯{}", start, end), delay);
        flint(&format!("\r{}⣷{}", start, end), delay);
    }
    show();
}

#[allow(dead_code)]
pub fn mini_dots_spinner(start: &str, end: &str, times: u32, delay: u64) {
    hide();
    for _ in 0..times {
        flint(&format!("\r{}⠋{}", start, end), delay);
        flint(&format!("\r{}⠙{}", start, end), delay);
        flint(&format!("\r{}⠹{}", start, end), delay);
        flint(&format!("\r{}⠸{}", start, end), delay);
        flint(&format!("\r{}⠼{}", start, end), delay);
        flint(&format!("\r{}⠴{}", start, end), delay);
        flint(&format!("\r{}⠦{}", start, end), delay);
        flint(&format!("\r{}⠧{}", start, end), delay);
        flint(&format!("\r{}⠇{}", start, end), delay);
        flint(&format!("\r{}⠏{}", start, end), delay);
    }
    show();
}

#[allow(dead_code)]
pub fn pulse(start: &str, end: &str, times: u32, delay: u64) {
    hide();
    for _ in 0..times {
        flint(&format!("\r{}█{}", start, end), delay);
        flint(&format!("\r{}▓{}", start, end), delay);
        flint(&format!("\r{}▒{}", start, end), delay);
        flint(&format!("\r{}░{}", start, end), delay);
    }
    show();
}

#[allow(dead_code)]
pub fn loading_bar(start: &str, end: &str, num_shaft: u32, delay: u64) {
    hide();
    let mut arrow: Vec<String> = vec![];
    for _ in 0..num_shaft {
        sleep(Duration::from_millis(delay));
        arrow.push(String::from("█"));

        let formatted_arrow = format!("{}{}{}", start, arrow.join(""), end);
        let trimmed_arrow = formatted_arrow.trim_matches('"');

        print!("\r{}", trimmed_arrow);
        match io::stdout().flush() {
            Ok(value) => value,
            Err(e) => {
                eprintln!("{e}");
                exit(1);
            }
        };
    }
    show();
}

#[allow(dead_code)]
pub fn bouncing_equals(start: &str, end: &str, times: u32, delay: u64) {
    hide();
    for _ in 0..times {
        flint(&format!("\r{}={}", start, end), delay);
        flint(&format!("\r{}={}", start, end), delay);
    }
    show();
}

#[allow(dead_code)]
pub fn custom_loading(start: &str, end: &str, num_shaft: u32, delay: u64, char: &str) {
    hide();
    let mut arrow: Vec<String> = vec![];
    for _ in 0..num_shaft {
        sleep(Duration::from_millis(delay));
        arrow.push(String::from(char));

        let formatted_arrow = format!("{}{}{}", start, arrow.join(""), end);
        let trimmed_arrow = formatted_arrow.trim_matches('"');

        print!("\r{}", trimmed_arrow);
        match io::stdout().flush() {
            Ok(value) => value,
            Err(e) => {
                eprintln!("{e}");
                exit(1);
            }
        };
    }
    show();
}

#[allow(dead_code)]
pub fn custom(start: &str, end: &str, frames: Vec<&str>, times: u32, delay: u64) {
    hide();
    for _ in 0..times {
        for frame in &frames {
            flint(&format!("\r{}{}{}", start, frame, end), delay);
        }
    }
    show();
}
