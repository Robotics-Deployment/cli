use clap::{
    Arg, ArgAction, Command, builder::PossibleValue,
};

fn main() {
    let m = Command::new("Robotics Deployment CLI")
        .author("Deniz Hofmeister, deniz@roboticsdeployment.com")
        .version("0.1.0")
        .about("A CLI for ROS2-containerized deployments, backups and testing.")
        .after_help("Clone, build and test locally, then build, test and compose in the cloud and finally deploy to your fleet.")
        .subcommands([
            Command::new("build")
                .about("Builds the project locally")
                .arg(
                    Arg::new("remote")
                        .long("remote")
                        .help("Build on a remote machine")
                        .action(ArgAction::SetTrue),
                ),
            Command::new("clone")
                .about("Git clone the dockerized ROS2 [Python/C++] package template")
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
                ),
            Command::new("demo")
        ])
        .get_matches();
}
