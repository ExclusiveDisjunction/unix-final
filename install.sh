#!/bin/sh
cd "$(dirname "$0")" || exit 1

EXTERNAL_IP=$(curl -s http://metadata.google.internal/computeMetadata/v1/instance/network-interfaces/0/access-configs/0/external-ip -H "Metadata-Flavor: Google")

echo "EXTERNAL_BACKEND_IP=$EXTERNAL_IP" > .env

#this checks to see if the npm dependency is installed, installs it if it isnt
if ! command -v "npm" 2>&1 >/dev/null; then
  npm install -y
fi

#allows for the deploy-swarm executable to be ran
chmod u+x deploy-swarm.sh 
#used for adding to the cron job
SCRIPT="./deploy-swarm.sh"
USER_HOME=$(eval echo "~$USER")

#this allows for the deploy-swarm to do the git pulls of the repo without user intervention
sudo git config --system --add safe.directory "$USER_HOME/unix-final"

#installs the dependencies from npm
npm install react-router-dom
npm install lucide-react

#checks to see if docker is installed and installs it if it isn't
if ! command -v "docker" 2>&1 >/dev/null; then
  apt install docker -y
fi

#checks to see if docker-compose is installed and installs it if it isn't
if ! command -v "docker-compose" 2>&1 >/dev/null; then
  apt install docker-compose -y
fi

#will check if ufw is preinstalled and install it otherwise
#also adds firewall rules to the system for the program to function and allows OpenSSH
if ! command -v "ufw" 2>&1 >/dev/null; then
  apt install ufw -y
  ufw allow OpenSSH
  ufw allow http
  ufw allow 8080
  echo "y" | ufw enable
  ufw reload
fi

#runs deploy swarm to build the images and deploy the swarm for the project
./deploy-swarm.sh

#asks for user input if they want deploy-swarm added as a cron job
#this will pull changes from the repo every 10 minutes and update the images
echo "Do you want deploy-swarm added through cron {every 10 minutes} (y/n)"
read -r ADD_CRON
if [ "$ADD_CRON" = "y" ]; then
  echo "Adding cron job as root..."
  SCRIPT_PATH="$(realpath "$SCRIPT")"
  LOG_PATH="$(dirname "$SCRIPT_PATH")/deploy-swarm.log"
  (sudo crontab -l 2>/dev/null | grep -v "$SCRIPT_PATH"; echo "*/10 * * * * $SCRIPT_PATH >> $LOG_PATH 2>&1") | sudo crontab -
  echo "Cron job added to root: runs every 10 minutes"
else
  echo "Skipping cron setup."
fi

