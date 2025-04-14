# syntax=docker/dockerfile:1
ARG NODE_VERSION=23.11.0
FROM node:${NODE_VERSION}-alpine AS base

WORKDIR /usr/src/app

# Copy only package.json and lock file first (for caching npm install)
COPY package*.json ./

# Install dependencies
RUN npm install

# Now copy the rest of the files
COPY . .

# Set environment variable
ENV NODE_ENV=development

# Expose port for Vite
EXPOSE 5174

# Run dev server
CMD ["npm", "run", "dev"]

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/

# Want to help us make this template better? Share your feedback here: https://forms.gle/ybq9Krt8jtBL3iCk7

#ARG NODE_VERSION=23.11.0

################################################################################
# Use node image for base image for all stages.
#FROM node:${NODE_VERSION}-alpine AS base

# Set working directory for all build stages.
#WORKDIR /usr/src/app

#ENV NODE_ENV=production

# Copy the rest of the source files into the image.
#RUN npm install --verbose

#COPY . .

# Use production node environment by default.
#ENV NODE_ENV=production

# Expose the port that the application listens on.
#EXPOSE 5174

#EXPOSE 8080

# Run the application.
#CMD ["npm", "run", "dev", "--verbose"]