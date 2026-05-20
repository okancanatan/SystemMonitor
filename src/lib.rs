pub mod hwmon {
    use std::fs;

    #[derive(Clone, Debug)]
    pub struct TemperatureSensor {
        pub name: String,
        pub value: f32,
    }

    pub struct HardwareMonitor {
        sensors: Vec<TemperatureSensor>,
    }

    impl HardwareMonitor {
        pub fn new() -> Self {
            let sensors = Self::read_sensors();
            HardwareMonitor { sensors }
        }

        pub fn temperatures(&self) -> Vec<TemperatureSensor> {
            self.sensors.clone()
        }

        fn read_sensors() -> Vec<TemperatureSensor> {
            let mut sensors = Vec::new();

            // Try to read from Linux hwmon interface
            #[cfg(target_os = "linux")]
            {
                if let Ok(entries) = fs::read_dir("/sys/class/hwmon") {
                    for entry in entries.flatten() {
                        if let Ok(metadata) = entry.metadata() {
                            if metadata.is_dir() {
                                Self::read_hwmon_dir(entry.path(), &mut sensors);
                            }
                        }
                    }
                }
            }

            // Fallback for non-Linux systems or if hwmon not available
            if sensors.is_empty() {
                sensors.push(TemperatureSensor {
                    name: "CPU".to_string(),
                    value: 0.0,
                });
            }

            sensors
        }

        #[cfg(target_os = "linux")]
        fn read_hwmon_dir(dir: std::path::PathBuf, sensors: &mut Vec<TemperatureSensor>) {
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if name.starts_with("temp") && name.ends_with("_input") {
                            if let Ok(content) = fs::read_to_string(&path) {
                                if let Ok(millidegrees) = content.trim().parse::<i32>() {
                                    let celsius = millidegrees as f32 / 1000.0;
                                    let sensor_name = name.replace("_input", "");
                                    sensors.push(TemperatureSensor {
                                        name: sensor_name,
                                        value: celsius,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    impl Default for HardwareMonitor {
        fn default() -> Self {
            Self::new()
        }
    }
}
