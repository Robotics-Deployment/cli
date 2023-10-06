pub mod clone {
    use crate::utils::utils;
    use git2::build::RepoBuilder;
    use std::io::{Error, ErrorKind, Result};
    use std::{env, io::Write, path::Path};
    use tempfile::TempDir;

    pub fn fetch(url: &str, destination: &Path, branch: &str, overwrite: bool) -> Result<()> {
        if overwrite && Path::new(destination).exists() {
            match std::fs::remove_dir_all(destination) {
                Ok(_) => println!("Removed old package directory, if it existed."),
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
                println!(
                    "Successfully cloned {} into {} from branch {}",
                    url,
                    destination.display(),
                    branch
                );
                Ok(())
            }
            Err(e) => Err(Error::new(
                ErrorKind::BrokenPipe,
                format!("Failed to clone: {}", e),
            )),
        }
    }

    pub fn clone(url: &str, destination: &Path, branch: &str, overwrite: bool) -> Result<()> {
        // temp directory
        let original_dir = env::current_dir()?;
        let temp_dir = TempDir::new().unwrap().into_path();
        env::set_current_dir(&temp_dir)?;

        // download
        let folder = None;
        fetch(url, &temp_dir, &branch, overwrite)?;
        utils::copy_package_sub_folder(&temp_dir, &destination, folder)?;

        // cleanup
        env::set_current_dir(&original_dir)?;

        Ok(())
    }
}
