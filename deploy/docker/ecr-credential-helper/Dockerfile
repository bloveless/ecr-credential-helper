FROM rust:1.38-slim-stretch as builder

RUN useradd -ms /bin/bash appuser \
    && apt-get update \
    && apt-get install -y libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/ecr-credential-helper
COPY . .

RUN cargo install --path .

USER appuser

CMD ["ecr-credential-helper"]

###### ------- PRODUCTION IMAGE ------- ######
FROM debian:slim-stretch as production

RUN useradd -ms /bin/bash appuser

COPY --from=builder /usr/local/cargo/bin/ecr-credential-helper /usr/local/bin/ecr-credential-helper

USER appuser

CMD ["ecr-credential-helper"]
