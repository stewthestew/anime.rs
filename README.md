chat gpt btw
> [!WARNING]
> This is made for my own personal use.
> 
> And it's consistantly changing, since it has not matured yet.
>
> I am in the proccess of learning rust. I made this project just to learn rust
# ANIME.rs
- A Simple Rust Animation Library for the Terminal.
- ANIME.rs is a minimalistic terminal animation library written in Rust. It provides basic text-based animations for enhancing the user experience in terminal-based applications.

Features:
- **Spinner**: Displays a simple spinner animation.
- **Arrow**: An animated arrow that progressively builds.
- **Arrow Brackets**: Displays an arrow with brackets around the shaft.
- **Dots**: A simple dot animation with an option to start empty.
- **Dots Spinner**: A spinner with rotating dot symbols.
- **Pulse**: A pulsing effect with different block characters.
- **Mini Dots Spinner**: A spinner with various dot symbols.
- **Loading Bar**: Simulate a loading bar with customizable steps and speed.
- **Bouncing equals**: A bouncing equals sign animation.

# Usage
``` rust
fn main() {
    let mut name = String::new();
    anime::loading_bar("Loading program: ", "", 10, 100);
    println!("\nWhat's your name?");
    stdin()
        .read_line(&mut name)
        .expect("Error happened with reading line");
    anime::dots("", "", true, 3, 100);
    println!("What's your Adress?");
    anime::hide(); // Hides the cursor permenantly until shown
    sleep(Duration::from_secs(1));
    anime::dots("", "", true, 3, 300);
    println!("Uhh...");
    anime::dots("", "", true, 2, 200);
    println!("No");
    anime::dots("", "", true, 2, 100);
    println!("Ok bye!");
    anime::dots("", "", true, 3, 200);
    println!("Bye?");
}
```
Notes:
- Aimed at simplicity ~~and speed~~
- Customizable delay and length for each animation type.
- If your cursor stays hidden forever, that's a you problem, not an anime.rs problem.


# Showcase
![showcase2](https://github.com/user-attachments/assets/45d5cf8b-d143-4595-a206-c140982c6673)

# Credit
- [Crossterm](http://github.com/crossterm-rs/crossterm) for powering anime.rs
- [Clap](https://github.com/clap-rs/clap) For powering the anime.rs tutorial cli tool

# Todo
- [ ] Update Documentation
