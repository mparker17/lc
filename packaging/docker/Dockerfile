FROM alpine:edge AS build
WORKDIR /tmp/lc
COPY . .
RUN apk --no-cache add cargo
ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN cargo build --release

FROM scratch
COPY --from=build /tmp/lc/target/release/lc /bin/
ENTRYPOINT ["/bin/lc"]
VOLUME ["/mnt"]
WORKDIR ["/mnt"]
