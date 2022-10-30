FROM rust:1.64.0 as builder
WORKDIR /usr/src/dustin
COPY . .
RUN cargo install --path .
 
FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/dustin /usr/local/bin/dustin
RUN apt-get update && apt-get install build-essential autoconf automake libtool m4
RUN apt-get install libopus-dev
RUN apt-get install ffmpeg
RUN apt-get install youtube-dl
CMD ["dustin"]