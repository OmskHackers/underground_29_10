import sqlite3

_connection = None

def _getConnection() -> sqlite3.Connection: 
    global _connection
    if not _connection:
        _connection = sqlite3.connect('zapiski.db')
        _connection.row_factory = sqlite3.Row
    return _connection # maybe this could cause collisions

def insertIntoDatabase(encryptedMessage : str) -> int | None:
    conn = _getConnection()
    cur = conn.cursor()
    cur.execute("INSERT INTO `zapiska` (encrypted_message) VALUES (?);", (encryptedMessage, ))
    id = cur.lastrowid
    conn.commit()
    cur.close()
    if id is None:
        return None
    return id

def getHighestId() -> int | None:
    conn = _getConnection()
    cur = conn.cursor()
    cur.execute("SELECT MAX(`id`) FROM `zapiska`;")
    fetchedValue = cur.fetchone()
    if fetchedValue is None:
        return None
    highestId = fetchedValue[0]
    return highestId

def getEncryptedMessageById(id : int) -> str | None:
    conn = _getConnection()
    cur = conn.cursor()
    cur.execute("SELECT * FROM `zapiska` WHERE `id` = ?", (id, ))
    fetchedValue = cur.fetchone()
    cur.close()
    if fetchedValue is None:
        return None
    return fetchedValue['encrypted_message']