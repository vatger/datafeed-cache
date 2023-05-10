FROM node:alpine

WORKDIR /opt

COPY . .

RUN npm install --quiet --unsafe-perm --no-progress --no-audit

CMD npm run start:prod