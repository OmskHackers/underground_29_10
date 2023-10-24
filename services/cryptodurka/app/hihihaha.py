from db import *

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

        tx.write(f"{patient[1]}\n".encode('utf-8'))

    except:
        tx.write(f"Ошибка при получении инфы о бро :(\n".encode('utf-8'))


async def write_recipe(tx, args):
    return


async def check_recipe(tx, args):
    return


async def get_list_bro(tx, args):
    return


async def get_params(tx, args):
    return
