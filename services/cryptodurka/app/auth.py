from db import *
from crypto import encrypt_hihi
import uuid

MAX_SIZE_LOG_PASS = 100
MAX_SIZE_ABOUT_ME = 100


async def register(tx, args):
    try:
        role = args[1]
        username = args[2]
        password = args[3]

        if role not in ['patient', 'therapist'] or (role == 'patient' and len(args) != 5):
            tx.write("Не забывай выбрать правильную роль\n".encode('utf-8'))
            return

        if is_bad_size(tx, username, MAX_SIZE_LOG_PASS) or is_bad_size(tx, password, MAX_SIZE_LOG_PASS):
            return

        if await get_user_by_username(username) is not None:
            tx.write("Человек с таким позывным уже с нами\n".encode('utf-8'))
            return

        user_id = await insert_user(username, password)
        session_uuid = str(uuid.uuid4())
        await insert_session(session_uuid, user_id)

        if role == "therapist":
            await insert_therapist(user_id)
        else:
            about_me = args[4]
            if is_bad_size(tx, about_me, MAX_SIZE_ABOUT_ME):
                return

            ct, secret, n, e = encrypt_hihi(about_me.encode('utf-8'))
            pub_key = f"n={n},e={e}"

            await insert_patient(str(ct), str(pub_key), str(secret), user_id)

        await commit_transaction()
        tx.write("Теперь ты один из нас\n".encode('utf-8'))

    except:
        tx.write(f"Ошибка при регистрации :(\n".encode('utf-8'))


async def login(tx, args):
    try:
        username = args[1]
        password = args[2]

        user = await get_user_by_username(username)
        user_id, user_password = user[0], user[2]

        if user is None or password != user_password:
            tx.write("Неправильный позывной или пароль\n".encode('utf-8'))
            return

        session = await get_session_by_user_id(user_id)
        tx.write(f"С возвращением в дурку! Твоя сессия: {session[1]}\n".encode('utf-8'))

    except:
        tx.write(f"Ошибка при входе :(\n".encode('utf-8'))
