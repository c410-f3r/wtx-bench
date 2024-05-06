FROM ghcr.io/instrumentisto/rust:beta-bookworm AS build
ARG IMPLEMENTATION
COPY . /$IMPLEMENTATION
WORKDIR /$IMPLEMENTATION
ENV CARGO_PROFILE_RELEASE_LTO=true
RUN RUSTFLAGS="-Ccodegen-units=1 -Copt-level=3 -Cpanic=abort -Cstrip=symbols -Ctarget-cpu=native" cargo build --release

FROM docker.io/library/debian:bookworm
ARG IMPLEMENTATION
WORKDIR /$IMPLEMENTATION
COPY --from=build /$IMPLEMENTATION/target/release/$IMPLEMENTATION .
ENV IMPLEMENTATION ${IMPLEMENTATION}
ENV RUST_BACKTRACE=1
CMD ./${IMPLEMENTATION}
