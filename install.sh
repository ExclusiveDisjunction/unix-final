#!/bin/sh
cd "$(dirname "$0")" || exit 1

if ! command -v "npm" 2>&1 >/dev/null; then
  npm install -y
fi

chmod u+x deploy-swarm.sh
SCRIPT="./deploy-swarm.sh"
USER_HOME=$(eval echo "~$USER")

sudo git config --system --add safe.directory "$USER_HOME/unix-final"

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
  echo "y" | ufw enable
  ufw reload
fi

./deploy-swarm.sh

echo "Do you want deploy-swarm added through cron {every 10 minutes} (y/n)"
read -r ADD_CRON
if [ "$ADD_CRON" = "y" ]; then
  echo "🛠 Adding cron job as root..."
  SCRIPT_PATH="$(realpath "$SCRIPT")"
  LOG_PATH="$(dirname "$SCRIPT_PATH")/deploy-swarm.log"
  (sudo crontab -l 2>/dev/null | grep -v "$SCRIPT_PATH"; echo "*/10 * * * * $SCRIPT_PATH >> $LOG_PATH 2>&1") | sudo crontab -
  echo "Cron job added to root: runs every 10 minutes"
else
  echo "Skipping cron setup."
fi

