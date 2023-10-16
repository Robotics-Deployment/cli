#!/bin/bash

# Login to GitHub Container Registry
docker login ghcr.io

# Build docker compose images
docker compose build

# Push image to GitHub Container Registry
docker compose push