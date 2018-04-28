FROM ubuntu:17.10
#
# Установка postgresql
#

RUN apt-get -y update && apt-get install -y wget git

ENV PGVER 10
RUN apt-get update -q
RUN apt-get install -q -y wget
RUN wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add - && echo "deb http://apt.postgresql.org/pub/repos/apt/ zesty-pgdg main" > /etc/apt/sources.list.d/pgdg.list
RUN apt-get update -q
RUN apt-get install -q -y git postgresql-10 postgresql-contrib-10

# Run the rest of the commands as the ``postgres`` user created by the ``postgres-$PGVER`` package when it was ``apt-get installed``
USER postgres

# Create a PostgreSQL role named ``docker`` with ``docker`` as the password and
# then create a database `docker` owned by the ``docker`` role.
RUN /etc/init.d/postgresql start &&\
    psql --command "CREATE USER mavr WITH SUPERUSER PASSWORD '951103';" &&\
    createdb -E UTF8 -T template0 -O mavr test &&\
    /etc/init.d/postgresql stop

# Adjust PostgreSQL configuration so that remote connections to the
# database are possible.
RUN echo "host all  all    0.0.0.0/0  md5" >> /etc/postgresql/$PGVER/main/pg_hba.conf

# And add ``listen_addresses`` to ``/etc/postgresql/$PGVER/main/postgresql.conf``
RUN echo "listen_addresses='*'" >> /etc/postgresql/$PGVER/main/postgresql.conf
RUN echo "synchronous_commit = off" >> /etc/postgresql/$PGVER/main/postgresql.conf
RUN echo "fsync = 'off'" >> /etc/postgresql/$PGVER/main/postgresql.conf
RUN echo "max_wal_size = 1GB" >> /etc/postgresql/$PGVER/main/postgresql.conf
RUN echo "shared_buffers = 128MB" >> /etc/postgresql/$PGVER/main/postgresql.conf
RUN echo "effective_cache_size = 256MB" >> /etc/postgresql/$PGVER/main/postgresql.conf
RUN echo "work_mem = 64MB" >> /etc/postgresql/$PGVER/main/postgresql.conf
# Expose the PostgreSQL port
EXPOSE 5432

# Add VOLUMEs to allow backup of config, logs and databases
VOLUME ["/etc/postgresql", "/var/log/postgresql", "/var/lib/postgresql"]


USER root
RUN apt-get install curl -q -y
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
# ENV RUST_VERSION=1.19.0
# FROM rust:1.19.0
# RUN apt-get install -q -y \
#     curl \
#     openssh-client \
#     libssl-dev \
#     pkg-config && \
#   curl -sO https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init && \
#   chmod +x rustup-init && \
#   ./rustup-init -y --default-toolchain $RUST_VERSION --no-modify-path && \
#   apt-get remove --purge -y curl && \
#   apt-get autoremove -y && \
#   rm -rf \
#     rustup-init \
#     /var/lib/apt/lists/* \
#     /tmp/* \
#     /var/tmp/* && \
# mkdir /source &&\
# export PATH="$HOME/.cargo/bin:$PATH"
# VOLUME [ "/source" ]

#RUN apt-get install curl -q -y
#RUN apt-get install rustc -q -y
#RUN curl -sSf https://static.rust-lang.org/rustup.sh | sh
#! /bin/bash

# exit if a command fails
# RUN set -e

# RUN triple=x86_64-unknown-linux-gnu

# # install curl (needed to install rust)
# RUN apt-get update && apt-get install -y curl gdb g++-multilib lib32stdc++6 libssl-dev libncurses5-dev

# # install rust
# RUN curl -sL https://static.rust-lang.org/dist/rust-nightly-$triple.tar.gz | tar xvz -C /tmp
# RUN /tmp/rust-nightly-$triple/install.sh

# # install cargo
# RUN curl -sL https://static.rust-lang.org/cargo-dist/cargo-nightly-$triple.tar.gz | tar xvz -C /tmp
# RUN /tmp/cargo-nightly-$triple/install.sh

# # cleanup package manager
# RUN apt-get remove --purge -y curl && apt-get autoclean && apt-get clean
# RUN rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

# # prepare dir
# RUN mkdir /source


# Копируем исходный код в Docker-контейнер
ENV WORK /opt/rust_db
ADD src/ $WORK/src/
ADD V1__userinit.sql $WORK/schema.sql
#ADD install.sh $WORK/install.sh

# install rust and cargo


#WORKDIR $WORK
#RUN chmod +x install.sh && ./install.sh && rm install.sh

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ADD Cargo.toml $WORK/Cargo.toml
RUN cargo update -p libc
RUN cargo build -v --release

EXPOSE 5000

ENV PGPASSWORD 951103
CMD service postgresql start && cd $WORK/ && psql -h localhost -U mavr -d test -f schema.sql && ./target/release/RustDb
