# Уязвимости сервиса красный октябрь

- RCE 

смотрите rce.py

- внедрение NULL Byte в GET

```python
s.sendall(b'/GET \x00\x00')
