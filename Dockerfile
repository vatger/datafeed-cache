FROM node:alpine

WORKDIR /opt

COPY . .

ARG NODE_ENV=production

RUN npm install --quiet --unsafe-perm --no-progress --no-audit

CMD npm run start:prod