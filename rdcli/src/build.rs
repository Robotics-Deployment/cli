pub mod build {
    use std::env;
    use std::io::{Error, ErrorKind, Result};
    use std::process::Command;

    pub fn build() -> Result<()> {
        // temp directory
        let package_dir = env::current_dir().unwrap();
        let docker_dir = package_dir.join("docker");
        env::set_current_dir(&docker_dir).unwrap();

        let result = Command::new("docker").args(&["compose", "build"]).status();
        // cleanup
        env::set_current_dir(&package_dir).unwrap();

        return match result {
            Ok(status) => {
                if status.success() {
                    Ok(())
                } else {
                    Err(Error::new(ErrorKind::Other, "Failed to build docker image"))
                }
            }
            Err(_) => Err(Error::new(ErrorKind::Other, "Failed to build docker image")),
        };
    }
}
