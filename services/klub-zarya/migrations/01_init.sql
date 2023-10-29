CREATE TABLE users (
    "id" SERIAL NOT NULL PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    "password" VARCHAR(50) NOT NULL
);

CREATE UNIQUE INDEX users_idx ON users (username);

CREATE TABLE users_friends (
    "id" SERIAL NOT NULL PRIMARY KEY,
    user_id SERIAL NOT NULL REFERENCES users ("id"),
    friend_id SERIAL NOT NULL REFERENCES users ("id"),
    is_confirmed BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE UNIQUE INDEX users_friends_idx ON users_friends (user_id, friend_id);

CREATE TABLE topics (
    "id" SERIAL NOT NULL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    author_id SERIAL NOT NULL REFERENCES users ("id"),
    theme VARCHAR(50) NOT NULL,
    is_public BOOLEAN NOT NULL DEFAULT TRUE,
    "description" TEXT NOT NULL
);

CREATE TABLE "comments" (
    "id" SERIAL NOT NULL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    topic_id SERIAL NOT NULL REFERENCES topics ("id"),
    author_id SERIAL NOT NULL REFERENCES users ("id"),
    "text" TEXT NOT NULL
);