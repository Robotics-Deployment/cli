from dulwich import porcelain

from ros2pkg.verb.create import CreateVerb
import os


def init(args):
    clone_template(args)
    create_ros_package(args)


def clone_template(args):
    print(f"Cloning template from {args.repo_url} on branch {args.branch}")
    destination = './docker'
    try:
        porcelain.clone(args.repo_url, destination, branch=args.branch)
    except FileExistsError:
        print(f'Git repository already exists at {destination}, skipping initialization')


def create_ros_package(args):
    print(f"Creating ROS package {args.package_name}")

    if not os.path.exists('./src'):
        os.makedirs('./src')

    original_dir = os.getcwd()

    try:
        # Change the working directory
        os.chdir('./src')

        create_verb = CreateVerb()
        create_verb.main(args=args)

    finally:
        # Change back to the original working directory
        os.chdir(original_dir)
