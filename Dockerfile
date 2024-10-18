ARG PYTHON_BASE_APPIMAGE_PATH=/python-base-appimage

FROM appimagecrafters/appimage-builder AS python-appimage-downloader
ARG PYTHON_BASE_APPIMAGE_PATH

WORKDIR /downloaded-appimage

RUN wget https://github.com/niess/python-appimage/releases/download/python3.11/python3.11.1-cp311-cp311-manylinux_2_24_x86_64.AppImage -O python3.11.AppImage

RUN chmod +x ./python3.11.AppImage

RUN ./python3.11.AppImage --appimage-extract

# ------------------------------------------------------------------------------

FROM python-appimage-downloader AS python-appimage
ARG PYTHON_BASE_APPIMAGE_PATH
ENV PYTHON_BASE_APPIMAGE_PATH=${PYTHON_BASE_APPIMAGE_PATH}

RUN --mount=type=cache,target=/root/.cache/pip \
    --mount=type=bind,source=requirements.txt,target=requirements.txt \
    ./squashfs-root/usr/bin/pip3 \
    install \
    -r requirements.txt

RUN cp -r ./squashfs-root $PYTHON_BASE_APPIMAGE_PATH

# ------------------------------------------------------------------------------

FROM ghcr.io/rust-lang/rust:nightly-buster-slim AS setup
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y \
        g++ cmake pkg-config \
        libfontconfig-dev \
        file

WORKDIR /ode-designer

# ------------------------------------------------------------------------------

FROM setup AS planner

RUN cargo install cargo-chef

RUN --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=src/,target=src/ \
    --mount=type=bind,source=crates/,target=crates/ \
    --mount=type=bind,source=.cargo/,target=.cargo/ \
    --mount=type=cache,target=/root/.cargo/ \
    cargo chef prepare --recipe-path recipe.json

# ------------------------------------------------------------------------------

FROM planner AS cacher

COPY Cargo.toml Cargo.lock ./
COPY src/ src/
COPY crates/ crates/
COPY .cargo/ .cargo/

RUN --mount=type=cache,target=/root/.cargo/ \
    cargo chef cook --release --recipe-path recipe.json

# ------------------------------------------------------------------------------

FROM setup AS builder-dependencies
ARG PYTHON_BASE_APPIMAGE_PATH
ENV PYTHON_BASE_APPIMAGE_PATH=${PYTHON_BASE_APPIMAGE_PATH}

COPY --from=cacher $CARGO_HOME $CARGO_HOME

COPY --from=python-appimage $PYTHON_BASE_APPIMAGE_PATH $PYTHON_BASE_APPIMAGE_PATH

COPY --from=appimagecrafters/appimage-builder /opt/appimage-tool.AppDir /opt/appimage-tool.AppDir
RUN ln -s /opt/appimage-tool.AppDir/AppRun /usr/bin/appimagetool

# ------------------------------------------------------------------------------

FROM builder-dependencies AS builder

COPY . .

RUN appimage/scripts/build-appimage.sh

# ------------------------------------------------------------------------------

FROM scratch

COPY --from=builder /ode-designer/target/appimage/ode-designer-rs.AppImage /ode-designer.AppImage
