FROM docker.io/library/rust:1.80-slim-bookworm aS build
ARG IMPLEMENTATION
COPY . /$IMPLEMENTATION
WORKDIR /$IMPLEMENTATION
RUN apt-get update && apt-get install gcc protobuf-compiler -y
ENV CARGO_PROFILE_RELEASE_LTO=true
RUN RUSTFLAGS="-Ccodegen-units=1 -Copt-level=3 -Cpanic=abort -Cstrip=symbols -Ctarget-cpu=native" cargo build --release

FROM docker.io/library/debian:bookworm-slim
ARG IMPLEMENTATION
WORKDIR /$IMPLEMENTATION
COPY --from=build /$IMPLEMENTATION/target/release/$IMPLEMENTATION .
ENV IMPLEMENTATION ${IMPLEMENTATION}
ENV RUST_BACKTRACE=1
CMD ./${IMPLEMENTATION}
