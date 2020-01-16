FROM rust:1.40-buster as builder

RUN useradd -ms /bin/bash appuser

WORKDIR /home/appuser

USER appuser

COPY Cargo.lock Cargo.toml /home/appuser/

RUN cargo fetch

COPY src /home/appuser/src

RUN cargo build --release

CMD ["ecr-credential-helper"]

###### ------- PRODUCTION IMAGE ------- ######
FROM gcr.io/distroless/cc

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /home/appuser/target/release/ecr-credential-helper /usr/local/bin/

USER appuser

WORKDIR /home/appuser

ENTRYPOINT ["ecr-credential-helper"]

