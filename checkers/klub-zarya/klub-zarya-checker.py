#!/usr/bin/env python3
from transliterate import translit
from faker import Faker
import random
import string
import requests
import sys

user = Faker(locale='ru_RU')

topics = [
    'Сбор средств на постройку ракеты', 
    'Установка нового памятника Горбачеву', 
    'Летов реально в тайге?', 
    'Пластмассовый мир не победит',
    'Погода на Марсе',
    'Как собрать квантовый генератор',
    'Как сделать радиоактивный чайный гриб',
    'Что такое жвачка? Капиталистическая фигня',
    'Когда США наконец развалится? Устал ждать...',
    'Когда СССР полетит в центр млечного пути?',
    'Тайны базы Союза на Венере',
    'Генсек КПСС в тайне пьёт Кока-Колу!'
]

def generator(size=12, chars=string.digits + string.ascii_letters):
    return ''.join(random.choice(chars) for _ in range(size))

def gen_user():
    return translit(user.unique.last_name().replace('ь', 'b') + user.unique.first_name().replace('ь', 'b'), reversed=True) + generator(6)


OK, CORRUPT, MUMBLE, DOWN = 101, 102, 103, 104
port = 7000

def close(code):
    print('Exit with code {}'.format(code), file=sys.stderr)
    exit(code)

cmd = sys.argv[1]

def check():
    ip = sys.argv[2]
    url = f'http://{ip}:{port}'
    try:
        requests.get(url + '/topics?page=0')
        close(OK)
    except Exception as e:
        print(str(e))
        close(DOWN)

def get():
    ip = sys.argv[2]
    flag_id = sys.argv[3] 
    flag = sys.argv[4]

    url = f'http://{ip}:{port}'
    creds = flag_id.split(':')
    username = creds[0]
    password = creds[1]

    try:
        s = requests.Session()

        res = s.post(url + '/login', json={
            'username': username,
            'password': password
        })
        if 'message' in res.text:
            s.close()
            print(res.text)
            close(CORRUPT)
        
        token = res.json()['token']
        user_id = res.json()['userId']

        res = s.get(url + '/friends?page=0', headers={
            'Authorization': 'Bearer ' + token
        })
        if 'message' in res.text:
            s.close()
            print(res.text)
            close(CORRUPT)

        for id in res.json()['friends']:
            res = s.post(url + f'/friends/{id}', headers={
                'Authorization': 'Bearer ' + token
            })
            if 'message' in res.text:
                s.close()
                print(res.text)
                close(CORRUPT)

        res = s.get(url + f'/topics/{user_id}?page=0', headers={
            'Authorization': 'Bearer ' + token
        })
        if 'message' in res.text:
            s.close()
            print(res.text)
            close(CORRUPT)

        for topic in res.json()['topics']:
            if not flag in topic['description']:
                s.close()
                print('flag not found in topic')
                close(CORRUPT)

        s.close()
        close(OK)
    except Exception as e:
        print(str(e))
        close(MUMBLE)

def put():
    ip = sys.argv[2]
    url = f'http://{ip}:{port}'

    username = gen_user()
    password = generator()

    flag_id = f'{username}:{password}' 
    flag = sys.argv[4]

    try:
        s = requests.Session()

        res = s.post(url + '/register', json={
            'username': username,
            'password': password
        })
        if 'message' in res.text:
            s.close()
            print(res.text)
            close(CORRUPT)

        res = s.get(url + '/users')
        if 'message' in res.text:
            s.close()
            print(res.text)
            close(CORRUPT)

        if not username in res.text:
            s.close()
            print('user not found in /users')
            close(CORRUPT)

        res = s.post(url + '/login', json={
            'username': username,
            'password': password
        })
        
        if 'message' in res.text:
            s.close()
            print(res.text)
            close(CORRUPT)
        
        token = res.json()['token']
        user_id = res.json()['userId']

        if user_id > 1:
            res = s.post(url + f'/friends?targetUserId={user_id - 1}', headers={
                'Authorization': 'Bearer ' + token
            })
            if 'message' in res.text:
                s.close()
                print(res.text)
                close(CORRUPT)

        res = s.post(url + f'/topics', headers={
            'Authorization': 'Bearer ' + token
        }, json={
            'theme': topics[random.randint(0, len(topics) - 1)],
            'description': flag,
            'isPublic': False
        })
        if 'message' in res.text:
            s.close()
            print(res.text)
            close(CORRUPT)

        s.close()
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