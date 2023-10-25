from db import *
from crypto import encrypt_haha, get_parameters

MAX_SIZE_RECIPE = 100


async def learn_about_bro(tx, args):
    try:
        session_uuid = args[1]
        bro_username = args[2]

        if await get_user_id_by_session_uuid(session_uuid) is None:
            tx.write("Ты не один из нас\n".encode('utf-8'))
            return

        patient = await get_patient_by_username(bro_username)
        if patient is None:
            tx.write("Такого бро у нас нет\n".encode('utf-8'))
            return

        tx.write(f"about_me={patient[1]},pub_key:{patient[2]},secret={patient[3]}\n".encode('utf-8'))

    except:
        tx.write(f"Ошибка при получении инфы о бро :(\n".encode('utf-8'))


async def write_recipe(tx, args):
    try:
        session_uuid = args[1]
        bro_username = args[2]
        recipe_text = args[3]

        if is_bad_size(tx, recipe_text, MAX_SIZE_RECIPE):
            return

        user_id = await get_user_id_by_session_uuid(session_uuid)
        if user_id is None:
            tx.write("Ты не один из нас\n".encode('utf-8'))
            return

        therapist = await get_therapist_by_user_id(user_id[0])
        if therapist is None:
            tx.write("На тебе должен быть колпак\n".encode('utf-8'))
            return

        patient = await get_patient_by_username(bro_username)
        if patient is None:
            tx.write("Такого бро у нас нет\n".encode('utf-8'))
            return

        if len(await get_recipes_by_therapist_id_and_patient_id(therapist[0], patient[0])) != 0:
            tx.write("Ты уже выписал ему колёса. Хватит\n".encode('utf-8'))
            return

        ct, secret = encrypt_haha(recipe_text.encode('utf-8'))

        await insert_recipe(str(ct), str(secret), therapist[0], patient[0])
        await commit_transaction()
        tx.write(f"Колёса выписаны успешно\n".encode('utf-8'))

    except:
        tx.write(f"Ошибка при выписывании колёс :(\n".encode('utf-8'))


async def check_recipes(tx, args):
    try:
        session_uuid = args[1]
        bro_username = args[2]

        user_id = await get_user_id_by_session_uuid(session_uuid)
        if user_id is None:
            tx.write("Ты не один из нас\n".encode('utf-8'))
            return

        therapist = await get_therapist_by_user_id(user_id[0])
        if therapist is None:
            tx.write("На тебе должен быть колпак\n".encode('utf-8'))
            return

        patient = await get_patient_by_username(bro_username)
        if patient is None:
            tx.write("Такого бро у нас нет\n".encode('utf-8'))
            return

        recipes = await get_recipes_by_patient_id(patient[0])
        tx.write(f"Выписанные колёса: {recipes}\n".encode('utf-8'))

    except:
        tx.write(f"Ошибка при получении рецептов бро :(\n".encode('utf-8'))


async def get_list_bro(tx, args):
    try:
        session_uuid = args[1]

        user_id = await get_user_id_by_session_uuid(session_uuid)
        if user_id is None:
            tx.write("Ты не один из нас\n".encode('utf-8'))
            return

        patients_username = await get_last_1000_username_patients()
        tx.write(f"Список братишек: {patients_username}\n".encode('utf-8'))

    except:
        tx.write(f"Ошибка при получении братишек :(\n".encode('utf-8'))


async def get_params(tx, args):
    try:
        session_uuid = args[1]

        user_id = await get_user_id_by_session_uuid(session_uuid)
        if user_id is None:
            tx.write("Ты не один из нас\n".encode('utf-8'))
            return

        tx.write(get_parameters().encode('utf-8'))

    except:
        tx.write(f"Ошибка при получении параметров :(\n".encode('utf-8'))
