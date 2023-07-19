pub mod clone {
    use git2::build::RepoBuilder;
    use std::{io::Write, path::Path};
    use std::io::{Error, ErrorKind, Result};


    pub fn clone(url: &str, destination: &Path, branch: &str, overwrite: bool) -> Result<()> {
        if overwrite && Path::new(destination).exists() {
            match std::fs::remove_dir_all(destination.clone()) {
                Ok(_) => println!("Removed old package directory"),
                Err(e) => {
                    let _ = writeln!(
                        std::io::stderr(),
                        "Failed to remove old package directory: {}",
                        e
                    );
                }
            }
        }
        // Initialize a new RepoBuilder and configure it
        let mut builder = RepoBuilder::new();
        builder.branch(branch);

        // Attempt to clone the repository
        match builder.clone(url, destination) {
            Ok(_) => {
                println!("Successfully cloned {} into {}", url, destination.display());
                Ok(())
            },
            Err(e) => {
                eprintln!("Failed to clone: {}", e);
                Err(Error::new(
                    ErrorKind::BrokenPipe,
                    format!("Failed to clone: {}", e),
                ))
            },
        }
    }
}
