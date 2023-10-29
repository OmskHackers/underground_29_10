1 вулна: secret = max(p,q) % min(p,q). Поскольку кол-во бит одинаково, secret = p - q.
x^2 - secret * x - n = 0
x1,x2 = p,-q
далее дефолт RSA

2 вулна: заметим(можно и не замечать), что 2 и 4 коэффициенты по модулю p сравнимы с 205 и 0
решаем модулярное уравнение x^3 + 205*x^2 - 91*x - secret = 0

p = 2**801 - 895
K = GF(p)
F.<x> = PolynomialRing(K, implementation='NTL')
r = factor(1*x^3+205*x^2-91*x-secret)
if r[1][0].degree(x) == 2:
  print(p-r[0][0][0])
else
  print(p-r[2][0][0])
затем long to bytes
