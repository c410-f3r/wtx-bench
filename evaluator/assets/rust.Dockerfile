FROM docker.io/library/rust:1.89-slim-bookworm aS build
ARG IMPLEMENTATION
COPY . /$IMPLEMENTATION
WORKDIR /$IMPLEMENTATION
RUN apt-get update && apt-get install gcc protobuf-compiler -y
ENV CARGO_PROFILE_RELEASE_LTO=true
RUN RUSTFLAGS="-Ctarget-cpu=x86-64-v3" cargo build --release

FROM docker.io/library/debian:bookworm-slim
ARG IMPLEMENTATION
WORKDIR /$IMPLEMENTATION
COPY --from=build /$IMPLEMENTATION/target/release/$IMPLEMENTATION .
ENV IMPLEMENTATION ${IMPLEMENTATION}
ENV RUST_BACKTRACE=1
CMD ./${IMPLEMENTATION}
