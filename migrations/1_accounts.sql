CREATE TABLE accounts(
    id TEXT NOT NULL UNIQUE,
    PRIMARY KEY (id),
    email TEXT NOT NULL UNIQUE,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
);