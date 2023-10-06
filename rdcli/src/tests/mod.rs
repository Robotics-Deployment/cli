#[cfg(test)]
mod tests {
    use crate::clone::clone::clone;
    // use crate::build::build::build;
    // use crate::inject::inject::inject;
    // use crate::run::run::run;
    // use crate::test::test::test;
    use tempfile::TempDir;

    #[test]
    fn test_clone_package() {
        let original_dir = std::env::current_dir().unwrap();
        let url: &str = "https://github.com/Robotics-Deployment/template.git";
        let dir = TempDir::new().unwrap().into_path();
        std::env::set_current_dir(&dir).unwrap();
        let result = clone(url, &dir, "cpp", true);
        std::env::set_current_dir(&original_dir).unwrap();
        assert!(result.is_ok());
    }
}
