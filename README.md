# Errox
[![docs](https://docs.rs/errox/badge.svg)](https://docs.rs/rusterm)
[![dependency status](https://deps.rs/crate/errox/0.1.0/status.svg)](https://deps.rs/crate/errox/0.1.0)
[![build status](https://github.com/tduck973564/errox/workflows/Rust/badge.svg)](https://github.com/tduck973564/rusterm/actions)
## A simple and minimal error logging library.
Errox is a minimal error logging library to log Err return types and print them to stderr, with an optional timestamp.
## Examples
```rust
use errox::*; // This will use the default configuration.

fn return_err() -> Result<&'static str, &'static str> {
    Err("Error here")
}
fn return_ok() -> Result<&'static str, &'static str> {
    Ok("Nothing wrong!")
}
fn main() {
    return_err().error(); // Will print a message that looks like '[timestamp] error: Error here'
    return_err().warning(); // Will print a message that looks like '[timestamp] warning: Error here'
    return_ok().error(); // Won't output anything to stderr. 
}
```
## Log leveling
Log leveling works by not showing any errors below the log level you have set. Log levels (in ascending order for priority) are as follows:
`Error`
`Warning`
`Info`
`Debug`
`Trace`
If you choose the warning log level, warnings and errors will be printed, while anything below warning will not be printed.
## Writing a config file
To write a config file, you must make a file in the working directory of the binary with the name `errox_config.toml`.
Options for the file are log_level (variant of log_level, shown above) and time (boolean), to toggle the timestamp.
### Example
```toml
time = true
log_level = "Error"
```
### Default configuration
The default configuration is loaded when no configuration file has been created.
```toml
time = true
log_level = "Trace"
```
