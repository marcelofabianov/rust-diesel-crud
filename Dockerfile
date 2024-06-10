FROM rust:1.78

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /usr/src/app

RUN apt-get update -yqq && \
  apt-get upgrade -y && \
  apt-get install -y --no-install-recommends \
  git \
  wget \
  curl \
  unzip \
  nano \
  tree \
  build-essential \
  && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-watch

COPY . .

RUN cargo build

EXPOSE 8000

CMD ["cargo", "watch", "-x", "run"]
