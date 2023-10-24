#!/usr/bin/env python3
from transliterate import translit
from faker import Faker
import random
from pwn import *

user = Faker(locale='ru_RU')

def generator(size=12, chars=string.digits + string.ascii_letters):
    return ''.join(random.choice(chars) for _ in range(size))

def gen_user():
    return translit(user.unique.last_name().replace('ь', 'b') + user.unique.first_name().replace('ь', 'b'), reversed=True) + generator(6)

def send_msg(s, msg: str) -> str:
    s.send(f'{msg}\n')
    return s.recvrepeat(timeout=1).decode('utf8')

OK, CORRUPT, MUMBLE, DOWN = 101, 102, 103, 104
port = 2067

def close(code):
    print('Exit with code {}'.format(code), file=sys.stderr)
    exit(code)

cmd = sys.argv[1]

def check():
    ip = sys.argv[2]
    try:
        s = remote(ip, port, timeout=5)
        s.close()
        close(OK)
    except Exception as e:
        print(str(e))
        close(DOWN)

def get():
    ip = sys.argv[2]
    flag_id = sys.argv[3] 
    flag = sys.argv[4]

    creds = flag_id.split(':')
    username = creds[0]
    password = creds[1]

    try:
        s = remote(ip, port)

        # login
        send_msg(s, '2')
        send_msg(s, username)
        if not 'вошли в систему' in send_msg(s, password):
            close(CORRUPT)

        try:
            order_id = int(send_msg(s, '3').split('\n\n')[0].split('\n')[1].split(' ')[1].split('-')[1].removesuffix(':'))
            send_msg(s, '4')
            res = send_msg(s, order_id)
            if not flag in res:
                raise Exception("flag not found in orders")
            send_msg(s, '6')
            s.close()
            close(OK)
        except Exception as e:
            print(str(e))
            close(CORRUPT)
        
    except Exception as e:
        print(str(e))
        close(MUMBLE)

def put():
    ip = sys.argv[2]

    username = gen_user()
    password = generator()

    flag_id = f'{username}:{password}' 
    flag = sys.argv[4]

    try:
        s = remote(ip, port)

        s.recvrepeat(timeout=1)

        # register
        send_msg(s, '3')
        send_msg(s, username)
        if not 'зарегистрированы' in send_msg(password):
            close(CORRUPT)

        # login
        send_msg(s, '2')
        send_msg(s, username)
        if not 'вошли в систему' in send_msg(s, password):
            close(CORRUPT)

        try:
            flight_table = send_msg(s, '1').split('\n\n')[0]
            flight_notes = flight_table.split('\n')
            
            rand_flight_note = random.randint(0, len(flight_notes) - 1)
            flight_id = int(rand_flight_note.strip().split('|')[1].split('-')[1])
            send_msg(s, '2')
            send_msg(s, flight_id)
            send_msg(s, '')
            send_msg(s, flag)
            send_msg(s, '6')
            s.close()
        except Exception as e:
            print(str(e))
            close(CORRUPT)

        print(flag_id)
        close(OK)
    except Exception as e:
        print(str(e))
        close(MUMBLE)

if __name__ == "__main__":
    if cmd == 'check':
        check()
    elif cmd == 'get':
        get()
    elif cmd == 'put':
        put()
    elif cmd == 'info':
        print('1') 
