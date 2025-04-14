ARG NODE_VERSION=23.11.0
FROM node:${NODE_VERSION}-alpine AS base

WORKDIR /app

COPY package.json .

RUN npm install

COPY . .

EXPOSE 8080

CMD [ "npm", "run", "dev" ]