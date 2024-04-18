FROM docker.io/library/node:21-bookworm
ARG IMPLEMENTATION
COPY . /$IMPLEMENTATION
WORKDIR /$IMPLEMENTATION
RUN npm install
CMD node main.js
