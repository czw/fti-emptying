# Create an empty shell project and build the depencies listed in the Cargo
# files. We do this to utilize Docker caching and speed up builds when only
# the code has been changed.
FROM rust:slim AS build
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN cargo new --bin fti-emptying
WORKDIR /fti-emptying
COPY ./Cargo.* .
RUN cargo fetch
RUN cargo build -r -p anyhow
RUN cargo build -r -p chrono
RUN cargo build -r -p clap
RUN cargo build -r -p minidom
RUN cargo build -r -p ureq

# Do a proper release build
COPY ./src ./src
RUN cargo build -r --no-default-features

# Build the runtime image
FROM debian:stable-slim AS deploy
COPY --from=build /fti-emptying/target/release/fti-emptying /
CMD /fti-emptying --notify-ntfy-host $NTFY_HOST $STATION_IDS
