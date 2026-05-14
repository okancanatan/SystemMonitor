use sysinfo::Components;
use std::thread::sleep;
use std::time::Duration;
use std::io::{self, Write};

fn read_seconds() -> u64 {
    loop {
        print!("Enter update interval in seconds (S): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input. Try again.");
            continue;
        }
        match input.trim().parse::<u64>() {
            Ok(0) => {
                println!("Please enter a positive integer greater than 0.");
                continue;
            }
            Ok(n) => return n,
            Err(_) => {
                println!("Invalid integer, please try again.");
                continue;
            }
        }
    }
}

fn main() {
    if std::env::consts::OS == "windows" {
        println!("Not Compatible... sorry");
        return;
    }

    let secs = read_seconds();

    loop {
        let components = Components::new_with_refreshed_list();
        for component in components.iter() {
            println!("{} = temperature - {:.1} C", component.label(), component.temperature());
        }
        sleep(Duration::from_secs(secs));
    }
}
