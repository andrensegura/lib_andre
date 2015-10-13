//Takes a string literal as an argument and returns true if the user exists. Else, returns false.

use std::process::Command;

pub fn is_valid_user(user: &str) -> bool {
    let command = format!("{}{}", "id -u ", user);
    let output = Command::new("sh")
                            .arg("-c")
                            .arg(command)
                            .output()
                            .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    let output_str = String::from_utf8_lossy(&output.stdout);

    if output_str.is_empty() {
        false
    } else {
        true
    }
}
