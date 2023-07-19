mod clone;
mod inject;
mod tests;

use clap::{builder::PossibleValue, Arg, ArgAction, Command};
use std::env;

fn main() {
    let default_url: &str = "https://github.com/Robotics-Deployment/template.git";
    let default_path: std::path::PathBuf =
        env::current_dir().expect("Failed to get current directory");

    let m = Command::new("Robotics Deployment CLI")
        .author("Deniz Hofmeister, deniz@roboticsdeployment.com")
        .version("0.1.0")
        .about("A CLI for ROS2-containerized compiling, testing and deployment")
        .after_help("build, run and test locally or on a remote machine")
        .subcommands([
            // CLONE
            Command::new("clone")
                .about("Git clone the dockerized ROS2 [Python/C++] template package")
                .arg(
                    Arg::new("overwrite")
                        .long("overwrite")
                        .help("overwrite existing files")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("language")
                        .long("language")
                        .short('l')
                        .value_parser([
                            PossibleValue::new("python").help("Python Package Template"),
                            PossibleValue::new("cpp").help("C++ Package Template"),
                        ])
                        .default_value("cpp"),
                )
                .arg(Arg::new("url").long("url").default_value(default_url)),
            // INJECT
            Command::new("inject").arg(Arg::new("url").long("url").default_value(default_url)),
            // BUILD
            Command::new("build")
                .about("Compile the the package in the docker containers")
                .arg(
                    Arg::new("remote")
                        .long("remote")
                        .help("Build on a remote machine")
                        .action(ArgAction::SetTrue),
                ),
            // RUN
            Command::new("run").about("Run the package"),
            // TEST
            Command::new("test").about("Colcon test"),
            // SCAN
            Command::new("scan").about("Vulnerabilities scan"),
        ])
        .get_matches();

    match m.subcommand() {
        // CLONE
        Some(("clone", c)) => {
            let url = c.get_one::<String>("url").unwrap();
            let branch = c.get_one::<String>("language").unwrap();
            let path = default_path.join("package");
            println!("Cloning...");
            if c.get_flag("overwrite") {
                println!("Overwriting...");
            }

            match clone::clone::clone(url, &path, &branch, c.get_flag("overwrite")) {
                Ok(_) => println!("Clone successful"),
                Err(e) => eprintln!("Clone error: {:?}", e),
            }
        }

        // INJECT
        Some(("inject", c)) => {
            println!("Injecting...");
            let url = c.get_one::<String>("url").unwrap();

            match inject::inject::inject(&url) {
                Ok(_) => println!("Injection successful"),
                Err(e) => eprintln!("Injection error: {:?}", e),
            }
        }

        // TODO: Implement
        Some(("build", c)) => {
            if c.get_flag("remote") {
                println!("Building on remote machine...");
            } else {
                println!("Building locally...");
            }
        }
        Some(("run", _)) => {
            println!("Running...");
        }
        Some(("test", _)) => {
            println!("Testing...");
        }
        _ => {},
    }
}
