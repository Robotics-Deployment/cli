pub mod verify {
    use std::process::Command;
    pub fn verify() {
        match Command::new("docker").arg("-v").output() {
            Ok(output) => {
                let version = String::from_utf8_lossy(&output.stdout);
                println!("Docker installed: {}", version);
            }
            Err(_) => {
                println!("Docker not found.");
            }
        }
        match Command::new("docker").arg("compose").arg("-v").output() {
            Ok(output) => {
                let version = String::from_utf8_lossy(&output.stdout);
                println!("Docker Compose installed: {}", version);
            }
            Err(_) => {
                println!("Docker Compose not found.");
            }
        }
    }
}
