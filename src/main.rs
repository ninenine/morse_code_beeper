use clap::Parser;
use once_cell::sync::Lazy;
use rodio::{
    source::{SineWave, Source},
    OutputStream, Sink,
};
use std::{
    collections::HashMap,
    io::{self, Write},
};

// Define the mapping of characters to Morse code
static MORSE_CODE_DICT: Lazy<HashMap<char, &'static str>> = Lazy::new(|| {
    [
        ('A', ".-"),
        ('B', "-..."),
        ('C', "-.-."),
        ('D', "-.."),
        ('E', "."),
        ('F', "..-."),
        ('G', "--."),
        ('H', "...."),
        ('I', ".."),
        ('J', ".---"),
        ('K', "-.-"),
        ('L', ".-.."),
        ('M', "--"),
        ('N', "-."),
        ('O', "---"),
        ('P', ".--."),
        ('Q', "--.-"),
        ('R', ".-."),
        ('S', "..."),
        ('T', "-"),
        ('U', "..-"),
        ('V', "...-"),
        ('W', ".--"),
        ('X', "-..-"),
        ('Y', "-.--"),
        ('Z', "--.."),
        ('1', ".----"),
        ('2', "..---"),
        ('3', "...--"),
        ('4', "....-"),
        ('5', "....."),
        ('6', "-...."),
        ('7', "--..."),
        ('8', "---.."),
        ('9', "----."),
        ('0', "-----"),
        (',', "--..--"),
        ('.', ".-.-.-"),
        ('?', "..--.."),
        ('/', "-..-."),
        ('-', "-....-"),
        ('(', "-.--."),
        (')', "-.--.-"),
    ]
    .iter()
    .cloned()
    .collect()
});

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    /// The message to convert to Morse code
    #[arg(short, long)]
    msg: String,
}

impl Args {
    fn check(&self) -> Result<(), String> {
        if self.msg.is_empty() {
            Err("Message cannot be empty".to_string())
        } else if self.msg.len() > 100 {
            Err("Message cannot be longer than 100 characters".to_string())
        } else if self
            .msg
            .chars()
            .any(|c| !MORSE_CODE_DICT.contains_key(&c.to_uppercase().next().unwrap()) && c != ' ')
        {
            Err("Message can only contain letters and numbers".to_string())
        } else {
            Ok(())
        }
    }
}

fn main() {
    let args = Args::parse();
    args.check().unwrap_or_else(|e| {
        println!("Error: {:?}", e);
        std::process::exit(1);
    });
    let msg = args.msg;
    // Get the output stream
    let (_stream, stream_handle) = match OutputStream::try_default() {
        Ok(o) => o,
        Err(e) => {
            println!("Error: {:?}", e);
            std::process::exit(1);
        }
    };

    // Create a sink to the output stream
    let sink = match Sink::try_new(&stream_handle) {
        Ok(s) => s,
        Err(e) => {
            println!("Error: {:?}", e);
            std::process::exit(1);
        }
    };

    // Convert the msg to uppercase
    let msg = msg.to_uppercase();
    // Loop through each character in the msg
    for c in msg.chars() {
        io::stdout().write(c.to_string().as_bytes()).unwrap();
        // Check if the character is a space
        if c == ' ' {
            io::stdout().write(b"\n").unwrap();
            // Average time between spaces in words: 3 dots (or 3 * the time to play a dot)
            std::thread::sleep(std::time::Duration::from_millis(150));
        } else {
            // Get the Morse code for the character
            let morse_code = MORSE_CODE_DICT.get(&c).unwrap();
            // Loop through each dot and dash in the Morse code
            for code in morse_code.chars() {
                io::stdout().write(code.to_string().as_bytes()).unwrap();
                if code == '.' {
                    // Play a beep for a dot
                    let source = SineWave::new(700.0);
                    // Time to play a dot: 1 unit (often described as 1 "dit")
                    sink.append(source.take_duration(std::time::Duration::from_millis(50)));
                    sink.sleep_until_end();
                } else if code == '-' {
                    // Play a beep for a dash
                    let source = SineWave::new(700.0);
                    // Time to play a dash: 3 units (often described as 3 "dits")
                    sink.append(source.take_duration(std::time::Duration::from_millis(150)));
                    sink.sleep_until_end();
                }
            }
            // Add a short pause between characters
            std::thread::sleep(std::time::Duration::from_millis(300));
            io::stdout().write(b"\n").unwrap();
        }
    }
}
