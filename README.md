chat gpt btw
# Caution
- This is made for my own personal use.
# ANIME.rs
A Simple Rust Animation Library for the Terminal
ANIME.rs is a minimalistic terminal animation library written in Rust. It provides basic text-based animations for enhancing the user experience in terminal-based applications.

Features:
- **Flint**: Prints text with custom timing and clears the line.
- **Spinner**: Displays a simple spinner animation.
- **Arrow**: An animated arrow that progressively builds.
- **Arrow Brackets**: Displays an arrow with brackets around the shaft.
- **Dots**: A simple dot animation with an option to start empty.
- **Dots Spinner**: A spinner with rotating dot symbols.
- **Pulse**: A pulsing effect with different block characters.
- **Mini Dots Spinner**: A spinner with various dot symbols.
- **Loading Bar**: Simulate a loading bar with customizable steps and speed.
Usage:
```
anime::loading_bar("Loading... ", 20, 50);
anime::dots(true, 3, 100);
```
Notes:
- Aimed at simplicity ~~and speed~~
- Customizable delay and length for each animation type.
