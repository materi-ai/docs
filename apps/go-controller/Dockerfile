FROM golang:1.21-alpine as builder
WORKDIR /app
COPY . .
RUN go mod tidy
RUN go build -o go-controller main.go

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/go-controller .
CMD ["./go-controller"]
