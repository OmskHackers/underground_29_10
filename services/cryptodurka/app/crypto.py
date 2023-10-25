from Crypto.Util.number import *

p = 2**801 - 895
e = 65537
g = 3
coefficients = [1,
                13336028865759708548159703581442515594289516644631816320792515623528074475635264143042864401743108581485859821186866480891777603308238730160726712104661660092190315159028029116926156571823628049457930032271773203963381496074952922582327754062,
                -91,
                53344115463038834192638814325770062377158066578527265283170062494112297902541056572171457606972434325943439284747465923567110413232954920642906848418646640368761260636112116467704626287294512197831720129087092815853525984299811690329311015428]


def encrypt_hihi(pt):
    a = getPrime(512)
    b = getPrime(512)

    n = a*b

    pt_long = bytes_to_long(pt)
    ct = pow(pt_long, e, n)

    aa = max(a, b)
    bb = min(a, b)

    secret = aa % bb

    return ct, secret, n, e


def encrypt_haha(pt):
    x = bytes_to_long(pt)
    ct = pow(g, x, p)
    secret = ((x**3) * coefficients[0] + (x**2) * coefficients[1] + (x**1) * coefficients[2] + (x**0) * coefficients[3]) % p

    return ct, secret


def get_parameters():
    return f"p={p},g={g},coefficients={coefficients}\n"
