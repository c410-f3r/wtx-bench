FROM docker.io/library/debian:bookworm-slim AS build
ARG IMPLEMENTATION
COPY . /$IMPLEMENTATION
WORKDIR /$IMPLEMENTATION
RUN apt-get update && apt-get install curl gcc -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly-2024-07-10 --profile minimal -y
ENV CARGO_PROFILE_RELEASE_LTO=true
RUN RUSTFLAGS="-Ccodegen-units=1 -Copt-level=3 -Cpanic=abort -Cstrip=symbols -Ctarget-cpu=native" $HOME/.cargo/bin/cargo build --release

FROM docker.io/library/debian:bookworm-slim
ARG IMPLEMENTATION
WORKDIR /$IMPLEMENTATION
COPY --from=build /$IMPLEMENTATION/target/release/$IMPLEMENTATION .
ENV IMPLEMENTATION ${IMPLEMENTATION}
ENV RUST_BACKTRACE=1
CMD ./${IMPLEMENTATION}
