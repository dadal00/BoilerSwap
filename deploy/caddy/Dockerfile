FROM caddy:2-builder-alpine@sha256:3548087c3e7ecaac874989ab376b9232a7af4a7ef8a1df4f9081be851a4c74e6

RUN xcaddy build \
    --with github.com/caddy-dns/cloudflare \
    --with github.com/greenpau/caddy-security
