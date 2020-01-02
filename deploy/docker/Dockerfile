FROM rust:1.40-buster as builder

RUN useradd -ms /bin/bash appuser \
    && apt-get update \
    && apt-get install -y libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /home/appuser

USER appuser

COPY Cargo.lock Cargo.toml /home/appuser/

RUN cargo fetch

COPY src /home/appuser/src

RUN cargo build --release

CMD ["ecr-credential-helper"]

###### ------- PRODUCTION IMAGE ------- ######
FROM scratch

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /home/appuser/target/release/ecr-credential-helper /usr/local/bin/

USER appuser

WORKDIR /home/appuser

CMD ["ecr-credential-helper"]
