import argparse

from build import build
from init import init


def main():
    # Create a dictionary to map command names to command instances
    commands = {
        'init': init,
        'build': build,
    }

    parser = argparse.ArgumentParser(prog='cli_program')
    subparsers = parser.add_subparsers(dest='command')

    # Create subparser for 'initialize' command
    initialize_parser = subparsers.add_parser('init')
    initialize_parser.add_argument('--name', help='Name of the package')

    # Create subparser for 'build' command
    build_parser = subparsers.add_parser('build')
    build_parser.add_argument('--pytest-args', nargs='+', help='Arguments for pytest')

    args = parser.parse_args()

    # Get the appropriate command and run it
    command = commands[args.command]
    command(args)


if __name__ == '__main__':
    main()
