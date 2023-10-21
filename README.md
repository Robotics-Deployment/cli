# Robotics Deployment CLI

--- 

[![Build](https://github.com/Robotics-Deployment/cli/actions/workflows/build.yml/badge.svg)](https://github.com/Robotics-Deployment/cli/actions/workflows/build.yml)

The command line interface for interacting with the Robotics Deployment Platform.

---

```bash
$ rdcli -h
```

```
A CLI for ROS2-containerized compiling, testing and deployment

Usage: rdcli [COMMAND]

Commands:
  clone   Git clone the dockerized ROS2 [Python/C++] template package
  inject  Adds docker folder and files from the template package to your package
  build   Compile the the package in the docker containers
  run     Run the package
  test    Colcon test
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

build, run and test locally then push it to the platform
```
