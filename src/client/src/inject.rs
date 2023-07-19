pub mod inject {
    use crate::clone::clone;
    use std::env;
    use std::fs::File;
    use std::io::{BufReader, Error, ErrorKind, Result};
    use std::path::{Path, PathBuf};
    use walkdir::WalkDir;
    use xml::reader::{EventReader, XmlEvent};

    pub fn inject(url: &str) -> Result<()> {
        // get current directory
        let path = env::current_dir().expect("Failed to get current directory");

        // validate
        validate(&path)?;

        // prepare
        let file_path = construct_file_path(&path, "package.xml");
        let build_type = get_build_type(&file_path)?;
        let branch = get_branch(&build_type)?;

        // execute
        let temp_dir = env::temp_dir();
        let temp_dir = temp_dir.join("robotics_deployment");
        std::fs::create_dir_all(&temp_dir)?;

        download(url, &branch, &temp_dir)?;
        copy_docker(&temp_dir, &path)?;
        std::fs::remove_dir_all(temp_dir)?;

        Ok(())
    }

    fn validate(path: &PathBuf) -> Result<()> {
        if !path.is_dir() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "The provided path is not a directory.",
            ));
        }

        let file_path = construct_file_path(path, "package.xml");
        if !file_path.exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                "package.xml does not exist!",
            ));
        }

        let docker_path = construct_file_path(path, "docker");
        if docker_path.is_dir() {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "Docker directory already exist!",
            ));
        }

        let dockerfile_path = construct_file_path(&docker_path, "Dockerfile");
        if dockerfile_path.exists() {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "Dockerfile already exist!",
            ));
        }

        let docker_compose_path = construct_file_path(&docker_path, "docker-compose.yaml");
        if docker_compose_path.exists() {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "docker-compose.yaml already exist!",
            ));
        }

        Ok(())
    }

    fn construct_file_path(path: &Path, file: &str) -> PathBuf {
        let mut file_path = PathBuf::from(path);
        file_path.push(file);
        return file_path;
    }

    fn get_build_type(file_path: &PathBuf) -> Result<String> {
        let file = File::open(&file_path)?;
        let file = BufReader::new(file);

        let parser = EventReader::new(file);
        let mut depth = 0;
        let mut build_type = String::new();

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    if name.local_name == "build_type" {
                        depth += 1;
                    }
                }
                Ok(XmlEvent::Characters(s)) if depth > 0 => {
                    build_type = s;
                }
                Ok(XmlEvent::EndElement { name }) => {
                    if name.local_name == "build_type" {
                        depth -= 1;
                        if build_type == "ament_cmake" || build_type == "ament_python" {
                            return Ok(build_type);
                        } else {
                            build_type.clear();
                        }
                    }
                }
                Err(e) => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("Error while parsing XML: {}", e),
                    ));
                }
                _ => {}
            }
        }
        Err(Error::new(ErrorKind::InvalidData, "No build type found"))
    }

    fn get_branch(build_type: &str) -> Result<String> {
        let branch = match build_type {
            "ament_cmake" => "cpp",
            "ament_python" => "python",
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Invalid build type found",
                ))
            }
        };
        Ok(branch.to_string())
    }

    fn download(url: &str, branch: &str, temp_dir: &PathBuf) -> Result<()> {
        clone::clone(url, &temp_dir, branch, true)?;
        Ok(())
    }

    fn copy_docker(source: &Path, destination: &Path) -> Result<()> {
        let package_folder = find_package_folder(&source)?;
        let source_path = construct_file_path(&package_folder, "docker");
        let destination_path = construct_file_path(destination, "docker");

        std::fs::copy(source_path, destination_path)?;

        Ok(())
    }

    fn find_package_folder(root: &Path) -> Result<PathBuf> {
        for entry in WalkDir::new(root) {
            match entry {
                Ok(entry) => {
                    if entry.file_type().is_dir() {
                        let package_path = entry.path().join("package.xml");
                        if package_path.exists() {
                            return Ok(entry.into_path().to_path_buf());
                        }
                    }
                }
                _ => {}
            }
        }
        Err(Error::new(
            ErrorKind::NotFound,
            "No package.xml found in the provided path",
        ))
    }
}
