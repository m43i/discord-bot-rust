FROM rust:1.64.0 as builder
WORKDIR /usr/src/dustin
COPY . .
RUN apt-get update && apt-get install -y cmake
RUN cargo install --path .
 
FROM ubuntu:latest
COPY --from=builder /usr/local/cargo/bin/dustin /usr/local/bin/dustin
RUN apt-get clean
RUN apt-get update
RUN apt-get install -y libopus-dev && apt-get install -y ffmpeg && apt-get install -y youtube-dl
RUN rm -rf /var/lib/apt/lists/*
CMD ["dustin"]