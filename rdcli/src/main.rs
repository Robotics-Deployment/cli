mod build;
mod clone;
mod inject;
mod run;
mod test;
mod utils;

#[cfg(test)]
mod tests;

fn main() {
    use clap::{builder::PossibleValue, Arg, ArgAction, Command};
    use std::env;
    let default_url: &str = "https://github.com/Robotics-Deployment/template.git";
    let default_path: std::path::PathBuf =
        env::current_dir().expect("Failed to get current directory");

    let m = Command::new("Robotics Deployment CLI")
        .author("Deniz Hofmeister, deniz@roboticsdeployment.com")
        .version("0.2.0")
        .about("A CLI for ROS2-containerized compiling, testing and deployment")
        .after_help("build, run and test locally then push it to the platform")
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
                            PossibleValue::new("python")
                                .help("Python Package Template"),
                            PossibleValue::new("cpp")
                                .help("C++ Package Template"),
                        ])
                        .default_value("cpp"),
                )
                .arg(Arg::new("url")
                    .long("url")
                    .default_value(default_url))
                .after_help("This command will clone the template package \
                from the Robotics Deployment Template Repository."),
            // INJECT
            Command::new("inject")
                .about("Adds docker folder and files from the template package to your package")
                .arg(Arg::new("url")
                    .long("url")
                    .default_value(default_url))
                .after_help("This command assumes that you are in the root of your package."),
            // BUILD
            Command::new("build")
                .about("Compile the the package in the docker containers")
                .after_help("This command will build the package in the docker containers. \
                It assumes the package follow the template structure defined by Robotics Deployment Template Repository."),
            // RUN
            Command::new("run")
                .about("Run the package"),
            // TEST
            Command::new("test")
                .about("Colcon test"),
        ])
        .arg_required_else_help(true)
        .get_matches();

    // Check if we are in the root of the package directory
    if m.subcommand_matches("clone").is_none() {
        let result = utils::utils::validate_package_directory(&default_path);
        match result {
            Ok(_) => {}
            Err(e) => {
                eprintln!(
                    "Please run rdcli in the root of your package directory or clone first: {}",
                    e
                );
                std::process::exit(1);
            }
        }
    }

    // Check if docker is installed
    if m.subcommand_matches("clone").is_none() && m.subcommand_matches("inject").is_none() {
        let result = utils::utils::validate_docker_engine();
        match result {
            Ok(_) => {}
            Err(e) => {
                let command = m.subcommand_name().unwrap();
                eprintln!("Command {} requires docker: {}", command, e);
                std::process::exit(1);
            }
        }
    }

    // Match subcommands
    match m.subcommand() {
        // CLONE
        Some(("clone", c)) => {
            println!("Cloning...");
            let url = c.get_one::<String>("url").unwrap();
            let branch = c.get_one::<String>("language").unwrap();
            if c.get_flag("overwrite") {
                println!("Overwriting...");
            }

            match clone::clone::clone(url, &default_path, &branch, c.get_flag("overwrite")) {
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

        Some(("build", _)) => {
            println!("Building...");
            match build::build::build() {
                Ok(_) => println!("Build successful"),
                Err(e) => eprintln!("Build error: {:?}", e),
            }
        }

        Some(("run", _)) => {
            println!("Running...");
            match run::run::run() {
                Ok(_) => println!("Run successful"),
                Err(e) => eprintln!("Run error: {:?}", e),
            }
        }

        Some(("test", _)) => {
            println!("Testing...");
            match test::test::test() {
                Ok(_) => println!("Test successful"),
                Err(e) => eprintln!("Test error: {:?}", e),
            }
        }
        _ => {}
    }
}
