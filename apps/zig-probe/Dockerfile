FROM alpine:edge as builder
RUN apk add --no-cache zig
WORKDIR /app
COPY main.zig .
# Build static binary
RUN zig build-exe main.zig -O ReleaseSmall -fstrip -target x86_64-linux-musl

FROM scratch
COPY --from=builder /app/main /main
CMD ["/main"]
