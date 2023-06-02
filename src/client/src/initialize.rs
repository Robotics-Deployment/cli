pub mod init {
    use git2::Repository;
    use std::env;
    use std::process::Command;
    use std::io::Write;

    pub fn init() {
        match Command::new("docker").arg("-v").output() {
            Ok(output) => {
                let version = String::from_utf8_lossy(&output.stdout);
                println!("Docker installed: {}", version);
            }
            Err(_) => {
                println!("Docker not found.");
            }
        }
    
        let url = "https://github.com/Robotics-Deployment/template.git";
        let path = env::current_dir().expect("Failed to get current directory");
        let package = path.join("package");
        match Repository::clone(url, package) {
            Ok(_) => println!("Cloned template repository to {:?}", url),
            Err(e) => {let _ = writeln!(std::io::stderr(), "Failed to clone: {}", e);}
        }
    }

}
