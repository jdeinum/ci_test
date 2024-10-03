FROM --platform=$BUILDPLATFORM rust:1-alpine3.20 AS base
RUN apk add openssl openssl-dev libc-dev gcc musl-dev

FROM base AS test
COPY . .
RUN cargo test --release

FROM base AS build
COPY . .
RUN cargo build --release

FROM alpine 
COPY --from=build /target/release/ci_test /usr/bin/ci_test

CMD [ "/usr/bin/ci_test" ]
