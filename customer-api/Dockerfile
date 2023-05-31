FROM rust:1.69 as build

# create a new empty shell project
RUN USER=root cargo new --bin customer-api
WORKDIR /customer-api

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
SHELL ["/bin/bash", "-c"]
# RUN [ "/bin/bash", "-c", "rm ./target/release/deps/customer-api*"]
RUN ls target/release/deps/ | grep customer-api | xargs -i echo target/release/deps/{}
RUN cargo build --release

# our final base
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /customer-api/target/release/customer-api .

# set the startup command to run your binary
CMD ["./customer-api"]