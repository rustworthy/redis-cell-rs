################################ BUILDER ######################################
FROM rust:1.89-bookworm AS builder

WORKDIR /redis-cell

RUN git clone -b valkey https://github.com/rustworthy/redis-cell.git .
RUN cargo build --release --features valkey

################################ RUNTIME ######################################
FROM bitnami/valkey:latest AS runtime

USER root
RUN mkdir -p /usr/local/lib/valkey/modules
COPY --from=builder /redis-cell/target/release/libredis_cell.so /usr/local/lib/valkey/modules/libredis_cell.so
RUN chown -R 1001:1001 /usr/local/lib/valkey

USER 1001

ENTRYPOINT [ "/opt/bitnami/scripts/valkey/entrypoint.sh" ]
CMD [ "/opt/bitnami/scripts/valkey/run.sh", "--loadmodule", "/usr/local/lib/valkey/modules/libredis_cell.so" ]
