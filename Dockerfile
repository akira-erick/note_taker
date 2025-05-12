FROM postgres:latest

ENV POSTGRES_USER=myuser
ENV POSTGRES_PASSWORD=mysecretpassword
ENV POSTGRES_DB=mydb

COPY init.sql /docker-entrypoint-initdb.d/

EXPOSE 5432
