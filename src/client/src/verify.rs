use std::fmt;
use std::process::Command;

pub enum Component {
    Docker,
    DockerCompose,
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Component::Docker => write!(f, "Docker"),
            Component::DockerCompose => write!(f, "Docker Compose"),
        }
    }
}

pub fn verify(component: Component) -> Result<String, String> {
    let (cmd, args) = match component {
        Component::Docker => ("docker", vec!["-v"]),
        Component::DockerCompose => ("docker", vec!["compose", "-v"]),
    };

    match Command::new(cmd).args(&args).output() {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout);
            Ok(format!("{} installed: {}", component, version))
        }
        Err(_) => Err(format!("{} not found.", component)),
    }
}
