# PNGMe
PNGMe is a Rust project designed to explore the manipulation of PNG files. It's crafted as a self-learning tool to invite experimentation with the Rust language, showcasing multiple approaches—even if some don't represent the best practices for production code.

## Disclaimer

This project is strictly for educational purposes. It explores basic concepts and offers simple techniques for working with PNG files in Rust, intentionally including shortcuts and unconventional methods.

## Important Notes

- Use PNGMe as a learning aid to deepen your understanding of Rust programming.
- The implementations are simplified and serve as a gateway to more robust practices.

## How to Use

PNGMe is a command-line utility that supports several operations on PNG files. Here are the available commands and their usage:

### Commands

- **encode**  
    Embed a secret message into a PNG file.
    Usage:  
    cargo run -- encode <input_png> <chunk_type> <secret_message> [output_png]  
    Example:  
    cargo run -- encode image.png ruSt "This is a secret message!" new_image.png

- **decode**  
    Retrieve a hidden message from a PNG file.
    Usage:  
    cargo run -- decode <input_png> <chunk_type>  
    Example:  
    cargo run -- decode new_image.png ruSt

- **remove**  
    Remove a specific chunk from a PNG file.
    Usage:  
    cargo run -- remove <input_png> <chunk_type>  
    Example:  
    cargo run -- remove image.png ruSt

- **print**  
    Display information about a specific chunk in a PNG file.
    Usage:  
    cargo run -- print <input_png> <chunk_type>  
    Example:  
    cargo run -- print image.png ruSt