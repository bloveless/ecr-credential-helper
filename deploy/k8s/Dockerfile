FROM alpine:3.10

ADD https://storage.googleapis.com/kubernetes-release/release/v1.16.2/bin/linux/amd64/kubectl /usr/local/bin/kubectl
ADD entrypoint.sh /usr/local/bin/entrypoint.sh

RUN set -x && \
    apk add --no-cache curl ca-certificates python3 && \
    chmod +x /usr/local/bin/kubectl && \
    chmod +x /usr/local/bin/entrypoint.sh && \
    adduser -S kubectl && \
    pip3 install --upgrade pip && \
    pip3 install --upgrade awscli

USER kubectl

ENTRYPOINT "/usr/local/bin/entrypoint.sh"
