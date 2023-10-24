import aiosqlite

global DB_CONN


async def connect_db():
    global DB_CONN
    DB_CONN = await aiosqlite.connect('data/sqlite.db')
