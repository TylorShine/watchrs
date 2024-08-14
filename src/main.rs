use std::env;
use std::io::{self, Write};
use std::process::Command;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <command> <args>... <restart_delay_seconds>", args[0]);
        return;
    }

    let cmd = args[1].clone();
    let cmd_args: Vec<String> = args[2..args.len() - 1].to_vec();
    let restart_delay = args[args.len() - 1].parse::<u64>().unwrap_or(5);

    let (tx, rx): (Sender<String>, Receiver<String>) = channel();

    let watchdog_handle = thread::spawn(move || {
        watchdog(&cmd, &cmd_args, restart_delay, rx);
    });

    interpreter(tx);

    watchdog_handle.join().unwrap();
}

fn watchdog(cmd: &str, cmd_args: &Vec<String>, restart_delay: u64, rx: Receiver<String>) {
    loop {
        println!("cmd: {}, args: {}", cmd, cmd_args.join(" "));
        let mut child = Command::new(cmd)
            .args(cmd_args)
            .spawn()
            .expect("failed to execute process");

        loop {
            if let Ok(command) = rx.try_recv() {
                match command.as_str() {
                    "/r" => {
                        println!("Restarting command...");
                        child.kill().expect("Command wasn't running");
                        break;
                    }
                    "/q" => {
                        println!("Quitting...");
                        child.kill().expect("Command wasn't running");
                        return;
                    }
                    _ => {}
                }
            }

            if let Some(status) = child.try_wait().expect("failed to wait on child") {
                if !status.success() {
                    println!("Command failed. Restarting in {} seconds...", restart_delay);
                    thread::sleep(Duration::from_secs(restart_delay));
                    break;
                } else {
                    println!("Command exited successfully. Restarting in {} seconds...", restart_delay);
                    thread::sleep(Duration::from_secs(restart_delay));
                    break;
                }
            }

            thread::sleep(Duration::from_millis(100));
        }
    }
}

fn interpreter(tx: Sender<String>) {
    loop {
        print!("Enter command (/r: restart, /q: quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let command = input.trim().to_string();
        tx.send(command.clone()).unwrap();

        if command == "/q" {
            break;
        }
    }
}
