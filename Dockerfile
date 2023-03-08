FROM rust
WORKDIR /src
COPY . .
RUN rustup default nightly 
RUN cargo install --path .
CMD ["./target/release/Rustico"]
EXPOSE 8000