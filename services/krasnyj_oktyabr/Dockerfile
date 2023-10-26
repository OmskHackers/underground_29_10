FROM cameronbrunner/tinycorelinux-extensions:latest

ENV LANG=C.UTF-8 LC_ALL=C

RUN tce-load -wi gcc.tcz
RUN tce-load -wi compiletc.tcz

USER root

WORKDIR /tmp
COPY hiredis-1.0.0/ /tmp/hiredis-1.0.0/
WORKDIR /tmp/hiredis-1.0.0
RUN make
RUN make install

COPY openssl-1.1.1k/ /tmp/openssl-1.1.1k
WORKDIR /tmp/openssl-1.1.1k
RUN ./config no-shared
RUN make

WORKDIR /src

ADD main.c /src/main.c 
RUN gcc -Wl,-z,norelro -g -fno-stack-protector -O0 -z execstack -o main main.c -pthread -static-libgcc -I/tmp/openssl-1.1.1k/include -L/tmp/openssl-1.1.1k -Wl,-Bstatic -lhiredis -lcrypto -lssl -Wl,-Bdynamic

CMD ./main
