DROP TABLE IF EXISTS zapiska;

CREATE TABLE zapiska (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT,
    `encrypted_message` VARCHAR(1024) NOT NULL
);