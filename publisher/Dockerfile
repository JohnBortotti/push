FROM rust:1.73.0

WORKDIR /app

COPY . .

RUN cargo install --path .

CMD ["notifications-api-publisher"]
