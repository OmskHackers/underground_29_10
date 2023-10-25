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


async def insert_patient(about_me, pub_key, secret, user_id):
    query = 'INSERT INTO patients (about_me, pub_key, secret, user_id) VALUES (?, ?, ?, ?);'
    async with DB_CONN.execute(query, (about_me, pub_key, secret, user_id)) as cursor:
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
    return user_id


async def get_patient_by_username(username):
    query = 'SELECT patients.* FROM patients JOIN users ON patients.user_id = users.id WHERE users.username = ?'
    async with DB_CONN.execute(query, (username,)) as cursor:
        patient = await cursor.fetchone()
    return patient


async def get_therapist_by_user_id(user_id):
    query = 'SELECT * FROM therapists WHERE user_id = ?'
    async with DB_CONN.execute(query, (user_id,)) as cursor:
        therapist = await cursor.fetchone()
    return therapist


async def insert_recipe(text, secret, therapist_id, patient_id):
    query = 'INSERT INTO recipes (text, secret, therapist_id, patient_id) VALUES (?, ?, ?, ?);'
    async with DB_CONN.execute(query, (text, secret, therapist_id, patient_id)) as cursor:
        recipe_id = cursor.lastrowid
    return recipe_id


async def get_recipes_by_patient_id(patient_id):
    query = 'SELECT text, secret FROM recipes WHERE patient_id = ?'
    async with DB_CONN.execute(query, (patient_id,)) as cursor:
        recipes = await cursor.fetchall()
    return recipes


async def get_recipes_by_therapist_id_and_patient_id(therapist_id, patient_id):
    query = 'SELECT text, secret FROM recipes WHERE therapist_id = ? AND patient_id = ?'
    async with DB_CONN.execute(query, (therapist_id, patient_id)) as cursor:
        recipes = await cursor.fetchall()
    return recipes


async def get_last_1000_username_patients():
    query = 'SELECT u.username FROM patients AS p JOIN users AS u ON p.user_id = u.id;'
    async with DB_CONN.execute(query) as cursor:
        username_patients = await cursor.fetchall()
    return username_patients


def is_bad_size(tx, data, size):
    if len(data) >= size:
        tx.write("Будь более краток\n".encode('utf-8'))
        return True
    return False


async def commit_transaction():
    await DB_CONN.commit()


async def connect_db():
    global DB_CONN
    DB_CONN = await aiosqlite.connect('data/sqlite.db')
