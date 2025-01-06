use anime;
use clap::{command, Arg};
fn main() {
    let matches = command!()
        .about("Shows examples of Anime.rs cli animations")
        .arg(
            //a dasdas
            Arg::new("animation")
                .short('a')
                .help("Specifies which animation to play. If Animation is not found, it will show you a list of animations.\nPlus help / documentation for them")
                .required(true),
        )
        .get_matches();
    let anim = matches.get_one::<String>("animation").unwrap().as_str();
    match anim {
        "arrow" => {
            anime::arrow("Text... ", 10, 100);
            println!("\nHey");
        }
        "arrow_bracket" => {
            anime::arrow_brackets("Text... ", 10, 100);
            println!("\nHey");
        }
        "dots" => {
            anime::dots(true, 3, 200);
            println!("Hey");
        }

        "mini_dots_spinner" => {
            anime::mini_dots_spinner(3, 100);
            println!("Hey");
        }

        "dots_spinner" => {
            anime::dots_spinner(3, 100);
            println!("Hey");
        }

        "spinner" => {
            anime::spinner(3, 100);
            println!("Hey");
        }

        "loading_bar" => {
            anime::loading_bar("Text... ", 10, 100);
            println!("\nHey");
        }
        "pulse" => {
            anime::pulse(3, 100);
            println!("Hey");
        }
        "bouncing_equals" => {
            anime::bouncing_equals(3, 100);
        }
        _ => {
            println!("Did not find animation... Showing help now.");
            let help = a_valible();
            println!("{help}");
        }
    }
}

fn a_valible() -> String {
    String::from(
        "anime.rs - CLI Animation Library for the Terminal\n\n\
        -------------------------------------------------\n\n\
        anime.rs provides smooth and customizable terminal animations to enhance CLI applications.\n\n\
        Available Animations:\n\n\
        1. arrow - Displays an animated arrow effect.\n\
           - Example: anime::arrow(\"Loading...\", 10, 100);\n\
           - Parameters:\n\
             - text: The text displayed alongside the animation.\n\
             - shaft_num: The number of repeating shaft elements in the arrow.\n\
             - delay: Delay in milliseconds between updates.\n\n\
        2. arrow_bracket - Displays an arrow bracket animation.\n\
           - Example: anime::arrow_brackets(\"Loading...\", 10, 100);\n\
           - Parameters:\n\
             - text: The text displayed alongside the animation.\n\
             - num_shaft: The number of repeating shaft elements in the arrow.\n\
             - delay: Delay in milliseconds between updates.\n\n\
        3. dots - Creates a simple dot animation (e.g., ...).\n\
           - Example: anime::dots(true, 3, 200);\n\
           - Parameters:\n\
             - empty_at_start: If true, starts with no dots; otherwise starts with one dot.\n\
             - times: Number of repetitions of the dot sequence.\n\
             - delay: Delay in milliseconds between updates.\n\n\
        4. mini_dots_spinner - Displays a compact dots spinner.\n\
           - Example: anime::mini_dots_spinner(3, 100);\n\
           - Parameters:\n\
             - times: Number of repetitions of the spinner sequence.\n\
             - delay: Delay in milliseconds between updates.\n\n\
        5. dots_spinner - Displays a larger dots spinner animation.\n\
           - Example: anime::dots_spinner(3, 100);\n\
           - Parameters:\n\
             - times: Number of repetitions of the spinner sequence.\n\
             - delay: Delay in milliseconds between updates.\n\n\
        6. spinner - Provides a spinning animation effect.\n\
           - Example: anime::spinner(3, 100);\n\
           - Parameters:\n\
             - times: Number of repetitions of the spinner sequence.\n\
             - delay: Delay in milliseconds between updates.\n\n\
        7. loading_bar - Displays an animated loading bar.\n\
           - Example: anime::loading_bar(\"Progress\", 10, 100);\n\
           - Parameters:\n\
             - text: The text displayed alongside the animation.\n\
             - num_shaft: Number of progress bar segments.\n\
             - delay: Delay in milliseconds between updates.\n\n\
        8. pulse - Creates a pulsing animation effect.\n\
           - Example: anime::pulse(3, 100);\n\
           - Parameters:\n\
             - times: Number of repetitions of the pulse sequence.\n\
             - delay: Delay in milliseconds between updates.\n\n\
        9. bouncing_equals - Displays a bouncing equals sign animation.\n\
           - Example: anime::bouncing_equals(3, 100);\n\
           - Parameters:\n\
             - times: Number of repetitions of the bouncing equals sequence.\n\
             - delay: Delay in milliseconds between updates.\n\n\
        -------------------------------------------------\n\n\
        Additional Functions:\n\n\
        show\n\
           - Ensures the terminal cursor is made visible after animations.\n\
           - Example: anime::show();\n\n\
        hide\n\
           - Hides the terminal cursor during animations.\n\
           - Used to enhance the animation effect by removing cursor distractions.\n\
           - Example: anime::hide();\n\n\
        flint\n\
           - A helper function for printing strings with a delay and clearing the current line.\n\
           - Example: anime::flint(\"Loading...\", 100);\n\
           - Parameters:\n\
             - str: The string to be printed.\n\
             - dur_ms: Duration in milliseconds before clearing the line.\n\
           - Purpose: Smoothly displays intermediate frames during animations.\n\n\
        Replace these examples with your use cases or refer to the anime.rs documentation for more details."
    )
}
