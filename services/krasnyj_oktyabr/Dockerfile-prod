FROM tatsushid/tinycore:6.0-x86_64

USER root

RUN adduser -D -h /home/user -s /bin/sh user
WORKDIR /home/user
COPY main /home/user/main

RUN chmod 555 /home/user/main 

USER user
WORKDIR /home/user

CMD ./main