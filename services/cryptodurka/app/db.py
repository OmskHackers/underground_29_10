import aiosqlite

global DB_CONN


async def insert_user(username, password):
    query = 'INSERT INTO users (username, password) VALUES (?, ?);'
    async with DB_CONN.execute(query, (username, password)) as cursor:
        user_id = cursor.lastrowid
    return user_id


async def insert_session(session_uuid, user_id):
    query = 'INSERT INTO sessions (session_uuid, user_id) VALUES (?, ?);'
    async with DB_CONN.execute(query, (session_uuid, user_id)) as cursor:
        session_id = cursor.lastrowid
    return session_id


async def insert_patient(about_me, user_id):
    query = 'INSERT INTO patients (about_me, user_id) VALUES (?, ?);'
    async with DB_CONN.execute(query, (about_me, user_id)) as cursor:
        patient_id = cursor.lastrowid
    return patient_id


async def insert_therapist(user_id):
    query = 'INSERT INTO therapists (user_id) VALUES (?);'
    async with DB_CONN.execute(query, (user_id,)) as cursor:
        therapist_id = cursor.lastrowid
    return therapist_id


async def get_user_by_username(username):
    query = 'SELECT * FROM users WHERE username = ?'
    async with DB_CONN.execute(query, (username,)) as cursor:
        user = await cursor.fetchone()
    return user


async def get_session_by_user_id(user_id):
    query = 'SELECT * FROM sessions WHERE user_id = ?'
    async with DB_CONN.execute(query, (user_id,)) as cursor:
        session = await cursor.fetchone()
    return session


async def get_user_id_by_session_uuid(session_uuid):
    query = 'SELECT user_id FROM sessions WHERE session_uuid = ?'
    async with DB_CONN.execute(query, (session_uuid,)) as cursor:
        user_id = await cursor.fetchone()
    return user_id[0]


async def get_patient_by_username(username):
    query = 'SELECT patients.* FROM patients JOIN users ON patients.user_id = users.id WHERE users.username = ?'
    async with DB_CONN.execute(query, (username,)) as cursor:
        patient = await cursor.fetchone()
    return patient


async def commit_transaction():
    await DB_CONN.commit()


async def connect_db():
    global DB_CONN
    DB_CONN = await aiosqlite.connect('data/sqlite.db')
