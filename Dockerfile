FROM rust:1.97.0-alpine AS builder
WORKDIR /app
COPY ./Cargo.toml ./Cargo.lock ./
RUN mkdir ./src && echo "fn main() {}" > ./src/main.rs
RUN cargo fetch && cargo build --release
COPY ./src ./src
# Подкидываем весь web как зависимость. В .dockerignore не нужные папки/файлы проигнорятся.
COPY ./web ./web
RUN cargo build -v --release

FROM alpine
ARG USERNAME=userx
ARG USER_UID=1000
ARG USER_GID=$USER_UID
RUN addgroup -g $USER_GID -S $USERNAME && adduser -u $USER_UID -S $USERNAME -G $USERNAME
WORKDIR /home/$USERNAME
COPY --chown=$USER_UID:$USER_GID --from=builder /app/target/release/rusland ./myapp
USER $USERNAME
# config.yaml - это общее название. В зависимости от окружения, подкидывается свой.
ENTRYPOINT ["./myapp", "--config=config.yaml"]
#CMD ["tail", "-f", "/dev/null"]