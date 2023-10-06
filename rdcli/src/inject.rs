pub mod inject {
    use crate::clone::clone;
    use crate::utils::utils;
    use std::env;
    use std::io::{Error, ErrorKind, Result};
    use std::path::PathBuf;
    use tempfile::TempDir;

    pub fn inject(url: &str) -> Result<()> {
        // get current directory
        let path = env::current_dir().expect("Failed to get current directory");

        // validate
        docker_not_present(&path)?;

        // prepare
        let branch = utils::get_package_language(&path)?;

        // temp directory
        let original_dir = env::current_dir()?;
        let temp_dir = TempDir::new().unwrap().into_path();
        env::set_current_dir(&temp_dir)?;

        // download
        let folder = Option::from("docker");
        clone::fetch(url, &temp_dir, &branch, true)?;
        utils::copy_package_sub_folder(&temp_dir, &path, folder)?;

        // cleanup
        env::set_current_dir(&original_dir)?;
        Ok(())
    }

    fn docker_not_present(path: &PathBuf) -> Result<()> {
        let docker_path = utils::get_file_path(path, "docker");
        if docker_path.is_dir() {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "Docker directory already exist!",
            ));
        }

        let dockerfile_path = utils::get_file_path(&docker_path, "Dockerfile");
        if dockerfile_path.exists() {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "Dockerfile already exist!",
            ));
        }

        let docker_compose_path = utils::get_file_path(&docker_path, "docker-compose.yaml");
        if docker_compose_path.exists() {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "docker-compose.yaml already exist!",
            ));
        }

        Ok(())
    }
}
