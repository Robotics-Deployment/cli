pub mod utils {
    use fs_extra::dir::{copy, CopyOptions};
    use std::fs::File;
    use std::io::{BufReader, Error, ErrorKind, Result};
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use xml::reader::{EventReader, XmlEvent};

    pub fn validate_package_directory(path: &PathBuf) -> Result<()> {
        if !path.is_dir() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "The provided path is not a directory.",
            ));
        }

        let file_path = get_file_path(path, "package.xml");
        if !file_path.exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                "package.xml does not exist!",
            ));
        }
        return Ok(());
    }

    pub fn validate_docker_engine() -> Result<()> {
        let status = Command::new("docker").args(&["version"]).status();

        return match status {
            Ok(status) => {
                if status.success() {
                    Ok(())
                } else {
                    Err(Error::new(ErrorKind::Other, "Docker engine not found"))
                }
            }
            Err(_) => Err(Error::new(ErrorKind::Other, "Docker engine not found")),
        };
    }

    pub fn get_package_language(build_type: &PathBuf) -> Result<String> {
        let build_type = get_build_type_from_package_xml(&build_type)?;
        let branch = match build_type.as_str() {
            "ament_cmake" => "cpp",
            "ament_python" => "python",
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Invalid build type found",
                ));
            }
        };
        Ok(branch.to_string())
    }

    pub fn get_file_path(path: &Path, file: &str) -> PathBuf {
        let mut file_path = PathBuf::from(path);
        file_path.push(file);
        return file_path;
    }

    pub fn get_build_type_from_package_xml(path: &PathBuf) -> Result<String> {
        let file_path = get_file_path(path, "package.xml");
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
        return Err(Error::new(ErrorKind::InvalidData, "No build type found"));
    }

    pub fn copy_package_sub_folder(
        source: &Path,
        destination: &Path,
        folder: Option<&str>,
    ) -> Result<()> {
        let package = source.join("package");
        let mut source_path = package.clone();
        let mut destination_path = PathBuf::from(destination);
        match folder {
            Some(folder) => {
                source_path = get_file_path(&package, folder);
                destination_path = get_file_path(destination, folder);
            }
            None => {}
        }

        std::fs::create_dir_all(&destination_path)?;

        let mut options = CopyOptions::new(); // Initialize with default values
        options.overwrite = true; // Replace the content of the destination directory

        let src_path = source_path.clone();
        let dst_path = destination_path.clone();
        match copy(source_path, destination, &options) {
            Ok(_) => match folder {
                Some(folder) => {
                    println!(
                        "Package {} configurations copied successfully from {} to {}.",
                        folder,
                        src_path.display(),
                        dst_path.display()
                    )
                }
                None => {
                    println!(
                        "Package copied successfully from {} to {}.",
                        src_path.display(),
                        dst_path.display()
                    )
                }
            },
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Failed to copy directory: {}", e),
                ));
            }
        }
        Ok(())
    }
}
