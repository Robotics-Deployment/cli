import argparse

try:
    from build import build
    from init import init
except ImportError:
    from rdcli.build import build
    from rdcli.init import init

from ros2pkg.verb.create import CreateVerb as CreatePackageVerb


def main():
    commands = {
        'init': init,
        'build': build,
    }

    parser = argparse.ArgumentParser(prog='rdcli')
    subparsers = parser.add_subparsers(dest='command')

    # Create subparser for 'initialize' command
    initialize_parser = subparsers.add_parser('init')
    initialize_parser.add_argument('-url', '--repo_url', help='git repo for template',
                                   default='https://github.com/Robotics-Deployment/template.git')
    initialize_parser.add_argument('-b', '--branch', help='branch other than main', default='main')
    CreatePackageVerb().add_arguments(initialize_parser, 'init')

    # Create subparser for 'build' command
    build_parser = subparsers.add_parser('build')
    build_parser.add_argument('-pt', '--pytest-args', nargs='+', help='Arguments for pytest, given to colcon test')
    build_parser.add_argument('-ct', '--ctest-args', nargs='+', help='Arguments for ctest, given to colcon test')

    args = parser.parse_args()

    # Get the appropriate command and run it
    command = commands[args.command]
    command(args)


if __name__ == '__main__':
    main()
