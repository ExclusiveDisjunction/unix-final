# Personal Library - COP3604

## Table of Contents

## Installation
Installation on a new Linux virtual machine is the easiest, since the project utilizes firewall rules that are implemented automatically. It is recommended to have either a decent amount of RAM or a swap partition setup on the device this is being implemented on. If installing on a GCP VM ensure that the VM has HTTP allow for the firewall rule and to make a firewall rule for port 8080 on all instances on the network on 0.0.0.0/0

To start, firstly make sure git is installed on the machine.
### `sudo apt install git`

After this clone this repo using the following:
### `sudo git clone https://github.com/ExclusiveDisjunction/unix-final`
This will create a new directory, "unix-final" that stores this repository.

Changing into the directory
### `cd /unix-final`

To now begin the installation, change the permissions of install.sh using the following command:
### `sudo chmod u+x install.sh`

This will make install.sh executable so you can now install the project:
### `sudo ./install.sh`

install.sh will install the dependencies needed to run the project, including: npm, docker, and docker-compose.

Alongside that, it will also initialize a ufw firewall for the machine. The specific rules it allows are for OpenSSH (for GCP VM SSHing) and HTTP. This will make the website visible to the outside world on port 80. 

The script will ask what IP you want bound to the docker swarm for advertisement if there is more than one IP on the machine. This is to account for machines that have multiple network adapters, either physical or virtual.

<<<<<<< HEAD
=======
The script will ask what IP you want bound to the docker swarm for advertisement if there is more than one IP on the machine. This is to account for machines that have multiple network adapters, either physical or virtual.

>>>>>>> app-deploy
Following that, it will initially run deploy-swarm.sh, which builds the images necessary to run the project as well as updating them from the repo as needed.

An optional entry at the end of this script is provided in case the user wants to add in a cron job for deploy-swarm which will act as an update service for the images being used. 

The project will still run if no cron job is created.



## Usage

## Acknoledgements

## Project Status

