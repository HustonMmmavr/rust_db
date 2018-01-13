FROM ubuntu:17.04
#
# Установка postgresql
#

RUN apt-get -y update

ENV PGVER 9.6
RUN apt-get install -y postgresql-$PGVER

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

# Expose the PostgreSQL port
EXPOSE 5432

# Add VOLUMEs to allow backup of config, logs and databases
VOLUME ["/etc/postgresql", "/var/log/postgresql", "/var/lib/postgresql"]


USER root

# install rust and cargo
RUN set -e
RUN triple=x86_64-unknown-linux-gnu

# install curl (needed to install rust)
RUN apt-get update && apt-get install -y curl gdb g++-multilib lib32stdc++6 libssl-dev libncurses5-dev

# install rust
RUN curl -sL https://static.rust-lang.org/dist/rust-nightly-$triple.tar.gz | tar xvz -C /tmp
/tmp/rust-nightly-$triple/install.sh

# install cargo
RUN curl -sL https://static.rust-lang.org/cargo-dist/cargo-nightly-$triple.tar.gz | tar xvz -C /tmp
/tmp/cargo-nightly-$triple/install.sh

# cleanup package manager
RUN apt-get remove --purge -y curl && apt-get autoclean && apt-get clean
RUN rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

# prepare dir
RUN mkdir /source


# Копируем исходный код в Docker-контейнер
ENV WORK /opt/rust_db
ADD src/ $WORK/src/
ADD V1__userinit.sql $WORK/schema.sql
ADD install.sh $WORK/install.sh




WORKDIR $WORK

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ADD Cargo.toml $WORK/Cargo.toml
RUN cargo build --release

EXPOSE 5000

ENV PGPASSWORD 951103
CMD service postgresql start
CMD cd $WORK
CMD psql -h localhost -U mavr -d test -f schema.sql 
CMD ./target/release/RustDb
