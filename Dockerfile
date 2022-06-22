FROM rust:bullseye
RUN apt update && apt -y install libasound2 libasound2-dev
ENV CARGO_TARGET_x86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS="-L /usr/lib/x86_64-linux-gnu -C link-args=-Wl,-rpath-link,/usr/lib/x86_64-linux-gnu $CARGO_TARGET_x86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS"