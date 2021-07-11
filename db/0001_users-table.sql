DROP TABLE IF EXISTS users;

CREATE TABLE users (
    name VARCHAR PRIMARY KEY,
    email VARCHAR UNIQUE NOT NULL
);

insert into users (name, email) values ('test', 'test@localhost');
insert into users (name, email) values ('admin', 'admin@localhost');
insert into users (name, email) values ('nobody', 'nobody@localhost');
