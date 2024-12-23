#![allow(unused)]
use std::env;
use std::fs::{self, write, File};
use std::io::BufReader;
use std::io::{self, Read, Write};
use std::process::exit;

fn help() {
    println!("Usage: mp3tobin [OPTION]... [FILE]...");
    println!("Converts audio files to binary and vice versa\n");
    println!("Options.");
    println!("  -b{:<10}Converts audio file to binary", "");
    println!("  {:<12}eg. mp3tobin -b file.mp3\n", "");
    println!("  -a{:<10}Converts binary to audio file", "");
    println!("  {:<12}eg. mp3tobin -a file.txt\n", "");
    println!("  -h{:<10}Shows this help message", "");
    println!("  {:<12}eg. mp3tobin -h\n", "");
    exit(1)
}

fn check_audio_file(file_name: &String) {
    let file_name: Option<String> = Some(file_name.to_string());
    let is_audio_file: bool = file_name.map_or(false, |f| f.ends_with(".mp3"));
    if !is_audio_file {
        println!("Error: [Not an Audio File]");
        help();
    }
}

fn convert_audio_to_binary(file_name: &String) {
    let mut audio_file = File::open(file_name).unwrap();
    let mut buffer = Vec::new();
    audio_file.read_to_end(&mut buffer).unwrap();
    let binary_file_name = file_name.replace(".mp3", ".txt");
    let binary_string: String = buffer.iter().map(|byte| format!("{:08b}", byte)).collect();

    println!("{}", binary_string);
    write(binary_file_name, binary_string).unwrap();
    println!("Binary file has been created");
}

fn check_binary_file(file_name: &String) {
    let file_name: Option<String> = Some(file_name.to_string());
    let is_binary_file: bool = file_name.map_or(false, |f| f.ends_with(".txt"));
    if !is_binary_file {
        println!("Error: [Not an Binary File]");
        help();
    }
}

fn convert_binary_to_audio(file_name: &String) {
    let binary_string = fs::read_to_string(file_name).unwrap();
    let mut bytes = Vec::new();

    for chunk in binary_string.as_bytes().chunks(8) {
        let byte_string = std::str::from_utf8(chunk).expect("Invalid UTF-8");
        let byte = u8::from_str_radix(byte_string, 2).expect("Invalid binary string");
        bytes.push(byte);
    }

    let output_file_name = file_name.replace(".txt", ".mp3");
    let mut output_file = File::create(output_file_name).unwrap();
    output_file.write_all(&bytes).unwrap();

    println!("Binary to Mp3 converted");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        help();
    }
    match args[1].as_str() {
        "-b" => {
            check_audio_file(&args[2]);
            convert_audio_to_binary(&args[2])
        }
        "-a" => {
            check_binary_file(&args[2]);
            convert_binary_to_audio(&args[2])
        }
        _ => {
            println!("Error: Invalid Option");
            help()
        }
    };
}
