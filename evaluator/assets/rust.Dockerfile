FROM docker.io/library/rust:1.77-bookworm AS build
ARG IMPLEMENTATION
COPY . /$IMPLEMENTATION
WORKDIR /$IMPLEMENTATION
RUN RUSTFLAGS="-Ccodegen-units=1 -Cllvm-args=-inline-threshold=400 -Copt-level=3 -Cpanic=abort -Cstrip=symbols -Ctarget-cpu=native" cargo build --release

FROM docker.io/library/debian:bookworm
ARG IMPLEMENTATION
WORKDIR /$IMPLEMENTATION
COPY --from=build /$IMPLEMENTATION/target/release/$IMPLEMENTATION .
ENV IMPLEMENTATION ${IMPLEMENTATION}
CMD ./${IMPLEMENTATION}
