FROM caddy:2-builder-alpine@sha256:3548087c3e7ecaac874989ab376b9232a7af4a7ef8a1df4f9081be851a4c74e6 AS builder

RUN xcaddy build \
    --with github.com/caddy-dns/cloudflare \
    --with github.com/greenpau/caddy-security@v1.1.30

FROM caddy:2-alpine@sha256:e2e3a089760c453bc51c4e718342bd7032d6714f15b437db7121bfc2de2654a6

COPY --from=builder /usr/bin/caddy /usr/bin/caddy
