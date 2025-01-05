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

// required componment
// Oh yeah  updated flint to use crossterm
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
/* does not take text
* only implements spinner
*/

#[allow(dead_code)]
pub fn spinner(times: u32, delay: u64) {
    hide();
    for _ in 0..times {
        flint("\r/", delay);
        flint("\r-", delay);
        flint("\r\\", delay);
        flint("\r|", delay);
    }
    show();
}

#[allow(dead_code)]
pub fn arrow(text: &str, shaft_num: u32, delay: u64) {
    hide();
    let mut arrow: Vec<String> = vec![];
    for _ in 0..shaft_num {
        sleep(Duration::from_millis(delay));
        arrow.push(String::from("="));

        let formatted_arrow = format!("{text}{}{}", arrow.join(""), ">");
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
pub fn arrow_brackets(text: &str, num_shaft: u32, delay: u64) {
    hide();
    let mut arrow: Vec<String> = vec![];
    for _ in 0..num_shaft {
        sleep(Duration::from_millis(delay));
        arrow.push(String::from("="));

        let formatted_arrow = format!("{text}[{}{}]", arrow.join(""), ">");
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
pub fn dots(empty_at_start: bool, times: u32, delay: u64) {
    hide();
    if empty_at_start == true {
        for _ in 0..times {
            flint("\r", delay);
            flint("\r.", delay);
            flint("\r..", delay);
            flint("\r...", delay);
        }
    } else {
        for _ in 0..times {
            flint("\r.", delay);
            flint("\r..", delay);
            flint("\r...", delay);
        }
    }
    show();
}
//⣾  ⣽  ⣻  ⢿  ⡿  ⣟  ⣯  ⣷
#[allow(dead_code)]
// The reccomended speed for this is 100 miliseconds
pub fn dots_spinner(times: u32, delay: u64) {
    hide();
    for _ in 0..times {
        flint("\r⣾", delay);
        flint("\r⣽", delay);
        flint("\r⣻", delay);
        flint("\r⢿", delay);
        flint("\r⡿", delay);
        flint("\r⣟", delay);
        flint("\r⣯", delay);
        flint("\r⣷", delay);
    }
    show();
}

// ⠋ ⠙ ⠹ ⠸ ⠼ ⠴ ⠦ ⠧ ⠇ ⠏
#[allow(dead_code)]
// the reccomended speed for this is 100 miliseconds
pub fn mini_dots_spinner(times: u32, delay: u64) {
    hide();
    for _ in 0..times {
        flint("\r⠋", delay);
        flint("\r⠙", delay);
        flint("\r⠹", delay);
        flint("\r⠸", delay);
        flint("\r⠼", delay);
        flint("\r⠴", delay);
        flint("\r⠦", delay);
        flint("\r⠧", delay);
        flint("\r⠇", delay);
        flint("\r⠏", delay);
    }
    show();
}
// █ ▓ ▒ ░
// the reccomended speed for this is 100 miliseconds
#[allow(dead_code)]
pub fn pulse(times: u32, delay: u64) {
    hide();
    for _ in 0..times {
        flint("\r█", delay);
        flint("\r▓", delay);
        flint("\r▒", delay);
        flint("\r░", delay);
    }
    show();
}

// █  ░

#[allow(dead_code)]
pub fn loading_bar(text: &str, num_shaft: u32, delay: u64) {
    hide();
    let mut arrow: Vec<String> = vec![];
    for _ in 0..num_shaft {
        sleep(Duration::from_millis(delay));
        arrow.push(String::from("█"));

        let formatted_arrow = format!("{text}{}", arrow.join(""));
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

// use the = symbold
#[allow(dead_code)]
pub fn bouncing_equals(times: u32, delay: u64) {
    hide();
    for _ in 0..times {
        flint("\r= ", delay);
        flint("\r =", delay);
    }
    show();
}
