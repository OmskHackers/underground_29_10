from Crypto.Util.number import *

e = 65537
g = 3
p = 2**801 - 895


def decrypt_one(pt):
    a = getPrime(512)
    b = getPrime(512)

    n = a*b

    pt_long = bytes_to_long(pt)
    ct = pow(pt_long, e, n)

    aa = max(a, b)
    bb = min(a, b)

    secret = aa % bb

    return ct, secret, n


def decrypt_two(pt):
    x = bytes_to_long(pt)
    ct = pow(3, x, p)
    secret = (x**3 + 205*x**2 - 91*x) % p

    return ct, secret
