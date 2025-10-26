####################################################################################################
FROM rust:latest AS builder

RUN update-ca-certificates

# Create appuser
ENV USER=skinnycert
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /skinnycert

COPY . .

# We no longer need to use the x86_64-unknown-linux-musl target
RUN cargo build --release

FROM gcr.io/distroless/cc

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /skinnycert

# Copy our build
COPY --from=builder /skinnycert/target/release/skinnycert ./

# Use an unprivileged user.
USER skinnycert:skinnycert

CMD ["/skinnycert/skinnycert"]
