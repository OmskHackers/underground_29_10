#!/usr/bin/env python
# -*- coding: utf-8 -*-
import sys
import socket
import string, random, uuid, re
OK, CORRUPT, MUMBLE, DOWN = 101, 102, 103, 104

cmd = sys.argv[1]
port = 8080

def close(code):
    print('Exit with code {}'.format(code), file=sys.stderr)
    exit(code)

def generator(size=12, chars=string.digits + string.ascii_letters):
    return ''.join(random.choice(chars) for _ in range(size))

def receive_and_send_data(host, port, message, buffer_size=1024, timeout=5):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.settimeout(timeout)
        # Подключаемся к серверу
        s.connect((host, port))

        # Сначала получаем приветствие от сервера
        greeting = s.recv(buffer_size).decode()
        #print("Server says:", greeting)

        # Теперь отправляем наше сообщение
        s.sendall(message.encode())

        # Получаем ответ
        response = s.recv(buffer_size)

        # Возвращаем данные в виде строки
        return response.decode()

def check_port(host, port, timeout=5):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.settimeout(timeout)
        try:
            s.connect((host, port))
            return True
        except socket.timeout:
            print(f"Timed out after {timeout} seconds.")
            return False
        except socket.error as err:
            print(f"Connection error: {err}")
            return False

def check():
    ip = sys.argv[2]
    if check_port(ip,port):
        close(OK)
    else:
        close(DOWN)

def put():
    ip = sys.argv[2]
    flag = sys.argv[4]
    username = generator()
    password = generator()

    response = ''
    try:
        response = receive_and_send_data(ip, port,'/REGISTER {} {}'.format(username, password))
        md5_pattern = re.compile(r'\b[a-f0-9]{32}\b')
        md5_hash = md5_pattern.search(response).group(0)
        print(md5_hash)
        response = receive_and_send_data(ip, port, '/BAKE {} {}'.format(md5_hash, flag))
    except:
        close(CORRUPT)

    if 'ОК' in response:
        close(OK)
    else:
        close(CORRUPT)


def get():
    ip = sys.argv[2]
    flag_id = sys.argv[3]
    flag = sys.argv[4]
    response = ''
    try:
        response = receive_and_send_data(ip, port, '/GET {}'.format(flag_id))
    except:
        close(CORRUPT)

    if flag in response:
        close(OK)
    else:
        close(CORRUPT)


if __name__ == "__main__":
    if cmd == 'check':
        check()
    elif cmd == 'get':
        get()
    elif cmd == 'put':
        put()
    elif cmd == 'info':
        print('1')
