FROM golang:1.13.5-buster as builder

RUN apt-get update \
    && apt-get install ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY go.mod go.sum /app/

WORKDIR /app

RUN go mod download

COPY main.go /app

RUN CGO_ENABLED=0 go build main.go

CMD ["./main"]

# ----------------------------------------------------------------------------------

FROM scratch

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /app/main /app/main

WORKDIR /app

CMD ["./main"]
