FROM node:24.2.0-alpine@sha256:7aaba6b13a55a1d78411a1162c1994428ed039c6bbef7b1d9859c25ada1d7cc5 AS builder

ARG PUBLIC_BACKEND_URL
ARG PUBLIC_MAX_CHARS
ARG PUBLIC_CODE_LENGTH
ARG PUBLIC_TEMP_SESSION_DURATION_SECS
ARG PUBLIC_MEILI_KEY
ARG PUBLIC_MEILI_URL
ARG PUBLIC_PAGE_SIZE
ARG PUBLIC_MIN_PASSWORD_LENGTH
ARG PORT

ENV NODE_ENV=production
ENV PUBLIC_MIN_PASSWORD_LENGTH=$PUBLIC_MIN_PASSWORD_LENGTH
ENV PUBLIC_BACKEND_URL=$PUBLIC_BACKEND_URL
ENV PUBLIC_CODE_LENGTH=$PUBLIC_CODE_LENGTH
ENV PUBLIC_MAX_CHARS=$PUBLIC_MAX_CHARS
ENV PUBLIC_MEILI_KEY=$PUBLIC_MEILI_KEY
ENV PUBLIC_TEMP_SESSION_DURATION_SECS=$PUBLIC_TEMP_SESSION_DURATION_SECS
ENV PUBLIC_MEILI_URL=$PUBLIC_MEILI_URL
ENV PUBLIC_PAGE_SIZE=$PUBLIC_PAGE_SIZE

WORKDIR /app

COPY package.json package-lock.json ./

RUN npm ci --omit=dev

COPY . .

RUN npm run build

FROM node:24.2.0-alpine@sha256:7aaba6b13a55a1d78411a1162c1994428ed039c6bbef7b1d9859c25ada1d7cc5

WORKDIR /app

COPY --from=builder --chown=node:node /app/build ./build
COPY --from=builder --chown=node:node /app/node_modules ./node_modules
COPY --chown=node:node package.json .

CMD [ "node", "build" ]
