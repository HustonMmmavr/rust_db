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
    psql --command "CREATE USER mavr1 WITH SUPERUSER PASSWORD '951103';" &&\
    createdb -E UTF8 -T template0 -O mavr1 test &&\
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

# Копируем исходный код в Docker-контейнер
ENV WORK /opt/rust_db
ADD src/ $WORK/src/
ADD V1__userinit.sql $WORK/schema.sql
ADD install.sh $WORK/install.sh

# install rust and cargo
RUN chmod +x $WORK/install.sh && .$WORK/install.sh && rm $WORK/install.sh


WORKDIR $WORK

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ADD Cargo.toml $WORK/Cargo.toml
RUN cargo build --release

EXPOSE 5000

ENV PGPASSWORD 951103
CMD service postgresql start
CMD cd $WORK
CMD psql -h localhost -U mavr1 -d test -f schema.sql
CMD ./target/release/RustDb
