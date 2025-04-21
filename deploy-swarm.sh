#!/bin/bash

cd "$(dirname "$0")" || exit 1


#if not using a github repo to manage the frontend, backend, and db comment out the next line, otherwise update with your github repo and branch
git pull https://github.com/ExclusiveDisjunction/unix-final.git main

DOCKER_BUILDKIT=1 docker build -t frontend ./frontend
DOCKER_BUILDKIT=1 docker build -t backend ./backend
docker pull postgres
#this is used to select an ip address for the swarm to linked to and advertise from
#primarily applicable on machines that have multiple IP addresses from having several network adapters either physical or virtual
if ! docker info | grep -q "Swarm: active"; then
  echo "Swarm is not initialized. Looking for available IP addresses..."

  IP_OPTIONS=($(hostname -I))
  echo "Multiple IP addresses found. Please choose one to advertise for Docker Swarm:"
  select IP_CHOICE in "${IP_OPTIONS[@]}"; do
    if [[ -n "$IP_CHOICE" ]]; then
      echo "Initializing swarm with IP: $IP_CHOICE"
      docker swarm init --advertise-addr "$IP_CHOICE"
      break
    else
      echo "Invalid choice. Please select a number from the list."
    fi
  done
else
  echo "Swarm already initialized. Skipping init."
fi

#this is used to select an ip address for the swarm to linked to and advertise from
#primarily applicable on machines that have multiple IP addresses from having several network adapters either physical or virtual
if ! docker info | grep -q "Swarm: active"; then
  echo "Swarm is not initialized. Looking for available IP addresses..."

  IP_OPTIONS=($(hostname -I))
  echo "Multiple IP addresses found. Please choose one to advertise for Docker Swarm:"
  select IP_CHOICE in "${IP_OPTIONS[@]}"; do
    if [[ -n "$IP_CHOICE" ]]; then
      echo "Initializing swarm with IP: $IP_CHOICE"
      docker swarm init --advertise-addr "$IP_CHOICE"
      break
    else
      echo "Invalid choice. Please select a number from the list."
    fi
  done
else
  echo "Swarm already initialized. Skipping init."
fi


DOCKER_BUILDKIT=1 docker stack deploy -c compose.yaml test-stack
