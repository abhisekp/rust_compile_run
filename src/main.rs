extern crate clap;
extern crate notify;
extern crate notify_rust;
extern crate tempfile;

use clap::{App, Arg};
use notify::{recommended_watcher, RecursiveMode, Watcher};
use notify_rust::Notification;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use tempfile::tempdir;

fn compile_and_run(file: &str, dir: &str, notify: bool) {
    // Clear the screen
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }

    let compile_status = Command::new("rustc")
        .arg(file)
        .arg("-o")
        .arg(format!("{}/executable", dir))
        .status()
        .expect("Failed to compile");

    if compile_status.success() {
        let run_status = Command::new(format!("{}/executable", dir))
            .status()
            .expect("Failed to run");

        if run_status.success() {
            std::fs::remove_file(format!("{}/executable", dir)).expect("Failed to delete");

            if notify {
                // Only notify if the --notify flag is passed
                Notification::new()
                    .summary(&format!("Rust Script: {}", file))
                    .body(&format!("Execution successful. File: {}", file))
                    .icon("dialog-information")
                    .show()
                    .unwrap();
            }
        }
    } else {
        if notify {
            // Only notify if the --notify flag is passed
            Notification::new()
                .summary(&format!("Rust Script: {}", file))
                .body(&format!("Compilation failed. File: {}", file))
                .icon("dialog-error")
                .urgency(notify_rust::Urgency::Critical)
                .show()
                .unwrap();
        }
    }
}

fn main() {
    let matches = App::new("Rust Compile and Run")
        .version("1.0")
        .author("Abhisek Pattnaik <abhisekp@engineer.com>")
        .about("Compiles, runs, and deletes the generated Rust executable.")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("The Rust file to compile and run")
                .required(false),
        )
        .arg(
            Arg::with_name("temp")
                .short("t")
                .long("temp")
                .help("Compile and run in a temporary directory"),
        )
        .arg(
            Arg::with_name("watch")
                .short("w")
                .long("watch")
                .help("Watch for file changes and recompile"),
        )
        .arg(
            Arg::with_name("notify")
                .short("n")
                .long("notify")
                .help("Enable notifications"),
        )
        .get_matches();

    let notify = matches.is_present("notify");
    let file = matches.value_of("file");
    let use_temp_dir = matches.is_present("temp");
    let watch_mode = matches.is_present("watch");

    let dir = if use_temp_dir {
        let temp_dir = tempdir().unwrap();
        temp_dir.path().to_str().unwrap().to_string()
    } else {
        ".".to_string()
    };

    if let Some(file) = file {
        if watch_mode {
            compile_and_run(file, &dir, notify);

            let (tx, rx) = channel();
            let mut watcher = recommended_watcher(tx).unwrap();
            let path = Path::new(file);
            watcher.watch(&path, RecursiveMode::NonRecursive).unwrap();

            let mut last_compile_time = Instant::now();
            let debounce_duration = Duration::from_millis(500);

            loop {
                match rx.recv() {
                    Ok(_) => {
                        if last_compile_time.elapsed() >= debounce_duration {
                            compile_and_run(file, &dir, notify);
                            last_compile_time = Instant::now();
                        }
                    }
                    Err(_) => println!("Watch error"),
                }
            }
        } else {
            compile_and_run(file, &dir, notify);
        }
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();

        // Create a temporary directory
        let temp_dir = tempdir().unwrap();
        let temp_file_path = temp_dir.path().join("temp.rs");

        // Write the buffer to a temporary file with a valid name
        let mut temp_file = File::create(&temp_file_path).unwrap();
        write!(temp_file, "{}", buffer).unwrap();

        // Get the path of the temporary file
        let temp_path = temp_file_path.to_str().unwrap();

        // Compile and run the temporary file
        compile_and_run(temp_path, &dir, notify);
    }
}
