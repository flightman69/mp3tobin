
# mp3tobin

**mp3tobin** is a simple yet powerful command-line tool written in Rust for converting audio files (MP3) into binary and vice versa. Whether you're looking to extract binary data from audio files or reconstruct audio from a binary representation, **mp3tobin** provides a straightforward and efficient solution.

## Features
- Convert an MP3 file to its binary representation.
- Convert binary data back to an MP3 file.
- Display a help message with usage instructions.

## Usage
```bash
mp3tobin [OPTION]... [FILE]...
```

### Options
- **`-b`**: Converts an audio file to binary  
  Example:  
  ```bash
  mp3tobin -b file.mp3
  ```
  
- **`-a`**: Converts a binary file back to audio  
  Example:  
  ```bash
  mp3tobin -a file.txt
  ```

- **`-h`**: Displays the help message  
  Example:  
  ```bash
  mp3tobin -h
  ```

## Installation

To install and run **mp3tobin**, you will need to have Rust and Cargo installed on your machine. Follow these steps:

1. Install Rust and Cargo:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone this repository:
   ```bash
   git clone https://github.com/flightman69/mp3tobin
   cd mp3tobin
   ```

3. Build the project using Cargo:
   ```bash
   cargo build --release
   ```

4. Once the build is complete, you can find the executable in the `target/release` directory:
   ```bash
   ./target/release/mp3tobin
   ```

## How to Run

After building the project, you can run **mp3tobin** using the following command format:

### Convert MP3 to Binary
```bash
./target/release/mp3tobin -b path_to_file.mp3
```
This will create a `.txt` file containing the binary representation of the MP3 file.

### Convert Binary to MP3
```bash
./target/release/mp3tobin -a path_to_file.txt
```
This will recreate the original MP3 file from the binary representation.

### Display Help
```bash
./target/release/mp3tobin -h
```
This will show the help message with usage information.

## License
This project is licensed under the **GNU General Public License (GPL)**. For more information, see the [LICENSE](LICENSE) file.

---

### GitHub Link
For more details, visit the [mp3tobin GitHub repository](https://github.com/flightman69/mp3tobin).

