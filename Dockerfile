FROM node:alpine

WORKDIR /opt/datafeed_cache

ARG NODE_ENV=production

COPY package*.json ./

RUN npm install --quiet --unsafe-perm --no-progress --no-audit

COPY . .

CMD npm run start:prod