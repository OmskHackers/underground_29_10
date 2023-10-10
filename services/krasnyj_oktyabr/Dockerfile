# Используем базовый образ TinyCore
FROM tatsushid/tinycore:6.0-x86_64

USER root


RUN adduser -D -h /home/user -s /bin/sh user
WORKDIR /home/user
ADD main /home/user/main

RUN chmod +x /home/user/main

USER user
WORKDIR /home/user

RUN ./main

