CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL,
    password TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS username_index ON users (username);

CREATE TABLE IF NOT EXISTS sessions (
    id INTEGER PRIMARY KEY,
    session_uuid TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS session_uuid_index ON sessions (session_uuid);

CREATE TABLE IF NOT EXISTS therapists (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS patients (
    id INTEGER PRIMARY KEY,
    about_me TEXT NOT NULL,
    pub_key TEXT NOT NULL,
    secret TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS recipes (
    id INTEGER PRIMARY KEY,
    text TEXT NOT NULL,
    secret TEXT NOT NULL,
    therapist_id INTEGER NOT NULL,
    patient_id INTEGER NOT NULL,
    FOREIGN KEY (therapist_id) REFERENCES therapists(id),
    FOREIGN KEY (patient_id) REFERENCES patients(id)
);

