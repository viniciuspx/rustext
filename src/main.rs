use console::Term;
use std::{self};

mod file_handler;
mod text_handler;
use crate::text_handler::text_handler::*;
use crate::file_handler::file_handler::*;

fn main() {
    clean_screen();
    let stdout = Term::buffered_stdout();
    'main_loop: loop {
        if let Ok(key) = stdout.read_key() {
            match key {
                console::Key::Char('i') => input_loop(),
                console::Key::Escape => break 'main_loop,
                _ => print_help(),
            }
        }
    }
}

fn input_loop() {
    let mut input = String::new();
    let mut buffer = String::new();
    let mut stop_loop = false;
    let expected_exit = String::from("exit()");
    clean_screen();
    println!("---------- Insert Mode ----------");
    while !stop_loop {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");
        if input.as_str().trim() == expected_exit.as_str() {
            stop_loop = true;
        } else {
            buffer.push_str(&input);
        }
    }
    save_file(buffer)
}

