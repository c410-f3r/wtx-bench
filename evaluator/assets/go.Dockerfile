FROM docker.io/library/golang:1.22-bookworm
ARG IMPLEMENTATION
COPY . /$IMPLEMENTATION
WORKDIR /$IMPLEMENTATION
RUN go mod init websocket
RUN go mod tidy
CMD go run main.go
