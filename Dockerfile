# Arch Linux used as build environment to provide `musl-gcc` et al.
FROM archlinux:latest AS builder

ARG ARCH=x86_64
ARG CARGO_BIN=/root/.cargo/bin/
ARG RUST_TOOLCHAIN=stable-${ARCH}-unknown-linux-gnu
ARG MUSL_TOOLCHAIN=${ARCH}-unknown-linux-musl

RUN pacman -Suy --noconfirm \
    gcc \
    musl
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y && \
    ${CARGO_BIN}/rustup target install ${MUSL_TOOLCHAIN}
COPY . .
RUN ${CARGO_BIN}/cargo build --target=${MUSL_TOOLCHAIN} --release && \
    cp target/${ARCH}-unknown-linux-musl/release/lambda_template lambda

# Alpine Linux used as runtime environment for small image size
FROM alpine:latest

ARG WD=/var/task
ARG TASK_BIN_DIR=${WD}/bin

RUN mkdir -p ${TASK_BIN_DIR}/
WORKDIR ${WD}

COPY --from=builder /lambda ${TASK_BIN_DIR}/
ENV PATH=$PATH:${TASK_BIN_DIR}

ENTRYPOINT [ "lambda" ]