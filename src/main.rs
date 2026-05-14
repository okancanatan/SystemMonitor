use sysinfo::Components;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    if std::env::consts::OS == "windows" {
        println!("Not Compatible... sorry");
        return;
    }

    loop {
        let components = Components::new_with_refreshed_list();
        for component in components.iter() {
            println!("{} = temperature - {:.1} C", component.label(), component.temperature());
        }
        sleep(Duration::from_secs(1));
    }
}
