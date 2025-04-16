#!/bin/bash

cd /home/connorkuziemko2021/unix-final || exit

git pull https://github.com/ExclusiveDisjunction/unix-final.git docker

DOCKER_BUILDKIT=1 docker build -t front ./frontend
DOCKER_BUILDKIT=1 docker build -t back ./backend

docker swarm init 2>/dev/null

DOCKER_BUILDKIT=1 docker stack deploy -c compose.yaml test-stack
