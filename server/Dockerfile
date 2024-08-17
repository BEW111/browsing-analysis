FROM rust:1-slim-bullseye

WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch
COPY . /code

# Build for development (this will be cached if dependencies don't change)
RUN cargo build

EXPOSE 8000

CMD ["cargo", "run", "--offline"]