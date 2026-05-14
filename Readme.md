# SystemMonitor

A tiny Rust system monitoring tool that prints component temperatures using the `sysinfo` crate.

## Compatibility
- Linux, macOS: supported
- Windows: Not compatible 

The program refreshes component temperatures every second and prints them to the console.

## Notes
- To reduce CPU usage, you can increase the sleep duration in `src/main.rs`.

## To Do:
Windows Compatibility.