FROM docker.io/library/debian:trixie-slim
ARG IMPLEMENTATION

RUN apt-get update
# local
RUN apt-get install -y cmake git
# vcpkg
RUN apt-get install -y build-essential curl tar unzip zip
# uWebSockets
RUN apt-get install -y ninja-build

ENV VCPKG_DEFAULT_TRIPLET=x64-linux
ENV VCPKG_DISABLE_METRICS=1
ENV VCPKG_FORCE_SYSTEM_BINARIES=1
ENV VCPKG_ROOT=/opt/vcpkg
RUN git clone https://github.com/microsoft/vcpkg.git $VCPKG_ROOT && $VCPKG_ROOT/bootstrap-vcpkg.sh

COPY . /$IMPLEMENTATION
WORKDIR /$IMPLEMENTATION
RUN $VCPKG_ROOT/vcpkg install --recurse
RUN cmake -B build -S . \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_CXX_FLAGS="-march=x86-64-v3" \
    -DCMAKE_TOOLCHAIN_FILE=$VCPKG_ROOT/scripts/buildsystems/vcpkg.cmake
RUN cmake --build build
CMD ./build/bench
