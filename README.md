# RustSecurePassGen

## Description
RustSecurePassGen is a secure password generator developed in the Rust programming language. This project is designed to create high-entropy, random passwords that can be used to enhance account security.

## Features
- **Length customization**: Users can choose the length of the password according to their needs.
- **Character set**: Ability to choose from various character sets, including:
 - Uppercase letters (A-Z)
 - Lowercase letters (a-z)
 - Numbers (0-9)
 - Special characters (!@#$%^&* and so on)
- **Exclusion of similar characters**: Option to exclude characters that can be easily confused (e.g., `0` and `O`).

## Technologies
The project uses the Rust programming language to ensure high performance and security. 

## Installation and Execution
To run the project, you need to have Rust and Cargo installed.

```bash
# Cloning the repository
git clone https://github.com/yourusername/RustSecurePassGen.git

# Navigating to the project directory
cd RustSecurePassGen

# Building and running
cargo run
