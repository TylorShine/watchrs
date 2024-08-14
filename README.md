# watchrs

This Rust minimal tool serves as a watchdog for running and managing a specified command. It automatically restarts the command if it fails or exits, and provides a simple interface for manual control.

## Features

- Automatically restarts the specified command after a configurable delay
- Allows manual restart and quit operations through a simple command-line interface
- Runs the specified command as a child process
- Handles both successful and unsuccessful command exits

## Usage


cargo run -- <command> <args>... <restart_delay_seconds>


- `<command>`: The command to run and watch
- `<args>`: Any arguments for the command (optional)
- `<restart_delay_seconds>`: The delay in seconds before restarting the command (default: 5)

## Example


cargo run -- ping google.com 10


This will run the `ping` command with the arguments `google.com` and `10`, and restart it every 10 seconds if it exits.

## Commands

While the watchdog is running, you can use the following commands:

- `/r`: Restart the command immediately
- `/q`: Quit the watchdog and the running command

## Building

To build the project, ensure you have Rust installed and run:


cargo build --release


The compiled binary will be available in the `target/release` directory.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is under the MIT License.
