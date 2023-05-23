# Robotics Deployment CLI interface

[![PyPI - Version](https://img.shields.io/pypi/v/cli.svg)](https://pypi.org/project/cli)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/cli.svg)](https://pypi.org/project/cli)

-----

**Table of Contents**

- [Installation](#installation)
- [License](#license)

## Installation

```console
pip install rdcli
```

## Dependencies
ROS 2 Humble installed and sourced

## Usage
    
```bash
rdcli --help
```

```bash
rdcli init --node-name node package
```
arguments passed to ```ros2 pkg create``` are valid in rdcli init

## License

`cli` is distributed under the terms of the [MIT](https://spdx.org/licenses/MIT.html) license.
