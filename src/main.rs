
use std::thread;
use std::fs;
use std::fs::{File, OpenOptions};
use std::time::Duration;
use std::io::{ Read, Write, self };
use std::io::stdout;
use std::io::prelude::*;
use std::sync::mpsc;
fn main() {
    match fs::metadata("username.txt") {
        Ok(_) => println!("File exists."),
        Err(_) => {
            print!("Hello user, how should I call you? ");
            io::stdout().flush();
            File::create("username.txt").unwrap();
        },
    }
    let mut file = File::open("username.txt").unwrap();
    let mut username = String::new();
    file.read_to_string(&mut username);
    
    println!("Your name is '{}' is that ok? yes-no", username.trim());
    let file = File::create("username.txt");
    let mut r = String::new();
    io::stdin()
        .read_line(&mut r)
        .expect("Failed to read line.");
    if r.trim().to_lowercase() == "no" {
        println!("lol, let's change it then.");
        thread::sleep(Duration::from_secs(1));
        ask_name();
    }


    let builder = thread::Builder::new().name(String::from("Robert")); 
    
    println!("You have 30 seconds to find the password.");
    thread::sleep(Duration::from_secs_f32(1.2));
    println!("Ready?");
    thread::sleep(Duration::from_secs_f32(1.2));
    println!("Go!");
    thread::sleep(Duration::from_secs_f32(1.1));
    let robert_thread = builder.spawn(move || {
        let mut n = 30;
        while n > 0 {
            n -= 1;
            println!("Currently at: {}", n);
            thread::sleep(Duration::from_secs_f32(1.0));
        }
    });
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read input.");

    println!("Incorrect!");
}
fn ask_name() {
    let mut name = String::new();
    print!("How do I call you? ");
    stdout().flush();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name.");
    let mut file = OpenOptions::new()
        .append(true)
        .open("username.txt").unwrap();
    file.write_all(name.as_bytes()).unwrap();
    file.flush();
    

}

