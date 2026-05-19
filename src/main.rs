use sysinfo::{System, Components};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    if std::env::consts::OS == "windows" {
        println!("Not Compatible... sorry");
        return;
    }

    let mut _sys = System::new_all();
    let mut components = Components::new_with_refreshed_list();

    loop {
        components.refresh();
        for component in &components {
            println!("{} = temperature - {:.1} C", component.label(), component.temperature());
        }
        sleep(Duration::from_secs(5));
    }
}
