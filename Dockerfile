FROM rust:1.31.0

RUN curl -L https://github.com/ncopa/su-exec/archive/dddd1567b7c76365e1e0aac561287975020a8fad.tar.gz | tar xvz \
    && cd su-exec-* \
    && make && mv su-exec /usr/local/bin \
    && cd .. \
    && rm -rf su-exec-*

WORKDIR /workdir

COPY src ./src
COPY Cargo.toml Cargo.lock ./
COPY docker-entrypoint.sh /

RUN cargo build \
    && cargo install cargo-watch

ENTRYPOINT [ "/docker-entrypoint.sh" ]

CMD [ "cargo", "watch", "-c", "-x", "check", "-x", "test" ]
