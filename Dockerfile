FROM alpine:latest as builder

ARG MOCKSER_VERSION

RUN apk --no-cache add wget tar outils-sha256

RUN wget https://github.com/daxartio/mockser/releases/download/${MOCKSER_VERSION}/mockser-${MOCKSER_VERSION}-x86_64-unknown-linux-musl.tar.gz && \
    wget https://github.com/daxartio/mockser/releases/download/${MOCKSER_VERSION}/mockser-${MOCKSER_VERSION}-x86_64-unknown-linux-musl.tar.gz.sha256 && \
    tar -xvf mockser-${MOCKSER_VERSION}-x86_64-unknown-linux-musl.tar.gz && \
    mv mockser-${MOCKSER_VERSION}-x86_64-unknown-linux-musl/mockser /usr/local/bin/mockser && \
    if [ "$(sha256sum /usr/local/bin/mockser | awk '{print $1}')" != "$(cat mockser-${MOCKSER_VERSION}-x86_64-unknown-linux-musl.tar.gz.sha256)"]; then \
        echo "Checksum failed" && exit 1; \
    fi

FROM alpine:latest

ARG MOCKSER_VERSION
ENV MOCKSER_VERSION=${MOCKSER_VERSION}

COPY --from=builder /usr/local/bin/mockser /usr/local/bin/mockser

CMD ["mockser"]
