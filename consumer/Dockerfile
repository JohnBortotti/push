FROM rust:1.73.0

WORKDIR /user/src/app

COPY . .

RUN cargo install --path . 

CMD ["notifications-api-consumer"]
