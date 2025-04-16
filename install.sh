#!/bin/sh

if ! command -v "npm" 2>&1 >/dev/null; then
  npm install -y
fi

chmod u+x deploy-swarm.sh

sudo git config --system --add safe.directory /home/connorkuziemko2021/unix-final

npm install react-router-dom

if ! command -v "docker" 2>&1 >/dev/null; then
  apt install docker -y
fi

if ! command -v "docker-compose" 2>&1 >/dev/null; then
  apt install docker-compose -y
fi

if ! command -v "ufw" 2>&1 >/dev/null; then
  apt install ufw -y
  ufw allow OpenSSH
  ufw allow http
  ufw allow https
  ufw enable
  ufw reload
fi

./deploy-swarm.sh


