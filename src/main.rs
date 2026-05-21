use std::thread::sleep;
use std::time::Duration;

use simonlib::hwmon::HardwareMonitor;

fn main() {
    let monitor = HardwareMonitor::new();

    loop {
        for sensor in monitor.temperatures() {
            println!("System Status:");
            println!("{}: {:.1}°C", sensor.name, sensor.value);
        }
        sleep(Duration::from_secs(2));
    }
}