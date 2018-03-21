use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::process::Command;
use std::path::Path;

extern crate time;

fn log_action(log_text: String) {
    // append log_text to the log file
    let mut file = match
        OpenOptions::new()
        .write(true)
        .append(true)
        .open("admin/transactions.log") {
            Ok(f) => f,
            Err(_)    => {
                println!("Please run from the directory ~/Desktop/exam");
                panic!();
            }
        };
    if let Err(e) = file.write_all(log_text.as_bytes()) {
        println!("Please run from the directory ~/Desktop/exam");
        println!("{}", e);
    }
}

fn main() {
    // make sure you use the correct number of arguments
    let args: Vec<_> = env::args().collect();
    if args.len() != 2{
        println!("Reveal - a program for controlling computerized testing for CS169");
        println!("Andrew Halle, 2017");
        println!("Please run from the directory ~/Desktop/exam");
        println!("Usage:");
        println!("    initialize - prompts student for metadata name, SID");
        println!("    <#>        - reveals the hint for problem <#>");
        println!("    submit     - zips student code and hint record");
    } else {
        if args[1] == "initialize" {
            // prompt student for data
            print!("Name: ");
            let _ = io::stdout().flush();
            let mut name = String::new();
            io::stdin().read_line(&mut name)
                .expect("Failed to read name");
            print!("Student ID: ");
            let _ = io::stdout().flush();
            let mut sid = String::new();
            io::stdin().read_line(&mut sid)
                .expect("Failed to read SID");

            // log action (including name, sid)
            let log_text = format!("[reveal initialize, {}] initialized METADATA with Name: {}, SID: {}\n", time::now().rfc822(), name.trim(), sid.trim());
            log_action(log_text);
            
            // create METADATA file
            let contents = format!("Name: {}\nSID: {}\n", name.trim(), sid.trim());
            let mut file = match File::create("admin/METADATA") {
                Ok(f)  => f,
                Err(_) => {
                    println!("Please run from the directory ~/Desktop/exam");
                    panic!();
                }
            };
            if let Err(e) = file.write_all(contents.as_bytes()) {
                println!("{}", e);
            }
            
        } else if args[1] == "submit" {
            // log the action
            let log_text = format!("[reveal submit, {}] generated a submission\n", time::now().rfc822());
            log_action(log_text);

            // make sure METADATA exists
            let md = Path::new("admin/METADATA");
            if !md.exists() {
                println!("Please run `reveal initialize` before submitting.");
                println!("Aborting...");
                panic!();
            }
            // build zip archive
            let password = "[)#]7p@9T_urm9B@"; // TODO change this at compile time, don't commit to git repo
            Command::new("zip")
                .arg("-P")
                .arg(password)
                .arg("submission.zip")
                .arg("-r")
                .arg("rottenpotatoes-rails-intro") // TODO replace with relevant files
                .arg("admin")
                .output()
                .expect("failed to make first zip");

            // double zip
            Command::new("mkdir")
                .arg("submission")
                .output()
                .expect("failed to make submission directory");
            Command::new("mv")
                .arg("submission.zip")
                .arg("submission")
                .output()
                .expect("failed to move submission.zio");
            Command::new("zip")
                .arg("-P")
                .arg(password)
                .arg("submission")
                .arg("-r")
                .arg("submission")
                .output()
                .expect("failed to execute process");
            Command::new("rm")
                .arg("-rf")
                .arg("submission")
                .output()
                .expect("failed to clean up temp folder");
            
        } else {
            // don't reveal hint on accident
            println!("You requested to reveal a hint, are you sure you want to do that?");
            println!("You will lose 75% of the credit for this question.");
            println!("Continue? (y/n) ");
            let mut control = String::new();
            io::stdin().read_line(&mut control)
                .expect("Failed to read line");
            if control.trim() != "y" {
                println!("Aborting...");
                return;
            }

            // TODO replace with list of hint files
            let problems = ["1.1", "1.2", "1.3", "1.4", "2.1", "2.2", "2.3", "2.4", "3.1", "3.2", "3.3", "3.4", "4.1", "5.1", "5.2", "5.3"];
            match problems.iter().position(|&s| s == args[1]) {
                Some(index) => {
                    // log the action
                    let log_text = format!("[reveal {}, {}] reveal the hint for problem {}\n",
                                           args[1], time::now().rfc822(), args[1]);
                    log_action(log_text);

                    // update the hint record
                    let mut file = match File::open("admin/hint.record") {
                        Ok(f)  => f,
                        Err(_) => {
                            println!("Please run from the directory ~/Desktop/exam");
                            panic!();
                        }
                    };
                    let mut contents = String::new();
                    if let Err(e) = file.read_to_string(&mut contents) {
                        println!("{}", e);
                    }
                    let new_contents = format!("{}1{}", &contents[0..index], &contents[index+1..]);
                    let mut file = match File::create("admin/hint.record") {
                        Ok(f)  => f,
                        Err(_) => {
                            println!("Please run from the directory ~/Desktop/exam");
                            panic!();
                        }
                    };
                    if let Err(e) = file.write_all(new_contents.as_bytes()) {
                        println!("{}", e);
                    }

                    // output the hint
                    let filename = format!("admin/hints/{}.hint", args[1]);
                    let mut file = match File::open(filename) {
                        Ok(f)  => f,
                        Err(_) => {
                            println!("Please run from the directory ~/Desktop/exam");
                            panic!();
                        }
                    };
                    let mut contents = String::new();
                    if let Err(e) = file.read_to_string(&mut contents) {
                        println!("{}", e);
                    }
                    println!("{}", contents);
                },
                None => {
                    println!("There's no hint for that problem.");
                    println!("Acceptable inputs are '1.1'");             // TODO replace with list of acceptable hints
                    println!("Aborting...");
                }
            }
        }
    }
}
