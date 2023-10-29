FROM ubuntu:22.04

RUN apt-get update && apt-get -y upgrade
RUN apt-get install -y python3-pip

RUN apt-get install -y sqlite

RUN python3.10 -m pip install Flask
RUN python3.10 -m pip install WTForms
RUN python3.10 -m pip install flask-wtf
RUN python3.10 -m pip install bootstrap-flask

RUN mkdir /service
WORKDIR /service
RUN mkdir data

COPY . ./

RUN groupadd -g 2000 zapiskigroup\
&& useradd -m -u 2001 -g zapiskigroup pipiski

RUN chown -R pipiski /service

RUN chmod +x zapiski.py run.sh

CMD ["/service/run.sh"]
