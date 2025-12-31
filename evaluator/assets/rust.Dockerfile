FROM docker.io/library/rust:1.89-slim-trixie AS build
ARG IMPLEMENTATION
COPY . /$IMPLEMENTATION
WORKDIR /$IMPLEMENTATION
RUN apt-get update && apt-get install gcc protobuf-compiler -y
ENV CARGO_PROFILE_RELEASE_LTO=true
RUN RUSTFLAGS="-Ctarget-cpu=x86-64-v3" cargo build --release

FROM docker.io/library/debian:trixie-slim
ARG IMPLEMENTATION
WORKDIR /$IMPLEMENTATION
COPY --from=build /$IMPLEMENTATION/target/release/$IMPLEMENTATION-bench .
ENV IMPLEMENTATION ${IMPLEMENTATION}
CMD ./${IMPLEMENTATION}-bench
