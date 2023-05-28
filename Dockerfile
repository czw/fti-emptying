FROM rust:slim AS build
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# Create an empty shell project and build the depencies listed in the Cargo
# files. We do this to utilize Docker caching and speed up builds when only
# the code has been changed.
RUN cargo new --bin fti-emptying
WORKDIR /fti-emptying
COPY ./Cargo.* build.rs ./
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release --no-default-features

# Copy the real data, touch our main.rs and do a proper release build
COPY . .
RUN touch src/main.rs
RUN cargo build --release --no-default-features
RUN strip target/release/fti-emptying

# Build the runtime image
FROM debian:stable-slim AS deploy
COPY --from=build /fti-emptying/target/release/fti-emptying /
CMD /fti-emptying --notify-ntfy-host $NTFY_HOST $STATION_IDS
