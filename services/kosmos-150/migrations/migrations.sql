CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    password VARCHAR(50) NOT NULL
);

CREATE UNIQUE INDEX users_idx ON users (username);

CREATE TABLE spaceports (
    id SERIAL PRIMARY KEY,
    "name" VARCHAR(50) NOT NULL,
    star_system VARCHAR(50) NOT NULL,
    "location" VARCHAR(50) NOT NULL
);

CREATE UNIQUE INDEX spaceports_idx ON spaceports ("name", "location");

CREATE TABLE spaceships (
    id SERIAL PRIMARY KEY,
    "name" VARCHAR(50) NOT NULL,
    seats_number INT NOT NULL
);

CREATE UNIQUE INDEX spaceships_idx ON spaceships ("name");

CREATE TABLE flights (
    id SERIAL PRIMARY KEY,
    spaceship_id SERIAL REFERENCES spaceships (id),
    from_spaceport_id SERIAL NOT NULL REFERENCES spaceports (id),
    to_spaceport_id SERIAL NOT NULL REFERENCES spaceports (id),
    departure TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX flights_idx ON flights (spaceship_id, from_spaceport_id, to_spaceport_id, departure);

CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    user_id SERIAL NOT NULL REFERENCES users (id),
    flight_id SERIAL NOT NULL REFERENCES flights (id),
    occupied_seat INT NOT NULL,
    "comment" TEXT
);

CREATE UNIQUE INDEX orders_idx ON orders (user_id, flight_id);

INSERT INTO spaceports ("name", star_system, "location") VALUES ('Звезда', 'Шедар', 'Шедар-альфа');
INSERT INTO spaceports ("name", star_system, "location") VALUES ('Красный Октябрь', 'Солнце', 'Земля');
INSERT INTO spaceports ("name", star_system, "location") VALUES ('Аврора', 'Вега', 'Свердлов');
INSERT INTO spaceports ("name", star_system, "location") VALUES ('Горизонт', 'Солнце', 'Юпитер');
INSERT INTO spaceports ("name", star_system, "location") VALUES ('Пролетариат', 'Солнце', 'Сатурн');
INSERT INTO spaceports ("name", star_system, "location") VALUES ('Гагарин-100', 'Солнце', 'Венера');
INSERT INTO spaceports ("name", star_system, "location") VALUES ('Луч', 'Арктур', 'Бафисто');
INSERT INTO spaceports ("name", star_system, "location") VALUES ('Ленин', 'Ригель', 'Ригель-1Л');
INSERT INTO spaceports ("name", star_system, "location") VALUES ('Союз', 'Капелла', 'Ляпис');
INSERT INTO spaceports ("name", star_system, "location") VALUES ('Заря', 'Сириус', 'Сириус-Бета');

INSERT INTO spaceships ("name", seats_number) VALUES ('Восток', 50);
INSERT INTO spaceships ("name", seats_number) VALUES ('КСУ-27', 10);
INSERT INTO spaceships ("name", seats_number) VALUES ('КТУ-400', 150);
INSERT INTO spaceships ("name", seats_number) VALUES ('Союз', 25);
