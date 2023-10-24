import asyncio
from auth import register
from db import connect_db


class GlobalHandler(object):
    def __init__(self, rx, tx):
        self.rx = rx
        self.tx = tx

    async def run(self):
        self.tx.write("Добро пожаловать в дурку. Снова. Выбирай сторону: пациент или карательный психиатр\n".encode('utf-8'))
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
            role = args[1]
            if role not in ['patient', 'therapist']:
                self.tx.write("Нужно выбрать одну из ролей\n".encode('utf-8'))
                return
            await register(self.tx, args)
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
