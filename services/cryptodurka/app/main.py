import asyncio
from auth import register, login
from db import connect_db
from hihihaha import learn_about_bro, check_recipes, get_list_bro, get_params, write_recipe


class GlobalHandler(object):
    def __init__(self, rx, tx):
        self.rx = rx
        self.tx = tx

    async def run(self):
        self.tx.write("Добро пожаловать в дурку. Снова. Выбирай сторону: пациент или карательный психиатр. "
                      "Можешь рассказать нам о своих секретах, мы их бережно храним\n".encode('utf-8'))
        await self.tx.drain()
        while True:
            try:
                line = await self.rx.readline()

                input_data = line.strip()
                if not input_data:
                    break

                await self.process_command(input_data)
                await self.tx.drain()

            except:
                self.tx.write(f"Что-то пошло не так :( \n".encode('utf-8'))
                break

    async def process_command(self, input_data):
        args = input_data.decode().split(' ')
        command = args[0]

        if command == "register":
            if len(args) < 4:
                self.tx.write("Нужно выбрать одну из ролей\n".encode('utf-8'))
                return
            await register(self.tx, args)

        elif command == "login":
            if len(args) != 3:
                self.tx.write("Укажи свой позывной и пароль\n".encode('utf-8'))
                return
            await login(self.tx, args)

        elif command == "learn_about_bro":
            if len(args) != 3:
                self.tx.write("Укажи свою сессию и позывной братана, о котором хочешь побольше узнать\n".encode('utf-8'))
                return
            await learn_about_bro(self.tx, args)

        elif command == "write_recipe":
            if len(args) != 4:
                self.tx.write(
                    "Укажи свою сессию, позывной братана, и что ему хочешь выписать\n".encode('utf-8'))
                return
            await write_recipe(self.tx, args)

        elif command == "check_recipes":
            if len(args) != 3:
                self.tx.write("Укажи свой сессию и позывной братана, чтобы узнать инфу о назначенном лечении\n".encode('utf-8'))
                return
            await check_recipes(self.tx, args)

        elif command == "get_list_bro":
            if len(args) != 2:
                self.tx.write("Не забывай представиться. Такое доступно только своим\n".encode('utf-8'))
                return
            await get_list_bro(self.tx, args)

        elif command == "get_params":
            if len(args) != 2:
                self.tx.write("Только. Своим. Только.\n".encode('utf-8'))
                return
            await get_params(self.tx, args)

        else:
            self.tx.write("Такую услугу не оказываем\n".encode('utf-8'))


async def handle_connection(reader, writer):
    s = GlobalHandler(reader, writer)
    await s.run()
    writer.close()
    await writer.wait_closed()


async def main():
    await connect_db()

    server = await asyncio.start_server(handle_connection, host="0.0.0.0", port="6666")

    async with server:
        await server.serve_forever()


if __name__ == "__main__":
    asyncio.run(main())
