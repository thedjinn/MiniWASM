#!/bin/bash

# This script builds the development docker image and runs it with port
# 8000 exposed. It also mounts the current working directory in to /app
# inside the container so you can use an editor that is running outside
# of the container.

set -e

docker build -t miniwasm -f Dockerfile .

docker run -ti --mount type=bind,source="$PWD",target=/app -p 8000:8000 miniwasm
