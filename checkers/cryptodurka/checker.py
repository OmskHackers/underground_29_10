#!/usr/bin/env python3

import random
import string
import re
import hashlib
from Crypto.Util.number import *


import sys
original_argv = sys.argv.copy()
sys.argv = [arg for arg in sys.argv if not arg.startswith("--custom-arg")]
from pwn import *
sys.argv = original_argv


OK, CORRUPT, MUMBLE, DOWN = 101, 102, 103, 104
port = 6666
context.log_level = 'error'

def generator(size=12, chars=string.digits + string.ascii_letters):
    return ''.join(random.choice(chars) for _ in range(size))


def send_msg(socket, msg) -> str:
    socket.send(f'{msg}\n'.encode())
    return socket.recvline().decode('utf8')


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
    skip = sys.argv[4]
    vuln = int(sys.argv[5])

    if vuln == 1:
        try:
            s = remote(ip, port)
            s.settimeout(5)
            s.recvline()

            params = flag_id.split(':')
            therapist_session, username_patient, enc_flag = params[0], params[1], params[2]

            res = send_msg(s, f"get_list_bro {therapist_session}")

            if not username_patient in res:
                print('cannot get patients')
                s.close()
                close(CORRUPT)

            res = send_msg(s, f"check_recipes {therapist_session} {username_patient}")

            if not enc_flag in res:
                print('wrong check flag')
                s.close()
                close(CORRUPT)

            s.close()
            close(OK)

        except Exception as e:
            print(str(e))
            close(MUMBLE)
    elif vuln == 2:
        try:
            s = remote(ip, port)
            s.settimeout(5)
            s.recvline()

            params = flag_id.split(':')
            patient_session, username_patient, enc_flag = params[0], params[1], params[2]

            res = send_msg(s, f"get_list_bro {patient_session}")

            if not username_patient in res:
                print('cannot get patients')
                s.close()
                close(CORRUPT)

            res = send_msg(s, f"learn_about_bro {patient_session} {username_patient}")

            n_match = re.search(r'n=(\d+)', res)
            e_match = re.search(r',e=(\d+)', res)
            about_me_match = re.search(r'about_me=(\d+)', res)
            n = int(n_match.group(1)) if n_match else None
            e = int(e_match.group(1)) if e_match else None
            about_me = int(about_me_match.group(1)) if about_me_match else None
            x = bytes_to_long(enc_flag.encode())

            if pow(x,e,n) != about_me:
                print('wrong check flag')
                s.close()
                close(CORRUPT)

            s.close()
            close(OK)

        except Exception as e:
            print(str(e))
            close(MUMBLE)


def put():
    ip = sys.argv[2]
    skip = sys.argv[3]
    flag = sys.argv[4]
    vuln = int(sys.argv[5])

    username_patient, password_patient = generator(), generator()

    if vuln == 1:
        username_therapist, password_therapist = generator(), generator()
        try:
            s = remote(ip, port)
            s.settimeout(5)
            s.recvline()

            res = send_msg(s, f"register therapist {username_therapist} {password_therapist}")
            if not 'один из' in res:
                print('cannot register')
                s.close()
                close(CORRUPT)

            res = send_msg(s, f"login {username_therapist} {password_therapist}")
            if not 'сессия: ' in res:
                print('cannot login')
                s.close()
                close(CORRUPT)

            therapist_session = res.split('сессия: ')[1].split("\n")[0]

            about_me = generator()
            res = send_msg(s, f"register patient {username_patient} {password_patient} {about_me}")
            if not 'один из' in res:
                print('cannot register')
                s.close()
                close(CORRUPT)

            res = send_msg(s, f"get_params {therapist_session}")
            coefficients_match = re.search(r'coefficients=\[(.*?)\]', res)
            p_match = re.search(r'p=(\d+)', res)
            coefficients_str = coefficients_match.group(1) if coefficients_match else None
            p = int(p_match.group(1)) if coefficients_match else None
            coefficients = [int(x) for x in coefficients_str.split(',')] if coefficients_str else []
            try:
                res = send_msg(s, f"write_recipe {therapist_session} {username_patient} {flag}")
                if not 'выписаны' in res:
                    print('cannot write recipe')
                    s.close()
                    close(CORRUPT)
            except:
                print('cannot write recipe')
                s.close()
                close(CORRUPT)

            x = bytes_to_long(flag.encode())
            encrypt = (x ** 3 * coefficients[0] + x ** 2 * coefficients[1] + x ** 1 * coefficients[2] + coefficients[
                    3]) % p
            flag_id = f"{therapist_session}:{username_patient}:{encrypt}"
            print(flag_id)
            close(OK)

        except Exception as e:
            print(str(e))
            close(MUMBLE)

    elif vuln == 2:
        try:
            s = remote(ip, port)
            s.settimeout(5)
            s.recvline()

            res = send_msg(s, f"register patient {username_patient} {password_patient} {flag}")
            if not 'один из' in res:
                print('cannot register')
                s.close()
                close(CORRUPT)

            res = send_msg(s, f"login {username_patient} {password_patient}")
            if not 'сессия: ' in res:
                print('cannot login')
                s.close()
                close(CORRUPT)

            patient_session = res.split('сессия: ')[1].split("\n")[0]
            flag_id = f"{patient_session}:{username_patient}:{flag}"
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
        print("vulns: 1:1", file=sys.stdout)
        close(OK)

