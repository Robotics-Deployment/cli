#!/usr/bin/env bash

# Get current user and group
user=$(id -u -n)
group=$(id -g -n)

# Change owner of all subdirectories and files in the current directory
sudo chown -R $user:$group .
