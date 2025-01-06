This application will show previews of Anime.rs animations.
And it also shows documentation of the animations
```
        "anime.rs - CLI Animation Library for the Terminal\n\
        -------------------------------------------------\n\
        anime.rs provides smooth and customizable terminal animations to enhance CLI applications.\n\
\
        Available Animations:\n\
\
        1. arrow - Displays an animated arrow effect.\n\
           Example: anime::arrow(\"Loading...\", 10, 100);\n\
           Parameters:\n\
           - text: The text displayed alongside the animation.\n\
           - shaft_num: The number of repeating shaft elements in the arrow.\n\
           - delay: Delay in milliseconds between updates.\n\
\
        2. arrow_bracket - Displays an arrow bracket animation.\n\
           Example: anime::arrow_brackets(\"Loading...\", 10, 100);\n\
           Parameters:\n\
           - text: The text displayed alongside the animation.\n\
           - num_shaft: The number of repeating shaft elements in the arrow.\n\
           - delay: Delay in milliseconds between updates.\n\
\
        3. dots - Creates a simple dot animation (e.g., ...).\n\
           Example: anime::dots(true, 3, 200);\n\
           Parameters:\n\
           - empty_at_start: If true, starts with no dots; otherwise starts with one dot.\n\
           - times: Number of repetitions of the dot sequence.\n\
           - delay: Delay in milliseconds between updates.\n\
\
        4. mini_dots_spinner - Displays a compact dots spinner.\n\
           Example: anime::mini_dots_spinner(3, 100);\n\
           Parameters:\n\
           - times: Number of repetitions of the spinner sequence.\n\
           - delay: Delay in milliseconds between updates.\n\
\
        5. dots_spinner - Displays a larger dots spinner animation.\n\
           Example: anime::dots_spinner(3, 100);\n\
           Parameters:\n\
           - times: Number of repetitions of the spinner sequence.\n\
           - delay: Delay in milliseconds between updates.\n\
\
        6. spinner - Provides a spinning animation effect.\n\
           Example: anime::spinner(3, 100);\n\
           Parameters:\n\
           - times: Number of repetitions of the spinner sequence.\n\
           - delay: Delay in milliseconds between updates.\n\
\
        7. loading_bar - Displays an animated loading bar.\n\
           Example: anime::loading_bar(\"Progress\", 10, 100);\n\
           Parameters:\n\
           - text: The text displayed alongside the animation.\n\
           - num_shaft: Number of progress bar segments.\n\
           - delay: Delay in milliseconds between updates.\n\
\
        8. pulse - Creates a pulsing animation effect.\n\
           Example: anime::pulse(3, 100);\n\
           Parameters:\n\
           - times: Number of repetitions of the pulse sequence.\n\
           - delay: Delay in milliseconds between updates.\n\
\
        9. bouncing_equals - Displays a bouncing equals sign animation.\n\
           Example: anime::bouncing_equals(3, 100);\n\
           Parameters:\n\
           - times: Number of repetitions of the bouncing equals sequence.\n\
           - delay: Delay in milliseconds between updates.\n\
\
        -------------------------------------------------\n\
        Additional Functions:\n\
\
        show - Ensures the terminal cursor is made visible after animations.\n\
           Example: anime::show();\n\
           Purpose: Called at the end of animations to prevent the cursor from remaining hidden.\n\
\
        hide - Hides the terminal cursor during animations.\n\
           Example: anime::hide();\n\
           Purpose: Used to enhance the animation effect by removing cursor distractions.\n\
\
        flint - A helper function for printing strings with a delay and clearing the current line.\n\
           Example: anime::flint(\"Loading...\", 100);\n\
           Parameters:\n\
           - str: The string to be printed.\n\
           - dur_ms: Duration in milliseconds before clearing the line.\n\
           Purpose: Smoothly displays intermediate frames during animations.\n\
\
        Replace these examples with your use cases or refer to the anime.rs documentation for more details.
```
