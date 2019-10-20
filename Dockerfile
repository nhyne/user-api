FROM nhyne/rust-musl-builder-sccache:nightly-2019-09-05 AS build

ARG AWS_ACCESS_KEY_ID
ARG AWS_SECRET_ACCESS_KEY
ARG SCCACHE_BUCKET=nhyne-build-cache
ARG RUSTC_WRAPPER=sccache

ADD . /home/rust/src

RUN sccache --start-server && cargo build --bin user-api --release && sccache -s

FROM alpine:3.10.1 as runner

COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/user-api /user-api

CMD ["./user-api"]
