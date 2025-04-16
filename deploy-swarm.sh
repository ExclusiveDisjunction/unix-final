#!/bin/bash

cd "$(dirname "$0")" || exit 1


#if not using a github repo to manage the frontend, backend, and db comment out the next line, otherwise update with your github repo and branch
git pull https://github.com/ExclusiveDisjunction/unix-final.git main

DOCKER_BUILDKIT=1 docker build -t front ./frontend
DOCKER_BUILDKIT=1 docker build -t back ./backend



docker swarm init 2>/dev/null
DOCKER_BUILDKIT=1 docker stack deploy -c compose.yaml test-stack
