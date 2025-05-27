-- Add up migration script here
CREATE TABLE players (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    power INTEGER NOT NULL,
    leader INTEGER NOT NULL
);

CREATE TABLE draft (
    leader TEXT NOT NULL,
    member TEXT NOT NULL
);
