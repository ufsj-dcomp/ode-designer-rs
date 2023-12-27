FROM ghcr.io/rust-lang/rust:nightly-slim AS setup

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y \
        g++ cmake pkg-config \
        libfontconfig-dev \
        file

WORKDIR /ode-designer

COPY . .

# ------------------------------------------------------------------------------

FROM setup AS planner

RUN cargo install cargo-chef

RUN cargo chef prepare --recipe-path recipe.json

# ------------------------------------------------------------------------------

FROM planner AS cacher

RUN cargo chef cook --release --recipe-path recipe.json

# ------------------------------------------------------------------------------

FROM setup AS builder

RUN cargo install cargo-appimage

COPY --from=appimagecrafters/appimage-builder /opt/appimage-tool.AppDir /opt/appimage-tool.AppDir
RUN ln -s /opt/appimage-tool.AppDir/AppRun /usr/bin/appimagetool

COPY --from=cacher $CARGO_HOME $CARGO_HOME
