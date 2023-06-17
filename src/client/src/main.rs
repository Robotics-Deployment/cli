mod clone;
mod tests;

use clap::{builder::PossibleValue, Arg, ArgAction, Command};
use std::env;

fn main() {
    let url = "https://github.com/Robotics-Deployment/template.git";

    let m = Command::new("Robotics Deployment CLI")
        .author("Deniz Hofmeister, deniz@roboticsdeployment.com")
        .version("0.1.0")
        .about("A CLI for ROS2-containerized building, testing and deployment")
        .after_help("build, run and test locally or on a remote machine")
        .subcommands([
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
                        .default_value("cpp"))
                .arg(
                    Arg::new("url")
                        .long("url")
                        .default_value(url)),
            Command::new("build")
            .about("Compile the the package in the docker containers")
            .arg(
                Arg::new("remote")
                    .long("remote")
                    .help("Build on a remote machine")
                    .action(ArgAction::SetTrue),
            ),
            Command::new("run")
            .about("Run the package"),
            Command::new("test")
                .about("Colcon test"),
                Command::new("scan")
                    .about("Vulnerabilities scan")
                ])
        .get_matches();

    match m.subcommand() {
        Some(("clone", c)) => {
                let branch = c.get_one::<String>("language").unwrap();
                println!("Cloning {} package...", branch);
                if c.get_flag("overwrite") {
                    println!("Overwriting...");
                }
                
                let path = env::current_dir().expect("Failed to get current directory");
                let package = path.join("package");
                let _ = clone::clone::clone(url, &package, &branch, c.get_flag("overwrite"));
        }

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
        _ => unreachable!(),
    }
}
