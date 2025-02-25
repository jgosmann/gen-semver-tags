FROM rust:1 AS builder
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/gen-semver-tags /gen-semver-tags
ENTRYPOINT ["/gen-semver-tags"]
