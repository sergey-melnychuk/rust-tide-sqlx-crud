# https://dev.to/sergeyzenchenko/actix-web-in-docker-how-to-build-small-and-secure-images-2mjd

# docker build . -t rust-tide-sqlx-crud

#FROM rust:latest as build
FROM rust:1.86.0 as build

WORKDIR /usr/src/rust-tide-sqlx-crud
COPY . .

RUN cargo install --path .

# https://github.com/GoogleContainerTools/distroless
FROM gcr.io/distroless/cc-debian12

COPY --from=build /usr/local/cargo/bin/rust-tide-sqlx-crud /usr/local/bin/rust-tide-sqlx-crud
CMD ["rust-tide-sqlx-crud"]
