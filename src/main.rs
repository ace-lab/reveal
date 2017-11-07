use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::io;
use std::io::prelude::*;
extern crate time;

fn log_action(log_text: String) {
    // append log_text to the log file
    let mut file =
        OpenOptions::new()
        .write(true)
        .append(true)
        .open("transactions.log")
        .unwrap();
    if let Err(e) = file.write_all(log_text.as_bytes()) {
        println!("{}", e);
    }
}

fn main() {
    // make sure you use the correct number of arguments
    let args: Vec<_> = env::args().collect();
    if args.len() != 2{
        println!("Reveal - a program for controlling computerized testing for CS169");
        println!("Andrew Halle, 2017");
        println!("Usage:");
        println!("    initialize - creates a hint record");
        println!("    submit     - zips student code and hint record, makes http request to upload server");
        println!("    <#>        - reveals the hint for problem <#>")
    } else {
        if args[1] == "submit" {
            // TODO submission should generate an http request
            println!("You requested a submit action")
        } else if args[1] == "initialize" {
            // don't accidentally initialize
            println!("You requested an initialization, are you sure you want to do that?");
            println!("This must only be done once, at the beginning of the test.");
            println!("Continue? (y/n) ");
            let mut control = String::new();
            io::stdin().read_line(&mut control)
                .expect("Failed to read line");
            if control.trim() != "y" {
                println!("Aborting...");
                return;
            }

            // log the action
            let log_text = format!("[initialize {}] reset the hint file\n", time::now().rfc822());
            log_action(log_text);

            // create the hint record
            let mut file =
                OpenOptions::new()
                .create(true)
                .write(true)
                .mode(0o600)
                .open("hint.record")
                .unwrap();
            if let Err(e) = file.write_all(b"0000") { // TODO detect how many hints there are,
                println!("{}", e);                    //      initialize file dynamically
            }
        } else {
            // don't reveal hint on accident
            println!("You requested to reveal a hint, are you sure you want to do that?");
            println!("You will lose all credit for this question.");
            println!("Continue? (y/n) ");
            let mut control = String::new();
            io::stdin().read_line(&mut control)
                .expect("Failed to read line");
            if control.trim() != "y" {
                println!("Aborting...");
                return;
            }

            let problems = ["1a", "1b", "1c", "1d"];
            match problems.iter().position(|&s| s == args[1]) {
                Some(index) => {
                    // log the action
                    let log_text = format!("[reveal {}, {}] reveal the hint for problem {}\n",
                                           args[1], time::now().rfc822(), args[1]);
                    log_action(log_text);

                    // update the hint record
                    let mut file = match File::open("hint.record") {
                        Ok(f)  => f,
                        Err(_) => panic!()
                    };
                    let mut contents = String::new();
                    if let Err(e) = file.read_to_string(&mut contents) {
                        println!("{}", e);
                    }
                    let new_contents = format!("{}1{}", &contents[0..index], &contents[index+1..]);
                    let mut file = match File::create("hint.record") {
                        Ok(f)  => f,
                        Err(_) => panic!()
                    };
                    if let Err(e) = file.write_all(new_contents.as_bytes()) {
                        println!("{}", e);
                    }

                    // output the hint
                    let filename = format!("hints/{}.hint", args[1]);
                    let mut file = match File::open(filename) {
                        Ok(f)  => f,
                        Err(_) => panic!()
                    };
                    let mut contents = String::new();
                    if let Err(e) = file.read_to_string(&mut contents) {
                        println!("{}", e);
                    }
                    println!("{}", contents);
                },
                None => {
                    println!("There's no hint for that problem.");
                    println!("Aborting...")
                }
            }
        }
    }
}
