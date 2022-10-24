FROM rust
WORKDIR /src
COPY . .
RUN cargo build --release
CMD ["./target/release/Release"]
EXPOSE 8000