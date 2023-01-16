# Morse Code Beeper

This project is a simple command-line tool that converts a given text input into Morse code and plays the corresponding beep sounds. It uses the rodio library for playing the beep sounds.

## Usage

```bash
morse_code_beeper [OPTIONS]

OPTIONS:
  -m, --msg <MSG>  The message to convert to Morse code
  -h, --help       Print help
  -V, --version    Print version
```

## Requirements

- Rust
- Cargo

## Installation

1.  Clone the repository

`git clone https://github.com/ninenine/morse_code_beeper.git`

2.  Navigate to the project directory

`cd morse_code_beeper`

3.  Build the project

`cargo build --release`

4.  Run the binary

`./target/release/morse_code_beeper -m "text to be converted"`

The tool will convert the provided text to uppercase and play the corresponding beep sounds for each letter. The beep sound for a dot is played for 50 milliseconds, and a dash for 150 milliseconds.

## Note

The tool will check for invalid input such as empty input or input that contains characters other than letters and numbers. If invalid input is provided, it will print an error message and exit the program.
