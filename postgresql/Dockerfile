FROM postgres:latest

ENV POSTGRES_USER=user
ENV POSTGRES_PASSWORD=pasword
ENV POSTGRES_DB=db

COPY init.sql /docker-entrypoint-initdb.d/

EXPOSE 5432
