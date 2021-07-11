# https://dev.to/sergeyzenchenko/actix-web-in-docker-how-to-build-small-and-secure-images-2mjd

# docker build . -t rust-tide-sqlx-crud

FROM rust:1.52.1 as build
#FROM rust:latest as build

WORKDIR /usr/src/rust-tide-sqlx-crud
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/rust-tide-sqlx-crud /usr/local/bin/rust-tide-sqlx-crud
CMD ["rust-tide-sqlx-crud"]
