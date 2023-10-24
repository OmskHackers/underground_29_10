import db
import uuid


async def register(tx, args):
    try:
        username = args[2]
        password = args[3]

        check_bad_size(tx, username, 100)
        check_bad_size(tx, password, 100)

        async with db.DB_CONN.execute("SELECT id FROM users WHERE username = ?", (username,)) as cursor:
            if await cursor.fetchone() is not None:
                tx.write("Человек с таким позывным уже с нами\n".encode('utf-8'))
                return

        async with db.DB_CONN.execute('insert into users (username, password) values (?,?);', (username, password)) as cursor:
            user_id = cursor.lastrowid

        session_uuid = str(uuid.uuid4())
        await db.DB_CONN.execute('insert into sessions (session_uuid, user_id) values (?,?);', (session_uuid, user_id))

        if args[1] == "therapist":
            await db.DB_CONN.execute('insert into therapists (user_id) values (?);', (user_id,))
        else:
            about_me = args[4]
            check_bad_size(tx, about_me, 200)
            await db.DB_CONN.execute('insert into patients (about_me, user_id) values (?, ?);', (about_me, user_id))

        await db.DB_CONN.commit()
        tx.write("Теперь ты один из нас\n".encode('utf-8'))

    except:
        tx.write("Ошибка при регистрации :(\n".encode('utf-8'))


def check_bad_size(tx, data, size):
    if len(data) >= size:
        tx.write("Будь более краток\n".encode('utf-8'))
        return False
    return True
