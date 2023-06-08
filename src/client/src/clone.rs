pub mod clone {
    use clap::Parser;
    use git2::Repository;
    use std::env;
    use std::io::Write;

    #[derive(Parser, Debug)]
    pub struct CloneArgs {
        pub overwrite: bool,
    }

    pub fn clone() {
        let url = "https://github.com/Robotics-Deployment/template.git";
        let path = env::current_dir().expect("Failed to get current directory");
        let package = path.join("package");

        // if args.overwrite {
        //     match std::fs::remove_dir_all(package.clone()) {
        //         Ok(_) => println!("Removed old package directory"),
        //         Err(e) => {
        //             let _ = writeln!(
        //                 std::io::stderr(),
        //                 "Failed to remove old package directory: {}",
        //                 e
        //             );
        //         }
        //     }
        // }

        match Repository::clone(url, package) {
            Ok(_) => println!("Cloned template repository to {:?}", url),
            Err(e) => {
                let _ = writeln!(std::io::stderr(), "Failed to clone: {}", e);
            }
        }
    }
}
